use anyhow::Result;
use dotenv::dotenv;
use std::env;
use panini_engine::prompts::{ExtractionRequest, ExtractorPrompts};
use panini_langs::registry;
use rig::providers::gemini;
use rig::client::CompletionClient;
use panini_core::aggregable::digest::{BasicAggregator, Aggregator};
use panini_core::aggregable::Aggregable;
use panini_langs::arabic::ArabicMorphology;
use panini_core::domain::ExtractedFeature;
use panini_core::component::ExtractionResult;
use serde::Deserialize;

#[derive(Deserialize)]
struct MorphologySection<M> {
    target_features: Vec<ExtractedFeature<M>>,
    context_features: Vec<ExtractedFeature<M>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY must be set");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let prompts_path = std::path::Path::new(manifest_dir)
        .join("../../panini-cli/prompts/default.yml");
    let prompts = ExtractorPrompts::load(prompts_path.to_str().unwrap())?;

    let client = gemini::Client::new(&api_key)?;
    let model = client.completion_model("gemini-3.1-flash-lite-preview");

    // 1. Load and prepare corpus
    let data_path = std::path::Path::new(manifest_dir)
        .join("../data/arabic_sample.txt");
    let text = std::fs::read_to_string(data_path)?;
    let sentences: Vec<String> = text.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    println!("--- Analyzing Arabic Corpus ({} sentences) ---", sentences.len());

    let mut pos_agg = BasicAggregator::new();
    let mut root_agg = BasicAggregator::new();

    // 2. Batch processing logic (n=5)
    for (i, batch) in sentences.chunks(5).enumerate() {
        let batch_text = batch.join(" ");
        println!("\nProcessing batch {} ({} sentences)...", i + 1, batch.len());

        let request = ExtractionRequest {
            content: batch_text,
            targets: vec![], // Automated: extracts everything in context
            pedagogical_context: None,
            skill_path: None,
            learner_ui_language: "English".to_string(),
            linguistic_background: vec![],
            user_prompt: None,
        };

        // Extract features
        let result: ExtractionResult = registry::extract_erased_with_components(
            "ara",
            &model,
            &request,
            Some(&["morphology"]),
            0.2,
            20000,
            None,
            &prompts,
        ).await?;

        // Get the morphology features safely
        let morph: MorphologySection<ArabicMorphology> = result.get("morphology")?;
        
        // Record all features into aggregators
        for feat in morph.target_features.iter().chain(morph.context_features.iter()) {
            // Aggregate by PoS
            pos_agg.record(&feat.morphology);

            // Aggregate by Root
            let root = match &feat.morphology {
                ArabicMorphology::Noun { root, .. } => root.clone(),
                ArabicMorphology::Verb { root, .. } => root.clone(),
                ArabicMorphology::Adjective { root, .. } => root.clone(),
                _ => "no-root".to_string(),
            };
            root_agg.record(&feat.morphology.pivoted(|_| root.clone()));
        }
    }

    // 3. Final results display
    println!("\n==================================================");
    println!("FINAL AGGREGATION BY PoS (All Batches)");
    println!("==================================================");
    pos_agg.finish().print();

    println!("\n==================================================");
    println!("FINAL AGGREGATION BY ROOT (All Batches)");
    println!("==================================================");
    root_agg.finish().print();

    Ok(())
}
