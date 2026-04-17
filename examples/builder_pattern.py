#!/usr/bin/env python3
"""
Builder Pattern Example: Using chainable methods for flexible configuration.

This example shows how to:
1. Use the builder pattern for configuring calculations
2. Chain methods for fluent API
3. Configure various parameters
"""

from cheat_ccv3_pp_py import Beatmap, Difficulty, Performance

MAP_PATH = "example_map.osu"

def main():
    try:
        beatmap = Beatmap(path=MAP_PATH)
        
        print("=== Difficulty Builder Pattern ===\n")
        
        # Method 1: Constructor with kwargs
        diff1 = Difficulty(beatmap, mods=24, passed_objects=100)
        result1 = diff1.calculate()
        print(f"Constructor style (mods=24, passed_objects=100):")
        print(f"  Stars: {result1.stars:.2f}")
        
        # Method 2: Builder pattern with chaining
        diff2 = Difficulty(beatmap)
        result2 = diff2.mods(24).passed_objects(100).clock_rate(1.5).calculate()
        print(f"\nBuilder style (mods=24, passed_objects=100, clock_rate=1.5):")
        print(f"  Stars: {result2.stars:.2f}")
        
        print(f"\n=== Performance Builder Pattern ===\n")
        
        # Method 1: Constructor with kwargs
        perf1 = Performance(beatmap, mods=8, accuracy=98.5, misses=1)
        result3 = perf1.calculate()
        print(f"Constructor style (mods=8, accuracy=98.5%, misses=1):")
        print(f"  PP: {result3.pp:.2f}")
        
        # Method 2: Builder pattern with chaining
        perf2 = Performance(beatmap)
        result4 = perf2.mods(8).accuracy(98.5).misses(1).calculate()
        print(f"\nBuilder style (mods=8, accuracy=98.5%, misses=1):")
        print(f"  PP: {result4.pp:.2f}")
        
        # Method 3: Using specific hit counts instead of accuracy
        perf3 = Performance(beatmap)
        total_objects = beatmap.n_circles + beatmap.n_sliders
        result5 = (perf3.mods(8)
                   .n300(int(total_objects * 0.99))
                   .n100(int(total_objects * 0.01))
                   .misses(0)
                   .calculate())
        print(f"\nBuilder style with hit counts (99% 300s):")
        print(f"  PP: {result5.pp:.2f}")
        
        print(f"\n=== Complex Configuration ===\n")
        
        # A complex calculation with many parameters
        complex_perf = Performance(beatmap)
        result6 = (complex_perf
                   .mods(24)  # HD+HR
                   .combo(500)
                   .accuracy(97.5)
                   .misses(3)
                   .clock_rate(1.06)  # DT equivalent
                   .calculate())
        print(f"Complex config (mods=24, combo=500, 97.5%, 3 misses, 1.06x rate):")
        print(f"  PP: {result6.pp:.2f}")
        print(f"  Stars: {result6.stars:.2f}")
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
