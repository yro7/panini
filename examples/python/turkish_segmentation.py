import os
from dotenv import load_dotenv
import panini

def run_turkish_segmentation():
    load_dotenv()
    api_key = os.getenv("GOOGLE_API_KEY")
    if not api_key:
        print("Please set GOOGLE_API_KEY in your .env file")
        return

    # 1. Read sample text and split into sentences
    current_dir = os.path.dirname(os.path.abspath(__file__))
    data_path = os.path.join(current_dir, "..", "data", "turkish_sample.txt")
    with open(data_path, "r", encoding="utf-8") as f:
        sentences = [line.strip() for line in f if line.strip()]

    print(f"--- Analyzing Turkish Corpus ({len(sentences)} sentences) ---")
    
    # 2. Batching logic (n=3)
    batch_size = 3
    all_segments = []
    
    for i in range(0, len(sentences), batch_size):
        batch = sentences[i:i + batch_size]
        batch_text = " ".join(batch)
        print(f"\nProcessing batch {i // batch_size + 1} ({len(batch)} sentences)...")
        
        # Run extraction without manual targets
        result = panini.extract(
            provider="google",
            model="gemini-3.1-flash-lite-preview",
            api_key=api_key,
            language="tur",
            text=batch_text,
            targets=[],
            components=["morpheme_segmentation"]
        )
        
        batch_segments = result.get("morpheme_segmentation", [])
        all_segments.extend(batch_segments)

    # 3. Process and display decomposition
    print("\n" + "="*50)
    print("DECOMPOSITION SUMMARY:")
    print("="*50)
    
    for word_seg in all_segments:
        print(f"\nWord: [{word_seg['word']}]")
        for m in word_seg['morphemes']:
            func = m['function']
            category = func.get('category', 'unknown')
            val = func.get('value', '') or func.get('person', '')
            print(f"  - {m['surface']:<15} | {m['base_form']:<15} | {category}:{val}")

if __name__ == "__main__":
    run_turkish_segmentation()
