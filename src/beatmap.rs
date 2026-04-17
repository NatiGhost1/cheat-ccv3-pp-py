/// Beatmap file parsing and representation
///
/// Wraps cheat_ccv3_pp::Beatmap for Python use.

use cheat_ccv3_pp::Beatmap as RustBeatmap;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use crate::error::ParseError;

/// Beatmap parsed from .osu file
///
/// Load beatmap data from a .osu format file. Can be constructed from
/// a file path, raw string content, or bytes.
///
/// # Example
/// ```python
/// from cheat_ccv3_pp_py import Beatmap
/// beatmap = Beatmap(path="map.osu")
/// print(f"AR: {beatmap.ar}")
/// ```
#[pyclass(name = "Beatmap")]
#[derive(Clone)]
pub struct PyBeatmap {
    pub(crate) inner: RustBeatmap,
}

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
            RustBeatmap::from_path(path).map_err(|err| {
                ParseError::new_err(format!("Failed to parse beatmap from path: {err}"))
            })?
        } else if let Some(content) = extract_kwarg!(kwargs, "content", &str) {
            RustBeatmap::from_bytes(content.as_bytes()).map_err(|err| {
                ParseError::new_err(format!("Failed to parse beatmap from content: {err}"))
            })?
        } else if let Ok(bytes_obj) = extract_kwarg!(kwargs, "bytes", &PyBytes) {
            RustBeatmap::from_bytes(bytes_obj.as_bytes()).map_err(|err| {
                ParseError::new_err(format!("Failed to parse beatmap from bytes: {err}"))
            })?
        } else {
            return Err(ParseError::new_err(
                "Beatmap requires one of: 'path' (str), 'content' (str), or 'bytes' (bytes-like)",
            ));
        };

        Ok(PyBeatmap { inner: beatmap })
    }

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
}
