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
        _ => panic!("GrammaticalFunctionCatalog can only be derived for enums"),
    };

    let schema_entries = variants.iter().map(|variant| {
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

        let fields = match &variant.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unit => panic!("GrammaticalFunctionCatalog requires named fields"),
            Fields::Unnamed(_) => {
                panic!("GrammaticalFunctionCatalog does not support tuple variants")
            }
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

    let expanded = quote! {
        impl panini_core::traits::GrammaticalFunctionCatalog for #name {
            fn function_descriptors() -> Vec<panini_core::traits::FunctionVariantSchema> {
                vec![#(#schema_entries),*]
            }
        }
    };

    TokenStream::from(expanded)
}
