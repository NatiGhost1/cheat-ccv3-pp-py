# cheat-ccv3-pp-py

Python bindings for the `cheat-ccv3-pp` osu! performance point and star rating system.

## Installation

Build the native extension with `maturin`:

```bash
python3 -m pip install maturin
python3 -m maturin develop
```

## Usage

```python
import cheat_ccv3_pp_py

# Parse a beatmap from disk
beatmap = cheat_ccv3_pp_py.Beatmap(path="/path/to/map.osu")

# Create a performance builder
perf = beatmap.pp()
perf.mods(24)
perf.combo(1234)
perf.accuracy(99.2)
perf.n_misses(2)
result = perf.calculate()
print(result.pp, result.stars)

# Create a difficulty builder
diff = beatmap.stars()
diff.mods(16)
stars = diff.calculate().stars
print(stars)
```

## Features

- Parse `.osu` files from `path`, raw `content` string, or raw `bytes`
- Calculate mode-specific performance with `Beatmap.pp()` and `Performance.calculate()`
- Calculate difficulty stars with `Beatmap.stars()` and `Difficulty.calculate()`
- Expose `pp`, `stars`, and `max_combo` results from the underlying Rust calculator

## Notes

This wrapper binds directly to the `cheat-ccv3-pp` crate, so the build uses the same performance/timing logic as the PP system backed by that Rust library.
