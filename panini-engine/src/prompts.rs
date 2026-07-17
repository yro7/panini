use isolang::Language as IsoLang;
use panini_core::traits::LinguisticDefinition;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ----- Prompt Builder Errors -----

#[derive(Debug, thiserror::Error)]
pub enum PromptBuilderError {
    #[error("Failed to parse JSON schema: {0}")]
    SchemaParseError(#[from] serde_json::Error),
    #[error("Failed to load prompt config: {0}")]
    ConfigLoadError(String),
    #[error("Placeholder '{placeholder}' in template is not available in context")]
    PlaceholderNotAvailable { placeholder: String },
}

// ----- Prompt Config Structs -----

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtractorPrompts {
    pub system_role: String,
    pub target_language: String,
    pub extraction_directives: String,
    pub learner_profile: LearnerProfile,
    pub skill_context: SkillContextPrompts,
    pub user_context: String,
    pub output_instruction: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LearnerProfile {
    pub ui_language: String,
    pub linguistic_background_intro: String,
    pub linguistic_background_entry: String,
}

impl LearnerProfile {
    pub fn build_prompt(
        &self,
        ui_lang_name: &str,
        linguistic_background: &[panini_core::component::LanguageLevel],
    ) -> Result<String, PromptBuilderError> {
        let ui_lang_iso_code = IsoLang::from_name(ui_lang_name)
            .map_or_else(|| "eng".to_string(), |lang| lang.to_639_3().to_string());

        let mut global_ctx = HashMap::new();
        global_ctx.insert("language", ui_lang_name.to_string());
        global_ctx.insert("name", ui_lang_name.to_string());
        global_ctx.insert("iso", ui_lang_iso_code);

        let mut learner_profile_content = String::new();

        let ui_lang_str = interpolate(&self.ui_language, &global_ctx)?;
        learner_profile_content.push_str(&ui_lang_str);

        if !linguistic_background.is_empty() {
            learner_profile_content.push_str("\n\n");
            learner_profile_content.push_str(&self.linguistic_background_intro);
            learner_profile_content.push('\n');

            for lang in linguistic_background {
                let mut ctx = global_ctx.clone();
                ctx.insert("iso", lang.iso_639_3.clone());
                ctx.insert("level", lang.level.clone());
                let entry = interpolate(&self.linguistic_background_entry, &ctx)?;
                learner_profile_content.push_str(&entry);
                learner_profile_content.push('\n');
            }
        }

        Ok(wrap_tag("learner_profile", &learner_profile_content))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SkillContextPrompts {
    pub skill_tree_path: String,
    pub pedagogical_focus: String,
}

impl ExtractorPrompts {
    /// Load prompts from a YAML file.
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed.
    pub fn load(path: &str) -> Result<Self, PromptBuilderError> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            PromptBuilderError::ConfigLoadError(format!("Failed to read {path}: {e}"))
        })?;
        serde_yml::from_str(&content).map_err(|e| {
            PromptBuilderError::ConfigLoadError(format!("Failed to parse {path}: {e}"))
        })
    }
}

// ----- Extraction Request -----

/// Re-export from panini-core for backwards compatibility.
pub use panini_core::component::LanguageLevel;

/// Generic extraction request
#[derive(bon::Builder)]
pub struct ExtractionRequest {
    /// The text/card JSON to extract features from.
    pub content: String,
    /// Target words to focus extraction on.
    pub targets: Vec<String>,
    /// Optional pedagogical context (replaces skill node instructions).
    pub pedagogical_context: Option<String>,
    /// Optional skill/topic path for context.
    pub skill_path: Option<String>,
    /// Learner's UI language (for pedagogical explanation).
    #[builder(default = "English".to_string())]
    pub learner_ui_language: String,
    /// Learner's linguistic background.
    #[builder(default)]
    pub linguistic_background: Vec<LanguageLevel>,
    /// Optional user-provided context.
    pub user_prompt: Option<String>,
}

/// One card of a batched extraction request.
#[derive(Debug, Clone)]
pub struct ExtractionItem {
    /// The text/card JSON to extract features from.
    pub content: String,
    /// Target words to focus extraction on.
    pub targets: Vec<String>,
}

/// A batched extraction request: shared pedagogical context plus N cards
/// analyzed by one LLM call per component subset.
#[derive(Debug, Clone)]
pub struct BatchExtractionRequest {
    pub items: Vec<ExtractionItem>,
    /// Optional pedagogical context (replaces skill node instructions).
    pub pedagogical_context: Option<String>,
    /// Optional skill/topic path for context.
    pub skill_path: Option<String>,
    /// Learner's UI language (for pedagogical explanation).
    pub learner_ui_language: String,
    /// Learner's linguistic background.
    pub linguistic_background: Vec<LanguageLevel>,
    /// Optional user-provided context.
    pub user_prompt: Option<String>,
}

impl BatchExtractionRequest {
    /// The shared context as a single-card request shape, for prompt
    /// composition. Per-card `content`/`targets` stay empty — they live in
    /// the batched user message, one entry per card.
    pub(crate) fn shared_context(&self) -> ExtractionRequest {
        ExtractionRequest {
            content: String::new(),
            targets: Vec::new(),
            pedagogical_context: self.pedagogical_context.clone(),
            skill_path: self.skill_path.clone(),
            learner_ui_language: self.learner_ui_language.clone(),
            linguistic_background: self.linguistic_background.clone(),
            user_prompt: self.user_prompt.clone(),
        }
    }
}

// ----- Helper Functions -----

/// Wraps content in XML tags
#[must_use]
pub fn wrap_tag(tag: &str, content: &str) -> String {
    format!("<{tag}>\n{content}\n</{tag}>")
}

/// Interpolates placeholders in a template string
///
/// # Panics
/// Panics if the internal regex fails to compile.
///
/// # Errors
/// Returns an error if a placeholder requires a value not present in the context.
pub fn interpolate<V: AsRef<str>, S: std::hash::BuildHasher>(
    template: &str,
    context: &HashMap<&str, V, S>,
) -> Result<String, PromptBuilderError> {
    let placeholder_re = Regex::new(r"\{(\w+)\}").unwrap();
    let mut result = template.to_string();

    for cap in placeholder_re.captures_iter(template) {
        let placeholder = &cap[1];
        let value = context
            .get(placeholder)
            .ok_or_else(|| PromptBuilderError::PlaceholderNotAvailable {
                placeholder: placeholder.to_string(),
            })?
            .as_ref();
        result = result.replace(&format!("{{{placeholder}}}"), value);
    }

    Ok(result)
}

// ----- Feature Extractor Prompt Context -----

/// Builds the system prompt for the feature extractor.
///
/// # Errors
/// Returns an error if prompt interpolation fails (e.g. missing context variables).
pub fn build_extraction_prompt<L: LinguisticDefinition>(
    language: &L,
    request: &ExtractionRequest,
    extractor_prompts: &ExtractorPrompts,
) -> Result<String, PromptBuilderError> {
    let cfg = extractor_prompts;

    let ui_lang_name = &request.learner_ui_language;
    let ui_lang_iso_code = IsoLang::from_name(ui_lang_name)
        .map_or_else(|| "eng".to_string(), |lang| lang.to_639_3().to_string());

    let context_description = request.user_prompt.as_deref().unwrap_or("");
    let skill_path = request.skill_path.as_deref().unwrap_or("");
    let instructions = request.pedagogical_context.as_deref().unwrap_or("");

    let mut global_ctx = HashMap::new();
    global_ctx.insert("language", language.name().to_string());
    global_ctx.insert("directives", language.extraction_directives().to_string());
    global_ctx.insert("path", skill_path.to_string());
    global_ctx.insert("instructions", instructions.to_string());
    global_ctx.insert("iso", ui_lang_iso_code);
    global_ctx.insert("name", ui_lang_name.clone());
    global_ctx.insert("context_description", context_description.to_string());

    let mut blocks = Vec::new();

    // System role
    blocks.push(cfg.system_role.clone());

    // Target language section
    let language_context = interpolate(&cfg.target_language, &global_ctx)?;
    blocks.push(wrap_tag("target_language", &language_context));

    // Extraction directives section
    let extraction_directives = interpolate(&cfg.extraction_directives, &global_ctx)?;
    blocks.push(wrap_tag("extraction_directives", &extraction_directives));

    // Learner profile section
    let wrapped_profile = cfg
        .learner_profile
        .build_prompt(ui_lang_name, &request.linguistic_background)?;
    blocks.push(wrapped_profile);

    // Skill context section
    let mut skill_context_content = String::new();
    let skill_path_str = interpolate(&cfg.skill_context.skill_tree_path, &global_ctx)?;
    skill_context_content.push_str(&skill_path_str);

    if request.pedagogical_context.is_some() {
        skill_context_content.push('\n');
        let ped_focus_str = interpolate(&cfg.skill_context.pedagogical_focus, &global_ctx)?;
        skill_context_content.push_str(&ped_focus_str);
    }

    blocks.push(wrap_tag("skill_context", &skill_context_content));

    // User context section (if provided)
    if !context_description.is_empty() {
        let user_context_str = interpolate(&cfg.user_context, &global_ctx)?;
        blocks.push(wrap_tag("user_context", &user_context_str));
    }

    // Morpheme segmentation directives (agglutinative languages only)
    if let Some(morph_directives) = language.extra_extraction_directives() {
        blocks.push(wrap_tag("morpheme_segmentation", &morph_directives));
    }

    // Output instruction section
    blocks.push(wrap_tag("output", &cfg.output_instruction));

    Ok(blocks.join("\n\n"))
}
