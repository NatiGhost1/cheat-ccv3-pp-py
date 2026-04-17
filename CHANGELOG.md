# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-01-XX

### Added

- Initial release of cheat-ccv3-pp-py
- Python bindings for core osu! calculators:
  - `Beatmap`: Parse .osu files from path, content, or bytes
  - `Difficulty`: Calculate star ratings with builder pattern
  - `Performance`: Calculate performance points with flexible configuration
- Comprehensive result types:
  - `DifficultyAttributes`: Mode-specific difficulty properties
  - `PerformanceAttributes`: Mode-specific PP breakdown
- Full support for all game modes:
  - osu!standard (with aim, speed, flashlight metrics)
  - osu!taiko (with stamina, rhythm, color metrics)
  - osu!catch (with fruit/droplet metrics)
  - osu!mania (with strain metrics)
- Exception types:
  - `ParseError`: For beatmap parsing failures
  - `ArgsError`: For invalid arguments
- Builder pattern API for flexible calculation configuration
- Support for:
  - Arbitrary accuracy (0-100%)
  - Variable combo
  - Custom hit counts (n300, n100, n50, n_katu, n_geki)
  - Mod combinations
  - Clock rate multipliers
  - Partial object passes
  - Beatmap conversions

### Architecture

- Clean separation of concerns with organized sections
- Macro-based code generation for reduced boilerplate
- Comprehensive documentation with docstrings
- Mode-agnostic API with graceful Option<T> fallbacks
