//! LLM-powered language implementation generator.
//!
//! Generates a complete `panini-langs` language module (enums + `LinguisticDefinition` impl)
//! from a language name, using existing implementations as few-shot examples.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use rig::client::CompletionClient;
use rig::completion::CompletionModel;

use crate::config::{Config, Provider};

// ─── Embedded source files (compile-time) ────────────────────────────────────

const POLISH_EXAMPLE: &str = include_str!("../../panini-langs/src/polish.rs");
const TURKISH_EXAMPLE: &str = include_str!("../../panini-langs/src/turkish.rs");
const ARABIC_EXAMPLE: &str = include_str!("../../panini-langs/src/arabic.rs");
const TRAITS_DEF: &str = include_str!("../../panini-core/src/traits.rs");
const MORPHEME_DEF: &str = include_str!("../../panini-core/src/morpheme.rs");
const SHARED_ENUMS: &str = include_str!("../../panini-core/src/morphology_enums.rs");

// ─── Public entry point ──────────────────────────────────────────────────────

pub async fn run(
    config: &Config,
    language: &str,
    iso_code: Option<&str>,
    agglutinative: bool,
    temperature: f32,
    skip_check: bool,
) -> Result<()> {
    // Validate ISO code if provided.
    if let Some(code) = iso_code {
        isolang::Language::from_639_3(code).ok_or_else(|| {
            anyhow!("ISO 639-3 code '{code}' not found in the isolang crate")
        })?;
    }

    let workspace_root = find_workspace_root()?;
    let module_name = language.to_lowercase().replace(' ', "_").replace('-', "_");
    let struct_name = to_pascal_case(language);
    let lang_file = workspace_root
        .join("panini-langs/src")
        .join(format!("{module_name}.rs"));

    if lang_file.exists() {
        return Err(anyhow!(
            "Language file already exists: {}",
            lang_file.display()
        ));
    }

    println!("Generating {language} implementation…");

    match config.provider {
        Provider::Openai => {
            let client = rig::providers::openai::Client::new(&config.api_key)
                .map_err(|e| anyhow!("Failed to create OpenAI client: {e}"))?;
            let model = client.completion_model(&config.model);
            generate_with_retries(
                &model,
                language,
                iso_code,
                agglutinative,
                temperature,
                &workspace_root,
                &lang_file,
                &module_name,
                &struct_name,
                skip_check,
            )
            .await?;
        }
        Provider::Anthropic => {
            let client = rig::providers::anthropic::Client::new(&config.api_key)
                .map_err(|e| anyhow!("Failed to create Anthropic client: {e}"))?;
            let model = client.completion_model(&config.model);
            generate_with_retries(
                &model,
                language,
                iso_code,
                agglutinative,
                temperature,
                &workspace_root,
                &lang_file,
                &module_name,
                &struct_name,
                skip_check,
            )
            .await?;
        }
        Provider::Google => {
            let client = rig::providers::gemini::Client::new(&config.api_key)
                .map_err(|e| anyhow!("Failed to create Gemini client: {e}"))?;
            let model = client.completion_model(&config.model);
            generate_with_retries(
                &model,
                language,
                iso_code,
                agglutinative,
                temperature,
                &workspace_root,
                &lang_file,
                &module_name,
                &struct_name,
                skip_check,
            )
            .await?;
        }
    }

    println!("Language file written to: {}", lang_file.display());
    println!("Registry and lib.rs patched successfully.");
    Ok(())
}

// ─── Generation with self-correction ─────────────────────────────────────────

async fn generate_with_retries<M: CompletionModel>(
    model: &M,
    language: &str,
    iso_code: Option<&str>,
    agglutinative: bool,
    temperature: f32,
    workspace_root: &Path,
    lang_file: &Path,
    module_name: &str,
    struct_name: &str,
    skip_check: bool,
) -> Result<()> {
    let max_attempts = if skip_check { 1 } else { 3 };
    let mut previous_error: Option<String> = None;

    for attempt in 1..=max_attempts {
        println!("  Attempt {attempt}/{max_attempts}…");

        let code = call_llm(
            model,
            language,
            iso_code,
            agglutinative,
            temperature,
            previous_error.as_deref(),
        )
        .await?;

        let cleaned = clean_generated_code(&code);

        // Write the language file.
        std::fs::write(lang_file, &cleaned)
            .with_context(|| format!("Writing {}", lang_file.display()))?;

        // Patch lib.rs and registry.rs (idempotent — re-patches on retry).
        let iso = iso_code.unwrap_or("xxx");
        patch_lib_rs(workspace_root, module_name)?;
        patch_registry_rs(workspace_root, module_name, struct_name, iso)?;

        if skip_check {
            println!("  Skipping cargo check (--skip-check).");
            return Ok(());
        }

        // Run cargo check.
        match run_cargo_check(workspace_root) {
            Ok(()) => {
                println!("  cargo check passed!");
                return Ok(());
            }
            Err(compiler_output) => {
                if attempt == max_attempts {
                    return Err(anyhow!(
                        "Generated code failed to compile after {max_attempts} attempts.\n\
                         Last compiler output:\n{compiler_output}"
                    ));
                }
                println!("  Compilation failed, retrying with error feedback…");
                previous_error = Some(compiler_output);
            }
        }
    }

    unreachable!()
}

// ─── LLM call ────────────────────────────────────────────────────────────────

async fn call_llm<M: CompletionModel>(
    model: &M,
    language: &str,
    iso_code: Option<&str>,
    agglutinative: bool,
    temperature: f32,
    previous_error: Option<&str>,
) -> Result<String> {
    let system_prompt = build_system_prompt(agglutinative);
    let user_prompt = build_user_prompt(language, iso_code, agglutinative, previous_error);

    let builder = model
        .completion_request(user_prompt.as_str())
        .preamble(system_prompt)
        .temperature(temperature as f64)
        .max_tokens(8192u64);

    let response = builder.send().await?;

    let text = response
        .choice
        .into_iter()
        .find_map(|c| {
            if let rig::completion::message::AssistantContent::Text(t) = c {
                Some(t.text)
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow!("LLM returned no text content"))?;

    Ok(text)
}

// ─── Prompt construction ─────────────────────────────────────────────────────

fn build_system_prompt(agglutinative: bool) -> String {
    let mut prompt = String::with_capacity(32_000);

    prompt.push_str(
        "You are an expert computational linguist and Rust developer. \
         Your task is to generate a complete Rust module implementing linguistic \
         feature extraction for a given language, following the Panini framework's patterns exactly.\n\n"
    );

    prompt.push_str("## Trait definitions\n\n");
    prompt.push_str("### LinguisticDefinition (from traits.rs)\n```rust\n");
    prompt.push_str(TRAITS_DEF);
    prompt.push_str("\n```\n\n");

    prompt.push_str("### Shared morphological enums (from morphology_enums.rs)\n```rust\n");
    prompt.push_str(SHARED_ENUMS);
    prompt.push_str("\n```\n\n");

    if agglutinative {
        prompt.push_str("### Agglutinative trait and morpheme types (from morpheme.rs)\n```rust\n");
        prompt.push_str(MORPHEME_DEF);
        prompt.push_str("\n```\n\n");
    }

    prompt.push_str("## Existing language implementations (use as examples)\n\n");

    prompt.push_str("### Polish (simple, non-agglutinative)\n```rust\n");
    prompt.push_str(POLISH_EXAMPLE);
    prompt.push_str("\n```\n\n");

    prompt.push_str("### Arabic (complex, non-agglutinative, root+pattern system)\n```rust\n");
    prompt.push_str(ARABIC_EXAMPLE);
    prompt.push_str("\n```\n\n");

    prompt.push_str("### Turkish (agglutinative, with morpheme inventory)\n```rust\n");
    prompt.push_str(TURKISH_EXAMPLE);
    prompt.push_str("\n```\n\n");

    prompt.push_str("## Rules — you MUST follow these exactly\n\n");
    prompt.push_str(
        "1. Every variant of the Morphology enum MUST have `lemma: String` as the FIRST field.\n\
         2. The Morphology enum MUST use these exact derives:\n   \
            `#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::MorphologyInfo)]`\n\
         3. The Morphology enum MUST use `#[serde(tag = \"pos\")]` and `#[serde(rename_all = \"snake_case\")]`.\n\
         4. All language-specific enums (cases, genders, tenses, etc.) MUST use:\n   \
            `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]`\n   \
            and `#[serde(rename_all = \"snake_case\")]`.\n\
         5. Reuse shared enums from `panini_core::traits` (Person, BinaryNumber, TernaryNumber, BinaryGender, TernaryGender, SlavicAspect, BinaryVoice) where semantically appropriate.\n\
         6. The unit struct must be `pub struct <Name>;` with no fields.\n\
         7. The `iso_code()` method must return `IsoLang::<Variant>` — the variant must exist in the `isolang` crate (Rust crate `isolang` v2.4).\n\
         8. Import only from: `use serde::{Deserialize, Serialize};` and `use panini_core::traits::{...};`\n\
         9. Use UPOS tag set for morphology variants (Adjective, Adposition, Adverb, Auxiliary, CoordinatingConjunction, Determiner, Interjection, Noun, Numeral, Particle, Pronoun, ProperNoun, Punctuation, SubordinatingConjunction, Symbol, Verb, Other).\n"
    );

    if agglutinative {
        prompt.push_str(
            "10. Set `type GrammaticalFunction` to a custom wrapper enum (see Turkish example).\n\
             11. Implement the `Agglutinative` trait with a complete morpheme inventory.\n\
             12. Include `TypologicalFeature::Agglutination` in `typological_features()`.\n"
        );
    } else {
        prompt.push_str(
            "10. Set `type GrammaticalFunction = ();` (this language is NOT agglutinative).\n\
             11. Do NOT implement the `Agglutinative` trait.\n"
        );
    }

    prompt.push_str(
        "\n## Output format\n\n\
         Output ONLY the Rust source code for the language module. \
         No markdown fences, no explanations, no comments outside the code. \
         The output must be a complete, compilable Rust file.\n"
    );

    prompt
}

fn build_user_prompt(
    language: &str,
    iso_code: Option<&str>,
    agglutinative: bool,
    previous_error: Option<&str>,
) -> String {
    let mut prompt = format!(
        "Generate the complete Rust implementation for: {language}"
    );

    if let Some(code) = iso_code {
        prompt.push_str(&format!(" (ISO 639-3: {code})"));
    }

    if agglutinative {
        prompt.push_str(". This language IS agglutinative — implement the Agglutinative trait with a morpheme inventory.");
    } else {
        prompt.push_str(". This language is NOT agglutinative — set GrammaticalFunction = ().");
    }

    prompt.push_str("\n\nInclude all PoS categories relevant to this language's morphology, with linguistically accurate fields (cases, genders, tenses, moods, aspects, etc.).");

    if let Some(err) = previous_error {
        prompt.push_str(&format!(
            "\n\n## PREVIOUS ATTEMPT FAILED\n\
             The code you generated previously failed to compile. Here are the errors:\n\n\
             {err}\n\n\
             Please fix ALL the compilation errors and output the corrected complete file."
        ));
    }

    prompt
}

// ─── Code cleaning ───────────────────────────────────────────────────────────

fn clean_generated_code(raw: &str) -> String {
    let trimmed = raw.trim();

    // Strip markdown code fences if present.
    let stripped = if trimmed.starts_with("```") {
        let after_first_fence = trimmed
            .find('\n')
            .map(|i| &trimmed[i + 1..])
            .unwrap_or(trimmed);
        after_first_fence
            .strip_suffix("```")
            .unwrap_or(after_first_fence)
            .trim()
    } else {
        trimmed
    };

    let mut result = stripped.to_string();
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result
}

// ─── File patching ───────────────────────────────────────────────────────────

fn patch_lib_rs(workspace_root: &Path, module_name: &str) -> Result<()> {
    let lib_path = workspace_root.join("panini-langs/src/lib.rs");
    let content = std::fs::read_to_string(&lib_path)
        .with_context(|| format!("Reading {}", lib_path.display()))?;

    // Check if already patched (idempotent).
    let mod_line = format!("pub mod {module_name};");
    if content.contains(&mod_line) {
        return Ok(());
    }

    let marker = "#[cfg(feature = \"registry\")]";
    let insert_pos = content.find(marker).ok_or_else(|| {
        anyhow!("Could not find marker '{marker}' in lib.rs — file structure may have changed")
    })?;

    let use_line = format!("pub use {module_name}::*;");
    let insertion = format!("{mod_line}\n{use_line}\n\n");

    let mut patched = String::with_capacity(content.len() + insertion.len());
    patched.push_str(&content[..insert_pos]);
    patched.push_str(&insertion);
    patched.push_str(&content[insert_pos..]);

    std::fs::write(&lib_path, &patched)
        .with_context(|| format!("Writing {}", lib_path.display()))?;

    Ok(())
}

fn patch_registry_rs(
    workspace_root: &Path,
    _module_name: &str,
    struct_name: &str,
    iso_code: &str,
) -> Result<()> {
    let reg_path = workspace_root.join("panini-langs/src/registry.rs");
    let mut content = std::fs::read_to_string(&reg_path)
        .with_context(|| format!("Reading {}", reg_path.display()))?;

    // Check if already patched (idempotent).
    if content.contains(&format!("\"{iso_code}\"")) {
        return Ok(());
    }

    // 1. Patch the import line: use crate::{Arabic, Polish, Turkish};
    content = patch_import_line(&content, struct_name)?;

    // 2. Patch extract_erased match: insert before `_ => Err(anyhow!(...`
    content = patch_extract_erased(&content, iso_code, struct_name)?;

    // 3. Patch extract_erased_with_components match: insert before `_ => Err(anyhow!(...`
    content = patch_extract_with_components(&content, iso_code, struct_name)?;

    // 4. Patch supported_languages array.
    content = patch_supported_languages(&content, iso_code)?;

    std::fs::write(&reg_path, &content)
        .with_context(|| format!("Writing {}", reg_path.display()))?;

    Ok(())
}

fn patch_import_line(content: &str, struct_name: &str) -> Result<String> {
    let marker = "use crate::{";
    let start = content.find(marker).ok_or_else(|| {
        anyhow!("Could not find '{marker}' in registry.rs")
    })?;
    let end = content[start..].find("};").ok_or_else(|| {
        anyhow!("Could not find closing '}};\' for import in registry.rs")
    })? + start + 2;

    let import_line = &content[start..end];

    // Extract existing names, add new one, sort.
    let inner = import_line
        .strip_prefix(marker)
        .and_then(|s| s.strip_suffix("};"))
        .ok_or_else(|| anyhow!("Failed to parse import line"))?;

    let mut names: Vec<String> = inner
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if !names.contains(&struct_name.to_string()) {
        names.push(struct_name.to_string());
        names.sort();
    }

    let new_import = format!("use crate::{{{}}};", names.join(", "));
    let mut result = String::with_capacity(content.len() + 32);
    result.push_str(&content[..start]);
    result.push_str(&new_import);
    result.push_str(&content[end..]);

    Ok(result)
}

fn patch_extract_erased(content: &str, iso_code: &str, struct_name: &str) -> Result<String> {
    // Find the first `_ => Err(anyhow!("Unsupported language:` which is in extract_erased.
    let marker = "_ => Err(anyhow!(\"Unsupported language: {lang_code}\"))";
    let pos = content.find(marker).ok_or_else(|| {
        anyhow!("Could not find extract_erased wildcard arm in registry.rs")
    })?;

    let new_arm = format!(
        "\"{iso_code}\" => {{\n\
         \x20           let result = extract_features_via_llm(\n\
         \x20               &{struct_name},\n\
         \x20               model,\n\
         \x20               request,\n\
         \x20               temperature,\n\
         \x20               max_tokens,\n\
         \x20               previous_attempt,\n\
         \x20               extractor_prompts,\n\
         \x20           )\n\
         \x20           .await?;\n\
         \x20           Ok(serde_json::to_value(&result)?)\n\
         \x20       }}\n\
         \x20       "
    );

    let mut result = String::with_capacity(content.len() + new_arm.len());
    result.push_str(&content[..pos]);
    result.push_str(&new_arm);
    result.push_str(&content[pos..]);

    Ok(result)
}

fn patch_extract_with_components(
    content: &str,
    iso_code: &str,
    struct_name: &str,
) -> Result<String> {
    // Find the second `_ => Err(anyhow!("Unsupported language:` — skip the first one.
    let marker = "_ => Err(anyhow!(\"Unsupported language: {lang_code}\"))";
    let first = content.find(marker).ok_or_else(|| {
        anyhow!("Could not find first wildcard arm in registry.rs")
    })?;
    let second_offset = content[first + marker.len()..].find(marker).ok_or_else(|| {
        anyhow!("Could not find second wildcard arm (extract_erased_with_components) in registry.rs")
    })?;
    let pos = first + marker.len() + second_offset;

    let new_arm = format!(
        "\"{iso_code}\" => {{\n\
         \x20           extract_for_language(&{struct_name}, model, request, component_keys, temperature, max_tokens, previous_attempt, extractor_prompts).await\n\
         \x20       }}\n\
         \x20       "
    );

    let mut result = String::with_capacity(content.len() + new_arm.len());
    result.push_str(&content[..pos]);
    result.push_str(&new_arm);
    result.push_str(&content[pos..]);

    Ok(result)
}

fn patch_supported_languages(content: &str, iso_code: &str) -> Result<String> {
    let marker = "&[\"pol\"";
    let start = content.find(marker).ok_or_else(|| {
        anyhow!("Could not find supported_languages array marker '{marker}' in registry.rs")
    })?;
    let end = content[start..].find(']').ok_or_else(|| {
        anyhow!("Could not find closing ']' for supported_languages array")
    })? + start;

    let current = &content[start..end];
    let new_array = format!("{current}, \"{iso_code}\"");

    let mut result = String::with_capacity(content.len() + 16);
    result.push_str(&content[..start]);
    result.push_str(&new_array);
    result.push_str(&content[end..]);

    Ok(result)
}

// ─── Cargo check ─────────────────────────────────────────────────────────────

fn run_cargo_check(workspace_root: &Path) -> std::result::Result<(), String> {
    let output = Command::new("cargo")
        .args(["check", "-p", "panini-langs"])
        .current_dir(workspace_root)
        .output()
        .map_err(|e| format!("Failed to run cargo check: {e}"))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.to_string())
    }
}

// ─── Utilities ───────────────────────────────────────────────────────────────

fn find_workspace_root() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .context("Failed to run cargo metadata")?;

    if !output.status.success() {
        return Err(anyhow!("cargo metadata failed"));
    }

    let meta: serde_json::Value = serde_json::from_slice(&output.stdout)
        .context("Failed to parse cargo metadata output")?;

    let root = meta["workspace_root"]
        .as_str()
        .ok_or_else(|| anyhow!("Missing workspace_root in cargo metadata"))?;

    Ok(PathBuf::from(root))
}

fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == ' ' || c == '-' || c == '_')
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    upper + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("french"), "French");
        assert_eq!(to_pascal_case("modern greek"), "ModernGreek");
        assert_eq!(to_pascal_case("old-english"), "OldEnglish");
    }

    #[test]
    fn test_clean_generated_code_strips_fences() {
        let raw = "```rust\nuse serde::Serialize;\npub struct Foo;\n```";
        let cleaned = clean_generated_code(raw);
        assert!(cleaned.starts_with("use serde"));
        assert!(!cleaned.contains("```"));
    }

    #[test]
    fn test_clean_generated_code_plain() {
        let raw = "use serde::Serialize;\npub struct Foo;\n";
        let cleaned = clean_generated_code(raw);
        assert_eq!(cleaned, raw);
    }

    #[test]
    fn test_patch_import_line() {
        let content = "use crate::{Arabic, Polish, Turkish};";
        let result = patch_import_line(content, "French").unwrap();
        assert_eq!(result, "use crate::{Arabic, French, Polish, Turkish};");
    }

    #[test]
    fn test_patch_supported_languages() {
        let content = "    &[\"pol\", \"tur\", \"ara\"]\n";
        let result = patch_supported_languages(content, "fra").unwrap();
        assert!(result.contains("\"fra\""));
    }
}
