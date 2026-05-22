// Copyright 2026 Kevin Chege
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// File: ~/nairobi-connector-open-source/crates/nairobi-python/src/lib.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

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
