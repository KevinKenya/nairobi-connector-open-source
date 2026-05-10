// File: /home/chege/nairobi-connector-open-source/crates/nairobi-python/src/data_bridge.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-08

// nairobi-open-source-release/crates/nairobi-python/src/data_bridge.rs
//
// v0.1.2 REFIT: The Hub client now handles SHM_READY routing internally.
// The Python bridge API is unchanged — the iceoryx2 data plane is transparent
// to Python consumers. All functions still return JSON strings.
//
// LATENCY FIX: All bridge functions use a persistent Tokio runtime and
// cached D-Bus connection instead of creating new ones per call. This
// eliminates ~300-400ms of overhead per invocation.
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use crate::types::{ensure_client, get_registry, get_runtime, map_imperial_error};

#[pyfunction]
pub fn ingest(py: Python, file_path: String) -> PyResult<String> {
    let rt = get_runtime()?;

    let handle_id = py.allow_threads(|| {
        rt.block_on(async {
            let client = ensure_client().await
                .map_err(map_imperial_error)?;
            let fd = client.ingest(&file_path).await
                .map_err(map_imperial_error)?;

            let uuid = uuid::Uuid::new_v4().to_string();
            let registry = get_registry();
            registry.insert(uuid.clone(), fd).await;
            Ok::<String, PyErr>(uuid)
        })
    })?;

    Ok(handle_id)
}

#[pyfunction]
pub fn sql_query(py: Python, handle_id: String, query: String) -> PyResult<String> {
    let rt = get_runtime()?;

    py.allow_threads(|| {
        rt.block_on(async {
            let registry = get_registry();
            let fd = registry.get(&handle_id).await
                .ok_or_else(|| PyRuntimeError::new_err("Invalid handle ID"))?;

            let client = ensure_client().await
                .map_err(map_imperial_error)?;
            let result_fd = client.sql_query(fd, &query).await
                .map_err(map_imperial_error)?;

            let new_uuid = uuid::Uuid::new_v4().to_string();
            registry.insert(new_uuid.clone(), result_fd).await;
            Ok::<String, PyErr>(new_uuid)
        })
    })
}

#[pyfunction]
pub fn crunch(py: Python, handle_id: String, query: String) -> PyResult<String> {
    let rt = get_runtime()?;

    py.allow_threads(|| {
        rt.block_on(async {
            let registry = get_registry();
            let fd = registry.get(&handle_id).await
                .ok_or_else(|| PyRuntimeError::new_err("Invalid handle ID"))?;

            let client = ensure_client().await
                .map_err(map_imperial_error)?;

            // The Hub client handles SHM_READY routing internally.
            // analyze() returns a deserialized DistilledAnalytics struct.
            let analytics = client.analyze(fd, &query).await
                .map_err(map_imperial_error)?;

            serde_json::to_string(&analytics)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    })
}

#[pyfunction]
pub fn correlate(py: Python, handle_id: String, query: String) -> PyResult<String> {
    let rt = get_runtime()?;

    py.allow_threads(|| {
        rt.block_on(async {
            let registry = get_registry();
            let fd = registry.get(&handle_id).await
                .ok_or_else(|| PyRuntimeError::new_err("Invalid handle ID"))?;

            let client = ensure_client().await
                .map_err(map_imperial_error)?;

            // The Hub client handles SHM_READY routing internally.
            let correlation = client.correlation(fd, &query).await
                .map_err(map_imperial_error)?;

            serde_json::to_string(&correlation)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    })
}

/// Fused pipeline: ingest → crunch → correlate in a single D-Bus round trip.
/// Returns a JSON string containing all analytics and correlation results.
/// This is the highest-performance path for the full pipeline.
/// Data flows through iceoryx2 shared memory when available.
#[pyfunction]
pub fn pipeline(py: Python, file_path: String, column: String, corr_columns: String) -> PyResult<String> {
    let rt = get_runtime()?;

    py.allow_threads(|| {
        rt.block_on(async {
            let client = ensure_client().await
                .map_err(map_imperial_error)?;

            // The Hub client handles SHM_READY routing internally.
            let result = client.ingest_crunch_correlate(&file_path, &column, &corr_columns).await
                .map_err(map_imperial_error)?;

            serde_json::to_string(&result)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    })
}

/// Fused crunch + correlate on an already-ingested handle.
/// Single D-Bus round trip, single CSV parse.
/// Data flows through iceoryx2 shared memory when available.
#[pyfunction]
pub fn crunch_and_correlate(py: Python, handle_id: String, column: String, corr_columns: String) -> PyResult<String> {
    let rt = get_runtime()?;

    py.allow_threads(|| {
        rt.block_on(async {
            let registry = get_registry();
            let fd = registry.get(&handle_id).await
                .ok_or_else(|| PyRuntimeError::new_err("Invalid handle ID"))?;

            let client = ensure_client().await
                .map_err(map_imperial_error)?;

            // The Hub client handles SHM_READY routing internally.
            let result = client.crunch_and_correlate(fd, &column, &corr_columns).await
                .map_err(map_imperial_error)?;

            serde_json::to_string(&result)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    })
}

#[pyfunction]
pub fn get_fd(py: Python, handle_id: String) -> PyResult<i32> {
    let rt = get_runtime()?;

    let fd = py.allow_threads(|| {
        rt.block_on(async {
            let registry = get_registry();
            let owned_fd = registry.get(&handle_id).await
                .ok_or_else(|| PyRuntimeError::new_err("Invalid handle ID"))?;

            use std::os::unix::io::AsRawFd;
            let raw_fd = owned_fd.as_raw_fd();
            // Duplicate the FD so it stays open after owned_fd is dropped
            let leaked_fd = unsafe { libc::dup(raw_fd) };
            Ok::<i32, PyErr>(leaked_fd)
        })
    })?;

    Ok(fd)
}

#[pyfunction]
pub fn free(py: Python, handle_id: String) -> PyResult<()> {
    let rt = get_runtime()?;

    py.allow_threads(|| {
        rt.block_on(async {
            let registry = get_registry();
            registry.remove(&handle_id).await
                .ok_or_else(|| PyRuntimeError::new_err("Invalid handle ID"))?;
            Ok::<(), PyErr>(())
        })
    })?;

    Ok(())
}

pub fn init_module(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ingest, m)?)?;
    m.add_function(wrap_pyfunction!(sql_query, m)?)?;
    m.add_function(wrap_pyfunction!(crunch, m)?)?;
    m.add_function(wrap_pyfunction!(correlate, m)?)?;
    m.add_function(wrap_pyfunction!(pipeline, m)?)?;
    m.add_function(wrap_pyfunction!(crunch_and_correlate, m)?)?;
    m.add_function(wrap_pyfunction!(get_fd, m)?)?;
    m.add_function(wrap_pyfunction!(free, m)?)?;
    Ok(())
}
