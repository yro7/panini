use anyhow::Result;
use dotenv::dotenv;
use panini_engine::prompts::{ExtractionRequest, ExtractorPrompts};
use panini_langs::registry;
use rig::providers::gemini;
use rig::client::CompletionClient;
use std::env;
use panini_core::morpheme::WordSegmentation;
use panini_langs::turkish::TurkishGrammaticalFunction;
use panini_core::component::ExtractionResult;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("GOOGLE_API_KEY")
        .expect("GOOGLE_API_KEY must be set in .env or environment");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let prompts_path = std::path::Path::new(manifest_dir)
        .join("../../panini-cli/prompts/default.yml");
    let prompts = ExtractorPrompts::load(prompts_path.to_str().unwrap())?;

    let client = gemini::Client::new(&api_key)?;
    let model = client.completion_model("gemini-3.1-flash-lite-preview");

    let data_path = std::path::Path::new(manifest_dir)
        .join("../data/turkish_sample.txt");
    let text = std::fs::read_to_string(data_path)?;
    let sentences: Vec<String> = text.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    println!("--- Analyzing Turkish Corpus ({} sentences) ---", sentences.len());

    let mut all_segments: Vec<WordSegmentation<TurkishGrammaticalFunction>> = Vec::new();

    for (i, batch) in sentences.chunks(3).enumerate() {
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

        let result: ExtractionResult = registry::extract_erased_with_components(
            "tur",
            &model,
            &request,
            Some(&["morpheme_segmentation"]),
            0.2,
            8192,
            &prompts,
        ).await?;

        println!("API Response:");
        println!("{result:#?}");

        let segments: Vec<WordSegmentation<TurkishGrammaticalFunction>> = 
            result.get("morpheme_segmentation")?;
        
        all_segments.extend(segments);
    }

    println!("\n==================================================");
    println!("DECOMPOSITION SUMMARY:");
    println!("==================================================");
    for word_seg in all_segments {
        println!("\nWord: [{}]", word_seg.word);
        for m in word_seg.morphemes {
            println!("  - {:<10} | {:<10} | {:?}", m.surface, m.base_form, m.function);
        }
    }

    Ok(())
}