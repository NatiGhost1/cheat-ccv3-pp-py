//! cheat-ccv3-pp-py: Python bindings for cheat-ccv3-pp osu! calculator
//!
//! A high-performance Python library for calculating osu! difficulty ratings and
//! performance points using the cheat-ccv3-pp Rust implementation.
//!
//! # Example
//!
//! ```python
//! from cheat_ccv3_pp_py import Beatmap, Difficulty, Performance
//!
//! # Parse a beatmap
//! beatmap = Beatmap(path="map.osu")
//!
//! # Calculate star rating
//! diff = Difficulty(beatmap, mods=8)
//! result = diff.calculate()
//! print(f"Stars: {result.stars}")
//!
//! # Calculate performance points
//! perf = Performance(beatmap, accuracy=98.5, misses=1)
//! pp = perf.calculate()
//! print(f"PP: {pp.pp}")
//! ```

use pyo3::prelude::*;

// ==================== MODULES ====================

pub mod attributes;
pub mod beatmap;
pub mod difficulty;
pub mod error;
pub mod macros;
pub mod mode;
pub mod mods;
pub mod performance;
pub mod score_state;
pub mod strains;

// ==================== EXPORTS ====================

pub use attributes::{PyDifficultyAttributes, PyPerformanceAttributes};
pub use beatmap::PyBeatmap;
pub use difficulty::PyDifficulty;
pub use error::{ArgsError, ParseError};
pub use mods::PyGameMods;
pub use performance::PyPerformance;
pub use score_state::PyScoreState;
pub use strains::PyStrains;

/// Difficulty calculator with builder pattern
#[pyclass(name = "Difficulty")]
#[derive(Clone)]
pub struct PyDifficulty {
    map: Beatmap,
    mods: u32,
    passed_objects: Option<usize>,
    clock_rate: Option<f64>,
    is_convert: Option<bool>,
}

/// Performance calculator with builder pattern
#[pyclass(name = "Performance")]
#[derive(Clone)]
pub struct PyPerformance {
    map: Beatmap,
    mods: u32,
    combo: Option<usize>,
    accuracy: Option<f64>,
    n_misses: Option<usize>,
    n300: Option<usize>,
    n100: Option<usize>,
    n50: Option<usize>,
    n_katu: Option<usize>,
    n_geki: Option<usize>,
    passed_objects: Option<usize>,
    clock_rate: Option<f64>,
    is_convert: Option<bool>,
}

/// Difficulty calculation results
#[pyclass(name = "DifficultyAttributes")]
pub struct PyDifficultyAttributes {
    inner: DifficultyAttributes,
}

/// Performance calculation results
#[pyclass(name = "PerformanceAttributes")]
pub struct PyPerformanceAttributes {
    inner: PerformanceAttributes,
}

// ==================== BEATMAP IMPLEMENTATION ====================

#[pymethods]
impl PyBeatmap {
    /// Create a Beatmap from path, content, or bytes
    ///
    /// # Arguments
    /// * `path`: File path to .osu file
    /// * `content`: String content of .osu file
    /// * `bytes`: Binary content of .osu file
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        let kwargs = kwargs.ok_or_else(|| {
            ParseError::new_err("Beatmap() requires keyword argument 'path', 'content', or 'bytes'")
        })?;

        let beatmap = if let Some(path) = extract_kwarg!(kwargs, "path", &str) {
            Beatmap::from_path(path).map_err(|err| {
                ParseError::new_err(format!("Failed to parse beatmap from path: {err}"))
            })?
        } else if let Some(content) = extract_kwarg!(kwargs, "content", &str) {
            Beatmap::from_bytes(content.as_bytes()).map_err(|err| {
                ParseError::new_err(format!("Failed to parse beatmap from content: {err}"))
            })?
        } else if let Ok(bytes_obj) = extract_kwarg!(kwargs, "bytes", &PyBytes) {
            Beatmap::from_bytes(bytes_obj.as_bytes()).map_err(|err| {
                ParseError::new_err(format!("Failed to parse beatmap from bytes: {err}"))
            })?
        } else {
            return Err(ParseError::new_err(
                "Beatmap requires one of: 'path' (str), 'content' (str), or 'bytes' (bytes-like)",
            ));
        };

        Ok(PyBeatmap { inner: beatmap })
    }

    // ==================== BEATMAP GETTERS ====================

    /// Game mode (Osu, Taiko, Catch, Mania)
    #[getter]
    fn mode(&self) -> String {
        format!("{:?}", self.inner.mode)
    }

    /// Approach rate (0-13)
    #[getter]
    fn ar(&self) -> f32 {
        self.inner.ar
    }

    /// Circle size (0-10)
    #[getter]
    fn cs(&self) -> f32 {
        self.inner.cs
    }

    /// Health drain (0-10)
    #[getter]
    fn hp(&self) -> f32 {
        self.inner.hp
    }

    /// Overall difficulty (0-10)
    #[getter]
    fn od(&self) -> f32 {
        self.inner.od
    }

    /// Average BPM of the beatmap
    #[getter]
    fn bpm(&self) -> f64 {
        self.inner.bpm()
    }

    /// Number of circles
    #[getter]
    fn n_circles(&self) -> u32 {
        self.inner.n_circles
    }

    /// Number of sliders
    #[getter]
    fn n_sliders(&self) -> u32 {
        self.inner.n_sliders
    }

    /// Number of spinners
    #[getter]
    fn n_spinners(&self) -> u32 {
        self.inner.n_spinners
    }

    // ==================== CALCULATION BUILDERS ====================

    /// Create a Difficulty calculator for this map
    fn stars(&self) -> PyDifficulty {
        PyDifficulty::from_beatmap(self.inner.clone())
    }

    /// Create a Performance calculator for this map
    fn pp(&self) -> PyPerformance {
        PyPerformance::from_beatmap(self.inner.clone())
    }

    /// Calculate max PP with given mods (100% accuracy, perfect combo)
    fn max_pp(&self, mods: u32) -> PyPerformanceAttributes {
        self.inner.max_pp(mods).into()
    }

    /// Calculate max stars with given mods
    fn max_stars(&self, mods: u32) -> PyDifficultyAttributes {
        self.inner.stars().mods(mods).calculate().into()
    }
}

// ==================== DIFFICULTY CALCULATOR IMPLEMENTATION ====================

#[pymethods]
impl PyDifficulty {
    /// Create a new Difficulty calculator for a beatmap
    #[new]
    fn new(map: &PyBeatmap, **kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut difficulty = Self {
            map: map.inner.clone(),
            mods: extract_kwarg!(kwargs, "mods", u32).unwrap_or(0),
            passed_objects: extract_kwarg!(kwargs, "passed_objects", usize),
            clock_rate: extract_kwarg!(kwargs, "clock_rate", f64),
            is_convert: extract_kwarg!(kwargs, "is_convert", bool),
        };
        Ok(difficulty)
    }

    /// Internal constructor from beatmap
    #[doc(hidden)]
    fn from_beatmap(map: Beatmap) -> Self {
        PyDifficulty {
            map,
            mods: 0,
            passed_objects: None,
            clock_rate: None,
            is_convert: None,
        }
    }

    // ==================== BUILDER PATTERN METHODS ====================

    /// Set the mods for calculation (returns Self for chaining)
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
    ///
    /// Returns DifficultyAttributes with star rating and mode-specific values
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

// ==================== PERFORMANCE CALCULATOR IMPLEMENTATION ====================

#[pymethods]
impl PyPerformance {
    /// Create a new Performance calculator for a beatmap
    #[new]
    fn new(map: &PyBeatmap, **kwargs: Option<&PyDict>) -> PyResult<Self> {
        let kwargs = kwargs.unwrap_or_default();
        let mut perf = Self {
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

    /// Internal constructor from beatmap
    #[doc(hidden)]
    fn from_beatmap(map: Beatmap) -> Self {
        PyPerformance {
            map,
            mods: 0,
            combo: None,
            accuracy: None,
            n_misses: None,
            n300: None,
            n100: None,
            n50: None,
            n_katu: None,
            n_geki: None,
            passed_objects: None,
            clock_rate: None,
            is_convert: None,
        }
    }

    // ==================== BUILDER PATTERN METHODS ====================

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

    /// Set accuracy percentage (0-100) (returns Self for chaining)
    fn accuracy(&mut self, accuracy: f64) -> &mut Self {
        self.accuracy = Some(accuracy);
        self
    }

    /// Set number of misses (returns Self for chaining)
    fn misses(&mut self, n_misses: usize) -> &mut Self {
        self.n_misses = Some(n_misses);
        self
    }

    /// Legacy alias for misses()
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
    ///
    /// Returns PerformanceAttributes with PP and mode-specific values
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

// ==================== DIFFICULTY ATTRIBUTES IMPLEMENTATION ====================

#[pymethods]
impl PyDifficultyAttributes {
    // ==================== GENERAL GETTERS ====================

    /// Star rating
    #[getter]
    fn stars(&self) -> f64 {
        self.inner.stars()
    }

    /// Maximum combo
    #[getter]
    fn max_combo(&self) -> usize {
        self.inner.max_combo()
    }

    // ==================== OSU!STANDARD GETTERS ====================

    /// Aim difficulty (osu! only)
    #[getter]
    fn aim(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.aim),
            _ => None,
        }
    }

    /// Speed difficulty (osu! only)
    #[getter]
    fn speed(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.speed),
            _ => None,
        }
    }

    /// Flashlight difficulty (osu! only)
    #[getter]
    fn flashlight(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.flashlight),
            _ => None,
        }
    }

    /// Slider factor (osu! only)
    #[getter]
    fn slider_factor(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.slider_factor),
            _ => None,
        }
    }

    /// Speed note count (osu! only)
    #[getter]
    fn speed_note_count(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.speed_note_count),
            _ => None,
        }
    }

    /// Number of circles (osu! only)
    #[getter]
    fn n_circles(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.n_circles),
            _ => None,
        }
    }

    /// Number of sliders (osu! only)
    #[getter]
    fn n_sliders(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.n_sliders),
            _ => None,
        }
    }

    /// Number of spinners (osu! only)
    #[getter]
    fn n_spinners(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.n_spinners),
            _ => None,
        }
    }

    /// Local SR per minute (osu! only)
    #[getter]
    fn local_sr_per_minute(&self) -> Option<Vec<f64>> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.local_sr_per_minute.clone()),
            _ => None,
        }
    }

    /// Local SR per 15 seconds (osu! only)
    #[getter]
    fn local_sr_per_15s(&self) -> Option<Vec<f64>> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.local_sr_per_15s.clone()),
            _ => None,
        }
    }

    // ==================== UNIVERSAL DIFFICULTY GETTERS ====================

    /// Approach rate (osu!, catch)
    #[getter]
    fn ar(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.ar),
            DifficultyAttributes::Catch(attrs) => Some(attrs.ar),
            _ => None,
        }
    }

    /// Overall difficulty (osu! only)
    #[getter]
    fn od(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.od),
            _ => None,
        }
    }

    /// Health drain (osu! only)
    #[getter]
    fn hp(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.hp),
            _ => None,
        }
    }

    // ==================== TAIKO GETTERS ====================

    /// Stamina difficulty (taiko only)
    #[getter]
    fn stamina(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.stamina),
            _ => None,
        }
    }

    /// Rhythm difficulty (taiko only)
    #[getter]
    fn rhythm(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.rhythm),
            _ => None,
        }
    }

    /// Color difficulty (taiko only)
    #[getter]
    fn color(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.colour),
            _ => None,
        }
    }

    /// Peak difficulty (taiko only)
    #[getter]
    fn peak(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.peak),
            _ => None,
        }
    }

    // ==================== CATCH GETTERS ====================

    /// Number of fruits (catch only)
    #[getter]
    fn n_fruits(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Catch(attrs) => Some(attrs.n_fruits),
            _ => None,
        }
    }

    /// Number of droplets (catch only)
    #[getter]
    fn n_droplets(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Catch(attrs) => Some(attrs.n_droplets),
            _ => None,
        }
    }

    /// Number of tiny droplets (catch only)
    #[getter]
    fn n_tiny_droplets(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Catch(attrs) => Some(attrs.n_tiny_droplets),
            _ => None,
        }
    }

    // ==================== UNIVERSAL HIT WINDOW GETTER ====================

    /// Hit window (taiko, mania)
    #[getter]
    fn hit_window(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.hit_window),
            DifficultyAttributes::Mania(attrs) => Some(attrs.hit_window),
            _ => None,
        }
    }
}

// ==================== PERFORMANCE ATTRIBUTES IMPLEMENTATION ====================

#[pymethods]
impl PyPerformanceAttributes {
    // ==================== GENERAL GETTERS ====================

    /// Total performance points
    #[getter]
    fn pp(&self) -> f64 {
        self.inner.pp()
    }

    /// Star rating
    #[getter]
    fn stars(&self) -> f64 {
        self.inner.stars()
    }

    /// Maximum combo
    #[getter]
    fn max_combo(&self) -> usize {
        self.inner.max_combo()
    }

    // ==================== OSU!STANDARD GETTERS ====================

    /// Accuracy performance points (osu!, taiko)
    #[getter]
    fn pp_acc(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.pp_acc),
            PerformanceAttributes::Taiko(attrs) => Some(attrs.pp_acc),
            _ => None,
        }
    }

    /// Aim performance points (osu! only)
    #[getter]
    fn pp_aim(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.pp_aim),
            _ => None,
        }
    }

    /// Speed performance points (osu! only)
    #[getter]
    fn pp_speed(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.pp_speed),
            _ => None,
        }
    }

    /// Flashlight performance points (osu! only)
    #[getter]
    fn pp_flashlight(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.pp_flashlight),
            _ => None,
        }
    }

    // ==================== TAIKO & MANIA GETTERS ====================

    /// Difficulty performance points (taiko, mania)
    #[getter]
    fn pp_difficulty(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Taiko(attrs) => Some(attrs.pp_difficulty),
            PerformanceAttributes::Mania(attrs) => Some(attrs.pp_difficulty),
            _ => None,
        }
    }

    // ==================== EFFECTIVE MISS COUNT ====================

    /// Effective miss count (osu!, taiko)
    #[getter]
    fn effective_miss_count(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.effective_miss_count),
            PerformanceAttributes::Taiko(attrs) => Some(attrs.effective_miss_count),
            _ => None,
        }
    }

    // ==================== DIFFICULTY REFERENCE ====================

    /// Get underlying difficulty attributes
    #[getter]
    fn difficulty(&self) -> PyDifficultyAttributes {
        self.inner.difficulty_attributes().into()
    }
}

// ==================== TRAIT IMPLEMENTATIONS ====================

impl From<DifficultyAttributes> for PyDifficultyAttributes {
    fn from(inner: DifficultyAttributes) -> Self {
        Self { inner }
    }
}

impl From<PerformanceAttributes> for PyPerformanceAttributes {
    fn from(inner: PerformanceAttributes) -> Self {
        Self { inner }
    }
}

// ==================== MODULE INITIALIZATION ====================

/// Python module for cheat-ccv3-pp calculations
///
/// A Python wrapper around the cheat-ccv3-pp Rust library, providing
/// fast osu! difficulty and performance point calculations for all modes.
///
/// # Example
///
/// ```python
/// from cheat_ccv3_pp_py import Beatmap, Difficulty, Performance
///
/// # Parse a beatmap
/// beatmap = Beatmap(path="map.osu")
///
/// # Calculate star rating
/// diff = Difficulty(beatmap, mods=8)
/// stars = diff.calculate()
/// print(f"Stars: {stars.stars}")
///
/// # Calculate performance points
/// perf = Performance(beatmap, accuracy=98.5, misses=1)
/// pp = perf.calculate()
/// print(f"PP: {pp.pp}")
/// ```
#[pymodule]
fn cheat_ccv3_pp_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBeatmap>()?;
    m.add_class::<PyDifficulty>()?;
    m.add_class::<PyPerformance>()?;
    m.add_class::<PyDifficultyAttributes>()?;
    m.add_class::<PyPerformanceAttributes>()?;

    // Exception types
    m.add("ParseError", _py.get_type::<ParseError>())?;
    m.add("ArgsError", _py.get_type::<ArgsError>())?;

    Ok(())
}
