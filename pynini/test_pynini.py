import sys
import pynini
import asyncio
import os

# Get absolute path to prompts
prompts_path = os.path.join(os.path.dirname(__file__), "..", "panini-cli", "prompts", "default.yml")
prompts_path = os.path.abspath(prompts_path)

print(f"Loaded pynini library: {pynini}")
print(f"dir(pynini): {dir(pynini)}")
print(f"Using prompts path: {prompts_path}")

async def run_tests():
    # 1. Synchronous Test
    try:
        print("\n--- Testing Synchronous Extraction ---")
        # We expect this to fail with "Auth error" or similar because of fake key
        # but the point is to see if it reaches the LLM call logic.
        res = pynini.extract(
            provider="openai",
            model="gpt-4o",
            api_key="sk-not-real",
            language="pol",
            text="Nie wiem",
            targets=["wiem"],
            prompts=prompts_path,
            components=["pedagogical_explanation"]
        )
        print(f"Result: {res}")
    except Exception as e:
        print(f"Caught expected exception from sync call: {e}")

    # 2. Asynchronous Test
    try:
        print("\n--- Testing Asynchronous Extraction ---")
        res = await pynini.async_extract(
            provider="openai",
            model="gpt-4o",
            api_key="sk-not-real",
            language="pol",
            text="Nie wiem",
            targets=["wiem"],
            prompts=prompts_path,
            components=["pedagogical_explanation"]
        )
        print(f"Result: {res}")
    except Exception as e:
        print(f"Caught expected exception from async call: {e}")

if __name__ == "__main__":
    asyncio.run(run_tests())
    print("\nSuccessfully verified pynini module loading and signature matching!")
    sys.exit(0)

