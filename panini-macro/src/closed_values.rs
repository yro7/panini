use crate::helpers::{get_crate_path, get_serde_value, variant_serialized_name};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

// ─── ClosedValues derive ──────────────────────────────────────────────────────

/// Derive `ClosedValues` for unit-variant enums.
///
/// Respects `#[serde(rename_all = "snake_case")]` on the enum and
/// `#[serde(rename = "...")]` on individual variants.
///
/// When used inside `panini-core` itself, add `#[closed_values(crate = "crate")]`
/// so the generated impl uses `crate::` instead of `panini_core::`.
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let krate = get_crate_path(&input.attrs, "closed_values");
    let rename_all = get_serde_value(&input.attrs, "rename_all");

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("ClosedValues can only be derived for enums"),
    };

    for variant in variants {
        assert!(
            matches!(variant.fields, Fields::Unit),
            "ClosedValues: variant `{}` must be a unit variant (no fields)",
            variant.ident
        );
    }

    let serialized_names: Vec<String> = variants
        .iter()
        .map(|v| {
            get_serde_value(&v.attrs, "rename").unwrap_or_else(|| {
                variant_serialized_name(&v.ident.to_string(), rename_all.as_ref())
            })
        })
        .collect();

    let variant_str_arms = variants.iter().zip(serialized_names.iter()).map(|(v, s)| {
        let ident = &v.ident;
        quote! { Self::#ident => #s, }
    });

    let expanded = quote! {
        impl #krate::aggregable::ClosedValues for #name {
            fn all_variants() -> &'static [&'static str] {
                &[#(#serialized_names),*]
            }

            fn variant_str(&self) -> &str {
                match self {
                    #(#variant_str_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
