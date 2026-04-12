import matplotlib
import matplotlib.pyplot as plt
import os

# Use Agg backend for headless environments/background runs
matplotlib.use('Agg')

def plot_pos_distribution(result, output_path="pos_distribution.png"):
    """
    Plots a bar chart of Part-of-Speech categories and their frequencies.
    
    :param result: PyAggregationResult from pos_agg.finish()
    :param output_path: Path to save the generated PNG
    """
    groups = result.by_group
    # Filter out empty groups and sort by total descending
    data = sorted(groups.items(), key=lambda x: x[1].total, reverse=True)
    
    labels = [k for k, v in data]
    counts = [v.total for k, v in data]

    plt.figure(figsize=(10, 6))
    plt.bar(labels, counts, color='skyblue', edgecolor='navy', alpha=0.8)
    
    plt.xlabel('Part of Speech (Category)', fontweight='bold')
    plt.ylabel('Total Count', fontweight='bold')
    plt.title('Morphological Distribution by Part-of-Speech', fontsize=14, pad=20)
    plt.xticks(rotation=45, ha='right')
    plt.grid(axis='y', linestyle='--', alpha=0.6)
    
    plt.tight_layout()
    plt.savefig(output_path)
    print(f"Plot saved: {os.path.abspath(output_path)}")
    plt.show()

def plot_root_distribution(result, top_n=12, output_path="root_distribution.png"):
    """
    Plots the top N roots by frequency.
    
    :param result: PyAggregationResult from root_agg.finish()
    :param top_n: Number of top roots to display
    :param output_path: Path to save the generated PNG
    """
    groups = result.by_group
    # Filter out 'no-root' or technical placeholders
    roots = {k: v.total for k, v in groups.items() if k not in ["no-root", "None", ""]}
    
    # Sort by frequency descending
    sorted_roots = sorted(roots.items(), key=lambda x: x[1], reverse=True)[:top_n]
    
    if not sorted_roots:
        print("No valid root data found to plot.")
        return

    labels, counts = zip(*sorted_roots)

    plt.figure(figsize=(12, 7))
    plt.bar(labels, counts, color='salmon', edgecolor='darkred', alpha=0.8)
    
    plt.xlabel('3-Consonantal Root', fontweight='bold')
    plt.ylabel('Frequency (Occurrences in Corpus)', fontweight='bold')
    plt.title(f'Top {top_n} Arabic Roots identified by Panini', fontsize=14, pad=20)
    plt.grid(axis='y', linestyle='--', alpha=0.6)
    
    # Add value labels on top of bars
    for i, v in enumerate(counts):
        plt.text(i, v + 0.1, str(v), ha='center', fontweight='bold')

    plt.tight_layout()
    plt.savefig(output_path)
    print(f"Plot saved: {os.path.abspath(output_path)}")
    plt.show()

def plot_aggregation(result, mode="pos", **kwargs):
    """Convenience wrapper for both modes."""
    if mode == "pos":
        plot_pos_distribution(result, **kwargs)
    elif mode == "root":
        plot_root_distribution(result, **kwargs)
    else:
        print(f"Unknown plot mode: {mode}")

import json

def export_lexicon_json(all_features, output_path="lexicon_data.json"):
    """
    Exports clean morphological data for the D3.js Lexicon Explorer.
    
    :param all_features: List of (pos, features_dict) tuples
    :param output_path: Path to save the JSON
    """
    data = []
    for pos, feat in all_features:
        # Extract fields safely
        entry = {
            "pos": pos.upper(),
            "lemma": feat.get("lemma", "Unknown"),
            "root": feat.get("root", "no-root"),
            "case": feat.get("case", "None"),
            "gender": feat.get("gender", "None"),
            "number": feat.get("number", "None"),
            "tense": feat.get("tense", "None"),
            "mood": feat.get("mood", "None"),
        }
        data.append(entry)
        
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)
    print(f"Lexicon data exported: {os.path.abspath(output_path)}")
