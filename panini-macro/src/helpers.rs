use quote::quote;
use syn::{Attribute, Type};

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// PascalCase → snake_case (e.g. "MasculinePersonal" → "masculine_personal").
pub fn pascal_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

/// Extract a named string value from `#[serde(key = "...")]`.
///
/// Works for `rename`, `rename_all`, `tag`, etc.
pub fn get_serde_value(attrs: &[Attribute], key: &str) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }
        let mut found = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(key) {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                found = Some(s.value());
            }
            Ok(())
        });
        if found.is_some() {
            return found;
        }
    }
    None
}

/// Compute the serialized string for a variant name given enum-level rename_all.
pub fn variant_serialized_name(variant_name: &str, rename_all: &Option<String>) -> String {
    match rename_all.as_deref() {
        Some("snake_case") => pascal_to_snake_case(variant_name),
        Some("lowercase") => variant_name.to_lowercase(),
        Some("UPPERCASE") => variant_name.to_uppercase(),
        // kebab-case, SCREAMING_SNAKE_CASE etc. can be added if needed
        _ => variant_name.to_string(),
    }
}

/// Returns the identifier of the last path segment of a type (e.g. `String` in
/// `std::string::String`, `Option` in `Option<T>`). `None` for non-path types.
pub fn last_segment_ident(ty: &Type) -> Option<&syn::Ident> {
    if let Type::Path(type_path) = ty {
        type_path.path.segments.last().map(|seg| &seg.ident)
    } else {
        None
    }
}

pub fn is_string_type(ty: &Type) -> bool {
    last_segment_ident(ty).is_some_and(|i| i == "String")
}

pub fn is_bool_type(ty: &Type) -> bool {
    last_segment_ident(ty).is_some_and(|i| i == "bool")
}

pub fn is_option_type(ty: &Type) -> bool {
    last_segment_ident(ty).is_some_and(|i| i == "Option")
}

/// Coarse classification of a field type used by the aggregable codegen.
///
/// `Option` is **not** handled here — callers filter those out first.
#[derive(Clone, Copy)]
pub enum FieldClass {
    /// Plain `String` — emitted as `FieldKind::Open`.
    String,
    /// Plain `bool` — emitted as `FieldKind::Closed(&["true", "false"])`.
    Bool,
    /// Any other type — assumed to implement `ClosedValues`.
    Closed,
}

pub fn classify(ty: &Type) -> FieldClass {
    if is_string_type(ty) {
        FieldClass::String
    } else if is_bool_type(ty) {
        FieldClass::Bool
    } else {
        FieldClass::Closed
    }
}

/// Extract `#[<attr_name>(crate = "...")]` — defaults to `panini_core`.
pub fn get_crate_path(attrs: &[Attribute], attr_name: &str) -> proc_macro2::TokenStream {
    for attr in attrs {
        if !attr.path().is_ident(attr_name) {
            continue;
        }
        let mut crate_path: Option<String> = None;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                crate_path = Some(s.value());
            }
            Ok(())
        });
        if let Some(path) = crate_path {
            let ts: proc_macro2::TokenStream = path.parse().expect("invalid crate path");
            return ts;
        }
    }
    quote! { panini_core }
}
