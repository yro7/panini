use crate::helpers::{classify, is_option_type, FieldClass};
use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

// ─── MorphologyInfo derive ────────────────────────────────────────────────────

/// Generates the necessary boilerplate for a morphology defining enum:
/// 1. A `{Name}PosTag` enum containing all the variants (used for targeting specific pos tags).
/// 2. An implementation of `MorphologyInfo` returning the lemma, pos label, and pos tag.
/// 3. Getter methods for all unique aggregable fields (e.g. `pub fn gender(&self) -> Option<String>`).
/// 4. An implementation of `Aggregable` for statistical grouping and analysis.
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => {
            return syn::Error::new_spanned(name, "MorphologyInfo can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let has_serde_tag_pos = input.attrs.iter().any(|attr| {
        if !attr.path().is_ident("serde") {
            return false;
        }
        attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        )
        .map(|list| {
            list.iter().any(|meta| {
                if let syn::Meta::NameValue(nv) = meta {
                    if nv.path.is_ident("tag") {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(s),
                            ..
                        }) = &nv.value
                        {
                            return s.value() == "pos";
                        }
                    }
                }
                false
            })
        })
        .unwrap_or(false)
    });
    if !has_serde_tag_pos {
        return syn::Error::new_spanned(
            name,
            "MorphologyInfo: enum must have `#[serde(tag = \"pos\")]` for correct LLM deserialization",
        )
        .to_compile_error()
        .into();
    }

    // Single pass: validate each variant (named fields + a `lemma` field) and
    // collect its aggregable fields (non-lemma, non-Option) with their class.
    let mut variant_infos: Vec<VariantInfo> = Vec::new();
    for v in variants {
        let fields = match &v.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return syn::Error::new_spanned(
                    &v.ident,
                    format!(
                        "MorphologyInfo: variant `{}` must have named fields",
                        v.ident
                    ),
                )
                .to_compile_error()
                .into();
            }
        };

        let has_lemma = fields
            .iter()
            .any(|f| f.ident.as_ref().is_some_and(|id| id == "lemma"));
        if !has_lemma {
            return syn::Error::new_spanned(
                &v.ident,
                format!(
                    "MorphologyInfo: variant `{}` must have a named `lemma` field",
                    v.ident
                ),
            )
            .to_compile_error()
            .into();
        }

        let aggregable: Vec<(&syn::Field, FieldClass)> = fields
            .iter()
            .filter(|f| !is_option_type(&f.ty))
            .map(|f| (f, classify(&f.ty)))
            .collect();

        variant_infos.push(VariantInfo {
            ident: &v.ident,
            aggregable,
        });
    }

    let pos_tag_name = quote::format_ident!("{}PosTag", name);

    let ts_traits = generate_pos_tag_and_traits(name, &pos_tag_name, variants);
    let ts_getters = generate_field_getters(name, &variant_infos);
    let ts_aggregable = generate_aggregable_impl(name, &variant_infos);
    let ts_catalog = generate_catalog_impl(name, &variant_infos);
    let ts_pivots = match generate_pivot_fields(name, &variant_infos) {
        Ok(ts) => ts,
        Err(err) => return err.to_compile_error().into(),
    };

    let expanded = quote! {
        #ts_traits
        #ts_getters
        #ts_pivots
        #ts_aggregable
        #ts_catalog
    };

    TokenStream::from(expanded)
}

/// Generates a static schema catalog describing the morphology groups and their dimensions.
fn generate_catalog_impl(
    name: &syn::Ident,
    variant_infos: &[VariantInfo],
) -> proc_macro2::TokenStream {
    let schema_entries: Vec<proc_macro2::TokenStream> = variant_infos
        .iter()
        .map(|info| {
            let ident = info.ident;
            let label = ident.to_string();
            let key = label.to_snake_case();

            let descriptor_entries = info.aggregable.iter().map(|(f, class)| {
                let field_name = f.ident.as_ref().unwrap().to_string();
                let ty = &f.ty;
                match class {
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
                panini_core::traits::MorphologyGroupSchema {
                    key: #key.to_string(),
                    label: #label.to_string(),
                    dimensions: vec![#(#descriptor_entries,)*],
                }
            }
        })
        .collect();

    let mut closed_field_types: Vec<proc_macro2::TokenStream> = Vec::new();
    for info in variant_infos {
        for (f, class) in &info.aggregable {
            if matches!(class, FieldClass::Closed) {
                let ty = &f.ty;
                closed_field_types.push(quote! { #ty });
            }
        }
    }

    let mut seen = std::collections::HashSet::new();
    let unique_closed_types: Vec<_> = closed_field_types
        .into_iter()
        .filter(|t| seen.insert(t.to_string()))
        .collect();

    let closed_where_bounds = unique_closed_types.iter().map(|ty| {
        quote! { #ty: panini_core::aggregable::ClosedValues }
    });

    quote! {
        impl panini_core::traits::MorphologyCatalog for #name
        where
            #(#closed_where_bounds,)*
        {
            fn group_descriptors() -> Vec<panini_core::traits::MorphologyGroupSchema> {
                vec![#(#schema_entries),*]
            }
        }
    }
}

struct VariantInfo<'a> {
    ident: &'a syn::Ident,
    aggregable: Vec<(&'a syn::Field, FieldClass)>,
}

struct PivotFieldInfo<'a> {
    name: String,
    label: String,
    class: FieldClass,
    ty: &'a syn::Type,
}

fn field_label(name: &str) -> String {
    name.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn collect_pivot_fields<'a>(
    variant_infos: &'a [VariantInfo<'a>],
) -> Result<Vec<PivotFieldInfo<'a>>, syn::Error> {
    let mut fields: Vec<PivotFieldInfo<'a>> = Vec::new();

    for info in variant_infos {
        for (field, class) in &info.aggregable {
            let name = field.ident.as_ref().unwrap().to_string();
            if name == "lemma" {
                continue;
            }

            if let Some(existing) = fields.iter().find(|candidate| candidate.name == name) {
                let existing_ty = existing.ty;
                let current_ty = &field.ty;
                let same_type =
                    quote! { #existing_ty }.to_string() == quote! { #current_ty }.to_string();
                if existing.class != *class || !same_type {
                    return Err(syn::Error::new_spanned(
                        field,
                        format!(
                            "MorphologyInfo: field `{name}` appears with incompatible types across variants; use a single type for pivot generation"
                        ),
                    ));
                }
                continue;
            }

            fields.push(PivotFieldInfo {
                label: field_label(&name),
                name,
                class: *class,
                ty: &field.ty,
            });
        }
    }

    fields.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(fields)
}

fn generate_pivot_fields(
    name: &syn::Ident,
    variant_infos: &[VariantInfo],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let fields = collect_pivot_fields(variant_infos)?;

    let extractors = fields.iter().map(|field| {
        let field_name = &field.name;
        let fn_ident = quote::format_ident!("__pivot_{}", field_name);
        let arms = variant_infos.iter().map(|info| {
            let variant_ident = info.ident;
            let matching_field = info
                .aggregable
                .iter()
                .find(|(candidate, _)| candidate.ident.as_ref().unwrap() == field_name);

            if let Some((matching_field, class)) = matching_field {
                let field_ident = matching_field.ident.as_ref().unwrap();
                match class {
                    FieldClass::String => quote! {
                        Self::#variant_ident { #field_ident, .. } => Some(#field_ident.clone()),
                    },
                    FieldClass::Bool => quote! {
                        Self::#variant_ident { #field_ident, .. } => Some(#field_ident.to_string()),
                    },
                    FieldClass::Closed => quote! {
                        Self::#variant_ident { #field_ident, .. } => {
                            Some(panini_core::aggregable::ClosedValues::variant_str(#field_ident).to_string())
                        },
                    },
                }
            } else {
                quote! {
                    Self::#variant_ident { .. } => None,
                }
            }
        });

        quote! {
            fn #fn_ident(&self) -> Option<String> {
                match self {
                    #(#arms)*
                }
            }
        }
    });

    let constants = fields.iter().map(|field| {
        let const_ident = quote::format_ident!("PIVOT_{}", field.name.to_shouty_snake_case());
        let fn_ident = quote::format_ident!("__pivot_{}", field.name);
        let key = &field.name;
        let label = &field.label;
        let ty = field.ty;

        match field.class {
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
    });

    Ok(quote! {
        impl #name {
            #(#extractors)*
            #(#constants)*
        }
    })
}

/// Generates the `PosTag` enum representing isolated parts of speech,
/// and implements the `MorphologyInfo` core traits.
fn generate_pos_tag_and_traits(
    name: &syn::Ident,
    pos_tag_name: &syn::Ident,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> proc_macro2::TokenStream {
    let pos_tag_variants: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    let lemma_arms = variants.iter().map(|v| {
        let ident = &v.ident;
        quote! { Self::#ident { lemma, .. } => lemma, }
    });

    let pos_label_arms = variants.iter().map(|v| {
        let ident = &v.ident;
        let label = ident.to_string();
        quote! { Self::#ident { .. } => #label, }
    });

    let pos_tag_arms = variants.iter().map(|v| {
        let ident = &v.ident;
        quote! { Self::#ident { .. } => #pos_tag_name::#ident, }
    });

    quote! {
        /// Auto-generated POS tag enum for use in `MorphemeDefinition::applies_to`.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #pos_tag_name {
            #(#pos_tag_variants,)*
        }

        impl panini_core::traits::MorphologyInfo for #name {
            type PosTag = #pos_tag_name;

            fn lemma(&self) -> &str {
                match self {
                    #(#lemma_arms)*
                }
            }

            fn pos_tag(&self) -> #pos_tag_name {
                match self {
                    #(#pos_tag_arms)*
                }
            }

            fn pos_label(&self) -> &'static str {
                match self {
                    #(#pos_label_arms)*
                }
            }
        }
    }
}

/// Generates getter functions for every aggregable field found in the enum variants.
fn generate_field_getters(
    name: &syn::Ident,
    variant_infos: &[VariantInfo],
) -> proc_macro2::TokenStream {
    let mut all_fields = std::collections::HashSet::new();
    for info in variant_infos {
        for (f, _) in &info.aggregable {
            all_fields.insert(f.ident.as_ref().unwrap().to_string());
        }
    }

    let field_getters: Vec<_> = all_fields
        .into_iter()
        .map(|field_name| {
            let method_name = quote::format_ident!("{}", field_name);
            let arms = variant_infos.iter().map(|info| {
                let variant_ident = info.ident;
                let field = info
                    .aggregable
                    .iter()
                    .find(|(f, _)| f.ident.as_ref().unwrap() == &field_name);

                if let Some((f, class)) = field {
                    let field_ident = f.ident.as_ref().unwrap();
                    match class {
                        FieldClass::String => quote! {
                            Self::#variant_ident { #field_ident, .. } => Some(#field_ident.clone()),
                        },
                        FieldClass::Bool => quote! {
                            Self::#variant_ident { #field_ident, .. } => Some(#field_ident.to_string()),
                        },
                        FieldClass::Closed => quote! {
                            Self::#variant_ident { #field_ident, .. } => Some(panini_core::aggregable::ClosedValues::variant_str(#field_ident).to_string()),
                        },
                    }
                } else { quote! {
                    Self::#variant_ident { .. } => None,
                } }
            });

            quote! {
                pub fn #method_name(&self) -> Option<String> {
                    match self {
                        #(#arms)*
                    }
                }
            }
        })
        .collect();

    quote! {
        impl #name {
            #(#field_getters)*
        }
    }
}

/// Generates the `Aggregable` trait implementation so this morphology can be grouped and analyzed.
fn generate_aggregable_impl(
    name: &syn::Ident,
    variant_infos: &[VariantInfo],
) -> proc_macro2::TokenStream {
    let descriptor_arms: Vec<proc_macro2::TokenStream> = variant_infos
        .iter()
        .map(|info| {
            let ident = info.ident;
            let descriptor_entries = info.aggregable.iter().map(|(f, class)| {
                let field_name = f.ident.as_ref().unwrap().to_string();
                let ty = &f.ty;
                match class {
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
                            kind: panini_core::aggregable::FieldKind::Closed(<#ty as panini_core::aggregable::ClosedValues>::all_variants()),
                        }
                    },
                }
            });
            quote! {
                Self::#ident { .. } => vec![#(#descriptor_entries,)*],
            }
        })
        .collect();

    let mut closed_field_types: Vec<proc_macro2::TokenStream> = Vec::new();
    for info in variant_infos {
        for (f, class) in &info.aggregable {
            if matches!(class, FieldClass::Closed) {
                let ty = &f.ty;
                closed_field_types.push(quote! { #ty });
            }
        }
    }

    let mut seen = std::collections::HashSet::new();
    let unique_closed_types: Vec<_> = closed_field_types
        .into_iter()
        .filter(|t| seen.insert(t.to_string()))
        .collect();

    let closed_where_bounds = unique_closed_types.iter().map(|ty| {
        quote! { #ty: panini_core::aggregable::ClosedValues }
    });

    let observations_arms: Vec<proc_macro2::TokenStream> = variant_infos
        .iter()
        .map(|info| {
            let ident = info.ident;
            let field_idents: Vec<_> = info
                .aggregable
                .iter()
                .map(|(f, _)| f.ident.as_ref().unwrap())
                .collect();

            let obs_entries = info.aggregable.iter().map(|(f, class)| {
                let field_ident = f.ident.as_ref().unwrap();
                let field_name = field_ident.to_string();
                match class {
                    FieldClass::String => quote! { (#field_name.to_string(), #field_ident.clone()) },
                    FieldClass::Bool => quote! { (#field_name.to_string(), #field_ident.to_string()) },
                    FieldClass::Closed => quote! {
                        (#field_name.to_string(), panini_core::aggregable::ClosedValues::variant_str(#field_ident).to_string())
                    },
                }
            });

            let pattern = if field_idents.is_empty() {
                quote! { Self::#ident { .. } }
            } else {
                quote! { Self::#ident { #(#field_idents,)* .. } }
            };

            quote! {
                #pattern => vec![vec![#(#obs_entries,)*]],
            }
        })
        .collect();

    quote! {
        impl panini_core::aggregable::Aggregable for #name
        where
            #(#closed_where_bounds,)*
        {
            fn group_key(&self) -> String {
                panini_core::traits::MorphologyInfo::pos_label(self).to_string()
            }

            fn instance_descriptors(&self) -> Vec<panini_core::aggregable::FieldDescriptor> {
                match self {
                    #(#descriptor_arms)*
                }
            }

            fn observations(&self) -> Vec<Vec<(String, String)>> {
                match self {
                    #(#observations_arms)*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pivot_generation_rejects_incompatible_same_name_fields() {
        let input: DeriveInput = syn::parse_quote! {
            #[serde(tag = "pos")]
            enum BadMorphology {
                Noun { lemma: String, case: BadCase },
                Verb { lemma: String, case: String },
            }
        };
        let variants = match &input.data {
            Data::Enum(data_enum) => &data_enum.variants,
            _ => panic!("test input must be an enum"),
        };

        let variant_infos: Vec<VariantInfo<'_>> = variants
            .iter()
            .map(|variant| {
                let fields = match &variant.fields {
                    Fields::Named(fields) => &fields.named,
                    _ => panic!("test variants must have named fields"),
                };
                let aggregable = fields
                    .iter()
                    .filter(|field| !is_option_type(&field.ty))
                    .map(|field| (field, classify(&field.ty)))
                    .collect();

                VariantInfo {
                    ident: &variant.ident,
                    aggregable,
                }
            })
            .collect();

        let error = generate_pivot_fields(&input.ident, &variant_infos).unwrap_err();

        assert!(error
            .to_string()
            .contains("appears with incompatible types across variants"));
    }
}
