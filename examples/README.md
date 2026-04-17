# Examples

This directory contains example scripts demonstrating how to use `cheat-ccv3-pp-py`.

## Running Examples

Before running examples, ensure you have the library installed:

```bash
cd ..
maturin develop
cd examples
```

## Example Scripts

### 1. `basic.py` - Basic Usage

Demonstrates fundamental usage:
- Loading a beatmap
- Calculating star ratings
- Calculating performance points
- Getting max values

**Run**: `python3 basic.py`

### 2. `builder_pattern.py` - Builder Pattern

Shows the flexible builder pattern API:
- Constructor-based configuration
- Method chaining
- Complex parameter combinations
- Hit count specification

**Run**: `python3 builder_pattern.py`

### 3. `mode_specific.py` - Mode-Specific Attributes

Explores mode-specific properties:
- osu!standard (aim, speed, flashlight, etc.)
- osu!taiko (stamina, rhythm, color, etc.)
- osu!catch (fruits, droplets, AR)
- osu!mania (hit window)
- Safe property access with None checks

**Run**: `python3 mode_specific.py`

### 4. `batch_processing.py` - Batch Processing

Efficiently processes multiple scores:
- Load beatmap once
- Calculate multiple scores
- Display results in table format
- Compute statistics

**Run**: `python3 batch_processing.py`

## Tips for Using These Examples

### Providing Your Own Beatmap

To test with a specific .osu file, modify the `MAP_PATH` variable:

```python
MAP_PATH = "/path/to/your/map.osu"
```

### Understanding Mod Flags

Mods are combined as bitflags:

```python
NO_MOD = 0
EASY = 2
NOISY = 4
HIDDEN = 8
HARD_ROCK = 16
SUDDEN_DEATH = 32
DOUBLE_TIME = 64
HALF_TIME = 128
NIGHTCORE = 512 + 64  # DT + NC
FLASHLIGHT = 1024
```

### Working with Accuracy

Accuracy can be specified as a percentage (0-100):

```python
perf = Performance(beatmap, accuracy=99.5)  # 99.5% accuracy
```

Or using specific hit counts:

```python
perf = Performance(beatmap)
perf.n300(850).n100(5).n50(0).misses(2)
```

### Understanding Return Values

- **Difficulty Attributes**: Contains `stars` and mode-specific values
- **Performance Attributes**: Contains `pp`, `stars`, and mode-specific breakdowns

## Example Patterns

### Get Max PP

```python
from cheat_ccv3_pp_py import Beatmap

beatmap = Beatmap(path="map.osu")
max_pp = beatmap.max_pp(8)  # 8 = HD
print(f"Max PP with HD: {max_pp.pp}")
```

### Calculate with Various Mods

```python
from cheat_ccv3_pp_py import Performance

for mods in [0, 8, 16, 24]:  # No Mod, HD, HR, HD+HR
    perf = Performance(beatmap, mods=mods, accuracy=99.0)
    result = perf.calculate()
    print(f"Mod {mods}: {result.pp:.2f}pp")
```

### Check Mode-Specific Properties

```python
diff_attrs = diff.calculate()

# Safely access mode-specific attributes
if diff_attrs.aim is not None:
    print(f"This is osu!standard: aim {diff_attrs.aim}")

if diff_attrs.stamina is not None:
    print(f"This is osu!taiko: stamina {diff_attrs.stamina}")
```

### Error Handling

```python
from cheat_ccv3_pp_py import ParseError, ArgsError

try:
    beatmap = Beatmap(path="map.osu")
except ParseError as e:
    print(f"Failed to parse: {e}")
except ArgsError as e:
    print(f"Invalid arguments: {e}")
```

## Contributing Examples

If you have a useful example pattern, feel free to contribute!

1. Create a new `.py` file with a descriptive name
2. Add comprehensive comments
3. Include a docstring explaining the example
4. Add it to this README

## Resources

- [Main README](../README.md) - Full documentation
- [Architecture](../ARCHITECTURE.md) - Design details
- [cheat-ccv3-pp](https://github.com/NatiGhost1/cheat-ccv3-pp) - Rust library
