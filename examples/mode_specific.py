#!/usr/bin/env python3
"""
Mode-Specific Attributes Example: Access properties specific to each game mode.

This example shows how to:
1. Handle mode detection
2. Access mode-specific difficulty attributes
3. Access mode-specific performance attributes
4. Check if attributes are available
"""

from cheat_ccv3_pp_py import Beatmap, Difficulty, Performance

MAP_PATH = "example_map.osu"

def display_difficulty_attributes(diff_attrs, mode):
    """Display difficulty attributes based on game mode."""
    print(f"  General:")
    print(f"    Stars: {diff_attrs.stars:.2f}")
    print(f"    Max Combo: {diff_attrs.max_combo}")
    
    if mode == "Osu":
        print(f"  osu!standard specific:")
        print(f"    Aim: {diff_attrs.aim}")
        print(f"    Speed: {diff_attrs.speed}")
        print(f"    Flashlight: {diff_attrs.flashlight}")
        print(f"    Slider Factor: {diff_attrs.slider_factor}")
        print(f"    Speed Note Count: {diff_attrs.speed_note_count}")
        if diff_attrs.ar is not None:
            print(f"    Adjusted AR: {diff_attrs.ar:.2f}")
        if diff_attrs.od is not None:
            print(f"    Adjusted OD: {diff_attrs.od:.2f}")
        if diff_attrs.hp is not None:
            print(f"    Adjusted HP: {diff_attrs.hp:.2f}")
        print(f"    Circles: {diff_attrs.n_circles}")
        print(f"    Sliders: {diff_attrs.n_sliders}")
        print(f"    Spinners: {diff_attrs.n_spinners}")
    
    elif mode == "Taiko":
        print(f"  osu!taiko specific:")
        print(f"    Stamina: {diff_attrs.stamina}")
        print(f"    Rhythm: {diff_attrs.rhythm}")
        print(f"    Color: {diff_attrs.color}")
        print(f"    Peak: {diff_attrs.peak}")
        print(f"    Hit Window: {diff_attrs.hit_window}")
    
    elif mode == "Catch":
        print(f"  osu!catch specific:")
        if diff_attrs.ar is not None:
            print(f"    Adjusted AR: {diff_attrs.ar:.2f}")
        print(f"    Fruits: {diff_attrs.n_fruits}")
        print(f"    Droplets: {diff_attrs.n_droplets}")
        print(f"    Tiny Droplets: {diff_attrs.n_tiny_droplets}")
    
    elif mode == "Mania":
        print(f"  osu!mania specific:")
        if diff_attrs.hit_window is not None:
            print(f"    Hit Window: {diff_attrs.hit_window}")

def display_performance_attributes(perf_attrs, mode):
    """Display performance attributes based on game mode."""
    print(f"  General:")
    print(f"    PP: {perf_attrs.pp:.2f}")
    print(f"    Stars: {perf_attrs.stars:.2f}")
    print(f"    Max Combo: {perf_attrs.max_combo}")
    
    if mode == "Osu":
        print(f"  osu!standard specific:")
        if perf_attrs.pp_acc is not None:
            print(f"    PP (Accuracy): {perf_attrs.pp_acc:.2f}")
        if perf_attrs.pp_aim is not None:
            print(f"    PP (Aim): {perf_attrs.pp_aim:.2f}")
        if perf_attrs.pp_speed is not None:
            print(f"    PP (Speed): {perf_attrs.pp_speed:.2f}")
        if perf_attrs.pp_flashlight is not None:
            print(f"    PP (Flashlight): {perf_attrs.pp_flashlight:.2f}")
        if perf_attrs.effective_miss_count is not None:
            print(f"    Effective Miss Count: {perf_attrs.effective_miss_count:.2f}")
    
    elif mode == "Taiko":
        print(f"  osu!taiko specific:")
        if perf_attrs.pp_acc is not None:
            print(f"    PP (Accuracy): {perf_attrs.pp_acc:.2f}")
        if perf_attrs.pp_difficulty is not None:
            print(f"    PP (Difficulty): {perf_attrs.pp_difficulty:.2f}")
        if perf_attrs.effective_miss_count is not None:
            print(f"    Effective Miss Count: {perf_attrs.effective_miss_count:.2f}")
    
    elif mode == "Catch":
        print(f"  osu!catch specific:")
        if perf_attrs.pp_acc is not None:
            print(f"    PP (Accuracy): {perf_attrs.pp_acc:.2f}")
    
    elif mode == "Mania":
        print(f"  osu!mania specific:")
        if perf_attrs.pp_difficulty is not None:
            print(f"    PP (Difficulty): {perf_attrs.pp_difficulty:.2f}")

def main():
    try:
        beatmap = Beatmap(path=MAP_PATH)
        mode = beatmap.mode
        
        print(f"Game Mode: {mode}\n")
        
        # Calculate difficulty attributes
        print("=== Difficulty Attributes ===")
        diff = Difficulty(beatmap, mods=0)
        diff_attrs = diff.calculate()
        display_difficulty_attributes(diff_attrs, mode)
        
        # Calculate performance attributes
        print(f"\n=== Performance Attributes ===")
        perf = Performance(beatmap, accuracy=99.0, misses=1)
        perf_attrs = perf.calculate()
        display_performance_attributes(perf_attrs, mode)
        
        # Demonstrate safe property access
        print(f"\n=== Safe Property Access ===")
        print(f"aim (osu! only): {diff_attrs.aim}")  # None if not osu!
        print(f"stamina (taiko only): {diff_attrs.stamina}")  # None if not taiko
        print(f"n_fruits (catch only): {diff_attrs.n_fruits}")  # None if not catch
        
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
