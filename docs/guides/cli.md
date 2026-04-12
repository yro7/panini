# Command Line Interface (CLI)

---

## 🔧 Installation

```bash
# From the workspace root
cargo install --path panini-cli

# Or via cargo run (development)
cargo run -p panini-cli -- [commands]
```

**Success Output:**
```text
Finished dev [unoptimized + debuginfo] target(s) in 0.01s
Running `target/debug/panini [commands]`
```

---

## 🚀 Configuration (`panini.toml`)

The CLI uses a TOML configuration file to store your API keys and model preferences.

```toml
# panini.toml
provider     = "google"           # openai | anthropic | google
model        = "gemini-2.0-flash"
language     = "pol"              # pol | tur | ara
api_key      = "$GEMINI_API_KEY"  # Environment variable support
prompts_file = "panini-cli/prompts/default.yml"
```

---

## 🏗 Core Commands

### 1. `extract`
The main extraction tool.

```bash
  --target studentka --target czyta
```

**Extraction Result (Simplified):**
```json
{
  "word": "studentka",
  "morphology": { "lemma": "studentka", "pos": "noun", "case": "nominative" }
}
```

**Useful Options:**
- `--components`: Comma-separated list (e.g., `morphology,leipzig`).
- `--temperature`: Adjust LLM creativity (defaults to 0.2).
- `--ui-language`: Language for pedagogical explanations (Defaults to English).

### 2. `languages`
Displays the list of all ISO codes supported by your current installation.

```bash
panini languages
# => Supported languages: pol, tur, ara, fra...
```

### 3. `add-language`
Launches the LLM-assisted process for generating the code for a new language.

```bash
  --config panini.toml
```

**Generation Output:**
```text
[INFO] Analyzing Japanese morphology...
[INFO] Generated src/japanese.rs
[INFO] Registered 'jpn' in registry.rs
```

!!! warning "LLM Scaffolding Warning"
    The `add-language` tool is a powerful automation that relies on an LLM to describe linguistic types.
    - **Verify Everything**: You must always manually verify the generated Rust types and extraction directives.
    - **Use High-End Models**: Always use "Pro" or "Ultra" class models (e.g., Gemini 1.5 Pro, GPT-4o, Claude 3.5 Sonnet) for the best results.
    - **Linguistic Review**: We strongly recommend having the generated configuration reviewed by a professional linguist or native speaker.

---

## 🪄 Productivity Tips

### Pipes and JSON Output
The `panini` binary outputs pure JSON to stdout. You can use `jq` to filter results:

```bash
panini extract --text "..." --target "..." \
  | jq '.morphology.target_features'
```

### Logging
Use the `RUST_LOG` environment variable to see prompt assembly details or API calls:

```bash
RUST_LOG=info panini extract --text "..."
```

---

!!! tip "Scaffolding with the CLI"
    The `add-language` command doesn't just generate an `.rs` file; it also attempts to register the language in `registry.rs`. Always check the changes after execution.
