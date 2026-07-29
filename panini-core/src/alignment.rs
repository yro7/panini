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
    /// integrity. All violations are collected into one error
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
    }

    /// The translation, split into addressable segments.
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

    /// The source sentence of the pair, split into addressable segments. Has
    /// no `text` field: the source sentence is already known to the model
    /// from the input it was given, and its segments cover every
    /// non-whitespace character of it exactly once, so `text` is
    /// reconstructed server-side from the segments instead of being
    /// re-emitted.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct SourceSentence {
        /// All segments in reading order, covering every non-whitespace
        /// character of the source sentence exactly once. One segment per
        /// word by default; several when sub-word units align separately
        /// (agglutinative affixes, clitics, fused plurals). The stem is a
        /// segment too. Punctuation is its own word, usually in no link.
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
    }

    /// A sentence aligned with its translation, segment by segment.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct AlignedTranslation {
        /// The analyzed sentence, in the language being learned.
        pub source: SourceSentence,
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

            let source = derive_source_sentence(&self.source, &mut errors);
            let target = derive_sentence(
                &self.target.text,
                &self.target.segments,
                "target",
                &mut errors,
            );

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
        text: &str,
        wire_segments: &[AlignedSegment],
        side: &str,
        errors: &mut Vec<String>,
    ) -> super::AlignedSentence {
        let mut token: u32 = 0;
        let mut segments = Vec::with_capacity(wire_segments.len());
        for (i, seg) in wire_segments.iter().enumerate() {
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
                span: None,
            });
        }
        super::AlignedSentence {
            text: text.to_string(),
            segments,
        }
    }

    /// Reconstructs the source sentence's `text` from its segments alone —
    /// the source sentence is not re-emitted by the LLM — then derives ids
    /// and token indices exactly as [`derive_sentence`] does for the target.
    /// Segments cover every non-whitespace character of the source sentence
    /// exactly once, so concatenating them with one space before every
    /// segment whose `starts_new_token` is `true` (the first segment
    /// excepted) reconstructs `text` exactly — except closing punctuation
    /// (`.`, `,`, `;`, `:`, `!`, `?`, closing brackets/quotes, …), which is
    /// its own word (per the segmentation rules) but never preceded by
    /// whitespace in the original sentence.
    fn derive_source_sentence(
        sentence: &SourceSentence,
        errors: &mut Vec<String>,
    ) -> super::AlignedSentence {
        let mut text = String::new();
        for (i, seg) in sentence.segments.iter().enumerate() {
            if i > 0 && seg.starts_new_token && !attaches_without_leading_space(&seg.surface) {
                text.push(' ');
            }
            text.push_str(&seg.surface);
        }
        derive_sentence(&text, &sentence.segments, "source", errors)
    }

    /// Whether a punctuation-only segment attaches directly to the previous
    /// word with no space in between, as is the norm for closing punctuation
    /// across the supported languages.
    fn attaches_without_leading_space(surface: &str) -> bool {
        matches!(
            surface,
            "." | ","
                | ";"
                | ":"
                | "!"
                | "?"
                | ")"
                | "]"
                | "}"
                | "'"
                | "\u{2019}"
                | "\""
                | "\u{201D}"
                | "\u{2026}"
        )
    }

    /// Segment ids whose surface matches `surface`, in reading order. With
    /// `fold_case`, matching is case-insensitive (Unicode lowercase) — used as
    /// a fallback so the model's natural, capitalization-blind occurrence
    /// counting ("the third 'the'", counting a sentence-initial "The" too)
    /// resolves instead of being rejected.
    fn ids_matching(
        segments: &[super::AlignedSegment],
        surface: &str,
        fold_case: bool,
    ) -> Vec<u32> {
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

// ─── LLM wire format v2 (compact) ─────────────────────────────────────────────

/// Compact successor to [`wire`]: same alignment semantics, a fraction of the
/// output tokens.
///
/// [`wire`] spends most of its tokens on per-segment objects (`surface`,
/// `starts_new_token`) and per-reference objects (`surface`, `occurrence`) —
/// measured on real song alignments, enough to overflow `max_tokens`. Here the
/// word boundary is structural instead of flagged: a sentence is an array of
/// words, a word is an array of segment strings. Whitespace segments become
/// unrepresentable and the boundary flag disappears entirely. Link references
/// stay surface-based (autoregressive models are unreliable at maintaining
/// numbering systems — same rationale as [`wire`]) but collapse to a bare
/// string when the surface is unique in its sentence.
///
/// Key names are single letters because they repeat once per word and per
/// link; their meaning is carried by the schema descriptions below, which
/// double as the extraction spec shown to the LLM — keep them precise.
///
/// No tuples anywhere: JSON tuples become `prefixItems`, which strict
/// structured-output modes reject and rig's schema sanitizer does not
/// traverse. The occurrence-qualified reference is an object, and the
/// string-or-object union maps to `anyOf` via serde's untagged enum.
///
/// [`AlignedTranslation::resolve`](wire_v2::AlignedTranslation::resolve)
/// lowers to [`wire`] and delegates, so validation, the case-insensitive
/// occurrence fallback, and the self-correction error messages are shared
/// with v1 verbatim. The resolved, stored format is unchanged.
pub mod wire_v2 {
    use serde::{Deserialize, Serialize};

    use super::wire;

    /// Reference to one segment of a sentence, by its surface text: either
    /// the surface alone (a plain string), or an occurrence-qualified object
    /// when that surface appears more than once among the sentence's
    /// segments.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    #[serde(untagged)]
    pub enum SegRef {
        /// The referenced segment's text, copied exactly as it appears in
        /// that sentence's words (case-sensitive) — not the whole word, not
        /// a normalized form. Use this plain-string form when the segment
        /// text is unique among the sentence's segments.
        Surface(String),
        /// Occurrence-qualified reference, required when the same segment
        /// text appears more than once in the sentence.
        Occurrence {
            /// The referenced segment's text, copied exactly as it appears
            /// in that sentence's words (case-sensitive).
            s: String,
            /// 1-based position among this sentence's segments that have
            /// exactly this text, counted in reading order.
            o: u32,
        },
    }

    /// One correspondence between the two sentences. Many-to-many: either
    /// side may hold several segment references (discontinuous units like
    /// French "ne … pas" go in ONE link). Link ONLY segments that genuinely
    /// correspond in meaning or function; a segment with no counterpart in
    /// the other sentence appears in no link at all — never force a
    /// correspondence.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct Link {
        /// References to the source-sentence segments in this correspondence.
        #[schemars(length(min = 1))]
        pub s: Vec<SegRef>,
        /// References to the target-sentence segments in this correspondence.
        #[schemars(length(min = 1))]
        pub t: Vec<SegRef>,
    }

    /// The translation, split into words and segments.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct TargetSentence {
        /// The translated sentence exactly as displayed, unsegmented.
        pub x: String,
        /// The words of `x`, in reading order. Each word is the array of its
        /// segments: `["plaży"]` for a whole word, `["chc", "ę"]` when
        /// sub-word units align separately (agglutinative affixes, clitics,
        /// fused plurals — the stem is a segment too). Segments concatenate
        /// to the word exactly as written in `x` — never include whitespace,
        /// never merge two whitespace-separated words into one array. Each
        /// punctuation mark is its own one-segment word, usually in no link.
        pub w: Vec<Vec<String>>,
    }

    /// A sentence aligned with its translation, segment by segment.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
    pub struct AlignedTranslation {
        /// The words of the source sentence (the sentence being analyzed),
        /// in reading order. Each word is the array of its segments:
        /// `["plaży"]` for a whole word, `["d'", "eau"]` when sub-word units
        /// align separately (agglutinative affixes, clitics, fused plurals —
        /// the stem is a segment too). Segments concatenate to the word
        /// exactly as written — no added hyphens, no normalization, never
        /// any whitespace. Each punctuation mark is its own one-segment
        /// word. Every non-whitespace character of the source sentence must
        /// be covered exactly once.
        pub s: Vec<Vec<String>>,
        /// The translation, in the learner's UI language.
        pub t: TargetSentence,
        /// Word-by-word literal rendering of the source sentence in the
        /// target language, exposing the source's structure the way "pomme
        /// de terre" is literally "apple of earth". Follow the source's own
        /// word order and morphology. Null when it would read the same as
        /// `t.x`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lit: Option<String>,
        /// Many-to-many correspondences between source and target segments.
        pub l: Vec<Link>,
    }

    impl AlignedTranslation {
        /// Validates the compact structure and resolves it into the
        /// internal, id/token/span-addressed [`super::AlignedTranslation`]
        /// by lowering to the [`wire`] shape and delegating to
        /// [`wire::AlignedTranslation::resolve`] — ids, token indices, link
        /// resolution (occurrence fallback included) and character spans are
        /// all shared with v1.
        ///
        /// # Errors
        /// Returns the newline-joined list of violations, written for the
        /// LLM self-correction retry — every problem is reported at once.
        pub fn resolve(&self) -> Result<super::AlignedTranslation, String> {
            let mut errors = Vec::new();
            check_words(&self.s, "source", &mut errors);
            check_words(&self.t.w, "target", &mut errors);
            if !errors.is_empty() {
                return Err(errors.join("\n"));
            }
            self.lower().resolve()
        }

        /// Lowers the compact shape to the v1 wire shape: the first segment
        /// of each word starts a token, every other segment continues it.
        fn lower(&self) -> wire::AlignedTranslation {
            wire::AlignedTranslation {
                source: wire::SourceSentence {
                    segments: lower_words(&self.s),
                },
                target: wire::AlignedSentence {
                    text: self.t.x.clone(),
                    segments: lower_words(&self.t.w),
                },
                literal_translation: self.lit.clone(),
                links: self
                    .l
                    .iter()
                    .map(|link| wire::AlignmentLink {
                        source: link.s.iter().map(lower_ref).collect(),
                        target: link.t.iter().map(lower_ref).collect(),
                    })
                    .collect(),
            }
        }
    }

    /// Rejects empty word arrays — they would silently vanish in
    /// [`lower_words`]' flattening and shift every word boundary after them.
    fn check_words(words: &[Vec<String>], side: &str, errors: &mut Vec<String>) {
        for (i, word) in words.iter().enumerate() {
            if word.is_empty() {
                errors.push(format!(
                    "{side}: word {i} is an empty array — each word is a non-empty array of \
                     segment strings"
                ));
            }
        }
    }

    fn lower_words(words: &[Vec<String>]) -> Vec<wire::AlignedSegment> {
        words
            .iter()
            .flat_map(|word| {
                word.iter().enumerate().map(|(i, surface)| wire::AlignedSegment {
                    surface: surface.clone(),
                    starts_new_token: i == 0,
                })
            })
            .collect()
    }

    fn lower_ref(r: &SegRef) -> wire::SegmentRef {
        match r {
            SegRef::Surface(s) => wire::SegmentRef {
                surface: s.clone(),
                occurrence: None,
            },
            SegRef::Occurrence { s, o } => wire::SegmentRef {
                surface: s.clone(),
                occurrence: Some(*o),
            },
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(id: u32, token: u32, surface: &str) -> AlignedSegment {
        AlignedSegment {
            id,
            token,
            surface: surface.to_string(),
            span: None,
        }
    }

    fn link(source: &[u32], target: &[u32]) -> AlignmentLink {
        AlignmentLink {
            source: source.to_vec(),
            target: target.to_vec(),
        }
    }

    /// Turkish → French: one agglutinated word maps to four French units.
    fn demo() -> AlignedTranslation {
        AlignedTranslation {
            source: AlignedSentence {
                text: "Evlerimde kalıyorum".to_string(),
                segments: vec![
                    seg(0, 0, "Ev"),
                    seg(1, 0, "ler"),
                    seg(2, 0, "im"),
                    seg(3, 0, "de"),
                    seg(4, 1, "kal"),
                    seg(5, 1, "ıyor"),
                    seg(6, 1, "um"),
                ],
            },
            target: AlignedSentence {
                text: "Je reste dans mes maisons".to_string(),
                segments: vec![
                    seg(0, 0, "Je"),
                    seg(1, 1, "reste"),
                    seg(2, 2, "dans"),
                    seg(3, 3, "mes"),
                    seg(4, 4, "maison"),
                    seg(5, 4, "s"),
                ],
            },
            literal_translation: Some("dans mes maisons je-reste".to_string()),
            links: vec![
                link(&[0], &[4]),
                link(&[1], &[5]),
                link(&[2], &[3]),
                link(&[3], &[2]),
                link(&[4], &[1]),
                link(&[6], &[0]),
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
        a.source.segments.push(seg(7, 2, "."));
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
        a.links.push(link(&[5], &[]));
        let err = a.validate_structure().unwrap_err();
        assert!(err.contains("at least one segment"), "got: {err}");
    }

    #[test]
    fn legacy_json_with_gloss_and_kind_round_trips() {
        // Persisted rows predating the gloss/kind removal still carry those
        // fields; serde must ignore them and the row must keep deserializing.
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

    fn wseg(surface: &str, starts_new_token: bool) -> wire::AlignedSegment {
        wire::AlignedSegment {
            surface: surface.to_string(),
            starts_new_token,
        }
    }

    fn wref(surface: &str, occurrence: Option<u32>) -> wire::SegmentRef {
        wire::SegmentRef {
            surface: surface.to_string(),
            occurrence,
        }
    }

    fn wlink(source: Vec<wire::SegmentRef>, target: Vec<wire::SegmentRef>) -> wire::AlignmentLink {
        wire::AlignmentLink { source, target }
    }

    /// Turkish → French, same sentence as [`demo`] but in wire form.
    fn wire_demo() -> wire::AlignedTranslation {
        wire::AlignedTranslation {
            source: wire::SourceSentence {
                segments: vec![
                    wseg("Ev", true),
                    wseg("ler", false),
                    wseg("im", false),
                    wseg("de", false),
                    wseg("kal", true),
                    wseg("ıyor", false),
                    wseg("um", false),
                ],
            },
            target: wire::AlignedSentence {
                text: "Je reste dans mes maisons".to_string(),
                segments: vec![
                    wseg("Je", true),
                    wseg("reste", true),
                    wseg("dans", true),
                    wseg("mes", true),
                    wseg("maison", true),
                    wseg("s", false),
                ],
            },
            literal_translation: Some("dans mes maisons je-reste".to_string()),
            links: vec![
                wlink(vec![wref("Ev", None)], vec![wref("maison", None)]),
                wlink(vec![wref("ler", None)], vec![wref("s", None)]),
                wlink(vec![wref("im", None)], vec![wref("mes", None)]),
                wlink(vec![wref("de", None)], vec![wref("dans", None)]),
                wlink(vec![wref("kal", None)], vec![wref("reste", None)]),
                wlink(vec![wref("um", None)], vec![wref("Je", None)]),
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
            source: wire::SourceSentence {
                segments: vec![
                    wseg("Lubi", true),
                    wseg("ę", false),
                    wseg("kaw", true),
                    wseg("ę", false),
                    wseg(".", true),
                ],
            },
            target: wire::AlignedSentence {
                text: "I like coffee.".to_string(),
                segments: vec![
                    wseg("I", true),
                    wseg("like", true),
                    wseg("coffee", true),
                    wseg(".", true),
                ],
            },
            literal_translation: None,
            links: vec![
                wlink(vec![wref("Lubi", None)], vec![wref("like", None)]),
                wlink(vec![wref("ę", Some(1))], vec![wref("I", None)]),
                wlink(vec![wref("kaw", None)], vec![wref("coffee", None)]),
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
            source: wire::SourceSentence {
                segments: vec![wseg("Czytamy", true), wseg(".", true)],
            },
            target: wire::AlignedSentence {
                text: "We are reading.".to_string(),
                segments: vec![
                    wseg("We", true),
                    wseg("are", true),
                    wseg("reading", false), // wrongly fused into "are"'s word
                    wseg(".", true),
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
        a.links.push(wlink(vec![wref("kaw", None)], vec![]));
        let err = a.resolve().unwrap_err();
        assert!(err.contains("at least one segment"), "got: {err}");
    }

    #[test]
    fn wire_case_insensitive_occurrence_fallback() {
        // The model counts "the" three times (The + the + the); the exact
        // lowercase bucket has only two, so occurrence 3 must fall back to the
        // case-insensitive bucket and resolve to the last "the".
        let a = wire::AlignedTranslation {
            source: wire::SourceSentence {
                segments: vec![wseg("Pies", true), wseg(".", true)],
            },
            target: wire::AlignedSentence {
                text: "The dog and the cat and the bird.".to_string(),
                segments: vec![
                    wseg("The", true),
                    wseg("dog", true),
                    wseg("and", true),
                    wseg("the", true),
                    wseg("cat", true),
                    wseg("and", true),
                    wseg("the", true),
                    wseg("bird", true),
                    wseg(".", true),
                ],
            },
            literal_translation: None,
            links: vec![wlink(vec![wref("Pies", None)], vec![wref("the", Some(3))])],
        };
        let resolved = a
            .resolve()
            .expect("case-insensitive fallback should resolve");
        // "The"(0), "the"(3), "the"(6): occurrence 3 counted case-insensitively
        // is the segment at id 6.
        assert_eq!(resolved.links[0].target, vec![6]);
    }

    #[test]
    fn wire_llm_json_resolves() {
        let json = serde_json::json!({
            "source": {
                "segments": [
                    { "surface": "Lubi", "starts_new_token": true },
                    { "surface": "ę", "starts_new_token": false },
                    { "surface": "kaw", "starts_new_token": true },
                    { "surface": "ę", "starts_new_token": false },
                    { "surface": ".", "starts_new_token": true }
                ]
            },
            "target": {
                "text": "I like coffee.",
                "segments": [
                    { "surface": "I", "starts_new_token": true },
                    { "surface": "like", "starts_new_token": true },
                    { "surface": "coffee", "starts_new_token": true },
                    { "surface": ".", "starts_new_token": true }
                ]
            },
            "literal_translation": null,
            "links": [
                { "source": [{ "surface": "ę", "occurrence": 2 }], "target": [{ "surface": "coffee" }] }
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

    // ── Wire format v2 ──

    fn w2(segments: &[&str]) -> Vec<String> {
        segments.iter().map(|s| s.to_string()).collect()
    }

    fn v2ref(surface: &str) -> wire_v2::SegRef {
        wire_v2::SegRef::Surface(surface.to_string())
    }

    fn v2occ(surface: &str, o: u32) -> wire_v2::SegRef {
        wire_v2::SegRef::Occurrence {
            s: surface.to_string(),
            o,
        }
    }

    fn v2link(s: Vec<wire_v2::SegRef>, t: Vec<wire_v2::SegRef>) -> wire_v2::Link {
        wire_v2::Link { s, t }
    }

    /// Turkish → French, the exact sentence of [`wire_demo`] in v2 form.
    fn v2_demo() -> wire_v2::AlignedTranslation {
        wire_v2::AlignedTranslation {
            s: vec![w2(&["Ev", "ler", "im", "de"]), w2(&["kal", "ıyor", "um"])],
            t: wire_v2::TargetSentence {
                x: "Je reste dans mes maisons".to_string(),
                w: vec![
                    w2(&["Je"]),
                    w2(&["reste"]),
                    w2(&["dans"]),
                    w2(&["mes"]),
                    w2(&["maison", "s"]),
                ],
            },
            lit: Some("dans mes maisons je-reste".to_string()),
            l: vec![
                v2link(vec![v2ref("Ev")], vec![v2ref("maison")]),
                v2link(vec![v2ref("ler")], vec![v2ref("s")]),
                v2link(vec![v2ref("im")], vec![v2ref("mes")]),
                v2link(vec![v2ref("de")], vec![v2ref("dans")]),
                v2link(vec![v2ref("kal")], vec![v2ref("reste")]),
                v2link(vec![v2ref("um")], vec![v2ref("Je")]),
            ],
        }
    }

    #[test]
    fn v2_resolves_identically_to_v1() {
        let from_v2 = v2_demo().resolve().expect("v2 demo should resolve");
        let from_v1 = wire_demo().resolve().expect("v1 demo should resolve");
        assert_eq!(from_v2, from_v1);
    }

    /// French → Polish: discontinuous "ne … pas", subject fused into a verb
    /// ending, partitive expressed by a case ending.
    #[test]
    fn v2_discontinuous_negation_and_morpheme_fusion() {
        let a = wire_v2::AlignedTranslation {
            s: vec![
                w2(&["Je"]),
                w2(&["ne"]),
                w2(&["veux"]),
                w2(&["pas"]),
                w2(&["d'", "eau"]),
            ],
            t: wire_v2::TargetSentence {
                x: "Nie chcę wody".to_string(),
                w: vec![w2(&["Nie"]), w2(&["chc", "ę"]), w2(&["wod", "y"])],
            },
            lit: None,
            l: vec![
                v2link(vec![v2ref("ne"), v2ref("pas")], vec![v2ref("Nie")]),
                v2link(vec![v2ref("veux")], vec![v2ref("chc")]),
                v2link(vec![v2ref("Je")], vec![v2ref("ę")]),
                v2link(vec![v2ref("eau")], vec![v2ref("wod")]),
                v2link(vec![v2ref("d'")], vec![v2ref("y")]),
            ],
        };
        let resolved = a.resolve().expect("should resolve");

        // Source: Je=0 ne=1 veux=2 pas=3 d'=4 eau=5, tokens 0..4 with d'+eau
        // sharing token 4.
        let tokens: Vec<u32> = resolved.source.segments.iter().map(|s| s.token).collect();
        assert_eq!(tokens, vec![0, 1, 2, 3, 4, 4]);
        // ne…pas → Nie: one link, two source ids.
        assert_eq!(resolved.links[0].source, vec![1, 3]);
        assert_eq!(resolved.links[0].target, vec![0]);
        // Reconstructed source text keeps the apostrophe word intact.
        assert_eq!(resolved.source.text, "Je ne veux pas d'eau");
    }

    /// The combined torture case: a multi-word expression whose first word is
    /// a repeated AND fragmented surface, a repeated unlinked word, and `de`
    /// distinct from `des`.
    #[test]
    fn v2_mwe_with_repeated_fragmented_surfaces() {
        let a = wire_v2::AlignedTranslation {
            s: vec![
                w2(&["Je"]),
                w2(&["mange"]),
                w2(&["des"]),
                w2(&["pomme", "s"]),
                w2(&["de"]),
                w2(&["terre"]),
                w2(&["et"]),
                w2(&["des"]),
                w2(&["pomme", "s"]),
            ],
            t: wire_v2::TargetSentence {
                x: "I eat potatoes and apples".to_string(),
                w: vec![
                    w2(&["I"]),
                    w2(&["eat"]),
                    w2(&["potato", "es"]),
                    w2(&["and"]),
                    w2(&["apple", "s"]),
                ],
            },
            lit: Some("I eat some apples of earth and some apples".to_string()),
            l: vec![
                v2link(vec![v2ref("Je")], vec![v2ref("I")]),
                v2link(vec![v2ref("mange")], vec![v2ref("eat")]),
                v2link(
                    vec![v2occ("pomme", 1), v2ref("de"), v2ref("terre")],
                    vec![v2ref("potato")],
                ),
                v2link(vec![v2occ("s", 1)], vec![v2ref("es")]),
                v2link(vec![v2ref("et")], vec![v2ref("and")]),
                v2link(vec![v2occ("pomme", 2)], vec![v2ref("apple")]),
                v2link(vec![v2occ("s", 2)], vec![v2ref("s")]),
            ],
        };
        let resolved = a.resolve().expect("should resolve");

        // Flat ids: Je=0 mange=1 des=2 pomme=3 s=4 de=5 terre=6 et=7 des=8
        // pomme=9 s=10. The MWE picks the FIRST "pomme" plus "de"+"terre".
        assert_eq!(resolved.links[2].source, vec![3, 5, 6]);
        assert_eq!(resolved.links[2].target, vec![2]);
        // Fragment occurrences resolve across different words.
        assert_eq!(resolved.links[3].source, vec![4]);
        assert_eq!(resolved.links[6].source, vec![10]);
        // "des" (ids 2 and 8) is linked nowhere.
        let linked: std::collections::HashSet<u32> = resolved
            .links
            .iter()
            .flat_map(|l| l.source.iter().copied())
            .collect();
        assert!(!linked.contains(&2) && !linked.contains(&8));
    }

    /// ABACD → CADB: full reordering with a repeated source word of which
    /// only the second occurrence is linked.
    #[test]
    fn v2_reordering_with_repeated_word() {
        let a = wire_v2::AlignedTranslation {
            s: vec![w2(&["A"]), w2(&["B"]), w2(&["A"]), w2(&["C"]), w2(&["D"])],
            t: wire_v2::TargetSentence {
                x: "C A D B".to_string(),
                w: vec![w2(&["C"]), w2(&["A"]), w2(&["D"]), w2(&["B"])],
            },
            lit: None,
            l: vec![
                v2link(vec![v2occ("A", 2)], vec![v2ref("A")]),
                v2link(vec![v2ref("B")], vec![v2ref("B")]),
                v2link(vec![v2ref("C")], vec![v2ref("C")]),
                v2link(vec![v2ref("D")], vec![v2ref("D")]),
            ],
        };
        let resolved = a.resolve().expect("should resolve");
        assert_eq!(resolved.links[0].source, vec![2]); // second A
        assert_eq!(resolved.links[0].target, vec![1]);
        assert_eq!(resolved.links[1].target, vec![3]); // B moved last
    }

    #[test]
    fn v2_empty_word_is_rejected() {
        let mut a = v2_demo();
        a.s.push(Vec::new());
        let err = a.resolve().unwrap_err();
        assert!(err.contains("word 2 is an empty array"), "got: {err}");
    }

    #[test]
    fn v2_ambiguous_ref_without_occurrence_is_rejected() {
        let mut a = v2_demo();
        // "s" appears once; make it ambiguous by splitting "dans" into
        // ["dan", "s"], then reference "s" without an occurrence.
        a.t.w[2] = w2(&["dan", "s"]);
        let err = a.resolve().unwrap_err();
        assert!(
            err.contains("appears 2 times") && err.contains("occurrence"),
            "got: {err}"
        );
    }

    #[test]
    fn v2_llm_json_with_mixed_refs_resolves() {
        // Exactly what the model emits: plain-string refs, one
        // occurrence-qualified object ref, no `lit`.
        let json = serde_json::json!({
            "s": [["Lubi", "ę"], ["kaw", "ę"], ["."]],
            "t": {
                "x": "I like coffee.",
                "w": [["I"], ["like"], ["coffee"], ["."]]
            },
            "l": [
                { "s": ["Lubi"], "t": ["like"] },
                { "s": [{ "s": "ę", "o": 1 }], "t": ["I"] },
                { "s": ["kaw"], "t": ["coffee"] }
            ]
        });
        let a: wire_v2::AlignedTranslation =
            serde_json::from_value(json).expect("LLM-shaped v2 JSON should deserialize");
        let resolved = a.resolve().expect("should resolve");
        assert_eq!(resolved.links[1].source, vec![1]); // first 'ę'
        assert!(resolved.literal_translation.is_none());
        // Token derivation from word nesting: Lubię=0, kawę=1, "."=2.
        let tokens: Vec<u32> = resolved.source.segments.iter().map(|s| s.token).collect();
        assert_eq!(tokens, vec![0, 0, 1, 1, 2]);
    }

    #[test]
    fn v2_case_insensitive_occurrence_fallback_is_shared() {
        // Same scenario as `wire_case_insensitive_occurrence_fallback`,
        // through the v2 path: delegation to v1 keeps the fallback.
        let a = wire_v2::AlignedTranslation {
            s: vec![w2(&["Pies"]), w2(&["."])],
            t: wire_v2::TargetSentence {
                x: "The dog and the cat and the bird.".to_string(),
                w: vec![
                    w2(&["The"]),
                    w2(&["dog"]),
                    w2(&["and"]),
                    w2(&["the"]),
                    w2(&["cat"]),
                    w2(&["and"]),
                    w2(&["the"]),
                    w2(&["bird"]),
                    w2(&["."]),
                ],
            },
            lit: None,
            l: vec![v2link(vec![v2ref("Pies")], vec![v2occ("the", 3)])],
        };
        let resolved = a.resolve().expect("fallback should resolve");
        assert_eq!(resolved.links[0].target, vec![6]);
    }

    #[test]
    fn v2_segref_serialization_shapes() {
        // The two SegRef forms must serialize to a bare string and a {s,o}
        // object — the shapes the schema promises the model.
        let link = v2link(vec![v2ref("plaży")], vec![v2occ("la", 2)]);
        let out = serde_json::to_value(&link).expect("should serialize");
        assert_eq!(out["s"][0], serde_json::json!("plaży"));
        assert_eq!(out["t"][0], serde_json::json!({ "s": "la", "o": 2 }));
    }
}
