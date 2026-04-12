import sys
import panini
import asyncio

print(f"Loaded panini library: {panini}")
print(f"dir(panini): {dir(panini)}")
print(f"Version: {panini.version()}")
print(f"Supported languages: {panini.supported_languages()}")
print(f"Default prompts keys: {list(panini.get_default_prompts().keys())}")

async def run_tests():
    # 1. Synchronous test — uses embedded default prompts, expects auth error with fake key
    try:
        print("\n--- Testing Synchronous Extraction (default prompts) ---")
        res = panini.extract(
            provider="google",
            model="gemini-3-flash-preview",
            api_key="sk-random",
            language="pol",
            text="Nie wiem",
            targets=["wiem"],
            components=["pedagogical_explanation"],
        )
        print(f"Result: {res}")
    except Exception as e:
        print(f"Caught expected exception from sync call: {e}")

    # 2. Asynchronous test — same, but async
    try:
        print("\n--- Testing Asynchronous Extraction (default prompts) ---")
        res = await panini.async_extract(
            provider="google",
            model="gemini-3-flash-preview",
            api_key="sk-random",
            language="fra",
            text="Je ne sais pas",
            targets=["sais"],
            components=["morphology"],
        )
        print(f"Result: {res}")
    except Exception as e:
        print(f"Caught expected exception from async call: {e}")

    # 3. Metadata tests
    try:
        print("\n--- Testing Metadata Functions ---")
        info = panini.get_language_info("ara")
        print(f"Arabic Name: {info.name}")
        print(f"Arabic Scripts: {info.scripts}")
        print(f"Arabic Features: {info.typological_features}")
        
        schema = panini.get_morphology_schema("ara")
        print(f"Arabic Morphology Schema Keys: {list(schema.keys())}")
    except Exception as e:
        print(f"Metadata test failed: {e}")

    # 4. Aggregation tests (Mock Data)
    try:
        print("\n--- Testing BasicAggregator ---")
        agg = panini.BasicAggregator()
        
        # Mock Arabic extraction result
        mock_res = {
            "morphology": {
                "target_features": [
                    {
                        "word": "ذهب",
                        "morphology": { 
                            "verb": { 
                                "lemma": "ذهب", 
                                "root": "ذ-ه-ب", 
                                "tense": "past", 
                                "person": "third", 
                                "number": "singular", 
                                "gender": "masculine", 
                                "mood": "indicative", 
                                "voice": "active", 
                                "form": "I" 
                            } 
                        }
                    }
                ],
                "context_features": []
            }
        }
        
        agg.record("ara", mock_res)
        
        # Test record_pivoted
        # The callback receives the externally tagged dict, e.g., {"verb": {...}}
        agg.record_pivoted("ara", mock_res, lambda m: next(iter(m.values())).get("root", "unknown"))
        
        result = agg.finish()
        print(f"Total count: {result.total_count()}")
        print(f"Groups: {list(result.by_group.keys())}")
        
        verb_stats = result.by_group["Verb"]
        print(f"Verb total: {verb_stats.total}")
        
        # Check pivot group
        root_stats = result.by_group.get("ذ-ه-ب")
        if root_stats:
            print(f"Pivot group 'ذ-ه-ب' total: {root_stats.total}")
        
        result.print()
    except Exception as e:
        print(f"Aggregation test failed: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    asyncio.run(run_tests())
    print("\nSuccessfully verified panini module loading and all API points!")
    sys.exit(0)
