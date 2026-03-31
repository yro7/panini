use std::fmt::Debug;

use crate::component::{AnalysisComponent, ComponentContext};
use crate::traits::LinguisticDefinition;

/// Produces a Leipzig-style interlinear morpheme-by-morpheme gloss
/// following the Leipzig Glossing Rules conventions.
#[derive(Debug, Clone)]
pub struct LeipzigAlignment;

impl<L: LinguisticDefinition> AnalysisComponent<L> for LeipzigAlignment {
    fn name(&self) -> &'static str {
        "Leipzig Alignment"
    }

    fn schema_key(&self) -> &'static str {
        "leipzig_alignment"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "original_script": {
                    "type": "string",
                    "description": "The original sentence in its native script, unsegmented."
                },
                "words": {
                    "type": "array",
                    "description": "Word-by-word interlinear gloss aligned vertically (Rule 1). Each entry represents one word.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "source": {
                                "type": "string",
                                "description": "The word as it appears in the source, with morpheme boundaries marked by hyphens, clitic boundaries by '=', and reduplication by '~' (Rules 2, 10)."
                            },
                            "gloss": {
                                "type": "string",
                                "description": "The morpheme-by-morpheme gloss. Grammatical labels in UPPER CASE (Rule 3). Use hyphens to match source segmentation (Rule 2). Use '.' for one-to-many correspondences (Rule 4), '=' for clitics. Person+number not separated by '.' when adjacent in that order, e.g. 3SG (Rule 5)."
                            },
                            "lexical_translation": {
                                "type": "string",
                                "description": "A short lexical translation of the word (for content words) or null for pure function morphemes."
                            }
                        },
                        "required": ["source", "gloss"]
                    }
                },
                "free_translation": {
                    "type": "string",
                    "description": "An idiomatic translation of the whole sentence in the learner's UI language."
                }
            },
            "required": ["original_script", "words", "free_translation"]
        })
    }

    fn prompt_fragment(&self, _lang: &L, ctx: &ComponentContext) -> String {
        format!(
            "Produce a Leipzig-style interlinear morpheme-by-morpheme gloss of the sentence, \
             following the Leipzig Glossing Rules:\n\
             - Rule 1: Align word by word. Each element in `words` corresponds to one word in the source.\n\
             - Rule 2: Segment morphemes with hyphens (`-`). The number of hyphens in `source` and `gloss` \
               must match exactly. Mark clitic boundaries with `=`.\n\
             - Rule 3: Use standard abbreviated grammatical category labels in UPPER CASE \
               (e.g. NOM, ACC, GEN, DAT, PL, SG, PST, PRS, FUT, INF, PTCP, NEG, DEF, INDF, \
               PASS, CAUS, REFL, SBJV, IMP, PROG, PFV, IPFV, COMP, REL, DEM, COP, AUX, etc.).\n\
             - Rule 4: When one source morpheme maps to multiple gloss elements, join them with `.` \
               (e.g. GEN.PL). Optionally use `_` for multi-word lexical translations (e.g. come_out).\n\
             - Rule 5: Do not separate person and number with `.` when they co-occur in that order \
               (e.g. write 1PL, not 1.PL).\n\
             - Rule 10: Mark reduplication with `~` (e.g. IPFV~buy).\n\
             - Provide `original_script` with the raw unsegmented sentence.\n\
             - Provide `free_translation` as an idiomatic translation in {ui_lang}.",
            ui_lang = ctx.learner_ui_language
        )
    }
}
