# Usage & Pivots

Once your data is aggregated, you can manipulate it to extract specific information or change the analysis axis.

---

## 1. Creating a Digest: The BasicAggregator

The `BasicAggregator` is the default tool for ingesting `Aggregable` objects. It's designed to handle heterogeneous data (e.g., both `ExtractedFeature` and `WordSegmentation`).

```rust
let mut agg = BasicAggregator::new();

for feature in features {
    agg.record(&feature);
}

// Analyze the result without terminating the aggregator
let result = agg.result(); 

// Finish and retrieve the final result
let final_res = agg.finish();
```

---

## 2. The Pivot Concept

"Pivoting" is one of the most powerful features of the aggregation system. It allows you to change the grouping key on the fly.

By default, objects are grouped by POS (e.g., "Noun", "Verb"). With a pivot, you can choose any other criteria.

### Example: Grouping by Case rather than by POS
```rust
let result: AggregationResult = features
    .into_iter()
    .map(|f| {
        // Redefine the group key dynamically
        f.pivoted(|inner| inner.morphology.case().unwrap_or("NoCase").to_string())
    })
    .collect();
```

### Multi-Field Pivot
You can combine fields for finer analysis (e.g., Verbs by Root AND by Tense).

```rust
let mut agg = BasicAggregator::new();

for feature in features {
    let key = format!("{}-{}", 
        feature.morphology.root().unwrap_or_default(),
        feature.morphology.tense().unwrap_or_default()
    );
    
    // Enroll the object with its new pivoted key
    agg.record(&feature.pivoted(|_| key.clone()));
}

**Sample Result (Pivot on Root + Tense):**
```text
[KTR-present] total: 3
  |- case [2/7]: dative(2), accusative(1)
[SL-past] total: 1
  |- gender [1/3]: masculine(1)
```

---

## 3. Automatic Discovery (Zero Configuration)

If you add a new field to your morphology (e.g., Aspect for a Verb), the digest system will automatically detect it without needing to modify the aggregator.

Thanks to the Pāṇini macros, a new line will automatically appear in your reports:
```text
  |- aspect [2/2]: perfective(12), imperfective(8)
```

---

## 4. Performance Considerations

!!! warning "Loop Optimization"
    Avoid calling `item.observations()` multiple times in a processing loop. The `record` method is optimized as the single entry point. If you need complex logic, encapsulate it in a `pivoted()` closure or in a custom `Aggregator` implementation.
