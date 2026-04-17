#!/usr/bin/env python3
"""
Batch Processing Example: Calculate PP for multiple scores on the same map.

This example shows how to:
1. Load a beatmap once
2. Calculate PP for multiple different scores efficiently
3. Display results in a table format
"""

from cheat_ccv3_pp_py import Beatmap, Performance

MAP_PATH = "example_map.osu"

def main():
    try:
        # Load beatmap once
        beatmap = Beatmap(path=MAP_PATH)
        print(f"Beatmap loaded: {beatmap.mode}")
        print(f"AR: {beatmap.ar:.2f}, CS: {beatmap.cs:.2f}, OD: {beatmap.od:.2f}, HP: {beatmap.hp:.2f}\n")
        
        # Define test scores
        scores = [
            {"accuracy": 100.0, "misses": 0, "mods": 0, "label": "Perfect (No Mods)"},
            {"accuracy": 99.5, "misses": 0, "mods": 0, "label": "99.5% (No Mods)"},
            {"accuracy": 97.0, "misses": 2, "mods": 0, "label": "97% 2 miss (No Mods)"},
            {"accuracy": 99.0, "misses": 0, "mods": 8, "label": "99% (HD)"},
            {"accuracy": 98.5, "misses": 1, "mods": 24, "label": "98.5% 1 miss (HD+HR)"},
            {"accuracy": 95.0, "misses": 5, "mods": 0, "label": "95% 5 miss (No Mods)"},
            {"accuracy": 100.0, "misses": 0, "mods": 24, "label": "Perfect (HD+HR)"},
        ]
        
        # Calculate PP for all scores
        results = []
        for score in scores:
            perf = Performance(beatmap, **score)
            perf_attrs = perf.calculate()
            results.append({
                "label": score["label"],
                "pp": perf_attrs.pp,
                "stars": perf_attrs.stars,
            })
        
        # Display results in table format
        print("Score Results:")
        print(f"{'Score':<30} {'PP':<10} {'Stars':<10}")
        print("-" * 50)
        for i, result in enumerate(results, 1):
            print(f"{i}. {result['label']:<27} {result['pp']:>8.2f}pp {result['stars']:>8.2f}★")
        
        # Summary statistics
        pp_values = [r["pp"] for r in results]
        print(f"\nStatistics:")
        print(f"  Minimum PP: {min(pp_values):.2f}pp")
        print(f"  Maximum PP: {max(pp_values):.2f}pp")
        print(f"  Average PP: {sum(pp_values) / len(pp_values):.2f}pp")
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
