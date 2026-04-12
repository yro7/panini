# Pāṇini

**A LLM-powered linguistic feature extraction & analysis framework.**

Pāṇini allows you to describe a language's morphology as Rust types, write extraction directives, and let the pipeline handle the rest: prompt assembly, JSON schema generation, LLM orchestration, response parsing, and validation.

<div class="grid cards" markdown="1">

-   :material-earth: __Linguistic Agnosticism__

    ---

    No universal schema imposed. Each language defines exactly the features it needs.

-   :material-shield-check: __Type-Safe Guarantee__

    ---

    Morphology validated at compile-time. Auto-generated JSON schemas. Every LLM response is verified against the target schema, with automatic retry and self-correction logic to ensure result integrity.

-   :material-language-rust: __Multi-Language Access__

    ---

    High-performance Rust core, accessible via CLI or Python package (`panini-lang`).

-   :material-sigma: __Corpus Analysis__

    ---

    Transform raw extractions into statistical coverage reports and lexical inventories.

</div>

---

## 🚀 Panini in 1 Minute

### 1. Describe Your Language
Model your grammar using standard Rust types. Your types become the "source of truth" for the AI.

```rust
pub enum PolishMorphology {
    Noun { lemma: String, gender: Gender, case: Case },
    Verb { lemma: String, aspect: Aspect, tense: Tense },
}
```

**Generated JSON Schema (Fragment):**
```json
{
  "anyOf": [
    {
      "type": "object",
      "properties": {
        "pos": { "const": "noun" },
        "lemma": { "type": "string" },
        "gender": { "$ref": "#/$defs/Gender" },
        "case": { "$ref": "#/$defs/Case" }
      }
    }
  ]
}
```

### 2. Extract!
Use the CLI or Python package to analyze any text.

```python
import panini
result = panini.extract(
    language="pol",
    text="Studentka czyta książkę.",
    targets=["studentka"]
)
```

**Sample Extraction Result:**
```json
{
  "morphology": {
    "target_features": [
      {
        "word": "studentka",
        "morphology": {
          "pos": "noun",
          "lemma": "studentka",
          "gender": "feminine",
          "case": "nominative"
        }
      }
    ]
  }
}
```

### 3. Analyze the Lexicon
Generate a statistical analysis of the corpus to track coverage and frequency.
 
**Extraction Output Sample (Polish):**
```json
{
  "word": "studentka",
  "morphology": {
    "pos": "noun",
    "lemma": "studentka",
    "gender": "feminine",
    "number": "singular",
    "case": "nominative"
  }
}
```
 
**Aggregation Report Sample:**
```text
[NOUN] total: 15
  |- case [3/7]: nominative(8), accusative(5), genitive(2)
  |- gender [2/3]: feminine(10), masculine(5)
```

---

## 📚 Where to Start?

-   **Researchers & Linguists**: Learn how to use the [Python Package](guides/python.md).
-   **Backend Developers**: Integrate Panini into your [Rust Project](guides/rust.md).
-   **Framework Architects**: Discover how to [Add a Language](languages/adding.md) or create a [Custom Component](components/creating.md).

---

!!! note "About the Name"
    Pāṇini is named after **Pāṇini**, the ancient Indian grammarian and author of the Aṣṭādhyāyī, the first systematic and formal description of the Sanskrit language. 
    
    *Note: Pāṇini Framework is a technical tool for feature extraction and does not strictly follow the specific "Paninian Framework" notation used in traditional NLP (e.g., Pāṇinian Syntactico-Semantic Relation Labels).*
