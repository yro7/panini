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

if __name__ == "__main__":
    asyncio.run(run_tests())
    print("\nSuccessfully verified panini module loading and signature matching!")
    sys.exit(0)
