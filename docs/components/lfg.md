# LFG f-structures

Select `lfg_f_structure` to propose one f-structure for the whole sentence.
The component uses the same grammar and prompt for every language. Its
morphological feature schema is generated from the language's existing types;
it does not require a prior `morphology` extraction.

## Functions and attributes

`LfgFunction` defines the governed functions accepted by this profile:
`SUBJ`, `OBJ`, `OBJ2`, `COMP`, `XCOMP`, `OBL`, `OBL_AG`, `OBL_GOAL`,
`OBL_SOURCE`, `OBL_LOC`, `OBL_INST`, and `PREDLINK`.

Only these functions can occur in a predicate's selected argument list.
Every selected argument must have a link, and every governed link must be
selected. This checks the model's declared frame, not independent lexical valency.

`LfgAttribute` defines each structural attribute's value type:

| Kind in the LLM output | Stored attribute | Value |
| --- | --- | --- |
| `function` | The selected `LfgFunction` | One structure reference |
| `adjunct` | `ADJUNCT` | A set of distinct structure references |
| `topic`, `focus` | `TOPIC`, `FOCUS` | One structure reference |
| `tense_aspect` | `TNS-ASP` | A reference to a nonempty, predicate-free feature bundle |
| `definite` | `DEF` | `+` or `-`, emitted from a boolean |
| `voice` | `VOICE` | Canonical `BinaryVoice` value |

ADJUNCT, TOPIC and FOCUS cannot be selected predicate arguments. Arbitrary
functions or semantic-role labels such as `AGENT` are not accepted.
The typed grammar is deliberately bounded, not a complete ParGram inventory.

## Functional control

A structure with XCOMP must declare `controller`, a non-clausal governed
function on that same structure. The referenced complement's SUBJ must point
to exactly the same structure as that controller. For example:

```text
The farmer tries to leave.
controller = SUBJ
try.SUBJ = try.XCOMP.SUBJ

The farmer persuades his neighbor to leave.
controller = OBJ
persuade.OBJ = persuade.XCOMP.SUBJ
```

Without XCOMP, controller must be null. COMP does not introduce this control
requirement. Semantic coreference alone is not structure sharing.
The validator checks the declared identity, but cannot independently establish
which controller the verb selects. This profile supports local functional
control, not every possible open-complement analysis.

## Reusing morphological inventories

An optional `morphology` bundle identifies one category and a partial list of
its features:

```json
{
  "category": "noun",
  "features": [
    { "name": "number", "value": "singular" },
    { "name": "case", "value": "nominative" }
  ]
}
```

`MorphologyCatalog::feature_descriptors()` supplies the fields and canonical
values, including optional fields, directly from `#[derive(MorphologyInfo)]`.
The existing aggregation catalog retains its original non-optional dimensions.
No new Number, Case, Gender or Tense inventory is defined for LFG.

The generated schema keeps each category with its own allowed features and
values. For example, the Polish noun inventory permits `accusative`, while
the Irish noun inventory does not; Irish pronoun `pronoun_case` values cannot
be applied to nouns. `SG`, `sg` and `singular` are not interchangeable spellings.
Open lexical dimensions remain open. Boolean dimensions use the catalog's
atomic strings `true` and `false`.

Only asserted features are emitted: this is not a full token morphology
record. Clause features may be contributed by auxiliaries or multiple words.
Correct category choice, attachment and lexical analysis still require linguistic
evaluation; a schema cannot establish them.

Tense, aspect, mood and evidentiality are placed in predicate-free feature
structures referenced through TNS-ASP, using the applicable category's inventory.
Number, case and other non-temporal features remain on the relevant predicate.
For syntactic voice, the common `voice` attribute uses `BinaryVoice`. When the
selected category has its own voice dimension, project that dimension instead;
its inventory takes precedence over the common fallback.

## Stored results and validation

Post-processing preserves the v1 graph representation: `root`, `structures`,
`pred` and attribute maps containing atoms, references or sets. Feature names
map to NUM, PERS and GEND for number, person and gender; other field names use
uppercase with hyphens. Values retain their catalog spelling.

Two attributes record validation context in the existing map representation:

- `MORPHOLOGY-GROUP` identifies the category of projected morphological features.
- `XCOMP-CONTROLLER` identifies the declared controller function.

`LfgAnalysis::validate()` retains v1 graph/frame validation for existing data.
`validate_for_language(&language)` additionally checks the new grammar,
category-specific values, TNS-ASP placement and control. Extraction always uses
the latter. The persisted container shape is unchanged, so `output_version`
remains 1; the new LLM transport schema does not require a storage migration.

## Live example

Set `GOOGLE_API_KEY`, or point `LFG_ENV_FILE` to a dotenv file containing it:

```bash
cargo run -p panini-examples --bin lfg_f_structure
```

The suite covers seven English, French and Irish sentences, including subject
and object control. `LFG_MODEL` selects the Gemini model and `LFG_OUTPUT` saves
the report. The injected Gemini transport omits gateway-specific metadata;
schema composition, validation and correction remain in Pāṇini.

The recorded outputs in `examples/data/lfg_grammar_live.json` are observations,
not gold annotations. For example, the relative-clause output
incorrectly assigns `DEF: "-"` to "the tractor"; accepted values do not imply
correct value selection. The original v1 snapshot is retained separately to
verify backward readability.
