//! A minimal [`LinguisticDefinition`] for testing components in this crate.
//!
//! Components are generic over the language even when their output does not
//! depend on one, so testing any of them needs *some* `L`. Reaching for a real
//! language would pull `panini-langs` in below its own dependency, so this
//! stands in: the smallest type that satisfies the bounds and nothing more.

use serde::{Deserialize, Serialize};

use crate::aggregable::{Aggregable, FieldDescriptor};
use crate::morphology_enums::Upos;
use crate::traits::{
    IsoLang, LinguisticDefinition, MorphologyCatalog, MorphologyGroupSchema, MorphologyInfo, Script,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "pos", rename_all = "lowercase")]
pub enum StubMorphology {
    Word { lemma: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StubPosTag {
    Word,
}

impl MorphologyInfo for StubMorphology {
    type PosTag = StubPosTag;

    fn lemma(&self) -> &str {
        match self {
            Self::Word { lemma } => lemma,
        }
    }

    fn pos_tag(&self) -> Self::PosTag {
        StubPosTag::Word
    }

    fn pos(&self) -> Upos {
        Upos::Noun
    }
}

impl MorphologyCatalog for StubMorphology {
    fn group_descriptors() -> Vec<MorphologyGroupSchema> {
        vec![]
    }
}

impl Aggregable for StubMorphology {
    fn group_key(&self) -> String {
        self.pos_label().to_string()
    }

    fn instance_descriptors(&self) -> Vec<FieldDescriptor> {
        vec![]
    }

    fn observations(&self) -> Vec<Vec<(String, String)>> {
        vec![vec![]]
    }
}

/// The stub language itself. Its ISO code is English so nothing reads a
/// meaningful language out of a test that should not depend on one.
#[derive(Debug, Clone, Copy)]
pub struct StubLanguage;

impl LinguisticDefinition for StubLanguage {
    type Morphology = StubMorphology;
    type MorphemeFunction = ();

    const ISO_LANG: IsoLang = IsoLang::Eng;

    fn supported_scripts(&self) -> &[Script] {
        &[Script::LATN]
    }

    fn default_script(&self) -> Script {
        Script::LATN
    }

    fn extraction_directives(&self) -> &str {
        ""
    }
}
