# Rust Integration

Pāṇini is a Rust workspace composed of 7 crates. It's designed to be used as a library (`panini-engine`) or through language implementations (`panini-langs`).

---

## 🔧 Installation

Add `panini-engine`, `panini-langs`, and `rig-core` to your `Cargo.toml`.

```toml
[dependencies]
panini-engine = { path = "…" }
panini-langs   = { path = "…" }
rig-core       = "0.33"
tokio          = { version = "1", features = ["full"] }
```

---

## 🚀 Basic Usage

The engine accepts any `rig::completion::CompletionModel`.

```rust
use panini_engine::{extract_features_via_llm, ExtractionOptions, ExtractionRequest};
use panini_engine::prompts::ExtractorPrompts;
use panini_langs::polish::Polish;
use rig::providers::openai;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize LLM client (via rig)
    let client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?)?;
    let model  = client.completion_model("gpt-4o");
    
    // 2. Load prompts
    let prompts = ExtractorPrompts::load("prompts/default.yml")?;

    // 3. Create an extraction request
    let request = ExtractionRequest::builder()
        .content("Dał kotowi mleko.")
        .targets(vec!["kotowi".to_string()])
        .build();

    let options = ExtractionOptions {
        temperature: 0.2,
        max_tokens: 4096,
        previous_attempt: None,
        extractor_prompts: &prompts,
    };

    // 4. Extract!
    let result = extract_features_via_llm(&Polish, &model, &request, options).await?;

    println!("{:#?}", result.target_features);
    Ok(())
}
```

**Sample Output:**
```text
[
    TargetFeature {
        word: "kotowi",
        morphology: Noun { lemma: "kot", case: Dative, ... }
    }
]
```

---

## 🧩 Multi-component Extraction

To dynamically select components to run, use `extract_with_components`.

```rust
use panini_engine::extractor::extract_with_components;
use panini_core::components::{MorphologyAnalysis, LeipzigAlignment};

let components = vec![
    &MorphologyAnalysis as &dyn AnalysisComponent<Polish>,
    &LeipzigAlignment as &dyn AnalysisComponent<Polish>,
];

let res = extract_with_components(
    &Polish, &model, &request, &components, options
).await?;
```

**Resulting Structure:**
The `res` object will contain both the `morphology` and `leipzig_alignment` fields, populated with data validated against the respective component schemas.

---

## 🪄 Using the PaniniResult Macro

The `#[derive(PaniniResult)]` macro is the recommended way to consume results in a type-safe manner.

```rust
#[derive(PaniniResult)]
pub struct MyAnalysis<L: LinguisticDefinition> {
    #[component(MorphologyAnalysis)]
    pub morphology: MorphologyResult<L::Morphology>,
    
    #[component(LeipzigAlignment)]
    pub leipzig: Option<LeipzigAlignmentResult>,
}

// Extraction becomes a single typesafe call:
let res: MyAnalysis<Arabic> = MyAnalysis::extract(&lang, &model, &req, opts).await?;

println!("Target lemma: {}", res.morphology.target_features[0].lemma);
```

---

## 🌍 The Dynamic Registry

If you're handling languages by their ISO code (e.g., in an API), use the registry in `panini-langs`.

```rust
use panini_langs::registry;

// Erased extraction (lose static types, gain runtime flexibility)
let result: ExtractionResult = registry::extract_erased(
    "ara", &model, &request, options
).await?;
```

**Registry Resolution Effect:**
The code dynamically fetches the `Arabic` definition from the internal registry. If the ISO code `"ara"` is not found, it returns an `Err(LanguageNotFound)`.

---

!!! info "Type Safety"
    Pāṇini guarantees compile-time type safety. If you change your morphology enum, the framework will force you to update your extraction calls.
