//! Bilingual translation alignment types.
//!
//! An [`AlignedTranslation`] pairs a sentence with its translation and links
//! the two through addressable [`AlignedSegment`]s — whole tokens, or single
//! morphemes where a sub-word unit corresponds to a separate unit in the other
//! language (agglutinative affixes, clitics, fused plurals). Links are
//! many-to-many; a segment with no counterpart simply appears in no link.
//!
//! Doc comments on these types double as the JSON-schema descriptions shown
//! to the LLM — they are the extraction spec, keep them precise.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

// ─── Alignment types ──────────────────────────────────────────────────────────

/// Character span into a sentence's `text`, counted in Unicode scalar values
/// (Rust `char`s), end-exclusive. Computed in post-processing, never produced
/// by the LLM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharSpan {
    pub start: usize,
    pub end: usize,
}

/// One displayable slice of a sentence: a whole token, or one morpheme of it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AlignedSegment {
    /// Sentence-scoped identifier, unique within this sentence. Links
    /// reference segments by this id. Use the segment's position in the
    /// `segments` list (0, 1, 2…).
    pub id: u32,
    /// 0-based index of the token this segment belongs to. Token indices are
    /// strictly increasing across the sentence and all segments of one token
    /// are adjacent.
    pub token: u32,
    /// The exact characters of this segment as they appear in the sentence —
    /// no added hyphens, no normalization. The surfaces of one token
    /// concatenate to the token exactly as written.
    pub surface: String,
    /// Leipzig-style gloss for grammatical morphemes and function words;
    /// null for content stems and punctuation. Compose freely from the
    /// standard Leipzig Glossing Rules abbreviations, joining any number of
    /// atoms with '.' (PL, LOC, 1SG.POSS, PST.PFV). Standard atoms only —
    /// any other label is rejected. Person and number fuse without a dot
    /// (1SG, 3PL — never 1.SG), and N prefixes an atom for "non-"
    /// (NPST = non-past).
    pub gloss: Option<String>,
    /// Where this segment sits in `text`. Filled by `post_process`; absent in
    /// LLM output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(skip)]
    pub span: Option<CharSpan>,
}

/// One sentence of the pair, split into addressable segments.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AlignedSentence {
    /// The sentence exactly as displayed, unsegmented.
    pub text: String,
    /// All segments in reading order, covering every non-whitespace character
    /// of `text` exactly once. One segment per token by default; several when
    /// sub-word units align separately (agglutinative affixes, clitics, fused
    /// plurals). The stem is a segment too. Punctuation is its own token,
    /// usually in no link.
    pub segments: Vec<AlignedSegment>,
}

/// The nature of one correspondence between segments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub enum LinkKind {
    /// Content-to-content correspondence (stem ↔ content word).
    Lexical,
    /// Grammatical correspondence: affix, function word, or agreement — a
    /// case suffix may map to a preposition, a person suffix to a pronoun.
    Grammatical,
    /// Whole-expression correspondence for idioms and multiword expressions
    /// whose word-by-word links would mislead.
    Phrasal,
}

/// One correspondence between the two sentences. Many-to-many: either side
/// may hold several segment ids (discontinuous units included). A segment
/// with no counterpart appears in no link at all.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AlignmentLink {
    /// Ids of the source-sentence segments in this correspondence.
    #[schemars(length(min = 1))]
    pub source: Vec<u32>,
    /// Ids of the target-sentence segments in this correspondence.
    #[schemars(length(min = 1))]
    pub target: Vec<u32>,
    /// What kind of correspondence this is.
    pub kind: LinkKind,
}

/// A sentence aligned with its translation, segment by segment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct AlignedTranslation {
    /// The analyzed sentence, in the language being learned.
    pub source: AlignedSentence,
    /// The translation, in the learner's UI language.
    pub target: AlignedSentence,
    /// Word-by-word literal rendering of the source sentence in the target
    /// language, exposing the source's structure the way "pomme de terre" is
    /// literally "apple of earth". Null when it would read the same as
    /// `target.text`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub literal_translation: Option<String>,
    /// Many-to-many correspondences between source and target segments.
    pub links: Vec<AlignmentLink>,
}

// ─── Gloss vocabulary ─────────────────────────────────────────────────────────

/// The standard abbreviations from the Leipzig Glossing Rules appendix
/// (Comrie, Haspelmath & Bickel, rev. May 2015). The appendix additionally
/// defines the bare person digits 1/2/3 and the `N-` "non-" prefix
/// (NSG non-singular, NPST non-past); [`is_valid_gloss_atom`] accepts those
/// compositionally rather than by listing.
pub const LEIPZIG_ATOMS: [&str; 80] = [
    "A", "ABL", "ABS", "ACC", "ADJ", "ADV", "AGR", "ALL", "ANTIP", "APPL", "ART", "AUX", "BEN",
    "CAUS", "CLF", "COM", "COMP", "COMPL", "COND", "COP", "CVB", "DAT", "DECL", "DEF", "DEM",
    "DET", "DIST", "DISTR", "DU", "DUR", "ERG", "EXCL", "F", "FOC", "FUT", "GEN", "IMP", "INCL",
    "IND", "INDF", "INF", "INS", "INTR", "IPFV", "IRR", "LOC", "M", "N", "NEG", "NMLZ", "NOM",
    "OBJ", "OBL", "P", "PASS", "PFV", "PL", "POSS", "PRED", "PRF", "PROG", "PROH", "PROX", "PRS",
    "PST", "PTCP", "PURP", "Q", "QUOT", "RECP", "REFL", "REL", "RES", "S", "SBJ", "SBJV", "SG",
    "TOP", "TR", "VOC",
];

/// Common non-standard labels mapped to their Leipzig equivalent. Models
/// frequently pick the right category but the wrong name (`SINGULAR` for `SG`,
/// `CONTINUOUS` for `PROG`); [`normalize_gloss`] rewrites these so the concept
/// survives instead of being dropped. Keys are compared upper-cased.
const GLOSS_ALIASES: &[(&str, &str)] = &[
    // number
    ("SINGULAR", "SG"),
    ("PLURAL", "PL"),
    ("DUAL", "DU"),
    // gender
    ("MASCULINE", "M"),
    ("MASC", "M"),
    ("FEMININE", "F"),
    ("FEM", "F"),
    ("NEUTER", "N"),
    ("NEUT", "N"),
    // case
    ("NOMINATIVE", "NOM"),
    ("ACCUSATIVE", "ACC"),
    ("GENITIVE", "GEN"),
    ("DATIVE", "DAT"),
    ("INSTRUMENTAL", "INS"),
    ("LOCATIVE", "LOC"),
    ("VOCATIVE", "VOC"),
    ("ABLATIVE", "ABL"),
    ("ALLATIVE", "ALL"),
    ("COMITATIVE", "COM"),
    ("ERGATIVE", "ERG"),
    ("ABSOLUTIVE", "ABS"),
    ("OBLIQUE", "OBL"),
    ("PREPOSITIONAL", "LOC"),
    // tense
    ("PRESENT", "PRS"),
    ("PRES", "PRS"),
    ("PAST", "PST"),
    ("FUTURE", "FUT"),
    // aspect
    ("PERFECTIVE", "PFV"),
    ("IMPERFECTIVE", "IPFV"),
    ("PROGRESSIVE", "PROG"),
    ("CONTINUOUS", "PROG"),
    ("CONT", "PROG"),
    ("HABITUAL", "IPFV"),
    ("PERFECT", "PRF"),
    ("PERF", "PRF"),
    ("DURATIVE", "DUR"),
    // mood / voice
    ("SUBJUNCTIVE", "SBJV"),
    ("IMPERATIVE", "IMP"),
    ("CONDITIONAL", "COND"),
    ("INDICATIVE", "IND"),
    ("IRREALIS", "IRR"),
    ("PASSIVE", "PASS"),
    ("CAUSATIVE", "CAUS"),
    ("REFLEXIVE", "REFL"),
    ("RECIPROCAL", "RECP"),
    // definiteness / finiteness
    ("DEFINITE", "DEF"),
    ("INDEFINITE", "INDF"),
    ("INDEF", "INDF"),
    ("INFINITIVE", "INF"),
    ("PARTICIPLE", "PTCP"),
    ("CONVERB", "CVB"),
    // pronominal / other
    ("NEGATION", "NEG"),
    ("NEGATIVE", "NEG"),
    ("DEMONSTRATIVE", "DEM"),
    ("RELATIVE", "REL"),
    ("DETERMINER", "DET"),
    ("ARTICLE", "ART"),
    ("AUXILIARY", "AUX"),
    ("COPULA", "COP"),
    ("POSSESSIVE", "POSS"),
    ("COMPARATIVE", "COMP"),
    ("PROXIMAL", "PROX"),
    ("DISTAL", "DIST"),
    // person (person-number compounds like 3SG are handled compositionally)
    ("FIRST", "1"),
    ("SECOND", "2"),
    ("THIRD", "3"),
    ("1S", "1SG"),
    ("2S", "2SG"),
    ("3S", "3SG"),
];

/// Person–number compound per Rule 5: a person digit optionally fused with a
/// number label, no separating dot (1, 3SG, 2DU, 3NSG).
fn is_person_number(atom: &str) -> bool {
    matches!(atom.as_bytes().first(), Some(b'1'..=b'3'))
        && matches!(&atom[1..], "" | "SG" | "PL" | "DU" | "NSG")
}

/// Whether one dot-separated gloss atom belongs to the accepted vocabulary:
/// a standard abbreviation, a person–number compound, or `N` + a standard
/// abbreviation (the appendix's "non-" prefix, e.g. NPST).
fn is_valid_gloss_atom(atom: &str) -> bool {
    LEIPZIG_ATOMS.contains(&atom)
        || is_person_number(atom)
        || atom
            .strip_prefix('N')
            .is_some_and(|rest| LEIPZIG_ATOMS.contains(&rest))
}

/// Best-effort cleanup of one gloss string. The gloss only powers a hover
/// tooltip in the UI, so it is never a hard validation failure — instead each
/// `.`-separated atom is upper-cased, mapped through [`GLOSS_ALIASES`], and
/// kept only if it is then a valid Leipzig atom. Unrecognized atoms are
/// dropped (a content stem the model wrongly glossed with an English lemma
/// becomes `None`, `give.PST.3SG.M` becomes `PST.3SG.M`). Returns `None` when
/// nothing recognizable survives, so every stored gloss is valid Leipzig by
/// construction.
pub fn normalize_gloss(gloss: Option<&str>) -> Option<String> {
    let raw = gloss?;
    let mut atoms = Vec::new();
    for atom in raw.split('.') {
        let atom = atom.trim();
        if atom.is_empty() {
            continue;
        }
        let upper = atom.to_uppercase();
        let mapped = GLOSS_ALIASES
            .iter()
            .find(|(from, _)| *from == upper)
            .map_or(upper.as_str(), |(_, to)| *to);
        if is_valid_gloss_atom(mapped) {
            atoms.push(mapped.to_string());
        }
    }
    if atoms.is_empty() {
        None
    } else {
        Some(atoms.join("."))
    }
}

// ─── Invariants ───────────────────────────────────────────────────────────────

impl AlignedSentence {
    /// Walks `text` and locates every segment, enforcing the coverage
    /// invariant: segments appear in reading order, grouped by token, the
    /// surfaces of one token are contiguous, and together the tokens cover
    /// every non-whitespace character of `text`.
    ///
    /// Returns one span per segment (`None` where location failed); every
    /// violation is pushed to `errors`, prefixed with `side`. The messages
    /// are written for the LLM self-correction retry — they state the rule,
    /// not just the failure.
    fn locate_segments(&self, side: &str, errors: &mut Vec<String>) -> Vec<Option<CharSpan>> {
        let mut spans = vec![None; self.segments.len()];

        let mut seen_ids = HashSet::new();
        for seg in &self.segments {
            if !seen_ids.insert(seg.id) {
                errors.push(format!("{side}: duplicate segment id {}", seg.id));
            }
        }

        let chars: Vec<char> = self.text.chars().collect();
        let mut pos = 0;
        let mut i = 0;
        let mut prev_token: Option<u32> = None;

        while i < self.segments.len() {
            let token = self.segments[i].token;
            if let Some(prev) = prev_token
                && token <= prev
            {
                errors.push(format!(
                    "{side}: token index {token} follows token {prev}; token indices must be \
                     strictly increasing, with all segments of one token adjacent"
                ));
                return spans;
            }
            prev_token = Some(token);

            while pos < chars.len() && chars[pos].is_whitespace() {
                pos += 1;
            }

            while i < self.segments.len() && self.segments[i].token == token {
                let seg = &self.segments[i];
                let expected: Vec<char> = seg.surface.chars().collect();
                if expected.is_empty() {
                    errors.push(format!(
                        "{side}: segment id {} has an empty surface",
                        seg.id
                    ));
                    return spans;
                }
                let matches = chars
                    .get(pos..pos + expected.len())
                    .is_some_and(|window| window == expected);
                if !matches {
                    let found: String = chars[pos.min(chars.len())..].iter().take(20).collect();
                    errors.push(format!(
                        "{side}: token {token}: expected segment '{}' (id {}) but the text reads \
                         «{found}». Surfaces must reproduce the text exactly, in reading order, \
                         with nothing between segments of the same token",
                        seg.surface, seg.id
                    ));
                    return spans;
                }
                spans[i] = Some(CharSpan {
                    start: pos,
                    end: pos + expected.len(),
                });
                pos += expected.len();
                i += 1;
            }
        }

        if let Some(rest) = (pos..chars.len()).find(|&p| !chars[p].is_whitespace()) {
            let uncovered: String = chars[rest..].iter().take(20).collect();
            errors.push(format!(
                "{side}: text not fully covered — «{uncovered}» belongs to no segment. Every \
                 non-whitespace character, punctuation included, must belong to a segment"
            ));
        }

        spans
    }

}

impl AlignedTranslation {
    /// Checks every link against the two segment inventories.
    fn check_links(&self, errors: &mut Vec<String>) {
        let source_ids: HashSet<u32> = self.source.segments.iter().map(|s| s.id).collect();
        let target_ids: HashSet<u32> = self.target.segments.iter().map(|s| s.id).collect();

        for (i, link) in self.links.iter().enumerate() {
            if link.source.is_empty() || link.target.is_empty() {
                errors.push(format!(
                    "link {i}: both sides must reference at least one segment; a unit with no \
                     counterpart is expressed by leaving its segment out of all links"
                ));
            }
            for id in &link.source {
                if !source_ids.contains(id) {
                    errors.push(format!("link {i}: unknown source segment id {id}"));
                }
            }
            for id in &link.target {
                if !target_ids.contains(id) {
                    errors.push(format!("link {i}: unknown target segment id {id}"));
                }
            }
            if has_duplicates(&link.source) {
                errors.push(format!("link {i}: duplicate ids on the source side"));
            }
            if has_duplicates(&link.target) {
                errors.push(format!("link {i}: duplicate ids on the target side"));
            }
        }
    }

    /// Validates all structural invariants without mutating anything:
    /// segment coverage of both texts, id uniqueness, token ordering, and link
    /// integrity. Glosses are NOT validated here — they only power a hover
    /// tooltip, so they are cleaned best-effort by [`normalize_gloss`] rather
    /// than blocking extraction. All violations are collected into one error
    /// string so the LLM self-correction retry sees every problem at once.
    ///
    /// # Errors
    /// Returns the newline-joined list of violations, if any.
    pub fn validate_structure(&self) -> Result<(), String> {
        let mut errors = Vec::new();
        let _ = self.source.locate_segments("source", &mut errors);
        let _ = self.target.locate_segments("target", &mut errors);
        self.check_links(&mut errors);
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }

    /// Fills in the char span of every segment by walking both texts.
    /// Offsets are computed here — deterministically — rather than requested
    /// from the LLM, which cannot count characters reliably.
    ///
    /// # Errors
    /// Returns the coverage violations encountered while walking, if any.
    pub fn locate_spans(&mut self) -> Result<(), String> {
        let mut errors = Vec::new();
        let source_spans = self.source.locate_segments("source", &mut errors);
        let target_spans = self.target.locate_segments("target", &mut errors);
        if !errors.is_empty() {
            return Err(errors.join("\n"));
        }
        for (seg, span) in self.source.segments.iter_mut().zip(source_spans) {
            seg.span = span;
        }
        for (seg, span) in self.target.segments.iter_mut().zip(target_spans) {
            seg.span = span;
        }
        Ok(())
    }
}

fn has_duplicates(ids: &[u32]) -> bool {
    let mut seen = HashSet::new();
    ids.iter().any(|id| !seen.insert(id))
}

// ─── LLM wire format ──────────────────────────────────────────────────────────

/// The alignment shape the LLM produces, before server-side resolution.
///
/// Deliberately free of model-maintained counters: autoregressive models are
/// unreliable at keeping several global numbering systems consistent, so
/// segments carry a local word-boundary flag instead of absolute token
/// indices, and links reference segments by surface text instead of numeric
/// ids. [`AlignedTranslation::resolve`](wire::AlignedTranslation::resolve)
/// derives ids, token indices and character spans deterministically and
/// returns the internal [`super::alignment::AlignedTranslation`] — the stored
/// format is unchanged.
///
/// Doc comments on these types double as the JSON-schema descriptions shown
/// to the LLM — they are the extraction spec, keep them precise.
pub mod wire {
    use serde::{Deserialize, Serialize};

    use super::LinkKind;

    /// One displayable slice of a sentence: a whole word, or one morpheme of it.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct AlignedSegment {
        /// The exact characters of this segment as they appear in the
        /// sentence — no added hyphens, no normalization. The surfaces of one
        /// word concatenate to that word exactly as written.
        pub surface: String,
        /// True when this segment begins a new word (words are separated by
        /// whitespace; each punctuation mark is its own word), false when it
        /// continues the previous word (affix, clitic, fused mark). The first
        /// segment is always true. Never fuse two whitespace-separated words
        /// into one word — a multi-word unit is expressed by one link
        /// spanning several segments, not by merging words.
        pub starts_new_token: bool,
        /// Leipzig-style gloss for grammatical morphemes and function words;
        /// null for content stems and punctuation. Compose freely from the
        /// standard Leipzig Glossing Rules abbreviations, joining any number of
        /// atoms with '.' (PL, LOC, 1SG.POSS, PST.PFV). Person and number fuse
        /// without a dot (1SG, 3PL — never 1.SG), and N prefixes an atom for
        /// "non-" (NPST = non-past). This field is best-effort: it feeds a
        /// tooltip, so unrecognized labels are normalized where possible and
        /// otherwise dropped rather than rejected — but staying within the
        /// standard atoms keeps the gloss intact.
        pub gloss: Option<String>,
    }

    /// One sentence of the pair, split into addressable segments.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct AlignedSentence {
        /// The sentence exactly as displayed, unsegmented.
        pub text: String,
        /// All segments in reading order, covering every non-whitespace
        /// character of `text` exactly once. One segment per word by default;
        /// several when sub-word units align separately (agglutinative
        /// affixes, clitics, fused plurals). The stem is a segment too.
        /// Punctuation is its own word, usually in no link.
        pub segments: Vec<AlignedSegment>,
    }

    /// Reference to one segment of a sentence, by its surface text.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct SegmentRef {
        /// The referenced segment's `surface`, copied exactly as it appears
        /// in that sentence's `segments` (case-sensitive) — not the whole
        /// word, not a normalized form.
        pub surface: String,
        /// 1-based position among this sentence's segments that have exactly
        /// this surface, in reading order. Required when the surface appears
        /// more than once in the sentence; omit when it is unique.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub occurrence: Option<u32>,
    }

    /// One correspondence between the two sentences. Many-to-many: either side
    /// may hold several segment references (discontinuous units included). A
    /// segment with no counterpart appears in no link at all.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct AlignmentLink {
        /// References to the source-sentence segments in this correspondence.
        #[schemars(length(min = 1))]
        pub source: Vec<SegmentRef>,
        /// References to the target-sentence segments in this correspondence.
        #[schemars(length(min = 1))]
        pub target: Vec<SegmentRef>,
        /// What kind of correspondence this is.
        pub kind: LinkKind,
    }

    /// A sentence aligned with its translation, segment by segment.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct AlignedTranslation {
        /// The analyzed sentence, in the language being learned.
        pub source: AlignedSentence,
        /// The translation, in the learner's UI language.
        pub target: AlignedSentence,
        /// Word-by-word literal rendering of the source sentence in the target
        /// language, exposing the source's structure the way "pomme de terre" is
        /// literally "apple of earth". Null when it would read the same as
        /// `target.text`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub literal_translation: Option<String>,
        /// Many-to-many correspondences between source and target segments.
        pub links: Vec<AlignmentLink>,
    }

    impl AlignedTranslation {
        /// Validates the wire structure and resolves it into the internal,
        /// id/token/span-addressed [`super::AlignedTranslation`]: ids are the
        /// segments' positions, token indices are derived from
        /// `starts_new_token`, link references are resolved by surface (and
        /// occurrence where ambiguous), and character spans are located.
        ///
        /// # Errors
        /// Returns the newline-joined list of violations, written for the LLM
        /// self-correction retry — every problem is reported at once.
        pub fn resolve(&self) -> Result<super::AlignedTranslation, String> {
            let mut errors = Vec::new();

            let source = derive_sentence(&self.source, "source", &mut errors);
            let target = derive_sentence(&self.target, "target", &mut errors);

            let mut links = Vec::with_capacity(self.links.len());
            for (i, link) in self.links.iter().enumerate() {
                if link.source.is_empty() || link.target.is_empty() {
                    errors.push(format!(
                        "link {i}: both sides must reference at least one segment; a unit with \
                         no counterpart is expressed by leaving its segment out of all links"
                    ));
                }
                let source_ids =
                    resolve_refs(&link.source, &source.segments, "source", i, &mut errors);
                let target_ids =
                    resolve_refs(&link.target, &target.segments, "target", i, &mut errors);
                links.push(super::AlignmentLink {
                    source: source_ids,
                    target: target_ids,
                    kind: link.kind,
                });
            }

            if !errors.is_empty() {
                return Err(errors.join("\n"));
            }

            let mut resolved = super::AlignedTranslation {
                source,
                target,
                literal_translation: self.literal_translation.clone(),
                links,
            };
            resolved.validate_structure()?;
            resolved.locate_spans()?;
            Ok(resolved)
        }
    }

    /// Assigns ids by position and derives token indices from the
    /// word-boundary flags. A false flag on the first segment is a violation.
    fn derive_sentence(
        sentence: &AlignedSentence,
        side: &str,
        errors: &mut Vec<String>,
    ) -> super::AlignedSentence {
        let mut token: u32 = 0;
        let mut segments = Vec::with_capacity(sentence.segments.len());
        for (i, seg) in sentence.segments.iter().enumerate() {
            if i == 0 && !seg.starts_new_token {
                errors.push(format!(
                    "{side}: the first segment ('{}') must have starts_new_token: true — it \
                     begins the first word",
                    seg.surface
                ));
            }
            if i > 0 && seg.starts_new_token {
                token += 1;
            }
            segments.push(super::AlignedSegment {
                id: i as u32,
                token,
                surface: seg.surface.clone(),
                gloss: super::normalize_gloss(seg.gloss.as_deref()),
                span: None,
            });
        }
        super::AlignedSentence {
            text: sentence.text.clone(),
            segments,
        }
    }

    /// Segment ids whose surface matches `surface`, in reading order. With
    /// `fold_case`, matching is case-insensitive (Unicode lowercase) — used as
    /// a fallback so the model's natural, capitalization-blind occurrence
    /// counting ("the third 'the'", counting a sentence-initial "The" too)
    /// resolves instead of being rejected.
    fn ids_matching(segments: &[super::AlignedSegment], surface: &str, fold_case: bool) -> Vec<u32> {
        segments
            .iter()
            .filter(|s| {
                if fold_case {
                    s.surface.to_lowercase() == surface.to_lowercase()
                } else {
                    s.surface == surface
                }
            })
            .map(|s| s.id)
            .collect()
    }

    /// Resolves one side of a link to segment ids. Segments are matched by
    /// exact surface first; when the exact bucket does not hold the requested
    /// occurrence (or the surface is absent verbatim), a case-insensitive
    /// bucket is tried — this forgives sentence-initial capitalization without
    /// giving up the precision of an exact copy. Every failure is reported
    /// with the rule it violates; duplicates are checked on the resolved ids
    /// so `{surface: "ę"}` twice is caught even without occurrences.
    fn resolve_refs(
        refs: &[SegmentRef],
        segments: &[super::AlignedSegment],
        side: &str,
        link_index: usize,
        errors: &mut Vec<String>,
    ) -> Vec<u32> {
        let mut resolved = Vec::with_capacity(refs.len());
        for r in refs {
            let surface = r.surface.as_str();
            let exact = ids_matching(segments, surface, false);
            let folded = ids_matching(segments, surface, true);

            // Prefer the exact bucket; fall back to the case-insensitive one
            // when the surface is absent verbatim, or the occurrence overflows
            // exact but fits folded (the sentence-initial "The"/"the" case).
            let ids: &[u32] = match r.occurrence {
                _ if exact.is_empty() => &folded,
                Some(o) if (o as usize) > exact.len() && (o as usize) <= folded.len() => &folded,
                _ => &exact,
            };

            match ids.len() {
                0 => errors.push(format!(
                    "{side}: link {link_index}: no segment has surface '{surface}' — copy one \
                     segment's surface as it appears in that sentence, not the whole word or a \
                     normalized form"
                )),
                1 => match r.occurrence {
                    None | Some(1) => resolved.push(ids[0]),
                    Some(o) => errors.push(format!(
                        "{side}: link {link_index}: surface '{surface}' appears only once; \
                         occurrence {o} is out of range"
                    )),
                },
                n => match r.occurrence {
                    None => errors.push(format!(
                        "{side}: link {link_index}: surface '{surface}' appears {n} times in \
                         this sentence's segments; add occurrence (1-{n}, in reading order) to \
                         say which one is meant"
                    )),
                    Some(0) => errors.push(format!(
                        "{side}: link {link_index}: occurrence is 1-based; use 1-{n} for \
                         surface '{surface}'"
                    )),
                    Some(o) if (o as usize) <= n => resolved.push(ids[o as usize - 1]),
                    Some(o) => errors.push(format!(
                        "{side}: link {link_index}: occurrence {o} is out of range for surface \
                         '{surface}' ({n} occurrences)"
                    )),
                },
            }
        }
        let mut seen = std::collections::HashSet::new();
        for id in &resolved {
            if !seen.insert(*id) {
                errors.push(format!(
                    "{side}: link {link_index}: duplicate reference to the same segment (id \
                     {id}) — reference each segment at most once per link side"
                ));
            }
        }
        resolved
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(id: u32, token: u32, surface: &str, gloss: Option<&str>) -> AlignedSegment {
        AlignedSegment {
            id,
            token,
            surface: surface.to_string(),
            gloss: gloss.map(str::to_string),
            span: None,
        }
    }

    fn link(source: &[u32], target: &[u32], kind: LinkKind) -> AlignmentLink {
        AlignmentLink {
            source: source.to_vec(),
            target: target.to_vec(),
            kind,
        }
    }

    /// Turkish → French: one agglutinated word maps to four French units.
    fn demo() -> AlignedTranslation {
        AlignedTranslation {
            source: AlignedSentence {
                text: "Evlerimde kalıyorum".to_string(),
                segments: vec![
                    seg(0, 0, "Ev", None),
                    seg(1, 0, "ler", Some("PL")),
                    seg(2, 0, "im", Some("1SG.POSS")),
                    seg(3, 0, "de", Some("LOC")),
                    seg(4, 1, "kal", None),
                    seg(5, 1, "ıyor", Some("PROG")),
                    seg(6, 1, "um", Some("1SG")),
                ],
            },
            target: AlignedSentence {
                text: "Je reste dans mes maisons".to_string(),
                segments: vec![
                    seg(0, 0, "Je", None),
                    seg(1, 1, "reste", None),
                    seg(2, 2, "dans", None),
                    seg(3, 3, "mes", None),
                    seg(4, 4, "maison", None),
                    seg(5, 4, "s", Some("PL")),
                ],
            },
            literal_translation: Some("dans mes maisons je-reste".to_string()),
            links: vec![
                link(&[0], &[4], LinkKind::Lexical),
                link(&[1], &[5], LinkKind::Grammatical),
                link(&[2], &[3], LinkKind::Grammatical),
                link(&[3], &[2], LinkKind::Grammatical),
                link(&[4], &[1], LinkKind::Lexical),
                link(&[6], &[0], LinkKind::Grammatical),
            ],
        }
    }

    #[test]
    fn demo_passes_validation() {
        demo().validate_structure().expect("demo should be valid");
    }

    #[test]
    fn locate_spans_computes_char_offsets() {
        let mut a = demo();
        a.locate_spans().expect("demo should locate");

        let spans: Vec<CharSpan> = a.source.segments.iter().map(|s| s.span.unwrap()).collect();
        assert_eq!(
            spans,
            vec![
                CharSpan { start: 0, end: 2 },   // Ev
                CharSpan { start: 2, end: 5 },   // ler
                CharSpan { start: 5, end: 7 },   // im
                CharSpan { start: 7, end: 9 },   // de
                CharSpan { start: 10, end: 13 }, // kal
                CharSpan { start: 13, end: 17 }, // ıyor
                CharSpan { start: 17, end: 19 }, // um
            ]
        );
        assert_eq!(
            a.target.segments.last().unwrap().span,
            Some(CharSpan { start: 24, end: 25 }) // the -s of maisons
        );
    }

    #[test]
    fn punctuation_is_its_own_token() {
        let mut a = demo();
        a.source.text.push('.');
        a.source.segments.push(seg(7, 2, ".", None));
        a.locate_spans().expect("punctuation token should locate");
        assert_eq!(
            a.source.segments.last().unwrap().span,
            Some(CharSpan { start: 19, end: 20 })
        );
    }

    #[test]
    fn surface_mismatch_is_reported_with_context() {
        let mut a = demo();
        a.source.segments[1].surface = "lar".to_string();
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("expected segment 'lar'"), "got: {err}");
    }

    #[test]
    fn uncovered_text_is_rejected() {
        let mut a = demo();
        a.target.segments.pop(); // drop the -s of "maisons"
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("not fully covered"), "got: {err}");
    }

    #[test]
    fn whitespace_inside_a_token_is_rejected() {
        let mut a = demo();
        a.source.text = "Ev lerimde kalıyorum".to_string();
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("expected segment 'ler'"), "got: {err}");
    }

    #[test]
    fn duplicate_segment_ids_are_rejected() {
        let mut a = demo();
        a.source.segments[3].id = 1;
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("duplicate segment id 1"), "got: {err}");
    }

    #[test]
    fn interleaved_token_indices_are_rejected() {
        let mut a = demo();
        a.source.segments[5].token = 0; // "ıyor" claims token 0 after token 1 started
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("strictly increasing"), "got: {err}");
    }

    #[test]
    fn dangling_link_id_is_rejected() {
        let mut a = demo();
        a.links[0].target = vec![99];
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("unknown target segment id 99"), "got: {err}");
    }

    #[test]
    fn empty_link_side_is_rejected() {
        let mut a = demo();
        a.links.push(link(&[5], &[], LinkKind::Grammatical));
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("at least one segment"), "got: {err}");
    }

    #[test]
    fn composed_gloss_atoms_survive_normalization() {
        // Valid composed atoms pass through normalize_gloss unchanged, and
        // never block validation (glosses are best-effort).
        for g in ["PST.PFV", "3PL", "NPST", "3", "NEG.1SG.POSS.COND"] {
            assert_eq!(normalize_gloss(Some(g)).as_deref(), Some(g));
        }
        let mut a = demo();
        a.source.segments[6].gloss = Some("NEG.1SG.POSS.COND".to_string());
        a.validate_structure()
            .expect("glosses never block validation");
    }

    #[test]
    fn normalize_maps_known_aliases() {
        // Right concept, wrong name — the concept survives.
        assert_eq!(normalize_gloss(Some("SINGULAR")).as_deref(), Some("SG"));
        assert_eq!(
            normalize_gloss(Some("singular.masculine")).as_deref(),
            Some("SG.M")
        );
        assert_eq!(normalize_gloss(Some("CONTINUOUS")).as_deref(), Some("PROG"));
        assert_eq!(normalize_gloss(Some("PRES")).as_deref(), Some("PRS"));
    }

    #[test]
    fn normalize_lowercases_and_drops_unknown_atoms() {
        // Lowercase standard atoms are recovered; unknown atoms (English
        // lemmas the model wrongly glossed a content word with) are dropped.
        assert_eq!(normalize_gloss(Some("pl")).as_deref(), Some("PL"));
        assert_eq!(
            normalize_gloss(Some("give.PST.3SG.M")).as_deref(),
            Some("PST.3SG.M")
        );
        assert_eq!(normalize_gloss(Some("1SG.WIBBLE")).as_deref(), Some("1SG"));
    }

    #[test]
    fn normalize_empties_collapse_to_none() {
        assert_eq!(normalize_gloss(Some("  ")), None);
        assert_eq!(normalize_gloss(Some("PST..PFV")).as_deref(), Some("PST.PFV"));
        assert_eq!(normalize_gloss(Some("dog")), None); // content stem → null
        assert_eq!(normalize_gloss(None), None);
    }

    #[test]
    fn llm_json_without_spans_round_trips() {
        let json = serde_json::json!({
            "source": {
                "text": "Evlerimde kalıyorum",
                "segments": [
                    { "id": 0, "token": 0, "surface": "Ev", "gloss": null },
                    { "id": 1, "token": 0, "surface": "lerimde", "gloss": "PL.1SG.POSS.LOC" },
                    { "id": 2, "token": 1, "surface": "kalıyorum", "gloss": null }
                ]
            },
            "target": {
                "text": "Je reste dans mes maisons",
                "segments": [
                    { "id": 0, "token": 0, "surface": "Je", "gloss": null },
                    { "id": 1, "token": 1, "surface": "reste", "gloss": null },
                    { "id": 2, "token": 2, "surface": "dans", "gloss": null },
                    { "id": 3, "token": 3, "surface": "mes", "gloss": null },
                    { "id": 4, "token": 4, "surface": "maisons", "gloss": null }
                ]
            },
            "links": [
                { "source": [0], "target": [4], "kind": "Lexical" }
            ]
        });
        let mut a: AlignedTranslation =
            serde_json::from_value(json).expect("LLM-shaped JSON should deserialize");
        a.validate_structure().expect("should validate");
        a.locate_spans().expect("should locate");

        let out = serde_json::to_value(&a).expect("should serialize");
        assert_eq!(out["source"]["segments"][1]["span"]["start"], 2);
        assert_eq!(out["source"]["segments"][1]["span"]["end"], 9);

        // LLM output predating (or omitting) `literal_translation` defaults to
        // None and the key stays absent on re-serialization.
        assert!(a.literal_translation.is_none());
        assert!(out.get("literal_translation").is_none());
    }

    #[test]
    fn literal_translation_round_trips() {
        let a = demo();
        let out = serde_json::to_value(&a).expect("should serialize");
        assert_eq!(out["literal_translation"], "dans mes maisons je-reste");

        let back: AlignedTranslation = serde_json::from_value(out).expect("should deserialize");
        assert_eq!(
            back.literal_translation.as_deref(),
            Some("dans mes maisons je-reste")
        );
    }

    // ── Wire format ──

    fn wseg(surface: &str, starts_new_token: bool, gloss: Option<&str>) -> wire::AlignedSegment {
        wire::AlignedSegment {
            surface: surface.to_string(),
            starts_new_token,
            gloss: gloss.map(str::to_string),
        }
    }

    fn wref(surface: &str, occurrence: Option<u32>) -> wire::SegmentRef {
        wire::SegmentRef {
            surface: surface.to_string(),
            occurrence,
        }
    }

    fn wlink(
        source: Vec<wire::SegmentRef>,
        target: Vec<wire::SegmentRef>,
        kind: LinkKind,
    ) -> wire::AlignmentLink {
        wire::AlignmentLink {
            source,
            target,
            kind,
        }
    }

    /// Turkish → French, same sentence as [`demo`] but in wire form.
    fn wire_demo() -> wire::AlignedTranslation {
        wire::AlignedTranslation {
            source: wire::AlignedSentence {
                text: "Evlerimde kalıyorum".to_string(),
                segments: vec![
                    wseg("Ev", true, None),
                    wseg("ler", false, Some("PL")),
                    wseg("im", false, Some("1SG.POSS")),
                    wseg("de", false, Some("LOC")),
                    wseg("kal", true, None),
                    wseg("ıyor", false, Some("PROG")),
                    wseg("um", false, Some("1SG")),
                ],
            },
            target: wire::AlignedSentence {
                text: "Je reste dans mes maisons".to_string(),
                segments: vec![
                    wseg("Je", true, None),
                    wseg("reste", true, None),
                    wseg("dans", true, None),
                    wseg("mes", true, None),
                    wseg("maison", true, None),
                    wseg("s", false, Some("PL")),
                ],
            },
            literal_translation: Some("dans mes maisons je-reste".to_string()),
            links: vec![
                wlink(
                    vec![wref("Ev", None)],
                    vec![wref("maison", None)],
                    LinkKind::Lexical,
                ),
                wlink(
                    vec![wref("ler", None)],
                    vec![wref("s", None)],
                    LinkKind::Grammatical,
                ),
                wlink(
                    vec![wref("im", None)],
                    vec![wref("mes", None)],
                    LinkKind::Grammatical,
                ),
                wlink(
                    vec![wref("de", None)],
                    vec![wref("dans", None)],
                    LinkKind::Grammatical,
                ),
                wlink(
                    vec![wref("kal", None)],
                    vec![wref("reste", None)],
                    LinkKind::Lexical,
                ),
                wlink(
                    vec![wref("um", None)],
                    vec![wref("Je", None)],
                    LinkKind::Grammatical,
                ),
            ],
        }
    }

    #[test]
    fn wire_demo_resolves_to_internal_form() {
        let resolved = wire_demo().resolve().expect("wire demo should resolve");

        let ids: Vec<u32> = resolved.source.segments.iter().map(|s| s.id).collect();
        assert_eq!(ids, vec![0, 1, 2, 3, 4, 5, 6]);
        let tokens: Vec<u32> = resolved.source.segments.iter().map(|s| s.token).collect();
        assert_eq!(tokens, vec![0, 0, 0, 0, 1, 1, 1]);
        assert_eq!(
            resolved.source.segments[1].span,
            Some(CharSpan { start: 2, end: 5 }) // ler
        );

        // "um" → "Je": last source segment to first target segment.
        let last = resolved.links.last().expect("links preserved");
        assert_eq!(last.source, vec![6]);
        assert_eq!(last.target, vec![0]);

        // The resolved form is the exact internal demo, spans aside.
        let mut expected = demo();
        expected.locate_spans().expect("demo should locate");
        assert_eq!(resolved, expected);
    }

    /// "Lubię kawę." segmented at morpheme level: the ending 'ę' appears twice,
    /// so refs to it must carry `occurrence`.
    fn wire_repeated() -> wire::AlignedTranslation {
        wire::AlignedTranslation {
            source: wire::AlignedSentence {
                text: "Lubię kawę.".to_string(),
                segments: vec![
                    wseg("Lubi", true, None),
                    wseg("ę", false, Some("1SG")),
                    wseg("kaw", true, None),
                    wseg("ę", false, Some("ACC")),
                    wseg(".", true, None),
                ],
            },
            target: wire::AlignedSentence {
                text: "I like coffee.".to_string(),
                segments: vec![
                    wseg("I", true, None),
                    wseg("like", true, None),
                    wseg("coffee", true, None),
                    wseg(".", true, None),
                ],
            },
            literal_translation: None,
            links: vec![
                wlink(
                    vec![wref("Lubi", None)],
                    vec![wref("like", None)],
                    LinkKind::Lexical,
                ),
                wlink(
                    vec![wref("ę", Some(1))],
                    vec![wref("I", None)],
                    LinkKind::Grammatical,
                ),
                wlink(
                    vec![wref("kaw", None)],
                    vec![wref("coffee", None)],
                    LinkKind::Lexical,
                ),
            ],
        }
    }

    #[test]
    fn repeated_surface_resolves_via_occurrence() {
        let resolved = wire_repeated().resolve().expect("should resolve");
        assert_eq!(resolved.links[1].source, vec![1]); // first 'ę'

        let mut second = wire_repeated();
        second.links[1].source = vec![wref("ę", Some(2))];
        let resolved = second.resolve().expect("second occurrence should resolve");
        assert_eq!(resolved.links[1].source, vec![3]); // 'ę' of "kawę"
    }

    #[test]
    fn repeated_surface_without_occurrence_is_rejected() {
        let mut a = wire_repeated();
        a.links[1].source = vec![wref("ę", None)];
        let err = a.resolve().unwrap_err();
        assert!(
            err.contains("appears 2 times") && err.contains("occurrence"),
            "got: {err}"
        );
    }

    #[test]
    fn occurrence_out_of_range_is_rejected() {
        let mut a = wire_repeated();
        a.links[1].source = vec![wref("ę", Some(3))];
        let err = a.resolve().unwrap_err();
        assert!(err.contains("out of range"), "got: {err}");

        let mut zero = wire_repeated();
        zero.links[1].source = vec![wref("ę", Some(0))];
        let err = zero.resolve().unwrap_err();
        assert!(err.contains("1-based"), "got: {err}");
    }

    #[test]
    fn occurrence_on_unique_surface() {
        let mut a = wire_repeated();
        a.links[0].source = vec![wref("Lubi", Some(1))]; // redundant but coherent
        a.resolve()
            .expect("occurrence 1 on a unique surface is fine");

        let mut bad = wire_repeated();
        bad.links[0].source = vec![wref("Lubi", Some(2))];
        let err = bad.resolve().unwrap_err();
        assert!(err.contains("appears only once"), "got: {err}");
    }

    #[test]
    fn unknown_ref_surface_is_rejected() {
        let mut a = wire_repeated();
        a.links[0].target = vec![wref("likes", None)];
        let err = a.resolve().unwrap_err();
        assert!(err.contains("no segment has surface 'likes'"), "got: {err}");
    }

    #[test]
    fn duplicate_refs_in_one_link_side_are_rejected() {
        let mut a = wire_repeated();
        a.links[0].source = vec![wref("Lubi", None), wref("Lubi", Some(1))];
        let err = a.resolve().unwrap_err();
        assert!(err.contains("duplicate reference"), "got: {err}");
    }

    #[test]
    fn first_segment_must_start_a_token() {
        let mut a = wire_repeated();
        a.source.segments[0].starts_new_token = false;
        let err = a.resolve().unwrap_err();
        assert!(err.contains("first segment"), "got: {err}");
    }

    #[test]
    fn fused_words_fail_coverage() {
        // "are" + "reading" claimed as one word: contiguity check must fire on
        // the whitespace between them.
        let a = wire::AlignedTranslation {
            source: wire::AlignedSentence {
                text: "Czytamy.".to_string(),
                segments: vec![wseg("Czytamy", true, None), wseg(".", true, None)],
            },
            target: wire::AlignedSentence {
                text: "We are reading.".to_string(),
                segments: vec![
                    wseg("We", true, None),
                    wseg("are", true, None),
                    wseg("reading", false, None), // wrongly fused into "are"'s word
                    wseg(".", true, None),
                ],
            },
            literal_translation: None,
            links: vec![],
        };
        let err = a.resolve().unwrap_err();
        assert!(err.contains("expected segment 'reading'"), "got: {err}");
    }

    #[test]
    fn empty_wire_link_side_is_rejected() {
        let mut a = wire_repeated();
        a.links
            .push(wlink(vec![wref("kaw", None)], vec![], LinkKind::Lexical));
        let err = a.resolve().unwrap_err();
        assert!(err.contains("at least one segment"), "got: {err}");
    }

    #[test]
    fn wire_glosses_are_normalized_not_rejected() {
        // A non-standard but recognizable gloss is normalized (PRES→PRS), and
        // a content-word lemma gloss is dropped — neither blocks resolution.
        let mut a = wire_repeated();
        a.source.segments[1].gloss = Some("PRES".to_string());
        a.source.segments[0].gloss = Some("like".to_string());
        let resolved = a.resolve().expect("best-effort gloss never rejects");
        assert_eq!(resolved.source.segments[1].gloss.as_deref(), Some("PRS"));
        assert_eq!(resolved.source.segments[0].gloss, None);
    }

    #[test]
    fn wire_case_insensitive_occurrence_fallback() {
        // The model counts "the" three times (The + the + the); the exact
        // lowercase bucket has only two, so occurrence 3 must fall back to the
        // case-insensitive bucket and resolve to the last "the".
        let a = wire::AlignedTranslation {
            source: wire::AlignedSentence {
                text: "Pies.".to_string(),
                segments: vec![wseg("Pies", true, None), wseg(".", true, None)],
            },
            target: wire::AlignedSentence {
                text: "The dog and the cat and the bird.".to_string(),
                segments: vec![
                    wseg("The", true, Some("DEF")),
                    wseg("dog", true, None),
                    wseg("and", true, None),
                    wseg("the", true, Some("DEF")),
                    wseg("cat", true, None),
                    wseg("and", true, None),
                    wseg("the", true, Some("DEF")),
                    wseg("bird", true, None),
                    wseg(".", true, None),
                ],
            },
            literal_translation: None,
            links: vec![wlink(
                vec![wref("Pies", None)],
                vec![wref("the", Some(3))],
                LinkKind::Lexical,
            )],
        };
        let resolved = a.resolve().expect("case-insensitive fallback should resolve");
        // "The"(0), "the"(3), "the"(6): occurrence 3 counted case-insensitively
        // is the segment at id 6.
        assert_eq!(resolved.links[0].target, vec![6]);
    }

    #[test]
    fn wire_llm_json_resolves() {
        let json = serde_json::json!({
            "source": {
                "text": "Lubię kawę.",
                "segments": [
                    { "surface": "Lubi", "starts_new_token": true, "gloss": null },
                    { "surface": "ę", "starts_new_token": false, "gloss": "1SG" },
                    { "surface": "kaw", "starts_new_token": true, "gloss": null },
                    { "surface": "ę", "starts_new_token": false, "gloss": "ACC" },
                    { "surface": ".", "starts_new_token": true, "gloss": null }
                ]
            },
            "target": {
                "text": "I like coffee.",
                "segments": [
                    { "surface": "I", "starts_new_token": true, "gloss": null },
                    { "surface": "like", "starts_new_token": true, "gloss": null },
                    { "surface": "coffee", "starts_new_token": true, "gloss": null },
                    { "surface": ".", "starts_new_token": true, "gloss": null }
                ]
            },
            "literal_translation": null,
            "links": [
                { "source": [{ "surface": "ę", "occurrence": 2 }], "target": [{ "surface": "coffee" }], "kind": "Grammatical" }
            ]
        });
        let a: wire::AlignedTranslation =
            serde_json::from_value(json).expect("LLM-shaped wire JSON should deserialize");
        let resolved = a.resolve().expect("should resolve");
        assert_eq!(resolved.links[0].source, vec![3]);
        assert_eq!(
            resolved.source.segments[3].span,
            Some(CharSpan { start: 9, end: 10 })
        );
    }
}
