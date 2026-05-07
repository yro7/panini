use heck::ToShoutySnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

use crate::helpers::{
    classify, get_serde_value, pascal_to_snake_case, variant_serialized_name, FieldClass,
};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let rename_all = get_serde_value(&input.attrs, "rename_all");
    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => {
            return syn::Error::new_spanned(
                name,
                "GrammaticalFunctionCatalog can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    // Pre-validate: all variants must have named fields.
    for v in variants.iter() {
        match &v.fields {
            Fields::Named(_) => {}
            Fields::Unit => {
                return syn::Error::new_spanned(
                    &v.ident,
                    format!(
                        "GrammaticalFunctionCatalog: variant `{}` must have named fields (not unit)",
                        v.ident
                    ),
                )
                .to_compile_error()
                .into();
            }
            Fields::Unnamed(_) => {
                return syn::Error::new_spanned(
                    &v.ident,
                    format!(
                        "GrammaticalFunctionCatalog: variant `{}` must not be a tuple variant",
                        v.ident
                    ),
                )
                .to_compile_error()
                .into();
            }
        }
    }

    let variant_infos: Vec<_> = variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;
            let label = ident.to_string();
            let key = get_serde_value(&variant.attrs, "rename").unwrap_or_else(|| {
                let inferred = variant_serialized_name(&label, rename_all.as_ref());
                if inferred == label {
                    pascal_to_snake_case(&label)
                } else {
                    inferred
                }
            });
            (variant, key, label)
        })
        .collect();

    let schema_entries = variant_infos.iter().map(|(variant, key, label)| {
        let fields = match &variant.fields {
            Fields::Named(fields) => &fields.named,
            _ => unreachable!("non-named variants rejected in pre-validation"),
        };

        let dimensions = fields.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let ty = &field.ty;
            match classify(ty) {
                FieldClass::String => quote! {
                    panini_core::aggregable::FieldDescriptor {
                        name: #field_name.into(),
                        kind: panini_core::aggregable::FieldKind::Open,
                    }
                },
                FieldClass::Bool => quote! {
                    panini_core::aggregable::FieldDescriptor {
                        name: #field_name.into(),
                        kind: panini_core::aggregable::FieldKind::Closed(&["true", "false"]),
                    }
                },
                FieldClass::Closed => quote! {
                    panini_core::aggregable::FieldDescriptor {
                        name: #field_name.into(),
                        kind: panini_core::aggregable::FieldKind::Closed(
                            <#ty as panini_core::aggregable::ClosedValues>::all_variants()
                        ),
                    }
                },
            }
        });

        quote! {
            panini_core::traits::FunctionVariantSchema {
                key: #key.to_string(),
                label: #label.to_string(),
                dimensions: vec![#(#dimensions,)*],
            }
        }
    });

    let pivot_extractors = variant_infos.iter().map(|(variant, key, _label)| {
        let ident = &variant.ident;
        let fn_ident = quote::format_ident!("__pivot_{}", key);
        let fields = match &variant.fields {
            Fields::Named(fields) => &fields.named,
            _ => unreachable!("non-named variants rejected in pre-validation"),
        };
        let field_idents: Vec<_> = fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap())
            .collect();
        let value_parts: Vec<_> = fields
            .iter()
            .map(|field| {
                let field_ident = field.ident.as_ref().unwrap();
                match classify(&field.ty) {
                    FieldClass::String => quote! { #field_ident.clone() },
                    FieldClass::Bool => quote! { #field_ident.to_string() },
                    FieldClass::Closed => quote! {
                        panini_core::aggregable::ClosedValues::variant_str(#field_ident).to_string()
                    },
                }
            })
            .collect();

        let value_expr = if value_parts.len() == 1 {
            quote! { #(#value_parts)* }
        } else {
            let fmt = value_parts
                .iter()
                .map(|_| "{}")
                .collect::<Vec<_>>()
                .join("_");
            quote! { format!(#fmt, #(#value_parts),*) }
        };

        quote! {
            fn #fn_ident(&self) -> Option<String> {
                match self {
                    Self::#ident { #(#field_idents,)* } => Some(#value_expr),
                    _ => None,
                }
            }
        }
    });

    let pivot_constants = variant_infos.iter().map(|(variant, key, label)| {
        let const_ident = quote::format_ident!("PIVOT_{}", key.to_shouty_snake_case());
        let fn_ident = quote::format_ident!("__pivot_{}", key);
        let fields = match &variant.fields {
            Fields::Named(fields) => &fields.named,
            _ => unreachable!("non-named variants rejected in pre-validation"),
        };

        if fields.len() == 1 {
            let field = fields.iter().next().unwrap();
            let ty = &field.ty;
            match classify(ty) {
                FieldClass::String => quote! {
                    pub const #const_ident: panini_core::pivot::PivotField<Self> =
                        panini_core::pivot::PivotField::open(#key, #label, Self::#fn_ident);
                },
                FieldClass::Bool => quote! {
                    pub const #const_ident: panini_core::pivot::PivotField<Self> =
                        panini_core::pivot::PivotField::closed(
                            #key,
                            #label,
                            panini_core::pivot::bool_values,
                            Self::#fn_ident,
                        );
                },
                FieldClass::Closed => quote! {
                    pub const #const_ident: panini_core::pivot::PivotField<Self> =
                        panini_core::pivot::PivotField::closed(
                            #key,
                            #label,
                            <#ty as panini_core::aggregable::ClosedValues>::all_variants,
                            Self::#fn_ident,
                        );
                },
            }
        } else {
            quote! {
                pub const #const_ident: panini_core::pivot::PivotField<Self> =
                    panini_core::pivot::PivotField::open(#key, #label, Self::#fn_ident);
            }
        }
    });

    let expanded = quote! {
        impl panini_core::traits::GrammaticalFunctionCatalog for #name {
            fn function_descriptors() -> Vec<panini_core::traits::FunctionVariantSchema> {
                vec![#(#schema_entries),*]
            }
        }

        impl #name {
            #(#pivot_extractors)*
            #(#pivot_constants)*
        }
    };

    TokenStream::from(expanded)
}
