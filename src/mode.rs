/// Game mode enumeration
///
/// Represents the four osu! game modes.

use pyo3::prelude::*;

/// Osu! game modes: Standard, Taiko, Catch, Mania
#[pyclass(name = "GameMode")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameMode {
    #[pyo3(name = "Osu")]
    Osu,
    #[pyo3(name = "Taiko")]
    Taiko,
    #[pyo3(name = "Catch")]
    Catch,
    #[pyo3(name = "Mania")]
    Mania,
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Osu
    }
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Osu => "Osu",
                Self::Taiko => "Taiko",
                Self::Catch => "Catch",
                Self::Mania => "Mania",
            }
        )
    }
}
