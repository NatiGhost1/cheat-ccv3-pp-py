/// Performance point calculation
///
/// Calculate PP and performance metrics for beatmaps and scores.

use cheat_ccv3_pp::BeatmapExt;
use pyo3::prelude::*;

use crate::beatmap::PyBeatmap;
use crate::attributes::PyPerformanceAttributes;

/// Calculate performance points and metrics
///
/// Builder for computing performance attributes with flexible score configuration.
///
/// # Example
/// ```python
/// from cheat_ccv3_pp_py import Beatmap, Performance
/// beatmap = Beatmap(path="map.osu")
/// perf = Performance(beatmap, accuracy=99.5, misses=1)
/// result = perf.calculate()
/// print(f"PP: {result.pp}")
/// ```
#[pyclass(name = "Performance")]
#[derive(Clone)]
pub struct PyPerformance {
    pub(crate) map: cheat_ccv3_pp::Beatmap,
    pub(crate) mods: u32,
    pub(crate) combo: Option<usize>,
    pub(crate) accuracy: Option<f64>,
    pub(crate) n_misses: Option<usize>,
    pub(crate) n300: Option<usize>,
    pub(crate) n100: Option<usize>,
    pub(crate) n50: Option<usize>,
    pub(crate) n_katu: Option<usize>,
    pub(crate) n_geki: Option<usize>,
    pub(crate) passed_objects: Option<usize>,
    pub(crate) clock_rate: Option<f64>,
    pub(crate) is_convert: Option<bool>,
}

#[pymethods]
impl PyPerformance {
    /// Create a new Performance calculator
    #[new]
    fn new(map: &PyBeatmap, **kwargs: Option<&pyo3::types::PyDict>) -> PyResult<Self> {
        let kwargs = kwargs.unwrap_or_default();
        let perf = Self {
            map: map.inner.clone(),
            mods: extract_kwarg!(kwargs, "mods", u32).unwrap_or(0),
            combo: extract_kwarg!(kwargs, "combo", usize),
            accuracy: extract_kwarg!(kwargs, "accuracy", f64),
            n_misses: extract_kwarg!(kwargs, "misses", usize),
            n300: extract_kwarg!(kwargs, "n300", usize),
            n100: extract_kwarg!(kwargs, "n100", usize),
            n50: extract_kwarg!(kwargs, "n50", usize),
            n_katu: extract_kwarg!(kwargs, "n_katu", usize),
            n_geki: extract_kwarg!(kwargs, "n_geki", usize),
            passed_objects: extract_kwarg!(kwargs, "passed_objects", usize),
            clock_rate: extract_kwarg!(kwargs, "clock_rate", f64),
            is_convert: extract_kwarg!(kwargs, "is_convert", bool),
        };
        Ok(perf)
    }

    /// Set the mods (returns Self for chaining)
    fn mods(&mut self, mods: u32) -> &mut Self {
        self.mods = mods;
        self
    }

    /// Set maximum combo (returns Self for chaining)
    fn combo(&mut self, combo: usize) -> &mut Self {
        self.combo = Some(combo);
        self
    }

    /// Set accuracy percentage 0-100 (returns Self for chaining)
    fn accuracy(&mut self, accuracy: f64) -> &mut Self {
        self.accuracy = Some(accuracy);
        self
    }

    /// Set number of misses (returns Self for chaining)
    fn misses(&mut self, n_misses: usize) -> &mut Self {
        self.n_misses = Some(n_misses);
        self
    }

    /// Alias for misses() (returns Self for chaining)
    fn n_misses(&mut self, n_misses: usize) -> &mut Self {
        self.n_misses = Some(n_misses);
        self
    }

    /// Set number of 300s (returns Self for chaining)
    fn n300(&mut self, n300: usize) -> &mut Self {
        self.n300 = Some(n300);
        self
    }

    /// Set number of 100s (returns Self for chaining)
    fn n100(&mut self, n100: usize) -> &mut Self {
        self.n100 = Some(n100);
        self
    }

    /// Set number of 50s (returns Self for chaining)
    fn n50(&mut self, n50: usize) -> &mut Self {
        self.n50 = Some(n50);
        self
    }

    /// Set number of katsu (100s in taiko) (returns Self for chaining)
    fn n_katu(&mut self, n_katu: usize) -> &mut Self {
        self.n_katu = Some(n_katu);
        self
    }

    /// Set number of geki (300s in taiko) (returns Self for chaining)
    fn n_geki(&mut self, n_geki: usize) -> &mut Self {
        self.n_geki = Some(n_geki);
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

    /// Calculate performance with current settings
    fn calculate(&self) -> PyPerformanceAttributes {
        let mut calc = self.map.pp().mods(self.mods);

        if let Some(combo) = self.combo {
            calc = calc.combo(combo);
        }

        if let Some(accuracy) = self.accuracy {
            calc = calc.accuracy(accuracy);
        }

        if let Some(n_misses) = self.n_misses {
            calc = calc.n_misses(n_misses);
        }

        if let Some(n300) = self.n300 {
            calc = calc.n300(n300);
        }

        if let Some(n100) = self.n100 {
            calc = calc.n100(n100);
        }

        if let Some(n50) = self.n50 {
            calc = calc.n50(n50);
        }

        if let Some(n_katu) = self.n_katu {
            calc = calc.n_katu(n_katu);
        }

        if let Some(n_geki) = self.n_geki {
            calc = calc.n_geki(n_geki);
        }

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
