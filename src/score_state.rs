/// Score state and hit results
///
/// Represents the hit results for a score submission.

use pyo3::prelude::*;

/// Score state with hit result breakdown
///
/// Comprehensive hit result data for a score, allowing flexible
/// specification of accuracy through individual hit counts.
#[pyclass(name = "ScoreState")]
#[derive(Clone, Debug)]
pub struct PyScoreState {
    /// Maximum combo achieved
    pub combo: usize,
    /// Number of 300 hit results (or Geki in taiko)
    pub n_300: usize,
    /// Number of 100 hit results (or Katsu in taiko)
    pub n_100: usize,
    /// Number of 50 hit results
    pub n_50: usize,
    /// Number of misses
    pub misses: usize,
}

#[pymethods]
impl PyScoreState {
    #[new]
    fn new(combo: usize, n_300: usize, n_100: usize, n_50: usize, misses: usize) -> Self {
        Self {
            combo,
            n_300,
            n_100,
            n_50,
            misses,
        }
    }

    #[getter]
    fn combo(&self) -> usize {
        self.combo
    }

    #[getter]
    fn n_300(&self) -> usize {
        self.n_300
    }

    #[getter]
    fn n_100(&self) -> usize {
        self.n_100
    }

    #[getter]
    fn n_50(&self) -> usize {
        self.n_50
    }

    #[getter]
    fn misses(&self) -> usize {
        self.misses
    }
}
