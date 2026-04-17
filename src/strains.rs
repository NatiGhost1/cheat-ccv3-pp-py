/// Difficulty strains and skill breakdown
///
/// Strain values representing different skill components of a beatmap.

use pyo3::prelude::*;

/// Strain values from difficulty calculation
///
/// Contains individual strain components for different skills.
#[pyclass(name = "Strains")]
#[derive(Clone, Debug)]
pub struct PyStrains {
    /// Aim strain (osu! only)
    pub aim: Option<f64>,
    /// Speed strain (osu! only)
    pub speed: Option<f64>,
    /// Flashlight strain (osu! only)
    pub flashlight: Option<f64>,
}

#[pymethods]
impl PyStrains {
    #[new]
    fn new(aim: Option<f64>, speed: Option<f64>, flashlight: Option<f64>) -> Self {
        Self {
            aim,
            speed,
            flashlight,
        }
    }

    #[getter]
    fn aim(&self) -> Option<f64> {
        self.aim
    }

    #[getter]
    fn speed(&self) -> Option<f64> {
        self.speed
    }

    #[getter]
    fn flashlight(&self) -> Option<f64> {
        self.flashlight
    }
}
