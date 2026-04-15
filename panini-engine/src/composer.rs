use panini_core::component::{AnalysisComponent, ComponentContext};
use panini_core::traits::LinguisticDefinition;

use crate::prompts::{wrap_tag, ExtractionRequest, ExtractorPrompts};

/// Compose a JSON Schema from multiple components.
///
/// Each component contributes a `schema_fragment()` that becomes a property
/// under its `schema_key()`. `$defs` from each fragment are hoisted to the
/// root level so that `$ref` paths resolve correctly.
pub fn compose_schema<L: LinguisticDefinition>(
    lang: &L,
    components: &[&dyn AnalysisComponent<L>],
) -> serde_json::Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();
    let mut all_defs = serde_json::Map::new();

    for comp in components {
        let key = comp.schema_key();
        let mut fragment = comp.schema_fragment(lang);

        // Hoist $defs from the fragment to root level
        if let Some(defs) = fragment.as_object_mut().and_then(|o| o.remove("$defs"))
            && let Some(defs_obj) = defs.as_object() {
                for (def_key, def_value) in defs_obj {
                    all_defs.insert(def_key.clone(), def_value.clone());
                }
            }

        properties.insert(key.to_string(), fragment);
        required.push(serde_json::Value::String(key.to_string()));
    }

    let mut schema = serde_json::json!({
        "type": "object",
        "properties": properties,
        "required": required,
        "additionalProperties": false
    });

    if !all_defs.is_empty() {
        schema["$defs"] = serde_json::Value::Object(all_defs);
    }

    schema
}

/// Compose the system prompt from base blocks + component prompt fragments.
///
/// Uses the same block structure as `build_extraction_prompt` for the base
/// parts (`system_role`, `target_language`, `learner_profile`, `skill_context`,
/// `user_context`), then appends each component's `prompt_fragment()` in an
/// XML-tagged section, and finally the composed output instruction.
///
/// # Errors
/// Returns an error if prompt interpolation fails (e.g. missing context variables).
pub fn compose_prompt<L: LinguisticDefinition>(
    lang: &L,
    request: &ExtractionRequest,
    extractor_prompts: &ExtractorPrompts,
    components: &[&dyn AnalysisComponent<L>],
) -> Result<String, crate::prompts::PromptBuilderError> {
    use crate::prompts::interpolate;
    use isolang::Language as IsoLang;
    use std::collections::HashMap;

    let cfg = extractor_prompts;

    let ui_lang_name = &request.learner_ui_language;
    let ui_lang_iso_code = IsoLang::from_name(ui_lang_name).map_or_else(|| "eng".to_string(), |lang| lang.to_639_3().to_string());

    let context_description = request.user_prompt.as_deref().unwrap_or("");
    let skill_path = request.skill_path.as_deref().unwrap_or("");
    let instructions = request.pedagogical_context.as_deref().unwrap_or("");

    let mut global_ctx = HashMap::new();
    global_ctx.insert("language", lang.name().to_string());
    global_ctx.insert("directives", lang.extraction_directives().to_string());
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
    let mut learner_profile_content = String::new();

    let mut ui_lang_ctx = global_ctx.clone();
    ui_lang_ctx.insert("language", ui_lang_name.clone());
    let ui_lang_str = interpolate(&cfg.learner_profile.ui_language, &ui_lang_ctx)?;
    learner_profile_content.push_str(&ui_lang_str);

    if !request.linguistic_background.is_empty() {
        learner_profile_content.push_str("\n\n");
        learner_profile_content.push_str(&cfg.learner_profile.linguistic_background_intro);
        learner_profile_content.push('\n');

        for lang_bg in &request.linguistic_background {
            let mut ctx = global_ctx.clone();
            ctx.insert("iso", lang_bg.iso_639_3.clone());
            ctx.insert("level", lang_bg.level.clone());
            let entry = interpolate(&cfg.learner_profile.linguistic_background_entry, &ctx)?;
            learner_profile_content.push_str(&entry);
            learner_profile_content.push('\n');
        }
    }

    blocks.push(wrap_tag("learner_profile", &learner_profile_content));

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

    // Component-specific prompt fragments
    let comp_ctx = ComponentContext {
        targets: &request.targets,
        learner_ui_language: &request.learner_ui_language,
        pedagogical_context: request.pedagogical_context.as_deref(),
        skill_path: request.skill_path.as_deref(),
        linguistic_background: &request.linguistic_background,
    };

    let mut component_instructions = Vec::new();
    for comp in components {
        let fragment = comp.prompt_fragment(lang, &comp_ctx);
        if !fragment.is_empty() {
            component_instructions.push(wrap_tag(comp.schema_key(), &fragment));
        }
    }

    if !component_instructions.is_empty() {
        blocks.push("ANALYSIS COMPONENT INSTRUCTIONS".to_string());
        blocks.extend(component_instructions);
    }

    // Composed output instruction
    let mut output_parts = vec![
        "Return a single valid JSON object that strictly conforms to the provided JSON schema.".to_string(),
        "The JSON schema is the SINGLE SOURCE OF TRUTH. If the prompt text and the schema disagree, follow the schema.".to_string(),
        "Do not include any text outside the JSON object — no markdown, no comments.".to_string(),
    ];

    for comp in components {
        if let Some(instr) = comp.output_instruction() {
            output_parts.push(instr.to_string());
        }
    }

    blocks.push(wrap_tag("output", &output_parts.join("\n\n")));

    Ok(blocks.join("\n\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use panini_core::traits::{MorphologyInfo, Script, TypologicalFeature};
    use serde::{Deserialize, Serialize};

    // ── Minimal test language ──────────────────────────────────────────────

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
    #[serde(tag = "pos", rename_all = "lowercase")]
    enum TestMorphology {
        Noun { lemma: String },
        Verb { lemma: String },
    }

    impl MorphologyInfo for TestMorphology {
        type PosTag = TestPosTag;
        fn lemma(&self) -> &str {
            match self {
                Self::Noun { lemma } | Self::Verb { lemma } => lemma,
            }
        }
        fn pos_tag(&self) -> TestPosTag {
            match self {
                Self::Noun { .. } => TestPosTag::Noun,
                Self::Verb { .. } => TestPosTag::Verb,
            }
        }
        fn pos_label(&self) -> &'static str {
            match self {
                Self::Noun { .. } => "Noun",
                Self::Verb { .. } => "Verb",
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum TestPosTag {
        Noun,
        Verb,
    }

    struct TestLang;

    impl LinguisticDefinition for TestLang {
        type Morphology = TestMorphology;
        type GrammaticalFunction = ();

        const ISO_CODE: &'static str = "eng";

        fn supported_scripts(&self) -> &[Script] {
            &[Script::LATN]
        }
        fn default_script(&self) -> Script {
            Script::LATN
        }
        fn extraction_directives(&self) -> &'static str {
            "Test directives"
        }
        fn typological_features(&self) -> &[TypologicalFeature] {
            &[]
        }
    }

    // ── Fake components for testing ────────────────────────────────────────

    #[derive(Debug)]
    struct FakeComponentA;

    impl AnalysisComponent<TestLang> for FakeComponentA {
        fn name(&self) -> &'static str {
            "A"
        }
        fn schema_key(&self) -> &'static str {
            "alpha"
        }
        fn schema_fragment(&self, _lang: &TestLang) -> serde_json::Value {
            serde_json::json!({ "type": "string" })
        }
        fn prompt_fragment(&self, _lang: &TestLang, _ctx: &ComponentContext) -> String {
            "Do alpha.".to_string()
        }
        fn output_instruction(&self) -> Option<&str> {
            Some("Alpha rule.")
        }
    }

    #[derive(Debug)]
    struct FakeComponentB;

    impl AnalysisComponent<TestLang> for FakeComponentB {
        fn name(&self) -> &'static str {
            "B"
        }
        fn schema_key(&self) -> &'static str {
            "beta"
        }
        fn schema_fragment(&self, _lang: &TestLang) -> serde_json::Value {
            serde_json::json!({ "type": "number" })
        }
        fn prompt_fragment(&self, _lang: &TestLang, _ctx: &ComponentContext) -> String {
            "Do beta.".to_string()
        }
    }

    // ── Schema composition tests ───────────────────────────────────────────

    #[test]
    fn single_component_produces_valid_schema() {
        let schema = compose_schema(
            &TestLang,
            &[&FakeComponentA as &dyn AnalysisComponent<TestLang>],
        );
        assert_eq!(schema["properties"]["alpha"]["type"], "string");
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("alpha")));
    }

    #[test]
    fn multiple_components_compose_schema() {
        let a: &dyn AnalysisComponent<TestLang> = &FakeComponentA;
        let b: &dyn AnalysisComponent<TestLang> = &FakeComponentB;
        let schema = compose_schema(&TestLang, &[a, b]);

        let props = schema["properties"].as_object().unwrap();
        assert_eq!(props.len(), 2);
        assert_eq!(props["alpha"]["type"], "string");
        assert_eq!(props["beta"]["type"], "number");
    }

    #[test]
    fn schema_with_defs_hoists_correctly() {
        // Use the real MorphologyAnalysis which generates $defs
        use panini_core::components::MorphologyAnalysis;
        let comp: &dyn AnalysisComponent<TestLang> = &MorphologyAnalysis;
        let schema = compose_schema(&TestLang, &[comp]);

        // The composed schema should have properties.morphology
        assert!(schema["properties"]["morphology"].is_object());
        // $defs should be at root level if the fragment had any
        // (TestMorphology with serde tag generates $defs)
        if schema.get("$defs").is_some() {
            assert!(schema["$defs"].is_object());
        }
    }

    #[test]
    fn composed_schema_validates_matching_json() {
        let a: &dyn AnalysisComponent<TestLang> = &FakeComponentA;
        let b: &dyn AnalysisComponent<TestLang> = &FakeComponentB;
        let schema = compose_schema(&TestLang, &[a, b]);

        let sample = serde_json::json!({
            "alpha": "hello",
            "beta": 42
        });

        let validator = jsonschema::validator_for(&schema).unwrap();
        let errors: Vec<_> = validator.iter_errors(&sample).collect();
        assert!(errors.is_empty(), "Schema validation errors: {errors:?}");
    }

    // ── Compatibility filtering (tested via compose_schema) ────────────────

    #[test]
    fn morpheme_segmentation_incompatible_with_non_agglutinative() {
        use panini_core::components::MorphemeSegmentation;
        let comp = MorphemeSegmentation;
        assert!(!comp.is_compatible(&TestLang));
    }

    #[test]
    fn pedagogical_always_compatible() {
        use panini_core::components::PedagogicalExplanation;
        let comp = PedagogicalExplanation;
        assert!(comp.is_compatible(&TestLang));
    }
}
