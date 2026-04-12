# Python Integration

Pāṇini is available on PyPI as `panini-lang`. It's the recommended interface for researchers, linguists, and developers who prefer the Python ecosystem.

---

## 🔧 Installation

```bash
pip install panini-lang
```

---

## 🚀 Basic Usage

The `panini` module provides a high-level interface to the Rust extraction engine.

### Synchronous Extraction

```python
import panini

# Extraction for a single word
result = panini.extract(
    provider="google",
    model="gemini-1.5-flash",
    api_key="...",
    language="pol",
    text="Studentka czyta książkę.",
    targets=["studentka"]
)

# Iterating over morphological features
for feat in result['morphology']['target_features']:
    m = feat['morphology']
    print(f"Word: {feat['word']}, Lemma: {m['lemma']}, POS: {m['pos']}")
```

**Sample Output:**
```text
Word: studentka, Lemma: studentka, POS: noun
```

### Asynchronous Extraction

```python
import asyncio
import panini

async def analyze():
    res = await panini.async_extract(
        provider="openai",
        model="gpt-4o",
        api_key="...",
        language="ara",
        text="ذهب الولد إلى المدرسة",
        targets=["ذهب"]
    )
    return res

asyncio.run(analyze())
```

**Sample Async Result:**
```json
{
  "morphology": {
    "target_features": [
      {
        "word": "ذهب",
        "morphology": { "root": "ذ-ه-ب", "wazn": "fa'ala", "tense": "past" }
      }
    ]
  }
}
```

---

## 🧬 Extraction Parameters

The `extract()` and `async_extract()` functions share the same parameters:

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `provider` | `str` | *(Required)* | `"openai"`, `"anthropic"`, or `"google"`. |
| `model` | `str` | *(Required)* | Model ID (e.g., `"gpt-4o"`). |
| `api_key` | `str` | *(Required)* | Provider-specific API key. |
| `language` | `str` | *(Required)* | ISO 639-3 target language code. |
| `text` | `str` | *(Required)* | The sentence or text to analyze. |
| `targets` | `list[str]` | *(Required)* | Specific words to focus extraction on. |
| `components` | `list[str]` | `None` | (Optional) List of component keys to run. |
| `temperature` | `float` | `0.2` | Sampling temperature. |

---

## 🎨 Prompt Customization

You can pass the `prompts` parameter as a path to a YAML file or as a Python dictionary.

```python
# 1. Get default prompts
custom_prompts = panini.get_default_prompts()

# 2. Modify a specific section
custom_prompts["system_role"] = "You are an expert linguist."

# 3. Use in extraction
res = panini.extract(..., prompts=custom_prompts)
```

**Custom Prompt Effect:**
The LLM will prioritize the new system role (e.g., "Expert Linguist"), leading to more detailed pedagogical explanations in the results.

---

## 🌍 Supported Languages

To see which languages are currently registered in the engine:

```python
print(panini.supported_languages())
# => ["pol", "tur", "ara", "fra", ...]
```

---

!!! info "Rust/Python Bridge Performance"
    The framework uses `pythonize` to convert Rust types directly into Python objects without intermediate JSON stringification, ensuring optimal performance.
