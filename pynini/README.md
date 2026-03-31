# Pynini

Python bindings for the Panini morphological feature extraction engine.

## Usage

```python
import pynini

# Synchronous usage
result = pynini.extract(
    provider="openai",
    model="gpt-4o",
    api_key="your_api_key",
    language="pol",
    text="Some text",
    targets=["text"],
    prompts="path/to/prompts.yml",
)
print(result)

# Asynchronous usage
import asyncio

async def test():
    result = await pynini.async_extract(
        provider="openai",
        model="gpt-4o",
        api_key="your_api_key",
        language="pol",
        text="Some text",
        targets=["text"],
        prompts="path/to/prompts.yml",
    )
    print(result)

asyncio.run(test())
```
