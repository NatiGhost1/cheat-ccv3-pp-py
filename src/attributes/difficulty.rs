/// Difficulty calculation results
///
/// Attributes returned from difficulty calculations containing mode-specific data.

use cheat_ccv3_pp::DifficultyAttributes;
use pyo3::prelude::*;

/// Result of difficulty calculation
///
/// Contains star rating and mode-specific difficulty metrics.
#[pyclass(name = "DifficultyAttributes")]
pub struct PyDifficultyAttributes {
    pub(crate) inner: DifficultyAttributes,
}

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

impl From<DifficultyAttributes> for PyDifficultyAttributes {
    fn from(inner: DifficultyAttributes) -> Self {
        Self { inner }
    }
}
