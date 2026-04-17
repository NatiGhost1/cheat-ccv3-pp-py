"""OPTIONAL REFERENCE: Bancho-style performance calculator for cheat-ccv3-pp.

This module is NOT part of the cheat_ccv3_pp_py binding. It's provided as
optional reference material for private servers that want a Bancho-compatible
score calculation API. Copy this file to your project if you need it.

It wraps the `cheat_ccv3_pp_py` binding and exposes a ScoreParams-based API
compatible with osuAkatsuki/bancho.py performance calculation patterns.
"""

from __future__ import annotations

import math
from collections.abc import Iterable
from dataclasses import dataclass
from typing import Optional, TypedDict

try:
    from cheat_ccv3_pp_py import Beatmap
except ImportError as exc:  # pragma: no cover
    Beatmap = None  # type: ignore
    _IMPORT_ERROR = exc

# osu! mod bitmask values.
MODS = {
    "NF": 1,
    "EZ": 2,
    "TD": 4,
    "HD": 8,
    "HR": 16,
    "SD": 32,
    "DT": 64,
    "RX": 128,
    "HT": 256,
    "NC": 512,
    "FL": 1024,
    "SO": 4096,
    "PF": 16384,
}

NIGHTCORE = MODS["NC"]
DOUBLETIME = MODS["DT"]

MODE_NAMES = {
    0: "Osu",
    1: "Taiko",
    2: "Catch",
    3: "Mania",
}


@dataclass
class ScoreParams:
    mode: Optional[int] = None
    mods: Optional[int] = None
    combo: Optional[int] = None
    acc: Optional[float] = None
    n300: Optional[int] = None
    n100: Optional[int] = None
    n50: Optional[int] = None
    ngeki: Optional[int] = None
    nkatu: Optional[int] = None
    nmiss: Optional[int] = None
    passed_objects: Optional[int] = None
    clock_rate: Optional[float] = None
    is_convert: Optional[bool] = None


class PerformanceRating(TypedDict, total=False):
    pp: float
    pp_acc: float
    pp_aim: float
    pp_speed: float
    pp_flashlight: float
    pp_difficulty: float
    effective_miss_count: float


class DifficultyRating(TypedDict, total=False):
    stars: float
    aim: float
    speed: float
    flashlight: float
    slider_factor: float
    speed_note_count: float
    stamina: float
    color: float
    rhythm: float
    peak: float
    hit_window: float
    ar: float
    od: float
    hp: float
    n_fruits: int
    n_droplets: int
    n_tiny_droplets: int
    max_combo: int


class PerformanceResultType(TypedDict):
    performance: PerformanceRating
    difficulty: DifficultyRating


def require_binding() -> None:
    if Beatmap is None:
        raise ImportError(
            "cheat_ccv3_pp_py module is required. "
            "Build and install the binding before using performance.py."
        )


def mods_from_names(names: list[str]) -> int:
    """Convert osu! mod names to a bitmask."""
    value = 0
    for name in names:
        canonical = name.strip().upper()
        if canonical not in MODS:
            raise ValueError(f"Unknown mod name: {name}")
        value |= MODS[canonical]
    return value


def _normalise_mods(mods: Optional[int]) -> int:
    mods_value = mods or 0
    if mods_value & NIGHTCORE:
        mods_value |= DOUBLETIME
    return mods_value


def _load_beatmap(path: str):
    require_binding()
    return Beatmap(path=path)


def _build_performance(beatmap, score: ScoreParams):
    if score.mode is not None:
        expected_mode = MODE_NAMES.get(score.mode)
        actual_mode = getattr(beatmap, "mode", None)

        if expected_mode is None:
            raise ValueError(f"Unsupported mode value: {score.mode}")

        if actual_mode != expected_mode:
            raise ValueError(
                f"Score mode {expected_mode} does not match beatmap mode {actual_mode}.",
            )

    perf = beatmap.pp()
    perf.mods(_normalise_mods(score.mods))

    if score.combo is not None:
        perf.combo(score.combo)
    if score.acc is not None:
        perf.accuracy(score.acc)
    if score.nmiss is not None:
        perf.n_misses(score.nmiss)
    if score.n300 is not None:
        perf.n300(score.n300)
    if score.n100 is not None:
        perf.n100(score.n100)
    if score.n50 is not None:
        perf.n50(score.n50)
    if score.nkatu is not None:
        perf.n_katu(score.nkatu)
    if score.ngeki is not None:
        perf.n_geki(score.ngeki)
    if score.passed_objects is not None:
        perf.passed_objects(score.passed_objects)
    if score.clock_rate is not None:
        perf.clock_rate(score.clock_rate)
    if score.is_convert is not None:
        perf.is_convert(score.is_convert)

    return perf.calculate()


def _round_pp(value: float) -> float:
    if math.isnan(value) or math.isinf(value):
        return 0.0
    return round(value, 3)


def _extract_attribute(obj, name: str):
    return getattr(obj, name, None)


def calculate_performances(
    osu_file_path: str,
    scores: Iterable[ScoreParams],
) -> list[PerformanceResultType]:
    """Calculate performance for multiple scores on a single beatmap."""
    beatmap = _load_beatmap(osu_file_path)
    results: list[PerformanceResultType] = []

    for score in scores:
        if score.acc is not None and any(
            value is not None
            for value in (score.n300, score.n100, score.n50, score.ngeki, score.nkatu)
        ):
            raise ValueError(
                "Cannot specify both accuracy and hit counts. Use either acc or 300/100/50/geki/katu.",
            )

        result = _build_performance(beatmap, score)
        pp = _round_pp(result.pp)

        difficulty = _extract_attribute(result, "difficulty")

        results.append(
            {
                "performance": {
                    "pp": pp,
                    "pp_acc": _extract_attribute(result, "pp_acc"),
                    "pp_aim": _extract_attribute(result, "pp_aim"),
                    "pp_speed": _extract_attribute(result, "pp_speed"),
                    "pp_flashlight": _extract_attribute(result, "pp_flashlight"),
                    "pp_difficulty": _extract_attribute(result, "pp_difficulty"),
                    "effective_miss_count": _extract_attribute(result, "effective_miss_count"),
                },
                "difficulty": {
                    "stars": _extract_attribute(difficulty, "stars"),
                    "max_combo": _extract_attribute(difficulty, "max_combo"),
                    "aim": _extract_attribute(difficulty, "aim"),
                    "speed": _extract_attribute(difficulty, "speed"),
                    "flashlight": _extract_attribute(difficulty, "flashlight"),
                    "slider_factor": _extract_attribute(difficulty, "slider_factor"),
                    "speed_note_count": _extract_attribute(difficulty, "speed_note_count"),
                    "stamina": _extract_attribute(difficulty, "stamina"),
                    "color": _extract_attribute(difficulty, "color"),
                    "rhythm": _extract_attribute(difficulty, "rhythm"),
                    "peak": _extract_attribute(difficulty, "peak"),
                    "hit_window": _extract_attribute(difficulty, "hit_window"),
                    "ar": _extract_attribute(difficulty, "ar"),
                    "od": _extract_attribute(difficulty, "od"),
                    "hp": _extract_attribute(difficulty, "hp"),
                    "n_fruits": _extract_attribute(difficulty, "n_fruits"),
                    "n_droplets": _extract_attribute(difficulty, "n_droplets"),
                    "n_tiny_droplets": _extract_attribute(difficulty, "n_tiny_droplets"),
                },
            }
        )

    return results


def calculate_difficulty(
    osu_file_path: str,
    mods: Optional[int] = None,
) -> DifficultyRating:
    """Calculate difficulty rating for a single beatmap."""
    beatmap = _load_beatmap(osu_file_path)
    diff = beatmap.stars()
    diff.mods(_normalise_mods(mods))
    result = diff.calculate()
    difficulty = _extract_attribute(result, "difficulty")
    return {
        "stars": result.stars,
        "max_combo": result.max_combo,
        "aim": _extract_attribute(difficulty, "aim"),
        "speed": _extract_attribute(difficulty, "speed"),
        "flashlight": _extract_attribute(difficulty, "flashlight"),
        "slider_factor": _extract_attribute(difficulty, "slider_factor"),
        "speed_note_count": _extract_attribute(difficulty, "speed_note_count"),
        "stamina": _extract_attribute(difficulty, "stamina"),
        "color": _extract_attribute(difficulty, "color"),
        "rhythm": _extract_attribute(difficulty, "rhythm"),
        "peak": _extract_attribute(difficulty, "peak"),
        "hit_window": _extract_attribute(difficulty, "hit_window"),
        "ar": _extract_attribute(difficulty, "ar"),
        "od": _extract_attribute(difficulty, "od"),
        "hp": _extract_attribute(difficulty, "hp"),
        "n_fruits": _extract_attribute(difficulty, "n_fruits"),
        "n_droplets": _extract_attribute(difficulty, "n_droplets"),
        "n_tiny_droplets": _extract_attribute(difficulty, "n_tiny_droplets"),
    }


__all__ = [
    "MODS",
    "NIGHTCORE",
    "DOUBLETIME",
    "ScoreParams",
    "PerformanceRating",
    "DifficultyRating",
    "PerformanceResultType",
    "mods_from_names",
    "calculate_performances",
    "calculate_difficulty",
]
