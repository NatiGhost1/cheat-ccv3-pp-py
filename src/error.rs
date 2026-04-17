/// Error types for cheat-ccv3-pp-py bindings
///
/// Custom exceptions that are exposed to Python for error handling.

use pyo3::create_exception;
use pyo3::exceptions;

/// Raised when beatmap parsing fails
pub create_exception!(cheat_ccv3_pp_py, ParseError, exceptions::PyValueError);

/// Raised when invalid arguments are passed to functions
pub create_exception!(cheat_ccv3_pp_py, ArgsError, exceptions::PyTypeError);
