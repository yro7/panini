use crate::aggregable::digest::{AggregationResult, Dimension, GroupResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for digest output formatting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestOptions {
    /// Max values shown per dimension before truncation. Default: 12.
    pub max_values_per_dimension: usize,
    /// Dimension keys to exclude from the output (e.g., virtual "status").
    pub exclude_dimensions: Vec<String>,
}

impl Default for DigestOptions {
    fn default() -> Self {
        Self {
            max_values_per_dimension: 12,
            exclude_dimensions: Vec::new(),
        }
    }
}

/// Whether a dimension tracks a closed set (distribution) or open set (inventory).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestDimensionKind {
    Distribution,
    Inventory,
}

/// A single value with its frequency count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigestValue {
    pub value: String,
    pub count: usize,
}

/// A dimension in a digest group — one morphological axis (case, gender, lemma…).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigestDimension {
    pub key: String,
    pub kind: DigestDimensionKind,
    /// Number of distinct values actually observed.
    pub unique_count: usize,
    /// For closed sets: total number of possible values.
    /// `None` for open sets.
    pub total_possible: Option<usize>,
    /// Top values, sorted by count descending, then alphabetically.
    pub values: Vec<DigestValue>,
    /// Whether the values list was truncated.
    pub truncated: bool,
}

/// Aggregated statistics for one group (e.g., "Noun", "Verb", "morpheme").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DigestGroup {
    pub key: String,
    pub total: usize,
    pub dimensions: Vec<DigestDimension>,
}

impl AggregationResult {
    /// Convert the raw aggregation result into a sorted, truncated, serializable digest.
    pub fn to_digest(&self, options: &DigestOptions) -> Vec<DigestGroup> {
        let mut groups: Vec<DigestGroup> = self
            .by_group
            .iter()
            .map(|(key, group)| format_group(key, group, options))
            .collect();
        groups.sort_by(|a, b| b.total.cmp(&a.total).then_with(|| a.key.cmp(&b.key)));
        groups
    }

    /// Shorthand with default options.
    pub fn digest(&self) -> Vec<DigestGroup> {
        self.to_digest(&DigestOptions::default())
    }

    /// Extract dimension counts for a specific dimension key from a specific group.
    /// Useful for extracting injected virtual dimensions (like "status").
    pub fn dimension_counts(
        &self,
        group_key: &str,
        dim_key: &str,
    ) -> Option<&HashMap<String, usize>> {
        self.by_group
            .get(group_key)
            .and_then(|g| g.dimensions.get(dim_key))
            .map(|dim| match dim {
                Dimension::Dist(d) => &d.counts,
                Dimension::Inv(i) => &i.counts,
            })
    }
}

fn format_group(key: &str, group: &GroupResult, options: &DigestOptions) -> DigestGroup {
    let mut dimensions: Vec<DigestDimension> = group
        .dimensions
        .iter()
        .filter(|(dim_key, _)| !options.exclude_dimensions.contains(dim_key))
        .map(|(dim_key, dim)| format_dimension(dim_key, dim, options))
        .collect();
    dimensions.sort_by(|a, b| a.key.cmp(&b.key));

    DigestGroup {
        key: key.to_string(),
        total: group.total,
        dimensions,
    }
}

fn format_dimension(key: &str, dim: &Dimension, options: &DigestOptions) -> DigestDimension {
    match dim {
        Dimension::Dist(d) => {
            let mut values: Vec<DigestValue> = d
                .counts
                .iter()
                .map(|(k, v)| DigestValue {
                    value: k.clone(),
                    count: *v,
                })
                .collect();
            values.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));
            let truncated = values.len() > options.max_values_per_dimension;
            values.truncate(options.max_values_per_dimension);

            DigestDimension {
                key: key.to_string(),
                kind: DigestDimensionKind::Distribution,
                unique_count: d.seen_count(),
                total_possible: Some(d.total_count()),
                values,
                truncated,
            }
        }
        Dimension::Inv(i) => {
            let mut values: Vec<DigestValue> = i
                .counts
                .iter()
                .map(|(k, v)| DigestValue {
                    value: k.clone(),
                    count: *v,
                })
                .collect();
            values.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.value.cmp(&b.value)));
            let truncated = values.len() > options.max_values_per_dimension;
            values.truncate(options.max_values_per_dimension);

            DigestDimension {
                key: key.to_string(),
                kind: DigestDimensionKind::Inventory,
                unique_count: i.counts.len(),
                total_possible: None,
                values,
                truncated,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aggregable::{Aggregable, FieldDescriptor, FieldKind};
    use crate::{AggregationSink, Aggregator, BasicAggregator};

    struct TestAggregable {
        group: String,
        case: String,
        lemma: String,
    }

    impl Aggregable for TestAggregable {
        fn group_key(&self) -> String {
            self.group.clone()
        }

        fn instance_descriptors(&self) -> Vec<FieldDescriptor> {
            vec![
                FieldDescriptor {
                    name: "case".to_string(),
                    kind: FieldKind::Closed(&["Nom", "Acc", "Gen"]),
                },
                FieldDescriptor {
                    name: "lemma".to_string(),
                    kind: FieldKind::Open,
                },
            ]
        }

        fn observations(&self) -> Vec<Vec<(String, String)>> {
            vec![vec![
                ("case".to_string(), self.case.clone()),
                ("lemma".to_string(), self.lemma.clone()),
            ]]
        }
    }

    #[test]
    fn test_to_digest_formatting() {
        let mut agg = BasicAggregator::new();
        agg.record(&TestAggregable {
            group: "Noun".to_string(),
            case: "Nom".to_string(),
            lemma: "dom".to_string(),
        });
        agg.record(&TestAggregable {
            group: "Noun".to_string(),
            case: "Nom".to_string(),
            lemma: "pies".to_string(),
        });
        agg.record(&TestAggregable {
            group: "Noun".to_string(),
            case: "Acc".to_string(),
            lemma: "pies".to_string(),
        });
        agg.record(&TestAggregable {
            group: "Verb".to_string(),
            case: "Nom".to_string(),
            lemma: "byc".to_string(),
        });

        let result = agg.finish();
        let opts = DigestOptions {
            max_values_per_dimension: 1,
            exclude_dimensions: vec![],
        };
        let groups = result.to_digest(&opts);

        // Group sorting: Noun (3) then Verb (1)
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].key, "Noun");
        assert_eq!(groups[0].total, 3);
        assert_eq!(groups[1].key, "Verb");
        assert_eq!(groups[1].total, 1);

        // Noun dimensions
        let noun_group = &groups[0];
        assert_eq!(noun_group.dimensions.len(), 2);

        // Dimension 0: "case" (Closed set -> Distribution)
        let case_dim = &noun_group.dimensions[0];
        assert_eq!(case_dim.key, "case");
        assert_eq!(case_dim.kind, DigestDimensionKind::Distribution);
        assert_eq!(case_dim.unique_count, 2);
        assert_eq!(case_dim.total_possible, Some(3)); // "Nom", "Acc", "Gen"
        // Truncated to 1 value (max_values_per_dimension = 1)
        assert_eq!(case_dim.values.len(), 1);
        assert_eq!(case_dim.values[0].value, "Nom");
        assert_eq!(case_dim.values[0].count, 2);
        assert!(case_dim.truncated);

        // Dimension 1: "lemma" (Open set -> Inventory)
        let lemma_dim = &noun_group.dimensions[1];
        assert_eq!(lemma_dim.key, "lemma");
        assert_eq!(lemma_dim.kind, DigestDimensionKind::Inventory);
        assert_eq!(lemma_dim.unique_count, 2); // "dom", "pies"
        assert_eq!(lemma_dim.total_possible, None);
        assert_eq!(lemma_dim.values.len(), 1);
        assert_eq!(lemma_dim.values[0].value, "pies"); // "pies" count is 2, "dom" is 1
        assert_eq!(lemma_dim.values[0].count, 2);
        assert!(lemma_dim.truncated);
    }
}
