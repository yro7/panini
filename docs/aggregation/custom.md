# Custom Aggregators

If you need a different output format (e.g., JSON export for a frontend or database insertion), you can implement the `Aggregator` trait yourself.

---

## 🔧 Implementing the Aggregator Trait

The `Aggregator` trait defines how `Aggregable` objects are consumed and what type of output they produce.

```rust
pub struct JsonAggregator {
    data: Vec<serde_json::Value>,
}

impl Aggregator for JsonAggregator {
    type Output = String;

    fn record<A: Aggregable>(&mut self, item: &A) {
        // Collect all observations into a simple JSON map
        let mut map = serde_json::Map::new();
        map.insert("group".into(), item.group_key().into());
        
        for obs in item.observations() {
            for (k, v) in obs {
                map.insert(k, v.into());
            }
        }
        self.data.push(serde_json::Value::Object(map));
    }

    fn finish(self) -> String {
        serde_json::to_string_pretty(&self.data).unwrap()
    }
}
```

---

## 🧬 Ingesting Heterogeneous Data

The `Aggregator` trait is unique because its type parameter `<A>` is at the `record` METHOD level, not at the trait level.

This design is **crucial**: it allows a single aggregator to ingest heterogeneous `Aggregable` types in the same instance (e.g., both analysis results and word segmentations).

```rust
let mut agg = MyAggregator::new();

agg.record(&feature);    // One type
agg.record(&morpheme);   // Another type

let res = agg.finish();
```

---

## 📊 Use Case: Learner Profiling

An advanced use case is to create a stateful aggregator that evolves over time. Instead of just counting, it could:

- **Detect gaps**: (e.g., "The user has never seen the Genitive case").
- **Track progress**: (e.g., "Number of new lemmas seen this week").
- **Generate recommendations**: (e.g., "Suggest more exercises on the Aorist tense").

!!! tip "Extensibility"
    By implementing `Aggregator`, you can plug Panini into any existing reporting or user tracking system.
