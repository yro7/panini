# Custom Aggregators

If you need a different output format (e.g., JSON export for a frontend or database insertion), you can implement the `AggregationSink` and `Aggregator` traits yourself.

---

## Architecture: Two-Layer Design

The aggregation system separates two concerns:

- **`AggregationSink`** — consumes contributions one at a time (stateful, incremental). Object-safe: usable via `dyn AggregationSink`.
- **`Aggregator: AggregationSink`** — extends `AggregationSink` with a `finish()` method to retrieve the final output.

```
AggregationContribution  ──►  AggregationSink::record_contribution()
                                          │
                               Aggregator::finish()
                                          │
                                    Output type
```

---

## Implementing a Custom Aggregator

```rust
use panini_core::aggregable::digest::{
    AggregationContribution, AggregationSink, Aggregator,
};

pub struct JsonAggregator {
    data: Vec<serde_json::Value>,
}

impl AggregationSink for JsonAggregator {
    fn record_contribution(&mut self, c: AggregationContribution) {
        let mut map = serde_json::Map::new();
        map.insert("group".into(), c.group.into());

        for obs_row in &c.observations {
            for (k, v) in obs_row {
                map.insert(k.clone(), v.clone().into());
            }
        }
        self.data.push(serde_json::Value::Object(map));
    }
}

impl Aggregator for JsonAggregator {
    type Output = String;

    fn finish(self) -> String {
        serde_json::to_string_pretty(&self.data).unwrap()
    }
}
```

### Using the typed `record<A>` shim

For convenience, `AggregationSink` provides a default `record<A: Aggregable>` method on all **concrete** (sized) sinks. It converts any `Aggregable` item to an `AggregationContribution` with `total_increment = 1` and calls `record_contribution` internally.

```rust
let mut agg = JsonAggregator { data: vec![] };

agg.record(&feature);    // ExtractedFeature<M>
agg.record(&morpheme);   // ExtractedMorpheme<F>

let json = agg.finish();
```

!!! note "On trait objects"
    `record<A>` is **not** available on `dyn AggregationSink` (it is `where Self: Sized`). When you hold a `&mut dyn AggregationSink`, use the free function `record_aggregable(sink, item)` instead.

---

## The `AggregationContribution` Struct

Every aggregation unit passes through the system as an `AggregationContribution`:

```rust
pub struct AggregationContribution {
    /// Group this contribution belongs to (e.g. "Noun", "morpheme").
    pub group: String,
    /// Field descriptors — define whether each field is Open (inventory) or Closed (distribution).
    pub descriptors: Vec<FieldDescriptor>,
    /// One or more observation rows: each row is a list of (field_name, value) pairs.
    pub observations: Vec<Vec<(String, String)>>,
    /// How much to increment the group's `total` counter (almost always 1).
    pub total_increment: usize,
}
```

This struct is the canonical unit crossing the `dyn` boundary. All other entry points (`record<A>`, `record_aggregable`, component `aggregate_section`) ultimately produce `AggregationContribution`s.

---

## Sink-Level Pivoting with `PivotingSink`

`PivotingSink` wraps any `AggregationSink` and overrides the `group` field of every contribution before forwarding it. This allows sink-level re-keying independent of the item type.

```rust
use panini_core::aggregable::digest::PivotingSink;

let mut inner = BasicAggregator::new();

{
    let mut pivoted = PivotingSink {
        inner: &mut inner,
        pivot: &|c| {
            c.observations
                .first()
                .and_then(|row| row.iter().find(|(k, _)| k == "root"))
                .map(|(_, v)| format!("root:{v}"))
                .unwrap_or_else(|| c.group.clone())
        },
    };

    for feature in &features {
        pivoted.record(feature);
    }
} // pivoted dropped; inner accessible again

let result = inner.finish();
```

For the common case of pivoting a single `Aggregable` item by a typed field, prefer the typed `.pivoted()` method — see [Usage & Pivots](digests.md).

---

## Use Case: Learner Profiling

An advanced use case is a stateful aggregator that evolves over time. Instead of just counting, it could:

- **Detect gaps** — e.g., "The user has never seen the Genitive case".
- **Track progress** — e.g., "Number of new lemmas seen this week".
- **Generate recommendations** — e.g., "Suggest more exercises on the Aorist tense".

!!! tip "Extensibility"
    By implementing `AggregationSink`, you can plug Pāṇini into any existing reporting or user tracking system. The component-driven dispatch handles all section routing; your sink only needs to implement `record_contribution`.
