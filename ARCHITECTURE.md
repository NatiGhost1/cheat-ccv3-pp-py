# Architecture

This document describes the architecture and design patterns of the `cheat-ccv3-pp-py` Python bindings.

## Overview

`cheat-ccv3-pp-py` is a PyO3-based wrapper around the [cheat-ccv3-pp](https://github.com/NatiGhost1/cheat-ccv3-pp) Rust library. It provides high-performance osu! difficulty and performance point calculations for all game modes.

## Design Principles

1. **Performance First**: Leverage Rust for speed-critical calculations
2. **Pythonic API**: Provide idiomatic Python interfaces
3. **Flexibility**: Support multiple calculation modes and configurations
4. **Type Safety**: Leverage Rust's type system for correctness

## Module Structure

### Core Components

```
src/lib.rs
├── Exceptions
│   ├── ParseError         - Beatmap parsing errors
│   └── ArgsError          - Invalid arguments
│
├── Macros
│   ├── extract_kwarg!     - Kwargs extraction with type checking
│   └── mode_specific_getter! - Mode-specific property matching
│
├── Core Classes
│   ├── PyBeatmap          - Beatmap representation
│   ├── PyDifficulty       - Star rating calculator (builder)
│   ├── PyPerformance      - PP calculator (builder)
│   ├── PyDifficultyAttributes - Difficulty results
│   └── PyPerformanceAttributes - Performance results
│
└── Module Registration
    └── cheat_ccv3_pp_py   - Python module initialization
```

## Class Design Patterns

### 1. PyBeatmap

**Purpose**: Parse and store beatmap data

**API Pattern**: Constructor-based
```python
beatmap = Beatmap(path="file.osu")
# or
beatmap = Beatmap(content=str)
# or
beatmap = Beatmap(bytes=bytes)
```

**Properties**: Read-only getters for beatmap metadata

**Methods**: Factory methods to create calculators
- `stars()` → PyDifficulty
- `pp()` → PyPerformance

### 2. PyDifficulty & PyPerformance

**Purpose**: Configure and compute difficulty/PP calculations

**API Pattern**: Builder pattern with chainable methods

```python
# Kwargs constructor
diff = Difficulty(beatmap, mods=8, passed_objects=100)

# Or builder pattern
diff = Difficulty(beatmap)
diff.mods(8).passed_objects(100).calculate()
```

**State**: Encapsulates calculation parameters
- mods, passed_objects, clock_rate, is_convert (for Difficulty)
- mods, combo, accuracy, hit counts (for Performance)

**Key Method**: `calculate()` returns corresponding Attributes class

### 3. PyDifficultyAttributes & PyPerformanceAttributes

**Purpose**: Hold and expose calculation results

**API Pattern**: Read-only properties organized by mode

**Organization**:
```python
# General properties (all modes)
attrs.stars        # Star rating
attrs.max_combo    # Maximum combo

# Mode-specific properties
attrs.aim          # osu! only
attrs.stamina      # taiko only
attrs.ar           # osu!, catch
attrs.hit_window   # taiko, mania
```

## Builder Pattern

Both `PyDifficulty` and `PyPerformance` use the builder pattern for flexible configuration:

```python
calc = Difficulty(beatmap)
calc.mods(8)              # Returns &mut Self
calc.passed_objects(100)  # Chainable
calc.clock_rate(1.5)      # ...
result = calc.calculate() # Returns Attributes
```

**Benefits**:
- Fluent, readable API
- Optional parameters without many constructor overloads
- Supports kwargs constructor for conciseness

## Error Handling

### Exception Hierarchy

```
Python Standard Exceptions
├── ValueError
│   └── ParseError (from .osu file parsing)
│
└── TypeError
    └── ArgsError (from invalid arguments)
```

### Usage Pattern

```python
try:
    beatmap = Beatmap(path="map.osu")
except ParseError as e:
    # Handle beatmap parsing failure
    print(f"Invalid beatmap: {e}")
```

## Macro System

### extract_kwarg!

Simplifies keyword argument extraction with type checking:

```rust
let mods = extract_kwarg!(kwargs, "mods", u32).unwrap_or(0);
```

Benefits:
- Cleaner code than manual extraction
- Type safety
- Default values with `.unwrap_or()`

## Mode-Specific Properties

Mode detection is done through pattern matching on the inner `DifficultyAttributes` or `PerformanceAttributes` enum:

```rust
#[getter]
fn aim(&self) -> Option<f64> {
    match &self.inner {
        DifficultyAttributes::Osu(attrs) => Some(attrs.aim),
        _ => None,  // Only available in osu!standard
    }
}
```

**Design**:
- Returns `Option<T>` for type-safe mode detection
- Python code can check `if attrs.aim is not None`
- No panics on missing mode attributes

## Performance Considerations

1. **Zero-Copy**: Uses references where possible
2. **Lazy Evaluation**: Calculations performed on `.calculate()` call
3. **Caching**: Each calculation produces independent Attributes
4. **Memory**: Compounds are relatively lightweight (~few KB per result)

## Integration with cheat-ccv3-pp

The Python bindings wrap the Rust library at the following levels:

1. **Beatmap Parsing**: Delegates to cheat_ccv3_pp::Beatmap::from_path/bytes
2. **Difficulty Calculation**: Uses cheat_ccv3_pp::Beatmap::stars() builder
3. **Performance Calculation**: Uses cheat_ccv3_pp::Beatmap::pp() builder
4. **Result Types**: Wraps DifficultyAttributes and PerformanceAttributes enums

## Future Improvements

- [ ] Gradual difficulty/performance calculation
- [ ] Hit result generation
- [ ] Beatmap attributes builder
- [ ] Performance optimizations
- [ ] Extended Python type hints (py.typed)
