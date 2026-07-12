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

/// Frequent non-standard labels mapped to their Leipzig equivalents, so the
/// rejection message can name the canonical form.
const GLOSS_SUGGESTIONS: [(&str, &str); 7] = [
    ("PRES", "PRS"),
    ("PAST", "PST"),
    ("PERF", "PRF"),
    ("INDEF", "INDF"),
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

    /// Checks every non-null gloss against the accepted Leipzig vocabulary:
    /// each `.`-separated atom must be a standard abbreviation, a
    /// person–number compound (1SG), or `N` + a standard abbreviation
    /// (NPST). Any number of atoms may be composed; only the atoms
    /// themselves are constrained — atom order is not checked. Violations
    /// are pushed to `errors` in the same LLM-facing style as
    /// `locate_segments`.
    fn check_glosses(&self, side: &str, errors: &mut Vec<String>) {
        for seg in &self.segments {
            let Some(gloss) = &seg.gloss else { continue };
            if gloss.trim().is_empty() {
                errors.push(format!(
                    "{side}: segment id {}: empty gloss — use null instead of an empty gloss",
                    seg.id
                ));
                continue;
            }
            let mut empty_atom_reported = false;
            for atom in gloss.split('.') {
                if atom.is_empty() {
                    if !empty_atom_reported {
                        errors.push(format!(
                            "{side}: segment id {}: gloss '{gloss}' contains an empty atom; \
                             join atoms with a single '.', no leading or trailing dot",
                            seg.id
                        ));
                        empty_atom_reported = true;
                    }
                    continue;
                }
                if is_valid_gloss_atom(atom) {
                    continue;
                }
                let upper = atom.to_uppercase();
                if upper != atom && is_valid_gloss_atom(&upper) {
                    errors.push(format!(
                        "{side}: segment id {}: gloss atom '{atom}' — grammatical labels are \
                         UPPER CASE, write '{upper}'",
                        seg.id
                    ));
                } else if let Some((_, fix)) =
                    GLOSS_SUGGESTIONS.iter().find(|(bad, _)| *bad == upper)
                {
                    errors.push(format!(
                        "{side}: segment id {}: gloss atom '{atom}' is not a standard Leipzig \
                         abbreviation — use '{fix}'",
                        seg.id
                    ));
                } else {
                    errors.push(format!(
                        "{side}: segment id {}: gloss atom '{atom}' is not a standard Leipzig \
                         abbreviation; compose glosses only from the Leipzig Glossing Rules \
                         atoms, joined by '.'",
                        seg.id
                    ));
                }
            }
        }
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
    /// segment coverage of both texts, id uniqueness, token ordering, gloss
    /// vocabulary, and link integrity. All violations are collected into one
    /// error string so the LLM self-correction retry sees every problem at
    /// once.
    ///
    /// # Errors
    /// Returns the newline-joined list of violations, if any.
    pub fn validate_structure(&self) -> Result<(), String> {
        let mut errors = Vec::new();
        let _ = self.source.locate_segments("source", &mut errors);
        let _ = self.target.locate_segments("target", &mut errors);
        self.source.check_glosses("source", &mut errors);
        self.target.check_glosses("target", &mut errors);
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
    fn composed_gloss_atoms_pass() {
        let mut a = demo();
        a.source.segments[1].gloss = Some("PST.PFV".to_string());
        a.source.segments[2].gloss = Some("3PL".to_string());
        a.source.segments[3].gloss = Some("NPST".to_string());
        a.source.segments[5].gloss = Some("3".to_string());
        a.source.segments[6].gloss = Some("NEG.1SG.POSS.COND".to_string());
        a.validate_structure()
            .expect("composed standard atoms should pass");
    }

    #[test]
    fn nonstandard_gloss_atom_is_rejected_with_suggestion() {
        let mut a = demo();
        a.source.segments[1].gloss = Some("PRES".to_string());
        let err = a.validate_structure().unwrap_err();
        assert!(
            err.contains("'PRES'") && err.contains("'PRS'"),
            "got: {err}"
        );
    }

    #[test]
    fn lowercase_gloss_atom_is_rejected() {
        let mut a = demo();
        a.source.segments[1].gloss = Some("pl".to_string());
        let err = a.validate_structure().unwrap_err();
        assert!(
            err.contains("UPPER CASE") && err.contains("'PL'"),
            "got: {err}"
        );
    }

    #[test]
    fn unknown_gloss_atom_is_rejected() {
        let mut a = demo();
        a.target.segments[5].gloss = Some("1SG.WIBBLE".to_string());
        let err = a.validate_structure().unwrap_err();
        assert!(
            err.contains("'WIBBLE'") && err.contains("standard Leipzig"),
            "got: {err}"
        );
    }

    #[test]
    fn empty_gloss_is_rejected() {
        let mut a = demo();
        a.source.segments[1].gloss = Some("  ".to_string());
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("use null"), "got: {err}");
    }

    #[test]
    fn empty_gloss_atom_is_rejected() {
        let mut a = demo();
        a.source.segments[1].gloss = Some("PST..PFV".to_string());
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("empty atom"), "got: {err}");
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
    }
}
