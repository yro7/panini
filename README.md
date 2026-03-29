# Pāṇini

**A framework for defining and running linguistic feature extraction with LLMs.**

Pāṇini lets you define *what* linguistic features to extract and *how* to extract them, for any language : You describe your language's morphology as Rust types, write extraction directives, and the framework handles the rest: prompt assembly, JSON schema generation, LLM orchestration, response parsing, and validation.

Pāṇini doesn't impose a universal schema — you define exactly the features that matter for your language, and the framework builds the extraction pipeline around your definitions.

## What you define, what the framework does

**You define:**
- A **morphology enum** — the features you want extracted (POS, case, tense, aspect, gender… whatever your language needs)
- **Extraction directives** — natural-language instructions that guide the LLM on how to analyze your language
- **Optional morpheme segmentation** — for agglutinative languages, a morpheme inventory with validation rules
- **Optional post-processing** — hooks to validate or enrich the LLM's output after parsing

**The framework handles:**
- **Prompt assembly** — combines your directives, the generated schema, learner context, and pedagogical focus into a structured prompt
- **JSON schema generation** — automatically derived from your Rust types, so the LLM is constrained to return exactly what you defined
- **LLM orchestration** — provider-agnostic; bring your own client (OpenAI, Anthropic, Google, local)
- **Response parsing & validation** — deserializes the LLM output into your typed structs, rejects malformed responses, supports retry with self-correction

## Design principles

- **No universal schema.** Each language defines its own morphology enum with exactly the features it needs. Polish has 7 cases and verbal aspect. Arabic has triliteral roots and wazn patterns. There is no lowest-common-denominator `Morphology` struct.
- **Type safety over convention.** Morphology variants are strongly typed Rust enums, validated at compile time via `#[derive(MorphologyInfo)]`. Every variant must carry a `lemma`. The LLM's JSON output is parsed into these types and rejected if it doesn't conform.
- **LLM as untrusted source.** Responses are validated against a JSON schema, deserialized into typed structs, then post-processed. On parse failure, the raw output and error are returned for retry with self-correction.
- **Provider-agnostic.** The `LlmClient` trait is a single async method (`chat_completion`). Wrap any provider (OpenAI, Anthropic, Google, local) in a few lines.
- **Opt-in complexity.** A simple language (Polish) needs a morphology enum and a few directives. An agglutinative language (Turkish) can opt into morpheme inventories, segmentation, and validation. You only implement what you need.

## Workspace structure

```
panini/              # Facade crate, re-exports everything
panini-core/         # Traits, domain types, morphology enums
panini-engine/       # LLM extraction pipeline, prompt assembly
panini-langs/        # Per-language implementations (Polish, Arabic, Turkish)
panini-macro/        # #[derive(MorphologyInfo)] proc macro
```

### panini-core

Core traits and types:

- **`LinguisticDefinition`** -- the main trait a language implements. Defines `Morphology` and `GrammaticalFunction` associated types, ISO code, scripts, extraction directives.
- **`MorphologyInfo`** -- derived trait providing `.lemma()`, `.pos_tag()`, `.pos_label()` on morphology enums.
- **`Agglutinative`** -- opt-in trait for morpheme segmentation (inventory, directives, validation).
- **`FeatureExtractionResponse<M, F>`** -- the structured output: pedagogical explanation, target/context features, multiword expressions, optional morpheme segmentation.

### panini-engine

The extraction pipeline:

- **`extract_features_via_llm()`** -- core function. Takes a `LinguisticDefinition`, an `LlmClient`, an `ExtractionRequest`, and prompt config. Returns typed `FeatureExtractionResponse`.
- **`LlmClient`** trait -- single `chat_completion` method. Implement this to plug in your provider.
- **Prompt assembly** -- builds the system prompt from language directives, extraction schema, learner profile, and skill context.

### panini-langs

Reference language implementations:

- **Polish** -- 7 cases, 3 genders, verbal aspect, 5 POS categories
- **Arabic** -- triliteral roots, wazn patterns, case/state, reverse gender agreement
- **Turkish** -- agglutinative, with full morpheme inventory and segmentation

### panini-macro

Procedural macro crate:

- **`#[derive(MorphologyInfo)]`** -- generates `lemma()`, `pos_tag()`, `pos_label()` implementations and a `<Name>PosTag` enum from a `#[serde(tag = "pos")]` morphology enum.

## Usage

```rust
use panini::{extract_features_via_llm, ExtractionRequest, LlmClient};
use panini::panini_core::traits::LinguisticDefinition;
use panini_langs::polish::Polish;

// 1. Implement LlmClient for your provider
struct MyLlmClient { /* ... */ }

#[async_trait::async_trait]
impl LlmClient for MyLlmClient {
    async fn chat_completion(
        &self,
        request: &panini::panini_engine::llm::LlmRequest,
    ) -> anyhow::Result<panini::panini_engine::llm::LlmResponse> {
        // Call your LLM provider here
        todo!()
    }
}

// 2. Build an extraction request
let request = ExtractionRequest {
    content: card_json.to_string(),
    targets: vec!["kotowi".to_string()],
    pedagogical_context: Some("Dative case".to_string()),
    skill_path: Some("grammar/cases/dative".to_string()),
    learner_ui_language: "en".to_string(),
    linguistic_background: vec![],
    user_prompt: None,
};

// 3. Extract features
let result = extract_features_via_llm(
    &Polish,
    &my_client,
    &request,
    0.0,          // temperature
    4096,         // max tokens
    None,         // no previous attempt
    &prompts,
).await?;

// result.target_features  -- Vec<ExtractedFeature<PolishMorphology>>
// result.pedagogical_explanation -- HTML string
// result.multiword_expressions -- Vec<MultiwordExpression>
```

## Adding a language

1. Create `panini-langs/src/<language>.rs`
2. Define a `Morphology` enum with `#[derive(MorphologyInfo)]` and `#[serde(tag = "pos")]` -- every variant must have `lemma: String` as its first field
3. Implement `LinguisticDefinition` on a unit struct
4. For agglutinative languages, also implement `Agglutinative` with a morpheme inventory

See `panini-langs/src/polish.rs` or `panini-langs/src/turkish.rs` as references.

## Building

```bash
cargo build
cargo test
```

## License

MIT
