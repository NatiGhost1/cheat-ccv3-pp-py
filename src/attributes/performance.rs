/// Performance calculation results
///
/// Attributes returned from performance calculations containing mode-specific PP data.

use cheat_ccv3_pp::PerformanceAttributes;
use pyo3::prelude::*;

use crate::attributes::PyDifficultyAttributes;

/// Result of performance calculation
///
/// Contains performance points and mode-specific PP metrics.
#[pyclass(name = "PerformanceAttributes")]
pub struct PyPerformanceAttributes {
    pub(crate) inner: PerformanceAttributes,
}

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

impl From<PerformanceAttributes> for PyPerformanceAttributes {
    fn from(inner: PerformanceAttributes) -> Self {
        Self { inner }
    }
}
