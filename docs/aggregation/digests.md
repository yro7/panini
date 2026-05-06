# Usage & Pivots

Once your data is aggregated, you can manipulate it to extract specific information or change the analysis axis.

---

## 1. Creating a Digest: The BasicAggregator

The `BasicAggregator` is the default tool for ingesting `Aggregable` objects. It handles heterogeneous data (e.g., both `ExtractedFeature` and `ExtractedMorpheme`).

```rust
use panini_core::aggregable::digest::{AggregationSink, BasicAggregator};

let mut agg = BasicAggregator::new();

for feature in &features {
    agg.record(feature);
}

// Peek at the result without consuming the aggregator
let result = agg.result();

// Finish and retrieve the final result
let final_res = agg.finish();
```

---

## 2. Aggregating Morpheme Segmentations

For agglutinative languages, morpheme segmentations are stored as `WordSegmentation<F>`, which contains a list of `ExtractedMorpheme<F>`. Each **morpheme** is the unit of aggregation — iterate the morphemes, not the segmentation objects.

```rust
if let Some(segs) = &metadata.morpheme_segmentation {
    for seg in segs {
        for morpheme in &seg.morphemes {
            agg.record(morpheme); // one contribution per morpheme
        }
    }
}
```

!!! info "total semantics for the `\"morpheme\"` group"
    `GroupResult.total` counts **individual morphemes** (not segmented words). Distribution counts and `total` are on the same scale.

---

## 3. The Pivot Concept

"Pivoting" is one of the most powerful features of the aggregation system. It allows you to change the grouping key on the fly.

By default, objects are grouped by POS (e.g., "Noun", "Verb"). With a pivot, you can choose any other criterion.

### Typed Pivot (single item, field-checked)

```rust
// Group Arabic verbs by triliteral root instead of POS
for feature in &features {
    let root = feature.morphology.root().unwrap_or_else(|| feature.group_key());
    agg.record(&feature.pivoted(|_| root.clone()));
}
```

### Sink-Level Pivot (any contribution stream)

Use `PivotingSink` when you want to re-key an entire contribution stream without touching individual items — see [Custom Aggregators](custom.md) for details.

### Multi-Field Pivot

You can combine fields for finer analysis (e.g., Arabic verbs by Root AND Tense):

```rust
let mut agg = BasicAggregator::new();

for feature in &features {
    let key = format!("{}-{}",
        feature.morphology.root().unwrap_or_default(),
        feature.morphology.tense().unwrap_or_default()
    );
    agg.record(&feature.pivoted(|_| key.clone()));
}
```

**Sample Result (Pivot on Root + Tense):**
```text
[KTR-present] total: 3
  |- case [2/7]: dative(2), accusative(1)
[SL-past] total: 1
  |- gender [1/3]: masculine(1)
```

---

## 4. Automatic Discovery (Zero Configuration)

If you add a new field to your morphology (e.g., Aspect for a Verb), the digest system will automatically detect it without needing to modify the aggregator.

Thanks to the Pāṇini macros, a new line will automatically appear in your reports:
```text
  |- aspect [2/2]: perfective(12), imperfective(8)
```

---

## 5. Performance Considerations

!!! warning "Loop Optimization"
    Avoid calling `item.observations()` multiple times in a processing loop. The `record` method is optimized as the single entry point. If you need complex logic, encapsulate it in a `pivoted()` closure or in a custom `AggregationSink` implementation.
