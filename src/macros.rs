/// Macros for reducing boilerplate in PyO3 bindings
///
/// Helper macros for common patterns in Python class implementations.

/// Extract optional keyword argument with type checking
#[macro_export]
macro_rules! extract_kwarg {
    ($kwargs:expr, $key:expr, $type:ty) => {{
        $kwargs
            .get_item($key)
            .and_then(|v| v.extract::<$type>().ok())
    }};
}

/// Convert Rust error to Python ValueError
#[macro_export]
macro_rules! pyerr_value {
    ($msg:expr) => {
        pyo3::PyErr::new::<pyo3::exceptions::PyValueError, _>($msg)
    };
}
