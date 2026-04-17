use cheat_ccv3_pp::{Beatmap, BeatmapExt, DifficultyAttributes, PerformanceAttributes};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

create_exception!(cheat_ccv3_pp_py, ParseError, pyo3::exceptions::PyValueError);

#[pyclass(name = "Beatmap")]
#[derive(Clone)]
pub struct PyBeatmap {
    inner: Beatmap,
}

#[pyclass(name = "Difficulty")]
#[derive(Clone)]
pub struct PyDifficulty {
    map: Beatmap,
    mods: u32,
    passed_objects: Option<usize>,
    clock_rate: Option<f64>,
    is_convert: Option<bool>,
}

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

#[pyclass(name = "DifficultyAttributes")]
pub struct PyDifficultyAttributes {
    inner: DifficultyAttributes,
}

#[pyclass(name = "PerformanceAttributes")]
pub struct PyPerformanceAttributes {
    inner: PerformanceAttributes,
}

#[pymethods]
impl PyBeatmap {
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        let kwargs = kwargs.ok_or_else(|| {
            ParseError::new_err("Beatmap constructor requires keyword argument 'path', 'content', or 'bytes'")
        })?;

        let mut beatmap = None;

        for (key, value) in kwargs {
            let key = key.extract::<&str>().map_err(|_| {
                ParseError::new_err("Beatmap constructor expects string keys for kwargs")
            })?;

            match key {
                "path" => {
                    let path = value.extract::<&str>().map_err(|_| {
                        ParseError::new_err("'path' must be a string")
                    })?;
                    beatmap = Some(Beatmap::from_path(path).map_err(|err| {
                        ParseError::new_err(format!("Failed to parse beatmap from path: {err}"))
                    })?);
                }
                "content" => {
                    let content = value.extract::<&str>().map_err(|_| {
                        ParseError::new_err("'content' must be a string")
                    })?;
                    beatmap = Some(Beatmap::from_bytes(content.as_bytes()).map_err(|err| {
                        ParseError::new_err(format!("Failed to parse beatmap from content: {err}"))
                    })?);
                }
                "bytes" => {
                    let bytes = value
                        .extract::<&PyBytes>()
                        .map_err(|_| ParseError::new_err("'bytes' must be a bytes-like object"))?;
                    beatmap = Some(Beatmap::from_bytes(bytes.as_bytes()).map_err(|err| {
                        ParseError::new_err(format!("Failed to parse beatmap from bytes: {err}"))
                    })?);
                }
                _ => {
                    return Err(ParseError::new_err(
                        "Unknown Beatmap constructor argument; expected 'path', 'content', or 'bytes'",
                    ));
                }
            }
        }

        let inner = beatmap.ok_or_else(|| {
            ParseError::new_err("Beatmap constructor requires 'path', 'content', or 'bytes'")
        })?;

        Ok(PyBeatmap { inner })
    }

    #[getter]
    fn mode(&self) -> String {
        format!("{:?}", self.inner.mode)
    }

    #[getter]
    fn ar(&self) -> f32 {
        self.inner.ar
    }

    #[getter]
    fn cs(&self) -> f32 {
        self.inner.cs
    }

    #[getter]
    fn hp(&self) -> f32 {
        self.inner.hp
    }

    #[getter]
    fn od(&self) -> f32 {
        self.inner.od
    }

    #[getter]
    fn bpm(&self) -> f64 {
        self.inner.bpm()
    }

    #[getter]
    fn n_circles(&self) -> u32 {
        self.inner.n_circles
    }

    #[getter]
    fn n_sliders(&self) -> u32 {
        self.inner.n_sliders
    }

    #[getter]
    fn n_spinners(&self) -> u32 {
        self.inner.n_spinners
    }

    fn pp(&self) -> PyPerformance {
        PyPerformance::new(self.inner.clone())
    }

    fn stars(&self) -> PyDifficulty {
        PyDifficulty::new(self.inner.clone())
    }

    fn max_pp(&self, mods: u32) -> PyPerformanceAttributes {
        self.inner.max_pp(mods).into()
    }

    fn max_stars(&self, mods: u32) -> PyDifficultyAttributes {
        self.inner.stars().mods(mods).calculate().into()
    }
}

#[pymethods]
impl PyDifficulty {
    #[new]
    fn new(map: Beatmap) -> Self {
        PyDifficulty {
            map,
            mods: 0,
            passed_objects: None,
            clock_rate: None,
            is_convert: None,
        }
    }

    fn mods(&mut self, mods: u32) {
        self.mods = mods;
    }

    fn passed_objects(&mut self, passed_objects: usize) {
        self.passed_objects = Some(passed_objects);
    }

    fn clock_rate(&mut self, clock_rate: f64) {
        self.clock_rate = Some(clock_rate);
    }

    fn is_convert(&mut self, is_convert: bool) {
        self.is_convert = Some(is_convert);
    }

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

#[pymethods]
impl PyPerformance {
    #[new]
    fn new(map: Beatmap) -> Self {
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

    fn mods(&mut self, mods: u32) {
        self.mods = mods;
    }

    fn combo(&mut self, combo: usize) {
        self.combo = Some(combo);
    }

    fn accuracy(&mut self, accuracy: f64) {
        self.accuracy = Some(accuracy);
    }

    fn n_misses(&mut self, n_misses: usize) {
        self.n_misses = Some(n_misses);
    }

    fn n300(&mut self, n300: usize) {
        self.n300 = Some(n300);
    }

    fn n100(&mut self, n100: usize) {
        self.n100 = Some(n100);
    }

    fn n50(&mut self, n50: usize) {
        self.n50 = Some(n50);
    }

    fn n_katu(&mut self, n_katu: usize) {
        self.n_katu = Some(n_katu);
    }

    fn n_geki(&mut self, n_geki: usize) {
        self.n_geki = Some(n_geki);
    }

    fn passed_objects(&mut self, passed_objects: usize) {
        self.passed_objects = Some(passed_objects);
    }

    fn clock_rate(&mut self, clock_rate: f64) {
        self.clock_rate = Some(clock_rate);
    }

    fn is_convert(&mut self, is_convert: bool) {
        self.is_convert = Some(is_convert);
    }

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

#[pymethods]
impl PyDifficultyAttributes {
    #[getter]
    fn stars(&self) -> f64 {
        self.inner.stars()
    }

    #[getter]
    fn max_combo(&self) -> usize {
        self.inner.max_combo()
    }

    #[getter]
    fn aim(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.aim),
            _ => None,
        }
    }

    #[getter]
    fn speed(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.speed),
            _ => None,
        }
    }

    #[getter]
    fn flashlight(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.flashlight),
            _ => None,
        }
    }

    #[getter]
    fn slider_factor(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.slider_factor),
            _ => None,
        }
    }

    #[getter]
    fn speed_note_count(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.speed_note_count),
            _ => None,
        }
    }

    #[getter]
    fn ar(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.ar),
            DifficultyAttributes::Catch(attrs) => Some(attrs.ar),
            _ => None,
        }
    }

    #[getter]
    fn od(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.od),
            _ => None,
        }
    }

    #[getter]
    fn hp(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.hp),
            _ => None,
        }
    }

    #[getter]
    fn stamina(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.stamina),
            _ => None,
        }
    }

    #[getter]
    fn rhythm(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.rhythm),
            _ => None,
        }
    }

    #[getter]
    fn color(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.colour),
            _ => None,
        }
    }

    #[getter]
    fn peak(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.peak),
            _ => None,
        }
    }

    #[getter]
    fn hit_window(&self) -> Option<f64> {
        match &self.inner {
            DifficultyAttributes::Taiko(attrs) => Some(attrs.hit_window),
            DifficultyAttributes::Mania(attrs) => Some(attrs.hit_window),
            _ => None,
        }
    }

    #[getter]
    fn n_fruits(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Catch(attrs) => Some(attrs.n_fruits),
            _ => None,
        }
    }

    #[getter]
    fn n_droplets(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Catch(attrs) => Some(attrs.n_droplets),
            _ => None,
        }
    }

    #[getter]
    fn n_tiny_droplets(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Catch(attrs) => Some(attrs.n_tiny_droplets),
            _ => None,
        }
    }

    #[getter]
    fn n_circles(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.n_circles),
            _ => None,
        }
    }

    #[getter]
    fn n_sliders(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.n_sliders),
            _ => None,
        }
    }

    #[getter]
    fn n_spinners(&self) -> Option<usize> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.n_spinners),
            _ => None,
        }
    }

    #[getter]
    fn local_sr_per_minute(&self) -> Option<Vec<f64>> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.local_sr_per_minute.clone()),
            _ => None,
        }
    }

    #[getter]
    fn local_sr_per_15s(&self) -> Option<Vec<f64>> {
        match &self.inner {
            DifficultyAttributes::Osu(attrs) => Some(attrs.local_sr_per_15s.clone()),
            _ => None,
        }
    }
}

#[pymethods]
impl PyPerformanceAttributes {
    #[getter]
    fn pp(&self) -> f64 {
        self.inner.pp()
    }

    #[getter]
    fn stars(&self) -> f64 {
        self.inner.stars()
    }

    #[getter]
    fn max_combo(&self) -> usize {
        self.inner.max_combo()
    }

    #[getter]
    fn pp_acc(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.pp_acc),
            PerformanceAttributes::Taiko(attrs) => Some(attrs.pp_acc),
            _ => None,
        }
    }

    #[getter]
    fn pp_aim(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.pp_aim),
            _ => None,
        }
    }

    #[getter]
    fn pp_speed(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.pp_speed),
            _ => None,
        }
    }

    #[getter]
    fn pp_flashlight(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.pp_flashlight),
            _ => None,
        }
    }

    #[getter]
    fn pp_difficulty(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Taiko(attrs) => Some(attrs.pp_difficulty),
            PerformanceAttributes::Mania(attrs) => Some(attrs.pp_difficulty),
            _ => None,
        }
    }

    #[getter]
    fn effective_miss_count(&self) -> Option<f64> {
        match &self.inner {
            PerformanceAttributes::Osu(attrs) => Some(attrs.effective_miss_count),
            PerformanceAttributes::Taiko(attrs) => Some(attrs.effective_miss_count),
            _ => None,
        }
    }

    #[getter]
    fn difficulty(&self) -> PyDifficultyAttributes {
        self.inner.difficulty_attributes().into()
    }
}

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

#[pymodule]
fn cheat_ccv3_pp_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBeatmap>()?;
    m.add_class::<PyDifficulty>()?;
    m.add_class::<PyPerformance>()?;
    m.add_class::<PyDifficultyAttributes>()?;
    m.add_class::<PyPerformanceAttributes>()?;
    m.add("ParseError", _py.get_type::<ParseError>())?;
    Ok(())
}
