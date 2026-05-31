use std::collections::HashMap;

use crate::{Aggregable, FieldDescriptor, FieldKind};

// ─── Dimension types ──────────────────────────────────────────────────────────

/// A closed-set dimension: all possible values are known upfront.
///
/// `possible` is populated at initialization from `FieldKind::Closed`.
/// `counts` may not contain all possible values (zero counts are omitted).
#[derive(Debug, Clone, Default)]
pub struct Distribution {
    pub possible: Vec<String>,
    pub counts: HashMap<String, usize>,
}

impl Distribution {
    #[must_use]
    pub fn new(possible: &[&'static str]) -> Self {
        Self {
            possible: possible
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            counts: HashMap::new(),
        }
    }

    /// Number of distinct possible values actually observed.
    #[must_use]
    pub fn seen_count(&self) -> usize {
        self.counts.len()
    }

    /// Total number of possible values.
    #[must_use]
    pub const fn total_count(&self) -> usize {
        self.possible.len()
    }

    /// Coverage: (seen, total)
    #[must_use]
    pub fn coverage(&self) -> (usize, usize) {
        (self.seen_count(), self.total_count())
    }

    /// Coverage percentage (0.0 to 1.0)
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn coverage_percent(&self) -> f64 {
        if self.possible.is_empty() {
            0.0
        } else {
            self.seen_count() as f64 / self.total_count() as f64
        }
    }

    fn merge_possible(&mut self, possible: impl IntoIterator<Item = String>) {
        for value in possible {
            if !self.possible.contains(&value) {
                self.possible.push(value);
            }
        }
        self.possible.sort();
    }
}

/// An open-set dimension: values are arbitrary strings (e.g. `lemma`, `base_form`).
#[derive(Debug, Clone, Default)]
pub struct Inventory {
    pub counts: HashMap<String, usize>,
}

/// A single dimension in a `GroupResult`.
#[derive(Debug, Clone)]
pub enum Dimension {
    Dist(Distribution),
    Inv(Inventory),
}

impl Dimension {
    fn record(&mut self, value: String) {
        match self {
            Self::Dist(d) => *d.counts.entry(value).or_insert(0) += 1,
            Self::Inv(i) => *i.counts.entry(value).or_insert(0) += 1,
        }
    }

    fn from_descriptor(descriptor: &FieldDescriptor) -> Self {
        match &descriptor.kind {
            FieldKind::Closed(variants) => Self::Dist(Distribution::new(variants)),
            FieldKind::Open => Self::Inv(Inventory::default()),
        }
    }

    fn merge_descriptor(&mut self, descriptor: &FieldDescriptor) {
        match (&mut *self, &descriptor.kind) {
            (Self::Inv(_), _) => {}
            (Self::Dist(_), FieldKind::Open) => self.promote_to_inventory(),
            (Self::Dist(dist), FieldKind::Closed(variants)) => {
                dist.merge_possible(variants.iter().map(|value| (*value).to_string()));
            }
        }
    }

    fn merge_dimension(&mut self, other: Self) {
        match (&mut *self, other) {
            (Self::Dist(existing), Self::Dist(incoming)) => {
                existing.merge_possible(incoming.possible);
                merge_counts(&mut existing.counts, incoming.counts);
            }
            (Self::Inv(existing), Self::Inv(incoming)) => {
                merge_counts(&mut existing.counts, incoming.counts);
            }
            (Self::Inv(existing), Self::Dist(incoming)) => {
                merge_counts(&mut existing.counts, incoming.counts);
            }
            (Self::Dist(_), Self::Inv(incoming)) => {
                self.promote_to_inventory();
                if let Self::Inv(existing) = self {
                    merge_counts(&mut existing.counts, incoming.counts);
                }
            }
        }
    }

    fn promote_to_inventory(&mut self) {
        if let Self::Dist(dist) = self {
            let counts = std::mem::take(&mut dist.counts);
            *self = Self::Inv(Inventory { counts });
        }
    }
}

fn merge_counts(target: &mut HashMap<String, usize>, source: HashMap<String, usize>) {
    for (value, count) in source {
        *target.entry(value).or_insert(0) += count;
    }
}

// ─── GroupResult ──────────────────────────────────────────────────────────────

/// Aggregated data for a single group (e.g. "Noun", "Verb", "morpheme").
///
/// `total` is the sum of all `AggregationContribution::total_increment` values
/// recorded for this group. For POS groups (`"Noun"`, `"Verb"`, …) it equals
/// the number of word tokens. For the `"morpheme"` group it equals the
/// **morpheme count** — one contribution per `ExtractedMorpheme`,
/// `total_increment = 1` — not the segmented-word count.
#[derive(Debug, Clone, Default)]
pub struct GroupResult {
    pub total: usize,
    pub dimensions: HashMap<String, Dimension>,
}

impl GroupResult {
    fn from_descriptors(descriptors: &[FieldDescriptor]) -> Self {
        let mut dimensions = HashMap::new();
        for d in descriptors {
            dimensions.insert(d.name.clone(), Dimension::from_descriptor(d));
        }
        Self {
            total: 0,
            dimensions,
        }
    }

    fn merge_descriptors(&mut self, descriptors: &[FieldDescriptor]) {
        for descriptor in descriptors {
            match self.dimensions.get_mut(&descriptor.name) {
                Some(dim) => dim.merge_descriptor(descriptor),
                None => {
                    self.dimensions.insert(
                        descriptor.name.clone(),
                        Dimension::from_descriptor(descriptor),
                    );
                }
            }
        }
    }
}

// ─── AggregationContribution ─────────────────────────────────────────────────

/// One unit of aggregation — emitted by a component (or by the typed
/// `Aggregable` shim) and consumed by an `AggregationSink`.
///
/// `total_increment` controls how much is added to `GroupResult::total`.
/// For most contributions this is `1`. Components may use other values when
/// a single logical unit spans multiple observations (e.g. weighted counts).
#[derive(Debug, Clone)]
pub struct AggregationContribution {
    pub group: String,
    pub descriptors: Vec<FieldDescriptor>,
    pub observations: Vec<Vec<(String, String)>>,
    pub total_increment: usize,
}

// ─── AggregationSink ─────────────────────────────────────────────────────────

/// Object-safe consumer of [`AggregationContribution`]s.
///
/// Implemented by [`BasicAggregator`], [`LearnerProfileAggregator`],
/// [`PivotingSink`], and any custom aggregation strategy.
///
/// The blanket `record<A: Aggregable>` shim is a default method — it converts
/// any `Aggregable` item into an `AggregationContribution` with
/// `total_increment = 1`. Concrete impls only need to provide
/// `record_contribution`.
pub trait AggregationSink {
    /// Low-level ingest of a pre-projected contribution.
    fn record_contribution(&mut self, c: AggregationContribution);

    /// Typed shim: converts any [`Aggregable`] to a contribution and records it.
    /// `total_increment` is always `1` via this path.
    ///
    /// Bounded `where Self: Sized` so `dyn AggregationSink` stays object-safe;
    /// call `record_contribution` directly on trait objects.
    fn record<A: Aggregable + ?Sized>(&mut self, item: &A)
    where
        Self: Sized,
    {
        self.record_contribution(AggregationContribution {
            group: item.group_key(),
            descriptors: item.instance_descriptors(),
            observations: item.observations(),
            total_increment: 1,
        });
    }
}

/// Records an [`Aggregable`] item into a `dyn AggregationSink`.
///
/// Equivalent to `sink.record(item)` for concrete sinks. Use this when `sink` is
/// a trait object — the generic `record<A>` method is not available on `dyn AggregationSink`.
pub fn record_aggregable<A: Aggregable + ?Sized>(sink: &mut dyn AggregationSink, item: &A) {
    sink.record_contribution(AggregationContribution {
        group: item.group_key(),
        descriptors: item.instance_descriptors(),
        observations: item.observations(),
        total_increment: 1,
    });
}

// ─── PivotingSink ────────────────────────────────────────────────────────────

/// Wraps any [`AggregationSink`] and overrides the `group` of every
/// contribution before forwarding it.
///
/// Used when the caller wants to re-key contributions (e.g. pivot morphology
/// stats by Arabic root or by skill node).
pub struct PivotingSink<'a, S: AggregationSink + ?Sized> {
    pub inner: &'a mut S,
    pub pivot: &'a dyn Fn(&AggregationContribution) -> String,
}

impl<S: AggregationSink + ?Sized> AggregationSink for PivotingSink<'_, S> {
    fn record_contribution(&mut self, mut c: AggregationContribution) {
        c.group = (self.pivot)(&c);
        self.inner.record_contribution(c);
    }
}

// ─── Aggregator trait ─────────────────────────────────────────────────────────

/// Extension of [`AggregationSink`] for aggregators that produce a typed output.
///
/// Enables generic code over "finishable sinks" — e.g. a mean aggregator, a
/// histogram builder, or a custom strategy can all satisfy `Aggregator<Output = T>`.
/// The `record_contribution` / `record<A>` methods are inherited from `AggregationSink`.
pub trait Aggregator: AggregationSink {
    /// The final result type produced by this aggregator.
    type Output;

    /// Consume the aggregator and return the final result.
    fn finish(self) -> Self::Output;
}

// ─── AggregationResult ────────────────────────────────────────────────────────

/// Aggregated statistics across all recorded contributions.
///
/// Can be built by collecting an iterator of [`Aggregable`] items, by driving
/// a [`BasicAggregator`], or by recording [`AggregationContribution`]s directly
/// (since `AggregationResult` itself implements [`AggregationSink`]).
#[derive(Debug, Clone, Default)]
pub struct AggregationResult {
    pub by_group: HashMap<String, GroupResult>,
}

impl AggregationSink for AggregationResult {
    fn record_contribution(&mut self, c: AggregationContribution) {
        let group_result = self
            .by_group
            .entry(c.group)
            .or_insert_with(|| GroupResult::from_descriptors(&c.descriptors));
        group_result.merge_descriptors(&c.descriptors);
        group_result.total += c.total_increment;
        for observation in c.observations {
            for (field, value) in observation {
                group_result
                    .dimensions
                    .entry(field)
                    .or_insert_with(|| Dimension::Inv(Inventory::default()))
                    .record(value);
            }
        }
    }
}

impl AggregationResult {
    /// Merge another result into this one (additive).
    pub fn merge(&mut self, other: Self) {
        for (group, other_group) in other.by_group {
            let entry = self.by_group.entry(group).or_default();
            entry.total += other_group.total;
            for (field, other_dim) in other_group.dimensions {
                match entry.dimensions.get_mut(&field) {
                    Some(existing) => existing.merge_dimension(other_dim),
                    None => {
                        entry.dimensions.insert(field, other_dim);
                    }
                }
            }
        }
    }

    /// Total number of items aggregated across all groups.
    #[must_use]
    pub fn total_count(&self) -> usize {
        self.by_group.values().map(|g| g.total).sum()
    }

    /// Number of distinct groups.
    #[must_use]
    pub fn group_count(&self) -> usize {
        self.by_group.len()
    }

    /// Print the aggregation result in a human-readable format.
    pub fn print(&self) {
        let opts = crate::aggregable::digest_output::DigestOptions {
            max_values_per_dimension: 5,
            exclude_dimensions: Vec::new(),
        };
        let groups = self.to_digest(&opts);
        for group in &groups {
            println!("\n[{}] total: {}", group.key.to_uppercase(), group.total);
            for dim in &group.dimensions {
                let header = match dim.total_possible {
                    Some(total) => format!("[{}/{}]", dim.unique_count, total),
                    None => format!("[{}unique]", dim.unique_count),
                };
                print!("  |- {} {}: ", dim.key, header);
                let summary: Vec<String> = dim
                    .values
                    .iter()
                    .map(|v| format!("{}({})", v.value, v.count))
                    .collect();
                let suffix = if dim.truncated { ", ..." } else { "" };
                println!("{}{}", summary.join(", "), suffix);
            }
        }
    }
}

impl<A: Aggregable> FromIterator<A> for AggregationResult {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let mut result = Self::default();
        for item in iter {
            result.record(&item);
        }
        result
    }
}

impl<A: Aggregable> Extend<A> for AggregationResult {
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
        for item in iter {
            self.record(&item);
        }
    }
}

// ─── BasicAggregator ──────────────────────────────────────────────────────────

/// Default aggregator — can ingest any [`AggregationContribution`] or any
/// [`Aggregable`] item via the typed shim.
#[derive(Debug, Clone, Default)]
pub struct BasicAggregator {
    result: AggregationResult,
}

impl BasicAggregator {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Borrow of the in-progress result (without consuming).
    #[must_use]
    pub const fn result(&self) -> &AggregationResult {
        &self.result
    }
}

impl AggregationSink for BasicAggregator {
    fn record_contribution(&mut self, c: AggregationContribution) {
        self.result.record_contribution(c);
    }
}

impl Aggregator for BasicAggregator {
    type Output = AggregationResult;

    fn finish(self) -> AggregationResult {
        self.result
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FieldDescriptor;

    // Mock Aggregable for testing
    #[derive(Debug, Clone)]
    struct MockAggregable {
        group: String,
        descriptors: Vec<FieldDescriptor>,
        observations: Vec<Vec<(String, String)>>,
    }

    impl MockAggregable {
        fn new(group: &str, descriptors: Vec<FieldDescriptor>) -> Self {
            Self {
                group: group.to_string(),
                descriptors,
                observations: Vec::new(),
            }
        }

        fn with_observation(mut self, obs: Vec<(String, String)>) -> Self {
            self.observations.push(obs);
            self
        }
    }

    impl Aggregable for MockAggregable {
        fn group_key(&self) -> String {
            self.group.clone()
        }

        fn instance_descriptors(&self) -> Vec<FieldDescriptor> {
            self.descriptors.clone()
        }

        fn observations(&self) -> Vec<Vec<(String, String)>> {
            self.observations.clone()
        }
    }

    #[test]
    fn basic_aggregator_on_mock_aggregable() {
        let descriptors = vec![FieldDescriptor {
            name: "case".to_string(),
            kind: FieldKind::Closed(&["Nominative", "Accusative", "Dative"]),
        }];

        let item1 = MockAggregable::new("Noun", descriptors.clone())
            .with_observation(vec![("case".to_string(), "Nominative".to_string())]);
        let item2 = MockAggregable::new("Noun", descriptors.clone())
            .with_observation(vec![("case".to_string(), "Nominative".to_string())]);
        let item3 = MockAggregable::new("Noun", descriptors)
            .with_observation(vec![("case".to_string(), "Accusative".to_string())]);

        let mut agg = BasicAggregator::new();
        agg.record(&item1);
        agg.record(&item2);
        agg.record(&item3);

        let result = agg.finish();
        assert_eq!(result.total_count(), 3);
        assert_eq!(result.group_count(), 1);

        let noun_group = &result.by_group["Noun"];
        assert_eq!(noun_group.total, 3);

        if let Dimension::Dist(case_dist) = &noun_group.dimensions["case"] {
            assert_eq!(case_dist.counts["Nominative"], 2);
            assert_eq!(case_dist.counts["Accusative"], 1);
            assert_eq!(case_dist.seen_count(), 2);
            assert_eq!(case_dist.total_count(), 3);
        } else {
            panic!("Expected Distribution for case");
        }
    }

    #[test]
    fn basic_aggregator_heterogeneous_input() {
        let descriptors1 = vec![FieldDescriptor {
            name: "case".to_string(),
            kind: FieldKind::Closed(&["Nominative", "Accusative"]),
        }];
        let descriptors2 = vec![FieldDescriptor {
            name: "tense".to_string(),
            kind: FieldKind::Closed(&["Present", "Past"]),
        }];

        let noun = MockAggregable::new("Noun", descriptors1)
            .with_observation(vec![("case".to_string(), "Nominative".to_string())]);
        let verb = MockAggregable::new("Verb", descriptors2)
            .with_observation(vec![("tense".to_string(), "Present".to_string())]);

        let mut agg = BasicAggregator::new();
        agg.record(&noun);
        agg.record(&verb);

        let result = agg.finish();
        assert_eq!(result.total_count(), 2);
        assert_eq!(result.group_count(), 2);
        assert!(result.by_group.contains_key("Noun"));
        assert!(result.by_group.contains_key("Verb"));
    }

    #[test]
    fn coverage_calculation_closed_vs_open() {
        let descriptors = vec![
            FieldDescriptor {
                name: "case".to_string(),
                kind: FieldKind::Closed(&["Nominative", "Accusative", "Dative"]),
            },
            FieldDescriptor {
                name: "lemma".to_string(),
                kind: FieldKind::Open,
            },
        ];

        let item1 = MockAggregable::new("Noun", descriptors.clone()).with_observation(vec![
            ("case".to_string(), "Nominative".to_string()),
            ("lemma".to_string(), "pies".to_string()),
        ]);
        let item2 = MockAggregable::new("Noun", descriptors).with_observation(vec![
            ("case".to_string(), "Accusative".to_string()),
            ("lemma".to_string(), "kot".to_string()),
        ]);

        let mut agg = BasicAggregator::new();
        agg.record(&item1);
        agg.record(&item2);

        let result = agg.finish();
        let noun = &result.by_group["Noun"];

        if let Dimension::Dist(case) = &noun.dimensions["case"] {
            assert_eq!(case.coverage(), (2, 3));
            assert!((case.coverage_percent() - 0.666).abs() < 0.01);
        } else {
            panic!("Expected Distribution for case");
        }

        if let Dimension::Inv(lemma) = &noun.dimensions["lemma"] {
            assert_eq!(lemma.counts["pies"], 1);
            assert_eq!(lemma.counts["kot"], 1);
        } else {
            panic!("Expected Inventory for lemma");
        }
    }

    #[test]
    fn merge_two_results() {
        let descriptors = vec![FieldDescriptor {
            name: "case".to_string(),
            kind: FieldKind::Closed(&["Nominative", "Accusative"]),
        }];

        let item1 = MockAggregable::new("Noun", descriptors.clone())
            .with_observation(vec![("case".to_string(), "Nominative".to_string())]);
        let item2 = MockAggregable::new("Noun", descriptors)
            .with_observation(vec![("case".to_string(), "Accusative".to_string())]);

        let result1: AggregationResult = std::iter::once(item1).collect();
        let result2: AggregationResult = std::iter::once(item2).collect();

        let mut merged = result1;
        merged.merge(result2);

        assert_eq!(merged.total_count(), 2);
        let noun = &merged.by_group["Noun"];
        if let Dimension::Dist(case) = &noun.dimensions["case"] {
            assert_eq!(case.counts["Nominative"], 1);
            assert_eq!(case.counts["Accusative"], 1);
        }
    }

    #[test]
    fn later_descriptors_in_same_group_are_recorded() {
        let case_item = MockAggregable::new(
            "Mixed",
            vec![FieldDescriptor {
                name: "case".to_string(),
                kind: FieldKind::Closed(&["Nominative", "Accusative"]),
            }],
        )
        .with_observation(vec![("case".to_string(), "Nominative".to_string())]);
        let tense_item = MockAggregable::new(
            "Mixed",
            vec![FieldDescriptor {
                name: "tense".to_string(),
                kind: FieldKind::Closed(&["Present", "Past"]),
            }],
        )
        .with_observation(vec![("tense".to_string(), "Present".to_string())]);

        let mut agg = BasicAggregator::new();
        agg.record(&case_item);
        agg.record(&tense_item);
        let result = agg.finish();
        let mixed = &result.by_group["Mixed"];

        assert!(mixed.dimensions.contains_key("case"));
        assert!(mixed.dimensions.contains_key("tense"));
        if let Dimension::Dist(tense) = &mixed.dimensions["tense"] {
            assert_eq!(tense.counts["Present"], 1);
            assert_eq!(tense.total_count(), 2);
        } else {
            panic!("Expected Distribution for tense");
        }
    }

    #[test]
    fn closed_descriptor_merge_unions_possible_values() {
        let first = MockAggregable::new(
            "Noun",
            vec![FieldDescriptor {
                name: "case".to_string(),
                kind: FieldKind::Closed(&["Nominative", "Accusative"]),
            }],
        )
        .with_observation(vec![("case".to_string(), "Nominative".to_string())]);
        let second = MockAggregable::new(
            "Noun",
            vec![FieldDescriptor {
                name: "case".to_string(),
                kind: FieldKind::Closed(&["Genitive", "Accusative"]),
            }],
        )
        .with_observation(vec![("case".to_string(), "Genitive".to_string())]);

        let mut agg = BasicAggregator::new();
        agg.record(&first);
        agg.record(&second);
        let result = agg.finish();

        if let Dimension::Dist(case) = &result.by_group["Noun"].dimensions["case"] {
            assert_eq!(case.total_count(), 3);
            assert_eq!(case.counts["Nominative"], 1);
            assert_eq!(case.counts["Genitive"], 1);
        } else {
            panic!("Expected Distribution for case");
        }
    }

    #[test]
    fn open_closed_conflict_promotes_to_inventory() {
        let closed = MockAggregable::new(
            "Noun",
            vec![FieldDescriptor {
                name: "case".to_string(),
                kind: FieldKind::Closed(&["Nominative", "Accusative"]),
            }],
        )
        .with_observation(vec![("case".to_string(), "Nominative".to_string())]);
        let open = MockAggregable::new(
            "Noun",
            vec![FieldDescriptor {
                name: "case".to_string(),
                kind: FieldKind::Open,
            }],
        )
        .with_observation(vec![("case".to_string(), "unexpected".to_string())]);

        let mut agg = BasicAggregator::new();
        agg.record(&closed);
        agg.record(&open);
        let result = agg.finish();

        if let Dimension::Inv(case) = &result.by_group["Noun"].dimensions["case"] {
            assert_eq!(case.counts["Nominative"], 1);
            assert_eq!(case.counts["unexpected"], 1);
        } else {
            panic!("Expected Inventory after open/closed conflict");
        }
    }

    #[test]
    fn observed_field_without_descriptor_is_open_inventory() {
        let item = MockAggregable::new("Noun", vec![])
            .with_observation(vec![("lemma".to_string(), "dom".to_string())]);

        let mut agg = BasicAggregator::new();
        agg.record(&item);
        let result = agg.finish();

        if let Dimension::Inv(lemma) = &result.by_group["Noun"].dimensions["lemma"] {
            assert_eq!(lemma.counts["dom"], 1);
        } else {
            panic!("Expected Inventory for undescribed observed field");
        }
    }

    #[test]
    fn merge_uses_same_open_closed_conflict_policy() {
        let closed = MockAggregable::new(
            "Noun",
            vec![FieldDescriptor {
                name: "case".to_string(),
                kind: FieldKind::Closed(&["Nominative"]),
            }],
        )
        .with_observation(vec![("case".to_string(), "Nominative".to_string())]);
        let open = MockAggregable::new(
            "Noun",
            vec![FieldDescriptor {
                name: "case".to_string(),
                kind: FieldKind::Open,
            }],
        )
        .with_observation(vec![("case".to_string(), "freeform".to_string())]);

        let mut merged: AggregationResult = std::iter::once(closed).collect();
        let incoming: AggregationResult = std::iter::once(open).collect();
        merged.merge(incoming);

        if let Dimension::Inv(case) = &merged.by_group["Noun"].dimensions["case"] {
            assert_eq!(case.counts["Nominative"], 1);
            assert_eq!(case.counts["freeform"], 1);
        } else {
            panic!("Expected Inventory after merge conflict");
        }
    }

    #[test]
    fn from_iterator_collect() {
        let descriptors = vec![FieldDescriptor {
            name: "case".to_string(),
            kind: FieldKind::Closed(&["Nominative", "Accusative"]),
        }];

        let items = vec![
            MockAggregable::new("Noun", descriptors.clone())
                .with_observation(vec![("case".to_string(), "Nominative".to_string())]),
            MockAggregable::new("Noun", descriptors)
                .with_observation(vec![("case".to_string(), "Accusative".to_string())]),
        ];

        let result: AggregationResult = items.into_iter().collect();

        assert_eq!(result.total_count(), 2);
        assert_eq!(result.group_count(), 1);
    }

    #[test]
    fn extend_chains() {
        let descriptors = vec![FieldDescriptor {
            name: "case".to_string(),
            kind: FieldKind::Closed(&["Nominative", "Accusative"]),
        }];

        let items1 = vec![
            MockAggregable::new("Noun", descriptors.clone())
                .with_observation(vec![("case".to_string(), "Nominative".to_string())]),
        ];
        let items2 = vec![
            MockAggregable::new("Noun", descriptors)
                .with_observation(vec![("case".to_string(), "Accusative".to_string())]),
        ];

        let mut result = AggregationResult::default();
        result.extend(items1);
        result.extend(items2);

        assert_eq!(result.total_count(), 2);
        assert_eq!(result.group_count(), 1);
    }

    #[test]
    fn record_contribution_direct() {
        let descriptors = vec![FieldDescriptor {
            name: "case".to_string(),
            kind: FieldKind::Closed(&["Nominative", "Accusative"]),
        }];

        let mut agg = BasicAggregator::new();
        agg.record_contribution(AggregationContribution {
            group: "Noun".to_string(),
            descriptors: descriptors.clone(),
            observations: vec![vec![("case".to_string(), "Nominative".to_string())]],
            total_increment: 1,
        });
        agg.record_contribution(AggregationContribution {
            group: "Noun".to_string(),
            descriptors,
            observations: vec![vec![("case".to_string(), "Accusative".to_string())]],
            total_increment: 1,
        });

        let result = agg.finish();
        assert_eq!(result.total_count(), 2);
        let noun = &result.by_group["Noun"];
        assert_eq!(noun.total, 2);
    }

    #[test]
    fn pivoting_sink_overrides_group() {
        let descriptors = vec![FieldDescriptor {
            name: "case".to_string(),
            kind: FieldKind::Closed(&["Nominative", "Accusative"]),
        }];

        let item = MockAggregable::new("Noun", descriptors)
            .with_observation(vec![("case".to_string(), "Nominative".to_string())]);

        let mut inner = BasicAggregator::new();
        {
            let pivot = |_: &AggregationContribution| "PivotedGroup".to_string();
            let mut sink = PivotingSink {
                inner: &mut inner,
                pivot: &pivot,
            };
            sink.record(&item);
        }

        let result = inner.finish();
        assert!(result.by_group.contains_key("PivotedGroup"));
        assert!(!result.by_group.contains_key("Noun"));
    }

    #[test]
    fn total_increment_respected() {
        let descriptors = vec![FieldDescriptor {
            name: "base_form".to_string(),
            kind: FieldKind::Open,
        }];

        let mut agg = BasicAggregator::new();
        // Emit 3 contributions each with total_increment = 1 (Option A: per morpheme)
        for base in &["DA", "(y)I", "lAr"] {
            agg.record_contribution(AggregationContribution {
                group: "morpheme".to_string(),
                descriptors: descriptors.clone(),
                observations: vec![vec![("base_form".to_string(), (*base).to_string())]],
                total_increment: 1,
            });
        }

        let result = agg.finish();
        let morpheme = &result.by_group["morpheme"];
        assert_eq!(morpheme.total, 3); // counts morphemes, not words
    }
}
