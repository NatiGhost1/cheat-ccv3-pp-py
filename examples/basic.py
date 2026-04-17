#!/usr/bin/env python3
"""
Basic Example: Calculate star rating and PP for a single beatmap.

This example shows how to:
1. Parse a .osu file
2. Calculate difficulty/star rating
3. Calculate performance points
"""

from cheat_ccv3_pp_py import Beatmap, Difficulty, Performance

# Replace with actual beatmap path
MAP_PATH = "example_map.osu"

def main():
    try:
        # Load beatmap
        print(f"Loading beatmap from {MAP_PATH}...")
        beatmap = Beatmap(path=MAP_PATH)
        
        # Display beatmap info
        print(f"\nBeatmap Information:")
        print(f"  Mode: {beatmap.mode}")
        print(f"  AR: {beatmap.ar:.2f}")
        print(f"  CS: {beatmap.cs:.2f}")
        print(f"  OD: {beatmap.od:.2f}")
        print(f"  HP: {beatmap.hp:.2f}")
        print(f"  BPM: {beatmap.bpm:.2f}")
        print(f"  Objects: {beatmap.n_circles + beatmap.n_sliders + beatmap.n_spinners}")
        
        # Calculate star rating with no mods
        print(f"\nCalculating star rating...")
        diff = Difficulty(beatmap, mods=0)
        diff_result = diff.calculate()
        print(f"  No Mods: {diff_result.stars:.2f}★")
        
        # Calculate star rating with HD+HR (mods = 8 + 16 = 24)
        diff_hd_hr = Difficulty(beatmap, mods=24)
        diff_result_hd_hr = diff_hd_hr.calculate()
        print(f"  HD+HR: {diff_result_hd_hr.stars:.2f}★")
        
        # Calculate PP with perfect score
        print(f"\nCalculating performance points...")
        perf_perfect = Performance(beatmap, accuracy=100.0, combo=beatmap.n_circles + beatmap.n_sliders * 2)
        perf_result = perf_perfect.calculate()
        print(f"  Perfect (100%): {perf_result.pp:.2f}pp")
        
        # Calculate PP with 99% accuracy and some misses
        perf_realistic = Performance(beatmap, accuracy=99.0, misses=2)
        perf_result_realistic = perf_realistic.calculate()
        print(f"  99% with 2 misses: {perf_result_realistic.pp:.2f}pp")
        
        # Calculate max values
        print(f"\nMax values:")
        max_pp_result = beatmap.max_pp(0)
        max_stars_result = beatmap.max_stars(0)
        print(f"  Max PP: {max_pp_result.pp:.2f}pp")
        print(f"  Max Stars: {max_stars_result.stars:.2f}★")
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
