# Panini Framework Examples

This directory contains demonstration codes to showcase the core features of the Panini framework in both **Rust** and **Python**.

## Directory Structure

- `data/`: Sample text corpuses for various languages.
- `rust/`: Rust implementation of the examples.
- `python/`: Python implementation using the `panini-lang` package.

---

## 🚀 Getting Started

### 1. Environment Setup

All examples require an LLM API key. Create a `.env` file in the project root (or in the `rust/` and `python/` folders) with your keys:

```bash
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
```

### 2. Python Examples

First, ensure you have the `panini-lang` package installed. You can install it from the local source:

```bash
cd examples/python
pip install -r requirements.txt
```

#### Run Environment Check
Verify your installation and see metadata discovery features:
```bash
python3 environment_check.py
```

#### Run Turkish Morpheme Segmentation
```bash
python3 turkish_segmentation.py
```

#### Run Arabic Pivot Aggregation
```bash
python3 arabic_pivot_aggregation.py
```

---

### 3. Rust Examples

Rust examples use the workspace crates directly.

```bash
cd examples/rust
```

#### Run Turkish Morpheme Segmentation
```bash
cargo run --bin turkish_segmentation
```

#### Run Arabic Pivot Aggregation
```bash
cargo run --bin arabic_pivot_aggregation
```

---

## 💡 Key Concepts Showcased

### Morpheme Segmentation (Turkish)
Showcases how Panini handles agglutinative languages by decomposing words into their constituent morphemes while mapping them back to archiphonemic base forms.

### Aggregation & Pivots (Arabic)
Showcases the powerful `BasicAggregator` which can:
1.  **Group by PoS**: The default behavior for morphological features.
2.  **Pivot by Custom Key**: Re-grouping data by arbitrary fields (like the Arabic **triconsonantal root**) using a simple closure/lambda.

### Metadata & Schema Discovery
Demonstrates how to programmatically query the framework for:
- Supported scripts and typological features.
- JSON schemas for the extraction results (useful for frontend integration).
