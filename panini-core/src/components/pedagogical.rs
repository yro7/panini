use std::fmt::Debug;

use crate::component::{AnalysisComponent, ComponentContext};
use crate::traits::LinguisticDefinition;

/// Produces a pedagogical explanation of the sentence in the learner's language.
#[derive(Debug, Clone)]
pub struct PedagogicalExplanation;

impl<L: LinguisticDefinition> AnalysisComponent<L> for PedagogicalExplanation {
    fn name(&self) -> &'static str {
        "Pedagogical Explanation"
    }

    fn schema_key(&self) -> &'static str {
        "pedagogical_explanation"
    }

    fn schema_fragment(&self, _lang: &L) -> serde_json::Value {
        serde_json::json!({
            "type": "string",
            "description": concat!(
                "PEDAGOGICAL EXPLANATION FORMAT:\n",
                "Write the ENTIRE explanation in the learner's interface language.\n",
                "The field must be an HTML string (no markdown). Structure it as follows:\n\n",
                "1. **Translations**: Start with literal and natural translations (if they differ).\n",
                "   Use: <p><b>Translations:</b><br><i>Lit:</i> ...<br><i>Nat:</i> ...</p>\n\n",
                "2. **Analysis**: A bullet list analyzing key grammatical components of the sentence.\n",
                "   - Focus on the grammar concepts relevant to the skill being tested.\n",
                "   - Highlight verbs in <span style='color:#e74c3c'><b>red</b></span>, ",
                "nouns/subjects in <span style='color:#3498db'><b>blue</b></span>, ",
                "grammar rules/cases in <span style='color:#27ae60'><b>green</b></span>.\n",
                "   - Do NOT analyze every single trivial word. Merge concepts where natural.\n",
                "   Use: <p><b>Analysis:</b></p><ul><li>...</li></ul>\n\n",
                "3. **Grammar Recap**: A summary box of the specific declensions, conjugations, or rules used.\n",
                "   Use: <div style='background-color:#3a3a3a;color:#e0e0e0;padding:10px;",
                "border-radius:5px;margin-top:10px;border-left:4px solid #3498db'>",
                "<b>Grammar Recap:</b><br>...</div>\n\n",
                "IMPORTANT: No introductory or concluding chatter. No \"Here is...\" or \"Great example!\". ",
                "Just the structured analysis."
            )
        })
    }

    fn prompt_fragment(&self, _lang: &L, ctx: &ComponentContext) -> String {
        let mut s = format!(
            "Write a clear, pedagogically useful explanation of the sentence in {}.",
            ctx.learner_ui_language
        );
        if !ctx.linguistic_background.is_empty() {
            s.push_str(
                " When genuinely helpful, use the learner's known languages as cognitive bridges.",
            );
        }
        s
    }
}
