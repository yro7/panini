# Component Lifecycle

The Pāṇini extractor orchestrates the flow from the ISO code to the validated result. Understanding this cycle is essential for mastering LLM cleaning and validation.

---

## 🏗 Extraction Orchestration

```mermaid
sequenceDiagram
    participant User
    participant Extractor
    participant Composer
    participant Component
    participant LLM

    User->>Extractor: extract(Request, Components)
    Extractor->>Component: is_compatible(Lang)?
    Extractor->>Composer: compose_schema & compose_prompt
    Composer->>Component: schema_fragment() & prompt_fragment()
    Composer-->>Extractor: Combined Schema + XML Prompt
    Extractor->>LLM: Send Request (via rig)
    LLM-->>Extractor: Raw JSON String
    Extractor->>Component: pre_process(raw)
    Extractor->>Extractor: Parse JSON
    Extractor->>Component: validate(section)
    Extractor->>Component: post_process(section)
    Extractor-->>User: ExtractionResult
```

---

## 🛠 Lifecycle Hooks

### 1. Pre-processing (`pre_process`)
Cleaning the raw JSON before parsing.

- **Usage**: Normalize POS tags from the LLM (e.g., `ADJ` -> `adjective`), remove stray characters.
- **Signature**: `fn pre_process(&self, raw: &str) -> String`

### 2. Validation (`validate`)
Semantic checking after parsing.

- **Usage**: Ensure a lemma isn't empty or that morphological categories are consistent.
- **Signature**: `fn validate(&self, lang: &L, section: &Value) -> Result<(), String>`

### 3. Post-processing (`post_process`)
Final mutation of the result before returning.

- **Usage**: Enrich the result with static data from a dictionary, or reformat a pedagogical explanation.
- **Signature**: `fn post_process(&self, lang: &L, section: &mut Value) -> Result<(), String>`

**Sample Output (Post-Processing Enrichment):**
```json
// Before validation/enrichment
{ "lemma": "studentka", "pos": "noun" }

// After post_process (dictionary lookup)
{ "lemma": "studentka", "pos": "noun", "translation": "female student" }
```

---

## 📦 Composition and Hoisting

When composing the schema, Pāṇini performs **$defs Hoisting**. This is crucial for validation.

```mermaid
graph TD
    subgraph "Component A Fragment"
        A1["Property: KeyA"]
        A2["$defs: {Def1, Def2}"]
    end
    subgraph "Component B Fragment"
        B1["Property: KeyB"]
        B2["$defs: {Def3}"]
    end

    A2 -->|Hoist| RootDefs
    B2 -->|Hoist| RootDefs

    RootDefs["Root $defs: {Def1, Def2, Def3}"]
    RootProps["Root properties: {KeyA, KeyB}"]

    RootDefs --- RootSchema
    RootProps --- RootSchema
    
    RootSchema["Final Composed Schema"]
```

!!! warning "Schema Validation"
    If your component uses complex structures with `$ref` references, ensure that the `$defs` definitions are present in the fragment. The framework will automatically hoist them to the root level for validation to work.
