/// Game modifier handling
///
/// Parsing and manipulation of game modifiers.

use pyo3::prelude::*;

/// Game modifiers bitmask
///
/// Represents osu! game mods encoded as a bitmask integer.
/// Individual mods can be combined using bitwise operations.
#[pyclass(name = "GameMods")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PyGameMods {
    /// The mods value as a bitflag
    pub bits: u32,
}

#[pymethods]
impl PyGameMods {
    #[new]
    fn new(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the mods bitflag value
    #[getter]
    fn bits(&self) -> u32 {
        self.bits
    }

    /// Check if a specific mod is enabled
    fn contains(&self, mod_bits: u32) -> bool {
        (self.bits & mod_bits) == mod_bits
    }

    /// Add mods to the current set
    fn add(&mut self, mod_bits: u32) {
        self.bits |= mod_bits;
    }

    /// Remove mods from the current set
    fn remove(&mut self, mod_bits: u32) {
        self.bits &= !mod_bits;
    }

    fn __repr__(&self) -> String {
        format!("GameMods({})", self.bits)
    }
}

// Common mod constants
pub const NO_MOD: u32 = 0;
pub const NF: u32 = 1;
pub const EZ: u32 = 2;
pub const TD: u32 = 4;
pub const HD: u32 = 8;
pub const HR: u32 = 16;
pub const SD: u32 = 32;
pub const DT: u32 = 64;
pub const RL: u32 = 128;
pub const HT: u32 = 256;
pub const NC: u32 = 512;
pub const FL: u32 = 1024;
pub const SO: u32 = 2048;
pub const AU: u32 = 4096;
pub const AP: u32 = 8192;
pub const CN: u32 = 16384;
