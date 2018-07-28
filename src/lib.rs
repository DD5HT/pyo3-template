#![feature(use_extern_macros)]

extern crate pyo3;

use pyo3::prelude::*;

/// This is the helloworld module!
#[pymodinit]
fn helloworld(py: Python, m: &PyModule) -> PyResult<()> {
    /// Add two numbers
    #[pyfn(m, "adder")]
    fn adder_py(a: i64, b: i64) -> PyResult<i64> {
        Ok(adder(a, b))
    }

    Ok(())
}

fn adder(a: i64, b: i64) -> i64 {
    a + b
}
