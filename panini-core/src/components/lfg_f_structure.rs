use std::fmt::Debug;

use crate::component::{AnalysisComponent, ComponentContext};
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
        let schema = r#gen.into_root_schema_for::<LfgWireAnalysis>();
        serde_json::to_value(&schema).unwrap()
    }

    fn prompt_fragment(&self, _lang: &L, _ctx: &ComponentContext) -> String {
        "Produce one LFG f-structure analysis of the complete sentence in its source language:\n\
         - `root` identifies the main clause. Give each structure a unique id and include only \
           structures reachable from root. The graph must be finite and acyclic; references \
           may share the same structure.\n\
         - `pred.lemma` is the source-language dictionary form, not a translation. \
           `pred.arguments` lists the selected grammatical functions of this construction. \
           Use an empty list for nominal predicates and `pred: null` for feature-only \
           structures such as TNS-ASP.\n\
         - Each attribute name occurs once, in UPPER CASE, with digits, hyphens or underscores \
           where needed. Use atom values for grammatical features, references for embedded \
           structures, and sets of distinct structure ids for set-valued attributes. \
           Represent PRED only through `pred`.\n\
         - Governed functions are SUBJ, OBJ, OBJ2, OBJ_<TYPE>, OBL, OBL_<TYPE>, COMP, XCOMP \
           and PREDLINK. Every selected argument must have a reference, and every governed \
           function present must appear in `pred.arguments`. Respect voice and word order; \
           grammatical functions are not semantic roles such as AGENT or PATIENT.\n\
         - COMP is a closed clausal complement; XCOMP is an open complement. For functional \
           control, share the same subject structure id between the controlling function \
           and XCOMP.SUBJ instead of copying it. Semantic coreference alone does not imply \
           structure sharing.\n\
         - ADJUNCT is a set of modifier structures, including relative clauses, not a selected \
           argument. Distinguish modification from complementation. TOPIC and FOCUS, when \
           justified, are references outside the selected argument list.\n\
         - Encode only grammatical features justified for the source language, such as NUM, \
           PERS, CASE, GEND, DEF and TNS-ASP. Do not impose another language's categories. \
           Represent implicit arguments when the construction warrants them and use the \
           construction's frame for passive or other valency-changing forms."
            .to_string()
    }

    fn validate(&self, _lang: &L, section: &serde_json::Value) -> Result<(), String> {
        let wire: LfgWireAnalysis =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        wire.resolve().map(|_| ())
    }

    fn post_process(&self, _lang: &L, section: &mut serde_json::Value) -> Result<(), String> {
        let wire: LfgWireAnalysis =
            serde_json::from_value(section.clone()).map_err(|e| e.to_string())?;
        let resolved = wire.resolve()?;
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
                {"name":"SUBJ","value":{"kind":"reference","target":"f1"}},
                {"name":"XCOMP","value":{"kind":"reference","target":"f2"}},
                {"name":"TNS-ASP","value":{"kind":"reference","target":"f3"}}
             ]},
            {"id":"f1","pred":{"lemma":"fermier","arguments":[]},"attributes":[
                {"name":"NUM","value":{"kind":"atom","value":"sg"}},
                {"name":"DEF","value":{"kind":"atom","value":"+"}}
             ]},
            {"id":"f2","pred":{"lemma":"partir","arguments":["SUBJ"]},
             "attributes":[{"name":"SUBJ","value":{"kind":"reference","target":"f1"}}]},
            {"id":"f3","pred":null,
             "attributes":[{"name":"TENSE","value":{"kind":"atom","value":"pres"}}]}
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
        assert!(section["structures"][3]["pred"].is_null());
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
            ("semantic_role", "invalid or duplicate predicate argument"),
        ];
        for (case, expected) in cases {
            let mut section = example();
            match *case {
                "root" => section["root"] = "missing".into(),
                "duplicate_id" => section["structures"][1]["id"] = "f0".into(),
                "dangling" => {
                    section["structures"][0]["attributes"][0]["value"]["target"] = "missing".into()
                }
                "cycle" => {
                    section["structures"][2]["attributes"][0]["value"]["target"] = "f0".into()
                }
                "unreachable" => section["structures"]
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::json!({"id":"unused","pred":null,"attributes":[]})),
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
            .push(serde_json::json!({"name":"ADJUNCT","value":{"kind":"set","members":["f4"]}}));
        section["structures"].as_array_mut().unwrap().push(
            serde_json::json!({"id":"f4","pred":{"lemma":"demain","arguments":[]},"attributes":[]}),
        );
        check(&section).unwrap();
        let attrs = section["structures"][0]["attributes"]
            .as_array_mut()
            .unwrap();
        attrs.last_mut().unwrap()["value"]["members"] = serde_json::json!(["f4", "f4"]);
        assert!(
            check(&section)
                .unwrap_err()
                .contains("duplicate set member")
        );
    }
}
