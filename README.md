# cheat-ccv3-pp-py

A fast Python library for calculating [osu!](https://osu.ppy.sh/) performance points, star ratings, and other difficulty attributes.

Powered by [cheat-ccv3-pp](https://github.com/NatiGhost1/cheat-ccv3-pp), a high-performance Rust implementation of the osu! ranking algorithms.

## Features

- 🚀 **Fast**: ~100x faster than implementing calculations in pure Python
- 🎮 **Multi-mode**: Full support for osu!standard, osu!taiko, osu!catch, and osu!mania
- 📊 **Comprehensive**: Calculate difficulty attributes, performance points, and various metrics
- 🔧 **Flexible**: Support for arbitrary accuracy, combo, misses, hit counts, and more
- 💾 **Lightweight**: Parse from file path, string content, or raw bytes

## Installation

### From PyPI (when available)

```bash
pip install cheat-ccv3-pp-py
```

### From Source

Clone the repository and install with `maturin`:

```bash
git clone https://github.com/NatiGhost1/cheat-ccv3-pp-py.git
cd cheat-ccv3-pp-py

pip install maturin
maturin develop --release  # or just 'maturin develop'
```

## Quick Start

```python
from cheat_ccv3_pp_py import Beatmap, Difficulty, Performance

# Parse a beatmap
beatmap = Beatmap(path="map.osu")

# Calculate star rating
difficulty = Difficulty(beatmap, mods=8)
stars_result = difficulty.calculate()
print(f"Stars: {stars_result.stars}")

# Calculate performance points with custom accuracy and misses
performance = Performance(beatmap, mods=8, accuracy=98.5, misses=1)
pp_result = performance.calculate()
print(f"PP: {pp_result.pp}")
```

## API Reference

### Beatmap

Parse and access beatmap information.

```python
# Create from file path
beatmap = Beatmap(path="map.osu")

# Or from raw content (string)
beatmap = Beatmap(content=osu_file_content)

# Or from raw bytes
beatmap = Beatmap(bytes=b"[Metadata]\nTitle:...")
```

**Properties:**
- `mode`: Game mode (Osu, Taiko, Catch, Mania)
- `ar`: Approach rate (0-13)
- `cs`: Circle size (0-10)
- `od`: Overall difficulty (0-10)
- `hp`: Health drain (0-10)
- `bpm`: Average beats per minute
- `n_circles`: Number of circles
- `n_sliders`: Number of sliders
- `n_spinners`: Number of spinners

**Methods:**
- `stars()`: Create a Difficulty calculator
- `pp()`: Create a Performance calculator
- `max_stars(mods)`: Calculate max star rating with given mods
- `max_pp(mods)`: Calculate max PP with given mods

### Difficulty

Calculate star ratings and difficulty attributes.

```python
# Constructor with keyword arguments
diff = Difficulty(
    beatmap,
    mods=8,
    passed_objects=100,
    clock_rate=1.5,
    is_convert=False
)
result = diff.calculate()

# Or builder pattern
diff = Difficulty(beatmap)
diff.mods(8).passed_objects(100).clock_rate(1.5).is_convert(False)
result = diff.calculate()
```

**Builder Methods:**
- `mods(mods: int)`: Set mod flags (e.g., 8 = HD)
- `passed_objects(count: int)`: Set number of passed objects
- `clock_rate(rate: float)`: Set clock rate multiplier
- `is_convert(is_convert: bool)`: Set if beatmap is converted
- `calculate()`: Calculate and return DifficultyAttributes

**DifficultyAttributes Properties:**
- `stars`: Calculated star rating
- `max_combo`: Maximum combo
- **osu! specific**: `aim`, `speed`, `flashlight`, `slider_factor`, `speed_note_count`, `n_circles`, `n_sliders`, `n_spinners`, `ar`, `od`, `hp`, `local_sr_per_minute`, `local_sr_per_15s`
- **taiko specific**: `stamina`, `rhythm`, `color`, `peak`, `hit_window`
- **catch specific**: `ar`, `n_fruits`, `n_droplets`, `n_tiny_droplets`
- **mania specific**: `hit_window`

### Performance

Calculate performance points and related metrics.

```python
# Constructor with keyword arguments
perf = Performance(
    beatmap,
    mods=8,
    accuracy=99.5,
    combo=500,
    misses=0
)
result = perf.calculate()

# Or builder pattern
perf = Performance(beatmap)
perf.mods(8).accuracy(99.5).combo(500).misses(0)
result = perf.calculate()

# Detailed hit counts (mutually exclusive with accuracy)
perf = Performance(beatmap)
perf.mods(8).n300(800).n100(10).n50(2).misses(1)
result = perf.calculate()
```

**Builder Methods:**
- `mods(mods: int)`: Set mod flags
- `accuracy(acc: float)`: Set accuracy percentage (0-100)
- `combo(combo: int)`: Set max combo
- `misses(misses: int)`: Set number of misses
- `n_misses(misses: int)`: Alias for `misses()`
- `n300(count: int)`: Set number of 300s (or Geki)
- `n100(count: int)`: Set number of 100s
- `n50(count: int)`: Set number of 50s
- `n_katu(count: int)`: Set number of Katu (taiko 100s)
- `n_geki(count: int)`: Set number of Geki (taiko 300s)
- `passed_objects(count: int)`: Set number of passed objects
- `clock_rate(rate: float)`: Set clock rate multiplier
- `is_convert(is_convert: bool)`: Set if beatmap is converted
- `calculate()`: Calculate and return PerformanceAttributes

**PerformanceAttributes Properties:**
- `pp`: Total performance points
- `stars`: Star rating
- `max_combo`: Maximum combo
- **osu! specific**: `pp_acc`, `pp_aim`, `pp_speed`, `pp_flashlight`, `effective_miss_count`
- **taiko specific**: `pp_acc`, `pp_difficulty`, `effective_miss_count`
- **catch specific**: `pp_acc`
- **mania specific**: `pp_difficulty`
- `difficulty`: DifficultyAttributes reference

## Supported Game Modes

- **osu!standard** (`mode: "Osu"`)
- **osu!taiko** (`mode: "Taiko"`)
- **osu!catch** (`mode: "Catch"`)
- **osu!mania** (`mode: "Mania"`)

## Mod Flags

Mods are passed as integer flags. Common values:

- `0` = No Mods
- `1` = NF (NoFail)
- `2` = EZ (Easy)
- `4` = TD (TouchDevice)
- `8` = HD (Hidden)
- `16` = HR (HardRock)
- `32` = SD (Sudden Death)
- `64` = DT (DoubleTime)
- `128` = RL (Relax)
- `256` = HT (HalfTime)
- `512` = NC (NightCore) = DT + 512
- `1024` = FL (Flashlight)
- `2048` = SO (Spun Out)
- `4096` = AU (AutoPilot)
- `8192` = AP (Auto)
- `16384` = CN (Cinema)

## Error Handling

```python
from cheat_ccv3_pp_py import ParseError, ArgsError

try:
    beatmap = Beatmap(path="nonexistent.osu")
except ParseError as e:
    print(f"Failed to parse beatmap: {e}")

try:
    perf = Performance(beatmap)
    perf.invalid_method()
except ArgsError as e:
    print(f"Invalid arguments: {e}")
```

## Performance Notes

This library uses Rust for speed. Calculations are typically:
- **1-5ms** per calculation on modern hardware
- **100x faster** than pure Python implementations
- **Suitable for batch processing** and real-time applications

## License

MIT - See [LICENSE](LICENSE) for details.

## Related Projects

- [cheat-ccv3-pp](https://github.com/NatiGhost1/cheat-ccv3-pp) - Core Rust library
- [rosu-pp](https://github.com/MaxOhn/rosu-pp) - Alternative Rust implementation
- [rosu-pp-py](https://github.com/MaxOhn/rosu-pp-py) - Python bindings for rosu-pp

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.
- Calculate mode-specific performance with `Beatmap.pp()` and `Performance.calculate()`
- Calculate difficulty stars with `Beatmap.stars()` and `Difficulty.calculate()`
- Expose `pp`, `stars`, and `max_combo` results from the underlying Rust calculator

## Notes

This wrapper binds directly to the `cheat-ccv3-pp` crate, so the build uses the same performance/timing logic as the PP system backed by that Rust library.
