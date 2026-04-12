use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use crate::helpers::*;

// ─── PaniniResult derive macro ────────────────────────────────────────────────

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;

    // We expect EXACTLY one generic parameter L (the language type) for now.
    let type_params: Vec<_> = generics.type_params().collect();
    if type_params.len() != 1 {
        panic!("PaniniResult: struct must have exactly one type parameter (the language type L). Support for multiple generics requires implementing a #[panini(lang = \"L\")] attribute.");
    }
    let lang_ident = &type_params[0].ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(f) => &f.named,
            _ => panic!("PaniniResult: only named fields are supported"),
        },
        _ => panic!("PaniniResult: can only be derived for structs"),
    };

    // Parse #[component(ComponentName)] attributes on each field.
    struct ComponentField {
        ident: syn::Ident,
        path: syn::Path,
        is_option: bool,
    }

    let mut component_fields = Vec::new();

    for field in fields {
        let field_ident = field.ident.as_ref().unwrap();
        let mut component_path: Option<syn::Path> = None;

        for attr in &field.attrs {
            if attr.meta.path().is_ident("component") {
                let path: syn::Path = attr.parse_args().expect(
                    "PaniniResult: #[component(Name)] expects a component type path",
                );
                component_path = Some(path);
            }
        }

        let Some(path) = component_path else {
            panic!(
                "PaniniResult: field `{}` must have a #[component(Name)] attribute",
                field_ident
            );
        };

        component_fields.push(ComponentField {
            ident: field_ident.clone(),
            path,
            is_option: is_option_type(&field.ty),
        });
    }

    // Aliases to reduce quote! block verbosity
    let ac = quote! { ::panini::__macro_support::panini_core::component::AnalysisComponent::<#lang_ident> };
    let ex_err = quote! { ::panini::__macro_support::panini_engine::extractor::ExtractionError };
    let res_err = quote! { ::panini::__macro_support::panini_core::component::ExtractionResultError };
    let ling = quote! { ::panini::__macro_support::panini_core::traits::LinguisticDefinition };
    let model = quote! { ::panini::__macro_support::rig::completion::CompletionModel };
    let req = quote! { ::panini::__macro_support::panini_engine::prompts::ExtractionRequest };
    let opts = quote! { ::panini::__macro_support::panini_engine::extractor::ExtractionOptions };
    let extract_fn = quote! { ::panini::__macro_support::panini_engine::extractor::extract_with_components };

    // Generate component instantiation: one let binding per component
    let component_lets: Vec<_> = component_fields
        .iter()
        .enumerate()
        .map(|(i, comp)| {
            let var = quote::format_ident!("__comp_{}", i);
            let path = &comp.path;
            quote! { let #var = <#path as ::std::default::Default>::default(); }
        })
        .collect();

    // Generate component references and field deserializations in parallel
    let component_refs: Vec<_> = (0..component_fields.len())
        .map(|i| {
            let var = quote::format_ident!("__comp_{}", i);
            quote! { &#var as &dyn #ac }
        })
        .collect();

    let field_deserializations: Vec<_> = component_fields
        .iter()
        .enumerate()
        .map(|(i, comp)| {
            let var = quote::format_ident!("__comp_{}", i);
            let field_ident = &comp.ident;
            let key_expr = quote! { #ac::schema_key(&#var) };
            if comp.is_option {
                quote! {
                    #field_ident: match __raw.get(#key_expr) {
                        Ok(val) => Some(val),
                        Err(#res_err::KeyNotFound { .. }) => None,
                        Err(e) => return Err(#ex_err::ResultMapping(e)),
                    }
                }
            } else {
                quote! { #field_ident: __raw.get(#key_expr).map_err(#ex_err::ResultMapping)? }
            }
        })
        .collect();

    // Generate ComponentRequires<L> bounds only for required (non-Option) fields.
    let requires_bounds: Vec<_> = component_fields
        .iter()
        .filter(|comp| !comp.is_option)
        .map(|comp| {
            let path = &comp.path;
            quote! { #path: ::panini::__macro_support::panini_core::component::ComponentRequires<#lang_ident> }
        })
        .collect();

    let existing_where = generics
        .where_clause
        .as_ref()
        .map(|w| {
            let preds = &w.predicates;
            quote! { #preds, }
        })
        .unwrap_or_default();

    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics
        where
            #existing_where
            #lang_ident: #ling + Send + Sync,
            #(#requires_bounds,)*
        {
            /// Extract features from text using an LLM, returning this typed result struct.
            pub async fn extract<__M: #model>(
                language: &#lang_ident,
                model: &__M,
                request: &#req,
                options: #opts<'_>,
            ) -> Result<Self, #ex_err> {
                #(#component_lets)*
                let __components: Vec<&dyn #ac> = vec![#(#component_refs,)*];
                let __raw = #extract_fn(language, model, request, &__components, options).await?;
                Ok(Self { #(#field_deserializations,)* })
            }
        }
    };

    TokenStream::from(expanded)
}
