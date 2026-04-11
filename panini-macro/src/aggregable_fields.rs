use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use crate::helpers::*;

// ─── AggregableFields derive ──────────────────────────────────────────────────

/// Derive `AggregableFields` for internally-tagged enums (GrammaticalFunction style).
///
/// Requires `#[serde(tag = "...")]` on the enum. Generates two aggregate dimensions:
/// - `function_category` (Closed) — the serialized variant name
/// - `function_value` (Open) — inner field values joined by `_`
///
/// Non-String, non-bool fields are assumed to implement `ClosedValues`.
/// String fields are used as-is.
///
/// Optional `#[aggregable_fields(crate = "...")]` overrides the `panini_core` path.
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let krate = get_crate_path(&input.attrs, "aggregable_fields");

    // Validate that a serde tag is present (required for the internally-tagged pattern).
    if get_serde_value(&input.attrs, "tag").is_none() {
        panic!("AggregableFields: #[serde(tag = \"...\")] is required on {}", name);
    }

    let rename_all = get_serde_value(&input.attrs, "rename_all");

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("AggregableFields can only be derived for enums"),
    };

    let serialized_names: Vec<String> = variants
        .iter()
        .map(|v| {
            get_serde_value(&v.attrs, "rename")
                .unwrap_or_else(|| variant_serialized_name(&v.ident.to_string(), &rename_all))
        })
        .collect();

    let field_value_arms = variants.iter().zip(serialized_names.iter()).map(|(v, cat_str)| {
        let ident = &v.ident;
        let fields = match &v.fields {
            Fields::Named(f) => &f.named,
            Fields::Unit => {
                return quote! {
                    Self::#ident => (#cat_str.to_string(), #cat_str.to_string()),
                };
            }
            _ => panic!("AggregableFields: tuple variants are not supported"),
        };

        let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

        let value_parts: Vec<_> = fields
            .iter()
            .map(|f| {
                let fi = f.ident.as_ref().unwrap();
                match classify(&f.ty) {
                    FieldClass::String => quote! { #fi.clone() },
                    FieldClass::Bool => quote! { #fi.to_string() },
                    FieldClass::Closed => quote! {
                        #krate::aggregable::ClosedValues::variant_str(#fi).to_string()
                    },
                }
            })
            .collect();

        let value_expr = if value_parts.len() == 1 {
            quote! { #(#value_parts)* }
        } else {
            let fmt_str = value_parts.iter().map(|_| "{}").collect::<Vec<_>>().join("_");
            quote! { format!(#fmt_str, #(#value_parts),*) }
        };

        quote! {
            Self::#ident { #(#field_idents,)* } => (#cat_str.to_string(), #value_expr),
        }
    });

    let expanded = quote! {
        impl #krate::aggregable::AggregableFields for #name {
            fn descriptors() -> Vec<#krate::aggregable::FieldDescriptor> {
                vec![
                    #krate::aggregable::FieldDescriptor {
                        name: "function_category".into(),
                        kind: #krate::aggregable::FieldKind::Closed(&[#(#serialized_names),*]),
                    },
                    #krate::aggregable::FieldDescriptor {
                        name: "function_value".into(),
                        kind: #krate::aggregable::FieldKind::Open,
                    },
                ]
            }

            fn field_values(&self) -> Vec<(String, String)> {
                let (cat, val) = match self {
                    #(#field_value_arms)*
                };
                vec![
                    ("function_category".into(), cat),
                    ("function_value".into(), val),
                ]
            }
        }
    };

    TokenStream::from(expanded)
}
