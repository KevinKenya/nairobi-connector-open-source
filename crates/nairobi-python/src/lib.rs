// File: ~/nairobi-connector-open-source/crates/nairobi-python/src/lib.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-python/src/lib.rs
use pyo3::prelude::*;

mod data_bridge;
mod types;

#[pymodule]
fn _core(py: Python, m: &PyModule) -> PyResult<()> {
    // Create data submodule
    let data_module = PyModule::new(py, "data")?;
    data_bridge::init_module(data_module)?;
    m.add_submodule(data_module)?;

    Ok(())
}
