/// Whether a pivot value comes from an open inventory or a statically known set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PivotValueKind {
    Open,
    Closed,
}

/// Typed handle for one pivotable field on an analysis item.
///
/// The `key` is the stable wire identifier exposed to clients. It is generated
/// by Panini macros from real Rust fields/variants; callers should curate by
/// referencing these handles instead of hand-authoring string keys.
///
/// Morphology derives emit handles such as `PolishMorphology::PIVOT_CASE`.
/// Grammatical-function derives emit handles such as
/// `TurkishGrammaticalFunction::PIVOT_POLARITY`.
///
/// A closed pivot has a static value inventory, while an open pivot accepts any
/// extracted string value (for example Arabic roots).
#[derive(Clone, Copy)]
pub struct PivotField<T: 'static> {
    /// Stable wire key for API/frontend state, generated from the Rust field or
    /// grammatical-function variant name.
    pub key: &'static str,
    /// Human-readable label generated from the field or variant name.
    pub label: &'static str,
    /// Whether values are open strings or a closed enum/bool inventory.
    pub value_kind: PivotValueKind,
    /// Value inventory for closed pivots. Open pivots return an empty slice.
    pub values: fn() -> &'static [&'static str],
    /// Typed extractor. Returns `None` when this pivot does not apply to the item.
    pub extract: fn(&T) -> Option<String>,
}

#[must_use]
pub const fn empty_values() -> &'static [&'static str] {
    &[]
}

#[must_use]
pub const fn bool_values() -> &'static [&'static str] {
    &["true", "false"]
}

impl<T: 'static> PivotField<T> {
    #[must_use]
    pub const fn open(
        key: &'static str,
        label: &'static str,
        extract: fn(&T) -> Option<String>,
    ) -> Self {
        Self {
            key,
            label,
            value_kind: PivotValueKind::Open,
            values: empty_values,
            extract,
        }
    }

    #[must_use]
    pub const fn closed(
        key: &'static str,
        label: &'static str,
        values: fn() -> &'static [&'static str],
        extract: fn(&T) -> Option<String>,
    ) -> Self {
        Self {
            key,
            label,
            value_kind: PivotValueKind::Closed,
            values,
            extract,
        }
    }

    #[must_use]
    pub fn value(&self, item: &T) -> Option<String> {
        (self.extract)(item)
    }

    #[must_use]
    pub fn values(&self) -> &'static [&'static str] {
        (self.values)()
    }
}

impl<T: 'static> std::fmt::Debug for PivotField<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PivotField")
            .field("key", &self.key)
            .field("label", &self.label)
            .field("value_kind", &self.value_kind)
            .field("values", &self.values())
            .finish_non_exhaustive()
    }
}

/// Serializable metadata view of a typed pivot.
#[derive(Debug, Clone, Copy)]
pub struct PivotMeta {
    pub key: &'static str,
    pub label: &'static str,
    pub value_kind: PivotValueKind,
    pub values: fn() -> &'static [&'static str],
}

impl<T: 'static> From<&PivotField<T>> for PivotMeta {
    fn from(value: &PivotField<T>) -> Self {
        Self {
            key: value.key,
            label: value.label,
            value_kind: value.value_kind,
            values: value.values,
        }
    }
}
