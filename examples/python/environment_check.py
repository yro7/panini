import os
from dotenv import load_dotenv
import panini

def check_env():
    load_dotenv()
    
    print(f"--- Panini Environment Check (v{panini.version()}) ---")
    
    # 1. Check API Keys
    providers = {
        "OPENAI_API_KEY": "OpenAI",
        "ANTHROPIC_API_KEY": "Anthropic",
        "GOOGLE_API_KEY": "Google (Gemini)"
    }
    
    found_any = False
    for env_var, name in providers.items():
        if os.getenv(env_var):
            print(f"✅ {name} API key found ({env_var})")
            found_any = True
        else:
            print(f"❌ {name} API key NOT found")
            
    if not found_any:
        print("\nWARNING: No API keys found. Extraction will fail unless you provide one explicitly.")

    # 2. Check Supported Languages
    langs = panini.supported_languages()
    print(f"\nSupported Languages: {', '.join(langs)}")

    # 3. Query Language Metadata (Arabic Example)
    if "ara" in langs:
        info = panini.get_language_info("ara")
        print(f"\n[ara] Name: {info.name}")
        print(f"[ara] Scripts: {', '.join(info.scripts)}")
        print(f"[ara] Features: {', '.join(info.typological_features)}")
        
        # 4. View Morphology Schema
        schema = panini.get_morphology_schema("ara")
        print(f"\n[ara] Morphology Schema Keys: {list(schema.keys())}")
        # Note: In a real app, you'd use this to validate your UI or generate forms

if __name__ == "__main__":
    check_env()
