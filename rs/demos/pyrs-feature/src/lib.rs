use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn feature(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn pyrs_feature(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(feature, m)?)?;
    Ok(())
}

