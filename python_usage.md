# Panini Python Integration: Developer & Usage Guide

This document is the definitive reference for utilizing the Panini framework via its Python package, `panini-lang`. It covers the API surface, internal orchestration, and advanced customization patterns.

---

## 1. File Map: Python Infrastructure

| Concept | Location | Purpose |
| :--- | :--- | :--- |
| **Rust Bindings** | `pynini/src/lib.rs` | PyO3 definitions and the bridge to the Rust engine. |
| **Package Config** | `pynini/pyproject.toml` | Build and metadata configuration for `maturin`. |
| **Testing Suite** | `pynini/test_pynini.py` | Integration tests and usage examples. |
| **Prompts** | `panini-cli/prompts/default.yml` | Embedded default extraction instructions. |

---

## 2. Core API Reference

The `panini` module provides a high-level interface to the extraction engine.

### `extract(...) -> dict`
The primary **synchronous** entry point. Built for ease of use in standard Python scripts or Jupyter notebooks.
- **Internal**: Orchestrates a dedicated `current_thread` Tokio runtime to manage the asynchronous Rust pipeline transparently.
- **Return Type**: A native Python dictionary containing the extraction results.

### `async_extract(...) -> Coroutine`
The **asynchronous** counterpart for use within `asyncio` loops.
- **Bridge**: Uses `pyo3-async-runtimes` to map Rust futures to Python awaitables.
- **Performance**: Ideal for concurrent extraction tasks in web servers (FastAPI/Flask) or high-throughput pipelines.

### `supported_languages() -> list[str]`
Returns a list of supported ISO 639-3 language codes (e.g., `["pol", "tur", "ara", "fra"]`). This list is dynamically generated from the Rust registry.

### `get_default_prompts() -> dict`
Returns the built-in, linguistically-informed extraction prompts as a Python dictionary.

---

## 3. Concrete Examples

### Basic Morphology Extraction
```python
import panini

# Synchronous extraction for a single word
result = panini.extract(
    provider="google",
    model="gemini-1.5-flash",
    api_key="...",
    language="pol",
    text="Studentka czyta książkę.",
    targets=["studentka"]
)

# Iterating over the morphology features
for feat in result['morphology']['target_features']:
    m = feat['morphology']
    print(f"Word: {feat['word']}, Lemma: {m['lemma']}, POS: {m['pos']}")
    if 'gender' in m:
        print(f"Gender: {m['gender']}")
```

### Full Analysis (Multiple Components)
Running morphology, pedagogical explanation, and Leipzig glossing at once.
```python
result = panini.extract(
    provider="openai",
    model="gpt-4o",
    api_key="...",
    language="fra",
    text="Il est allé à la plage.",
    targets=["allé"],
    components=["morphology", "pedagogical_explanation", "leipzig_alignment"]
)

# Rendering the HTML explanation
print("Pedagogical Explanation:")
print(result['pedagogical_explanation'])

# Accessing Leipzig glossing
print("\nLeipzig Alignment:")
for word in result['leipzig_alignment']['words']:
    print(f"{word['source']} -> {word['gloss']}")
```

### Asynchronous Pipeline
```python
import asyncio
import panini

async def analyze_text(text, targets):
    try:
        res = await panini.async_extract(
            provider="anthropic",
            model="claude-3-5-sonnet",
            api_key="...",
            language="ara",
            text=text,
            targets=targets,
            temperature=0.0 # Strict extraction
        )
        return res
    except Exception as e:
        print(f"Extraction failed: {e}")

asyncio.run(analyze_text("ذهب الولد إلى المدرسة", ["ذهب", "الولد"]))
```

---

## 4. Extraction Parameters

Both `extract()` and `async_extract()` share the same signature:

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `provider` | `str` | *(Required)* | LLM provider: `"openai"`, `"anthropic"`, or `"google"`. |
| `model` | `str` | *(Required)* | Model ID (e.g., `"gpt-4o"`, `"gemini-2.0-flash"`). |
| `api_key` | `str` | *(Required)* | Provider-specific API key. |
| `language` | `str` | *(Required)* | ISO 639-3 target language code. |
| `text` | `str` | *(Required)* | The sentence or text to analyze. |
| `targets` | `list[str]` | *(Required)* | Specific words to focus extraction on. |
| `prompts` | `str \| dict \| None` | `None` | Custom prompt overrides (YAML path or dictionary). |
| `components` | `list[str] \| None` | `None` | Selection of `AnalysisComponent` keys to run. |
| `ui_language` | `str` | `"English"` | Target language for pedagogical explanations. |
| `temperature` | `float` | `0.2` | Sampling temperature. |
| `max_tokens` | `int` | `4096` | Maximum response length. |

---

## 4. Prompt Customization & Overrides

Panini allows for deep customization of extraction behavior via the `prompts` parameter.

### Method A: YAML Configuration
Pass a string representing the absolute path to a YAML file matching the `ExtractorPrompts` schema.
```python
res = panini.extract(..., prompts="/path/to/custom_prompts.yml")
```

### Method B: Dictionary Injection
Pass a Python dictionary for runtime dynamic overrides. This is useful for adjusting instructions based on learner level or specific pedagogical goals.
```python
# 1. Start from the defaults
custom_prompts = panini.get_default_prompts()

# 2. Modify specific sections
custom_prompts["system_role"] = "You are a professional linguist and language tutor."
custom_prompts["output_instruction"] = "Return ONLY the JSON, no Markdown fences."

# 3. Use in extraction
res = panini.extract(..., prompts=custom_prompts)
```

### Prompt Placeholders
When writing custom prompts, you can use the following placeholders which are automatically interpolated by the engine:
- `{language}`: Full name of the target language.
- `{directives}`: Specific linguistic rules defined in Rust for this language.
- `{iso}`: ISO 639-3 code of the target language.
- `{name}`: Name of the learner's UI language.

---

## 6. Error Handling & Troubleshooting
```python
from panini import extract

try:
    res = extract(...)
except RuntimeError as e:
    if "API_KEY" in str(e):
        print("Authentication failed: Check your API key.")
    elif "Validation error" in str(e):
        print("The LLM produced JSON that didn't match the schema.")
    else:
        print(f"System error: {e}")
```

---

## 7. Advanced Implementation Details

### The Python↔Rust Bridge
- **`pythonize`**: The framework uses the `pythonize` crate for high-performance Serde bridging. Rust's `ExtractionResult` is converted directly to Python objects without intermediate JSON stringification.
- **`depythonize`**: Used to transform complex Python structures (like prompt dictionaries) back into typed Rust structs.

### Error Handling
Binding errors (e.g., invalid providers, missing API keys, or engine panics) are raised as `PyRuntimeError`.
> [!WARNING]
> Ensure the `api_key` is valid for the chosen `provider`. The bridge does not currently validate keys locally; it forwards them directly to the `rig-core` client.

### Component Selectivity
If `components` is `None` (default), the engine automatically runs **all compatible components** for the target language. For example, `morpheme_segmentation` will only run if the language implements the `Agglutinative` trait.

---

## 6. Local Development & Installation

### Building from Source
Panini uses **Maturin** to manage the Python/Rust boundary.
1.  Navigate to the `pynini` directory.
2.  Install the development version: `maturin develop`.

### CI/CD Pipeline
The `publish-panini-lang.yml` workflow automates cross-compilation for:
- **musllinux** & **manylinux** (x86_64/aarch64)
- **macOS** (universal2)
- **Windows** (x64)

---

> [!IMPORTANT]
> **ISO Code Validation**
> The `language` parameter must be a valid ISO 639-3 code registered in `panini-langs`. Use `panini.supported_languages()` to check available options at runtime.
