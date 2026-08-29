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

// File: crates/nairobi-protocol/src/arena.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

//! Shared memory arena types for the iceoryx2 data plane.
//!
//! The `ArenaHeader` is published via iceoryx2 (zero-copy) and contains
//! metadata pointing to the actual variable-length payload in a separate
//! POSIX shared memory region (`/dev/shm`).
//!
//! This hybrid approach is necessary because iceoryx2's publish-subscribe
//! requires fixed-size `#[repr(C)]` types, while our analytical results
//! (JSON strings, variable-length anomalies) are dynamically sized.
//!
//! Architecture:
//!   iceoryx2 channel → fixed ArenaHeader (offset, length, type)
//!   POSIX shm_open   → bulk data arena (up to 64MB)

/// Fixed-size header published via iceoryx2.
/// The actual payload bytes live in a separate POSIX shm region.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ArenaHeader {
    /// Unique monotonic ID for this result
    pub sequence_id: u64,
    /// Byte offset into the shared memory region
    pub offset: u64,
    /// Length of the payload in bytes
    pub length: u64,
    /// Payload type tag
    pub payload_type: PayloadType,
}

/// Payload type discriminator for the arena.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PayloadType {
    /// Serialized JSON result (FusedAnalyticsResult, DistilledAnalytics, etc.)
    JsonResult = 0,
    /// Arrow IPC buffer (future use)
    ArrowIpc = 1,
    /// Raw CSV bytes
    CsvBytes = 2,
    /// memfd handle reference (fallback)
    MemfdHandle = 3,
}

/// Service name for the iceoryx2 publish-subscribe channel.
pub const ARENA_SERVICE_NAME: &str = "nairobi_os_arena";

/// Name of the POSIX shared memory region for bulk data.
/// Must start with '/' for shm_open compatibility.
pub const ARENA_SHM_NAME: &[u8] = b"/nairobi_os_bulk_arena\0";

/// Maximum arena size (64MB).
pub const ARENA_MAX_SIZE: usize = 64 * 1024 * 1024;
