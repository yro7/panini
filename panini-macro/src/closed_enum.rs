use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

/// Macro attribute that reduces the standard 10-trait derive boilerplate on closed linguistic enums
/// down to a single attribute: `#[closed_enum]`.
///
/// Automatically adds:
/// - `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, panini_macro::ClosedValues)]`
/// - `#[serde(rename_all = "snake_case")]` (if not already specified)
/// - `#[closed_values(crate = "...")]` (if `crate = "..."` is passed as argument, e.g. for `panini-core`)
pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);

    let mut crate_override: Option<syn::LitStr> = None;
    if !attr.is_empty() {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("crate") {
                crate_override = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta
                    .error("unrecognized attribute for closed_enum; expected `crate = \"...\"`"))
            }
        });
        parse_macro_input!(attr with parser);
    }

    let closed_values_attr = if let Some(krate) = crate_override {
        quote! { #[closed_values(crate = #krate)] }
    } else {
        quote! {}
    };

    let has_rename_all = crate::helpers::get_serde_value(&input.attrs, "rename_all").is_some();
    let serde_rename_all = if has_rename_all {
        quote! {}
    } else {
        quote! { #[serde(rename_all = "snake_case")] }
    };

    let expanded = quote! {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            ::serde::Serialize,
            ::serde::Deserialize,
            ::schemars::JsonSchema,
            panini_macro::ClosedValues,
        )]
        #closed_values_attr
        #serde_rename_all
        #input
    };

    TokenStream::from(expanded)
}
