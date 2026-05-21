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

//! Neural error types for the Nairobi Connector semantic bridge.

use thiserror::Error;

/// Errors that can occur during AT-SPI2 operations.
#[derive(Error, Debug)]
pub enum NeuralError {
    /// D-Bus operation timed out.
    #[error("Timeout: {0}")]
    Timeout(String),

    /// zbus / AT-SPI communication error.
    #[error("Zbus error: {0}")]
    ZbusError(String),

    /// Node does not support the required AT-SPI interface.
    #[error("Interface not supported: {0}")]
    InterfaceNotSupported(String),

    /// Semantic action execution failed.
    #[error("Action failed: {0}")]
    ActionFailed(String),

    /// Element is not enabled or sensitive.
    #[error("Element disabled: {0}")]
    ElementDisabled(String),

    /// Window or node not found.
    #[error("Node not found: {0}")]
    NodeNotFound(String),

    /// RegistryLock prevented access.
    #[error("Window locked by RegistryLock: {0}")]
    WindowLocked(String),
}

/// Convenience type alias.
pub type Result<T> = std::result::Result<T, NeuralError>;

impl From<zbus::Error> for NeuralError {
    fn from(e: zbus::Error) -> Self {
        NeuralError::ZbusError(e.to_string())
    }
}

impl From<serde_json::Error> for NeuralError {
    fn from(e: serde_json::Error) -> Self {
        NeuralError::ZbusError(e.to_string())
    }
}