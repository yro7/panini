# panini-lang

Python bindings for [Panini](https://github.com/yro7/panini) — an LLM-powered morphological feature extraction engine for any language.

## Installation

```bash
pip install panini-lang
```

No other dependencies required. The package ships pre-compiled for macOS, Linux, and Windows.

## Quick Start

```python
from panini import extract

result = extract(
    provider="openai",        # "openai" | "anthropic" | "google"
    model="gpt-4o",
    api_key="sk-...",
    language="fra",           # ISO 639-3 code
    text="Je ne sais pas",
    targets=["sais"],
)

print(result)
# {
#   "target_features": [...],
#   "context_features": [...]
# }
```

## Async Support

```python
import asyncio
from panini import async_extract

async def main():
    result = await async_extract(
        provider="anthropic",
        model="claude-opus-4-5",
        api_key="sk-ant-...",
        language="pol",
        text="Nie wiem",
        targets=["wiem"],
    )
    print(result)

asyncio.run(main())
```

## Supported Languages

```python
from panini import supported_languages

supported_languages()
# ['pol', 'tur', 'ara', 'fra']
```

| Code  | Language |
| ----- | -------- |
| `pol` | Polish   |
| `tur` | Turkish  |
| `ara` | Arabic   |
| `fra` | French   |

## Supported Providers

| Provider  | `provider=` value | Example model        |
| --------- | ----------------- | -------------------- |
| OpenAI    | `"openai"`        | `"gpt-4o"`           |
| Anthropic | `"anthropic"`     | `"claude-opus-4-5"`  |
| Google    | `"google"`        | `"gemini-2.0-flash"` |

## Analysis Components

By default, all compatible components are run. You can select specific ones:

```python
result = extract(
    ...,
    components=["morphology", "pedagogical_explanation"],
)
```

| Key                       | Description                                                   |
| ------------------------- | ------------------------------------------------------------- |
| `morphology`              | POS tag, lemma, case/tense/gender/etc.                        |
| `pedagogical_explanation` | HTML explanation for language learners                        |
| `multiword_expressions`   | Idioms, collocations, phrasal verbs                           |
| `morpheme_segmentation`   | Morpheme-by-morpheme breakdown (agglutinative languages only) |
| `leipzig_alignment`       | Leipzig-style interlinear glossing                            |

## Custom Prompts

Panini ships with built-in extraction prompts. You can override them by passing a YAML file path or a dict:

```python
# From a YAML file
result = extract(..., prompts="/path/to/my_prompts.yml")

# From a dict
result = extract(..., prompts={
    "system_role": "You are a strict NLP extractor...",
    ...
})

# Inspect the built-in default prompts
from panini import get_default_prompts
print(get_default_prompts())
```

## Full API Reference

### `extract(...) -> dict`

Synchronous extraction.

| Parameter     | Type                  | Default     | Description                                         |
| ------------- | --------------------- | ----------- | --------------------------------------------------- |
| `provider`    | `str`                 | required    | LLM provider: `"openai"`, `"anthropic"`, `"google"` |
| `model`       | `str`                 | required    | Model name (e.g. `"gpt-4o"`)                        |
| `api_key`     | `str`                 | required    | Provider API key                                    |
| `language`    | `str`                 | required    | ISO 639-3 language code                             |
| `text`        | `str`                 | required    | Sentence to analyse                                 |
| `targets`     | `list[str]`           | required    | Target words to extract features for                |
| `prompts`     | `str \| dict \| None` | `None`      | Path to YAML, dict, or `None` for built-in defaults |
| `temperature` | `float`               | `0.2`       | LLM temperature                                     |
| `max_tokens`  | `int`                 | `4096`      | Max output tokens                                   |
| `ui_language` | `str`                 | `"English"` | Language for pedagogical explanations               |
| `components`  | `list[str] \| None`   | `None`      | Components to run (all if `None`)                   |

### `async_extract(...) -> dict`

Asynchronous version of `extract()`. Same parameters, returns a coroutine.

### `supported_languages() -> list[str]`

Returns the list of supported ISO 639-3 language codes.

### `get_default_prompts() -> dict`

Returns the built-in extraction prompts as a Python dict.

### `version() -> str`

Returns the current package version.

## License

MIT
