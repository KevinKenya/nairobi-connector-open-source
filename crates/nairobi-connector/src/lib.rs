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

// crates/nairobi-connector/src/lib.rs
// Author: Kevin Chege, Location: Nairobi, Date: 21st May 2026

//! Nairobi Connector — AT-SPI2 semantic bridge + MCP server.
//!
//! Provides Computer Use without pixels by exposing a compressed TOON
//! representation of the accessibility tree and semantic tools to LLMs.

pub mod action;
pub mod engine;
pub mod error;
pub mod safety;
pub mod server;
pub mod session;
pub mod toon;
pub mod toon_bridge;

pub use error::{NeuralError, Result};
pub use safety::WindowLock;
pub use session::NeuralSession;
pub use server::NairobiServer;