import os
from dotenv import load_dotenv
import panini
import helper

def run_arabic_aggregation():
    load_dotenv()
    api_key = os.getenv("GOOGLE_API_KEY")
    if not api_key:
        print("Please set GOOGLE_API_KEY in your .env file")
        return

    # 1. Read sample text and split into sentences
    current_dir = os.path.dirname(os.path.abspath(__file__))
    data_path = os.path.join(current_dir, "..", "data", "arabic_sample.txt")
    with open(data_path, "r", encoding="utf-8") as f:
        sentences = [line.strip() for line in f if line.strip()]

    print(f"--- Analyzing Arabic Corpus ({len(sentences)} sentences) ---")

    # 2. Aggregator initialization
    pos_agg = panini.BasicAggregator()
    root_agg = panini.BasicAggregator()
    all_raw_features = [] # To store data for Lexicon Explorer

    # 3. Batching logic (n=2)
    batch_size = 5
    for i in range(0, len(sentences), batch_size):
        batch = sentences[i:i + batch_size]
        batch_text = " ".join(batch)
        print(f"\nProcessing batch {i // batch_size + 1} ({len(batch)} sentences)...")

        # Run extraction without manual targets (extracts everything in context)
        result = panini.extract(
            provider="google",
            model="gemini-3.1-flash-lite-preview",
            api_key=api_key,
            language="ara",
            text=batch_text,
            targets=[],
            components=["morphology"],
            max_tokens=8192
        )

        # Record results into aggregators
        pos_agg.record("ara", result)
        root_agg.record_pivoted(
            "ara", 
            result, 
            lambda feat: next(iter(feat.values())).get("root", "no-root")
        )

        # Collect raw features for the Lexicon Explorer
        morph = result.get("morphology", {})
        for field in ["target_features", "context_features"]:
            for word_res in morph.get(field, []):
                if isinstance(word_res, dict) and "morphology" in word_res:
                    m = word_res["morphology"]
                    pos = next(iter(m.keys()))
                    feat = m[pos]
                    all_raw_features.append((pos, feat))

    # 4. Final results display
    print("\n" + "="*50)
    print("FINAL AGGREGATION BY PoS (All Batches)")
    print("="*50)
    pos_result = pos_agg.finish()
    pos_result.print()

    print("\n" + "="*50)
    print("FINAL AGGREGATION BY ROOT (All Batches)")
    print("="*50)
    root_result = root_agg.finish()
    root_result.print()

    # 5. Visual Results
    print("\nGenerating graphs...")
    helper.plot_aggregation(pos_result, mode="pos")
    helper.plot_aggregation(root_result, mode="root")

    # 6. Lexicon Explorer Data Export
    print("\nExporting data for Lexicon Explorer...")
    helper.export_lexicon_json(all_raw_features)
    explorer_path = os.path.abspath(os.path.join(current_dir, "lexicon_explorer.html"))
    print(f"\n>>> Lexicon Explorer ready! Open this file in your browser:")
    print(f"file://{explorer_path}")

if __name__ == "__main__":
    run_arabic_aggregation()
