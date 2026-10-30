use std::fmt::Debug;

use crate::component::{AnalysisComponent, ComponentContext};
use crate::lfg::LfgMorphologySchema;
use crate::lfg::wire::LfgWireAnalysis;
use crate::traits::LinguisticDefinition;

/// Proposes a language-independent LFG f-structure; does not run an XLE grammar.
#[derive(Debug, Clone, Default)]
pub struct LfgFStructure;

impl<L: LinguisticDefinition> crate::component::ComponentRequires<L> for LfgFStructure {}

impl<L: LinguisticDefinition> AnalysisComponent<L> for LfgFStructure {
    fn name(&self) -> &'static str {
        "LFG F-Structure"
    }

    fn schema_key(&self) -> &'static str {
        "lfg_f_structure"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        let r#gen = schemars::SchemaGenerator::default();
        let schema = r#gen.into_root_schema_for::<LfgWireAnalysis<LfgMorphologySchema<L>>>();
        serde_json::to_value(&schema).unwrap()
    }

    fn prompt_fragment(&self, _lang: &L, _ctx: &ComponentContext) -> String {
        "Produce one LFG f-structure for the complete source sentence, using the supplied grammar:\n\
         - `root` identifies the main predicate. Give each structure a unique id. The graph \
           must be acyclic and every structure must be reachable; references may share a node.\n\
         - `pred.lemma` is the source-language dictionary form. `pred.arguments` lists the \
           governed functions selected in this construction, accounting for voice and valency. \
           Nominal predicates select no arguments. Use pred:null only for feature bundles.\n\
         - Attributes are typed: `function` links a governed function to one target; `adjunct` \
           holds a set of modifier ids; `topic` and `focus` link discourse functions; \
           `tense_aspect` references a feature-only structure; `definite` is boolean; `voice` \
           uses the schema's canonical voice values. Each function or attribute occurs once.\n\
         - SUBJ is grammatical subject and OBJ grammatical object, not AGENT/PATIENT. Every \
           selected function must be linked, and every governed link must be selected. \
           ADJUNCT, TOPIC and FOCUS are never selected arguments.\n\
         - COMP is a closed clausal complement; XCOMP is open. When a node has XCOMP, set \
           `controller` to the governed function whose structure is shared with XCOMP.SUBJ. \
           Subject control (try/essayer): controller SUBJ. Object control (persuade): \
           controller OBJ. Reuse the same id, do not copy the subject. Without XCOMP, \
           controller is null. Every XCOMP target MUST include SUBJ in pred.arguments and \
           a function attribute linking SUBJ to that controller's target id, even when the \
           infinitive has no written subject. Semantic coreference alone is not sharing.\n\
         - Modification, including relative clauses, belongs in adjunct sets, not COMP. \
           Implicit arguments may be represented when justified by the construction.\n\
         - `morphology` is an optional PARTIAL feature projection from one category, not a \
           complete token analysis. Choose its category and feature values from the language's \
           schema. Keep canonical spellings exactly: do not replace singular by SG or sg. \
           Choose the correct category (noun, pronoun, verb, etc.), never merge inventories. \
           Null means no morphological features are asserted.\n\
         - Put tense, aspect, mood and evidentiality in a feature-only morphology bundle, \
           linked through `tense_aspect`. Its inventory comes from the appropriate category, \
           typically verb; other features such as number/case stay on the relevant predicate. \
           Do not infer case just from SUBJ/OBJ, invent unsupported features, or require every \
           morphological field. Syntactic features can come from multiple words or auxiliaries."
            .to_string()
    }

    fn validate(&self, lang: &L, section: &serde_json::Value) -> Result<(), String> {
        let wire: LfgWireAnalysis =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        wire.resolve(lang).map(|_| ())
    }

    fn post_process(&self, lang: &L, section: &mut serde_json::Value) -> Result<(), String> {
        let wire: LfgWireAnalysis =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        let resolved = wire.resolve(lang)?;
        *section = serde_json::to_value(&resolved).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn needs_pedagogical_context(&self) -> bool {
        false
    }

    fn needs_extraction_directives(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lfg::LfgAnalysis;
    use crate::test_support::StubLanguage;

    fn example() -> serde_json::Value {
        serde_json::json!({"root":"f0","structures":[
            {"id":"f0","pred":{"lemma":"essayer","arguments":["SUBJ","XCOMP"]},
             "attributes":[
                {"kind":"function","function":"SUBJ","target":"f1"},
                {"kind":"function","function":"XCOMP","target":"f2"}
             ],"morphology":null,"controller":"SUBJ"},
            {"id":"f1","pred":{"lemma":"fermier","arguments":[]},
             "attributes":[{"kind":"definite","value":true}],"morphology":null,"controller":null},
            {"id":"f2","pred":{"lemma":"partir","arguments":["SUBJ"]},
             "attributes":[{"kind":"function","function":"SUBJ","target":"f1"}],
             "morphology":null,"controller":null}
        ]})
    }

    fn check(section: &serde_json::Value) -> Result<(), String> {
        LfgFStructure.validate(&StubLanguage, section)
    }

    #[test]
    fn control_sharing_survives_post_processing_and_typed_round_trip() {
        let mut section = example();
        check(&section).unwrap();
        LfgFStructure
            .post_process(&StubLanguage, &mut section)
            .unwrap();
        let analysis: LfgAnalysis = serde_json::from_value(section.clone()).unwrap();
        analysis.validate().unwrap();
        assert_eq!(section["structures"][0]["attributes"]["SUBJ"]["ref"], "f1");
        assert_eq!(section["structures"][2]["attributes"]["SUBJ"]["ref"], "f1");
        assert_eq!(
            section["structures"][0]["attributes"]["XCOMP-CONTROLLER"],
            "SUBJ"
        );
    }

    #[test]
    fn object_control_shares_obj_instead_of_subj() {
        let mut section = example();
        section["structures"][0]["pred"]["lemma"] = "persuader".into();
        section["structures"][0]["pred"]["arguments"] = serde_json::json!(["SUBJ", "OBJ", "XCOMP"]);
        section["structures"][0]["controller"] = "OBJ".into();
        section["structures"][0]["attributes"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({"kind":"function","function":"OBJ","target":"f3"}));
        section["structures"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({
                "id":"f3","pred":{"lemma":"voisin","arguments":[]},
                "attributes":[],"morphology":null,"controller":null
            }));
        // The original subject-controlled analysis is invalid for this declared controller.
        assert!(check(&section).unwrap_err().contains("declared OBJ"));
        section["structures"][2]["attributes"][0]["target"] = "f3".into();
        check(&section).unwrap();
        LfgFStructure
            .post_process(&StubLanguage, &mut section)
            .unwrap();
        assert_eq!(
            section["structures"][0]["attributes"]["OBJ"],
            section["structures"][2]["attributes"]["SUBJ"]
        );
        assert_ne!(
            section["structures"][0]["attributes"]["SUBJ"],
            section["structures"][2]["attributes"]["SUBJ"]
        );
    }

    #[test]
    fn open_complements_require_declared_shared_control_but_comp_does_not() {
        let mut section = example();
        section["structures"][0]["controller"] = serde_json::Value::Null;
        assert!(
            check(&section)
                .unwrap_err()
                .contains("explicit XCOMP-CONTROLLER")
        );
        section["structures"][0]["pred"]["arguments"] = serde_json::json!(["SUBJ", "COMP"]);
        section["structures"][0]["attributes"][1]["function"] = "COMP".into();
        check(&section).unwrap();
        section["structures"][0]["controller"] = "SUBJ".into();
        assert!(check(&section).unwrap_err().contains("without XCOMP"));
    }

    #[test]
    fn missing_implicit_subject_feedback_names_the_exact_shared_target() {
        let mut section = example();
        section["structures"][2]["pred"]["arguments"] = serde_json::json!([]);
        section["structures"][2]["attributes"] = serde_json::json!([]);
        let error = check(&section).unwrap_err();
        assert!(error.contains("XCOMP target f2"), "{error}");
        assert!(error.contains("SUBJ in pred.arguments"), "{error}");
        assert!(error.contains("SUBJ targeting f1"), "{error}");
    }

    #[test]
    fn function_inventory_matches_serde_and_rejects_non_functions() {
        use crate::aggregable::ClosedValues;
        use crate::lfg::LfgFunction;
        for name in LfgFunction::all_variants() {
            let function: LfgFunction = serde_json::from_value((*name).into()).unwrap();
            assert_eq!(serde_json::to_value(function).unwrap(), *name);
            assert_eq!(function.as_str(), *name);
        }
        for name in ["ADJUNCT", "AGENT", "PATIENT", "OBL_MADE_UP"] {
            assert!(serde_json::from_value::<LfgFunction>(name.into()).is_err());
        }
    }

    #[test]
    fn invalid_graphs_and_frames_are_rejected() {
        let cases: &[(&str, &str)] = &[
            ("root", "missing"),
            ("duplicate_id", "duplicate"),
            ("dangling", "dangling"),
            ("cycle", "Cycle"),
            ("unreachable", "reachable"),
            ("missing_argument", "selected argument"),
            ("unlicensed_argument", "absent from pred.arguments"),
            ("duplicate_attribute", "duplicate attribute"),
            ("duplicate_argument", "duplicate predicate argument"),
            ("semantic_role", "unknown variant"),
        ];
        for (case, expected) in cases {
            let mut section = example();
            match *case {
                "root" => section["root"] = "missing".into(),
                "duplicate_id" => section["structures"][1]["id"] = "f0".into(),
                "dangling" => {
                    section["structures"][0]["attributes"][0]["target"] = "missing".into()
                }
                "cycle" => {
                    section["structures"][2]["attributes"][0]["target"] = "f0".into()
                }
                "unreachable" => section["structures"]
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::json!({"id":"unused","pred":null,"attributes":[],"morphology":null,"controller":null})),
                "missing_argument" => {
                    section["structures"][0]["attributes"]
                        .as_array_mut()
                        .unwrap()
                        .remove(0);
                }
                "unlicensed_argument" => {
                    section["structures"][0]["pred"]["arguments"] = serde_json::json!(["SUBJ"])
                }
                "duplicate_attribute" => {
                    let attr = section["structures"][0]["attributes"][0].clone();
                    section["structures"][0]["attributes"]
                        .as_array_mut()
                        .unwrap()
                        .push(attr);
                }
                "duplicate_argument" => {
                    section["structures"][0]["pred"]["arguments"] =
                        serde_json::json!(["SUBJ", "SUBJ", "XCOMP"])
                }
                "semantic_role" => {
                    section["structures"][0]["pred"]["arguments"] = serde_json::json!(["AGENT"])
                }
                _ => unreachable!(),
            }
            let error = check(&section).expect_err(case);
            assert!(error.contains(expected), "{case}: {error}");
        }
    }

    #[test]
    fn adjunct_sets_are_not_selected_arguments() {
        let mut section = example();
        section["structures"][0]["attributes"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({"kind":"adjunct","members":["f4"]}));
        section["structures"].as_array_mut().unwrap().push(
            serde_json::json!({"id":"f4","pred":{"lemma":"demain","arguments":[]},"attributes":[],"morphology":null,"controller":null}),
        );
        check(&section).unwrap();
        let attrs = section["structures"][0]["attributes"]
            .as_array_mut()
            .unwrap();
        attrs.last_mut().unwrap()["members"] = serde_json::json!(["f4", "f4"]);
        assert!(
            check(&section)
                .unwrap_err()
                .contains("duplicate set member")
        );
    }
}
