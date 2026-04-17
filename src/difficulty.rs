/// Difficulty calculation
///
/// Calculate star ratings and difficulty attributes for beatmaps.

use cheat_ccv3_pp::BeatmapExt;
use pyo3::prelude::*;

use crate::beatmap::PyBeatmap;
use crate::attributes::PyDifficultyAttributes;

/// Calculate star rating and difficulty metrics
///
/// Builder for computing difficulty attributes.
///
/// # Example
/// ```python
/// from cheat_ccv3_pp_py import Beatmap, Difficulty
/// beatmap = Beatmap(path="map.osu")
/// diff = Difficulty(beatmap, mods=8)
/// result = diff.calculate()
/// print(f"Stars: {result.stars}")
/// ```
#[pyclass(name = "Difficulty")]
#[derive(Clone)]
pub struct PyDifficulty {
    pub(crate) map: cheat_ccv3_pp::Beatmap,
    pub(crate) mods: u32,
    pub(crate) passed_objects: Option<usize>,
    pub(crate) clock_rate: Option<f64>,
    pub(crate) is_convert: Option<bool>,
}

#[pymethods]
impl PyDifficulty {
    /// Create a new Difficulty calculator
    #[new]
    fn new(map: &PyBeatmap, **kwargs: Option<&pyo3::types::PyDict>) -> PyResult<Self> {
        let kwargs = kwargs.unwrap_or_default();
        let difficulty = Self {
            map: map.inner.clone(),
            mods: extract_kwarg!(kwargs, "mods", u32).unwrap_or(0),
            passed_objects: extract_kwarg!(kwargs, "passed_objects", usize),
            clock_rate: extract_kwarg!(kwargs, "clock_rate", f64),
            is_convert: extract_kwarg!(kwargs, "is_convert", bool),
        };
        Ok(difficulty)
    }

    /// Set the mods (returns Self for chaining)
    fn mods(&mut self, mods: u32) -> &mut Self {
        self.mods = mods;
        self
    }

    /// Set number of passed objects (returns Self for chaining)
    fn passed_objects(&mut self, passed_objects: usize) -> &mut Self {
        self.passed_objects = Some(passed_objects);
        self
    }

    /// Set clock rate multiplier (returns Self for chaining)
    fn clock_rate(&mut self, clock_rate: f64) -> &mut Self {
        self.clock_rate = Some(clock_rate);
        self
    }

    /// Set whether to treat as a converted beatmap (returns Self for chaining)
    fn is_convert(&mut self, is_convert: bool) -> &mut Self {
        self.is_convert = Some(is_convert);
        self
    }

    /// Calculate difficulty with current settings
    fn calculate(&self) -> PyDifficultyAttributes {
        let mut calc = self.map.stars().mods(self.mods);

        if let Some(passed_objects) = self.passed_objects {
            calc = calc.passed_objects(passed_objects);
        }

        if let Some(clock_rate) = self.clock_rate {
            calc = calc.clock_rate(clock_rate);
        }

        if let Some(is_convert) = self.is_convert {
            calc = calc.is_convert(is_convert);
        }

        calc.calculate().into()
    }
}
