use panini_core::components::LfgFStructure;
use panini_core::{
    AnalysisComponent, FieldKind, LfgAnalysis, LinguisticDefinition, MorphologyCatalog,
};
use panini_lang_langs::{English, French, Irish, Polish};
use serde_json::{Value, json};

fn sentence(morphology: Value) -> Value {
    json!({"root":"f0","structures":[
        {"id":"f0","pred":{"lemma":"read","arguments":["SUBJ"]},"attributes":[
            {"kind":"function","function":"SUBJ","target":"f1"}
        ],"morphology":null,"controller":null},
        {"id":"f1","pred":{"lemma":"reader","arguments":[]},"attributes":[],
         "morphology":morphology,"controller":null}
    ]})
}

fn bundle(category: &str, name: &str, value: &str) -> Value {
    json!({"category":category,"features":[{"name":name,"value":value}]})
}

fn accepted<L: LinguisticDefinition>(language: &L, section: &Value) -> bool {
    let schema = LfgFStructure.schema_fragment(language);
    jsonschema::validator_for(&schema)
        .unwrap()
        .is_valid(section)
        && LfgFStructure.validate(language, section).is_ok()
}

#[test]
fn canonical_values_come_from_the_existing_category_not_a_global_union() {
    assert!(accepted(
        &Polish,
        &sentence(bundle("noun", "number", "singular"))
    ));
    for alias in ["SG", "sg", "Singular"] {
        assert!(!accepted(
            &Polish,
            &sentence(bundle("noun", "number", alias))
        ));
    }
    assert!(accepted(
        &Polish,
        &sentence(bundle("noun", "case", "accusative"))
    ));
    assert!(!accepted(
        &Irish,
        &sentence(bundle("noun", "case", "accusative"))
    ));
    assert!(accepted(
        &Irish,
        &sentence(bundle("noun", "case", "nominative"))
    ));
    assert!(accepted(
        &Irish,
        &sentence(bundle("pronoun", "pronoun_case", "objective"))
    ));
    assert!(!accepted(
        &Irish,
        &sentence(bundle("noun", "pronoun_case", "objective"))
    ));
    assert!(!accepted(
        &Polish,
        &sentence(bundle("verb", "case", "accusative"))
    ));
}

#[test]
fn optional_features_are_exposed_without_changing_aggregation_catalogs() {
    let aggregated = <Polish as LinguisticDefinition>::Morphology::group_descriptors();
    let complete = <Polish as LinguisticDefinition>::Morphology::feature_descriptors();
    let verb = aggregated.iter().find(|g| g.key == "verb").unwrap();
    assert!(!verb.dimensions.iter().any(|d| d.name == "tense"));
    let verb = complete.iter().find(|g| g.key == "verb").unwrap();
    let tense = verb.dimensions.iter().find(|d| d.name == "tense").unwrap();
    assert!(matches!(&tense.kind, FieldKind::Closed(v) if v.contains(&"present")));
}

#[test]
fn tense_aspect_has_one_structure_and_canonical_inventory() {
    let mut section = sentence(bundle("noun", "number", "singular"));
    section["structures"][0]["attributes"]
        .as_array_mut()
        .unwrap()
        .push(json!({"kind":"tense_aspect","target":"f2"}));
    section["structures"].as_array_mut().unwrap().push(json!({
        "id":"f2","pred":null,"attributes":[],"controller":null,
        "morphology":bundle("verb", "tense", "present")
    }));
    assert!(accepted(&Polish, &section));
    let mut stored = section.clone();
    LfgFStructure.post_process(&Polish, &mut stored).unwrap();
    assert_eq!(stored["structures"][2]["attributes"]["TENSE"], "present");
    let parsed: LfgAnalysis = serde_json::from_value(stored.clone()).unwrap();
    parsed.validate_for_language(&Polish).unwrap();
    stored["structures"][0]["attributes"]["TNS-ASP"] = "present".into();
    assert!(
        serde_json::from_value::<LfgAnalysis>(stored)
            .unwrap()
            .validate_for_language(&Polish)
            .is_err()
    );
    section["structures"][2]["morphology"] = bundle("verb", "tense", "PRES");
    assert!(!accepted(&Polish, &section));
    section["structures"][2]["morphology"] = bundle("noun", "case", "nominative");
    assert!(!accepted(&Polish, &section));
}

#[test]
fn schema_itself_constrains_categories_values_and_relation_types() {
    let schema = LfgFStructure.schema_fragment(&Irish);
    let serialized = serde_json::to_string(&schema).unwrap();
    assert!(!serialized.contains("\"const\""));
    assert!(!serialized.contains("\"oneOf\""));
    let validator = jsonschema::validator_for(&schema).unwrap();
    assert!(validator.is_valid(&sentence(bundle("noun", "case", "nominative"))));
    assert!(!validator.is_valid(&sentence(bundle("noun", "case", "accusative"))));
    assert!(!validator.is_valid(&sentence(bundle("noun", "pronoun_case", "objective"))));
    let mut section = sentence(Value::Null);
    section["structures"][0]["attributes"][0]["function"] = "AGENT".into();
    assert!(!validator.is_valid(&section));
    section["structures"][0]["attributes"][0] = json!({"kind":"adjunct","members":"f1"});
    assert!(!validator.is_valid(&section));
    section["structures"][0]["pred"]["arguments"] = json!(["ADJUNCT"]);
    assert!(!validator.is_valid(&section));
}

#[test]
fn v1_persisted_results_still_deserialize_and_validate() {
    let report: Value =
        serde_json::from_str(include_str!("../../examples/data/lfg_live.json")).unwrap();
    for case in report["cases"].as_array().unwrap() {
        let analysis: LfgAnalysis =
            serde_json::from_value(case["result"]["lfg_f_structure"].clone()).unwrap();
        analysis.validate().unwrap();
    }
}

#[test]
fn current_live_results_revalidate_with_their_language() {
    let report: Value =
        serde_json::from_str(include_str!("../../examples/data/lfg_grammar_live.json")).unwrap();
    for case in report["cases"].as_array().unwrap() {
        let analysis: LfgAnalysis =
            serde_json::from_value(case["result"]["lfg_f_structure"].clone()).unwrap();
        match case["language"].as_str().unwrap() {
            "eng" => analysis.validate_for_language(&English),
            "fra" => analysis.validate_for_language(&French),
            "gle" => analysis.validate_for_language(&Irish),
            other => panic!("Unexpected live language {other}"),
        }
        .unwrap();
    }
}
