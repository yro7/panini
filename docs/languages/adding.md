# Adding a Language

Defining a new language in Pāṇini consists of describing its morphology as Rust types. The framework then translates these types into JSON schemas for the AI.

---

## 🗺 Quick Selection
| Approach | Effort | Control | Best for... |
| :--- | :--- | :--- | :--- |
| **[Manual Implementation](#basic-steps)** | ⏳ High | 🛠 Total | Deeply custom or complex grammars. |
| **[Automated Scaffolding](#usage-automated-scaffolding)** | 🚀 Low | 🧪 Partial | Quick prototyping and common languages. |

---

## 🔧 Basic Steps

### 1. Define Morphology Enums
Each linguistic category (Case, Gender, Tense) must be a Rust enum.

!!! tip "Reuse Shared Enums"
    Check `panini-core/src/morphology_enums.rs` for common categories like `Person`, `BinaryNumber`, `TernaryGender`, `SlavicAspect`, etc. Use these instead of redefining them locally.

```rust
// Language-specific enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::ClosedValues)]
#[serde(rename_all = "snake_case")]
pub enum PolishCase {
    Nominative, Genitive, Dative, Accusative, Instrumental, Locative, Vocative,
}

// Using shared enums
use panini_core::traits::{BinaryNumber, TernaryGender, SlavicAspect};
```

### 2. Create the Morphology Enum
The `Morphology` enum defines the Part-of-Speech (POS) categories and their fields. Derive `MorphologyInfo` to automatically generate POS tags and lemma accessors.

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema, panini_macro::MorphologyInfo)]
#[serde(tag = "pos", rename_all = "snake_case")]
pub enum PolishMorphology {
    Noun {
        lemma: String,
        gender: PolishGender, // Custom local enum
        number: BinaryNumber, // Shared enum
        case: PolishCase,     // Local enum
    },
    Verb {
        lemma: String,
        aspect: SlavicAspect, // Shared enum
        tense: PolishTense,
        person: Person,       // Shared enum
    },
    // ...
}
```

**Resulting Schema Structure:**
The framework translates `PolishMorphology` into a JSON schema ensuring the LLM only outputs valid POS tags (`noun`, `verb`) and valid field values.

```json
{
  "properties": {
    "pos": { "enum": ["noun", "verb"] },
    "lemma": { "type": "string" },
    "gender": { "enum": ["masculine", "feminine", "neuter"] },
    "case": { "enum": ["nominative", "genitive", "..."] }
  }
}
```

### 3. Implement LinguisticDefinition
This is where you define the language's identity and its extraction instructions.

```rust
pub struct Polish;

impl LinguisticDefinition for Polish {
    type Morphology = PolishMorphology;
    type GrammaticalFunction = (); // Non-agglutinative
    const ISO_CODE: &'static str = "pol";

    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn extraction_directives(&self) -> &str {
        "1. Extraction: Provide the lemma, the POS, and all morphological features.\n\
         2. Case: Use standard Polish case names (nominative, etc.).\n\
         3. Aspect: Distinguish between perfective and imperfective verbs."
    }
}
```

**Prompts Composition Effect:**
The framework will merge these directives into the global system prompt to steer the LLM's logic for the target language.

---

## 🛠 Registration and Deployment

To make a language usable by the CLI or API, it must be registered in the `generate_registry!` macro in `panini-langs/src/registry.rs`.

```rust
// panini-langs/src/registry.rs

use crate::{Arabic, French, Italian, Polish, Turkish};

generate_registry!(Polish, Turkish, Arabic, French, Italian, MyNewLanguage);
```

**Registration Effect:**
The `extract_erased` function will now be able to resolve the `"mynewlanguage"` key at runtime.

This macro automatically generates:
1. `extract_erased_with_components`: The unified entry point used by the CLI.
2. `supported_languages()`: A function returning the list of supported ISO codes.

---

## 🚀 Usage (Automated Scaffolding)

If you have configured your `panini.toml`, you can use the CLI to automatically generate the skeleton for a new language:

```bash
cargo run -p panini-cli -- add-language \
  --language "Japanese" \
  --iso-code jpn \
  --config panini.toml
```

!!! warning "Manual Verification"
    Although the scaffolding is LLM-assisted, ALWAYS verify the generated linguistic definitions to ensure they correctly describe the target language.
