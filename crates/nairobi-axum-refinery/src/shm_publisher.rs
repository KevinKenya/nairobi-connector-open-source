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

// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-axum-refinery/src/shm_publisher.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-08

//! iceoryx2 Shared Memory Publisher — The Data Plane.
//!
//! Publishes analytical results into a POSIX shared memory arena,
//! signaling the Hub via iceoryx2 zero-copy headers.
//! This eliminates all D-Bus GVariant serialization and kernel socket
//! buffer copies from the data path.

use iceoryx2::prelude::*;
use nairobi_protocol::arena::*;
use nairobi_protocol::{ImperialError, ImperialResult};
use tracing::info;

/// The Refinery-side iceoryx2 publisher + POSIX shm arena manager.
///
/// Architecture:
///   - iceoryx2 publish-subscribe: sends fixed-size `ArenaHeader` (zero-copy)
///   - POSIX shm_open: hosts the variable-length bulk payload data
pub struct ShmPublisher {
    _service: iceoryx2::service::port_factory::publish_subscribe::PortFactory<
        iceoryx2::service::zero_copy::Service,
        ArenaHeader,
    >,
    publisher: iceoryx2::port::publisher::Publisher<
        iceoryx2::service::zero_copy::Service,
        ArenaHeader,
    >,
    shm_ptr: *mut u8,
    shm_fd: i32,
    shm_size: usize,
    write_offset: u64,
    sequence: u64,
}

unsafe impl Send for ShmPublisher {}
unsafe impl Sync for ShmPublisher {}

impl ShmPublisher {
    /// Initialize iceoryx2 service + mmap the bulk POSIX shm arena.
    ///
    /// Returns `Err` if iceoryx2 or POSIX shm initialization fails.
    /// The caller should catch this and fall back to D-Bus GVariant.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let service_name = ServiceName::new(ARENA_SERVICE_NAME)
            .map_err(|e| format!("Invalid iceoryx2 service name: {:?}", e))?;

        let service = zero_copy::Service::new(&service_name)
            .publish_subscribe()
            .open_or_create::<ArenaHeader>()
            .map_err(|e| format!("iceoryx2 service creation failed: {:?}", e))?;

        let publisher = service
            .publisher()
            .create()
            .map_err(|e| format!("iceoryx2 publisher creation failed: {:?}", e))?;

        // Create POSIX shared memory region for bulk data
        let shm_fd = unsafe {
            libc::shm_open(
                ARENA_SHM_NAME.as_ptr() as *const libc::c_char,
                libc::O_CREAT | libc::O_RDWR,
                0o600,
            )
        };

        if shm_fd < 0 {
            return Err(format!(
                "shm_open failed: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        // Set the arena size
        let ret = unsafe { libc::ftruncate(shm_fd, ARENA_MAX_SIZE as libc::off_t) };
        if ret < 0 {
            unsafe { libc::close(shm_fd) };
            return Err(format!(
                "ftruncate failed: {}",
                std::io::Error::last_os_error()
            )
            .into());
        }

        // mmap the arena
        let shm_ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                ARENA_MAX_SIZE,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                shm_fd,
                0,
            )
        };

        if shm_ptr == libc::MAP_FAILED {
            unsafe { libc::close(shm_fd) };
            return Err(format!("mmap failed: {}", std::io::Error::last_os_error()).into());
        }

        info!(
            "[ICEORYX2] Publisher initialized. Arena: {} ({} MB), Service: {}",
            std::str::from_utf8(&ARENA_SHM_NAME[1..ARENA_SHM_NAME.len() - 1]).unwrap_or("?"),
            ARENA_MAX_SIZE / (1024 * 1024),
            ARENA_SERVICE_NAME
        );

        Ok(Self {
            _service: service,
            publisher,
            shm_ptr: shm_ptr as *mut u8,
            shm_fd,
            shm_size: ARENA_MAX_SIZE,
            write_offset: 0,
            sequence: 0,
        })
    }

    /// Publish result bytes: write to shm arena, send header via iceoryx2.
    ///
    /// The data is written into the POSIX shm arena at the current offset,
    /// and a fixed-size ArenaHeader is published via iceoryx2 (zero kernel copies).
    pub fn publish(&mut self, data: &[u8], payload_type: PayloadType) -> ImperialResult<()> {
        let required = self.write_offset as usize + data.len();
        if required > self.shm_size {
            // Reset offset (circular arena)
            self.write_offset = 0;
            if data.len() > self.shm_size {
                return Err(ImperialError::SystemicSeizure(format!(
                    "Payload too large for arena: {} > {}",
                    data.len(),
                    self.shm_size
                )));
            }
        }

        let offset = self.write_offset;

        // Copy data into shm arena at offset
        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.shm_ptr.add(offset as usize),
                data.len(),
            );
        }

        // Publish header via iceoryx2 (zero-copy loan pattern)
        let sample = self.publisher.loan_uninit().map_err(|e| {
            ImperialError::SystemicSeizure(format!("iceoryx2 loan failed: {:?}", e))
        })?;

        let sample = sample.write_payload(ArenaHeader {
            sequence_id: self.sequence,
            offset,
            length: data.len() as u64,
            payload_type,
        });

        sample.send().map_err(|e| {
            ImperialError::SystemicSeizure(format!("iceoryx2 send failed: {:?}", e))
        })?;

        info!(
            "[ICEORYX2] Published seq={}, offset={}, len={}, type={:?}. Zero kernel copies.",
            self.sequence, offset, data.len(), payload_type
        );

        self.write_offset += data.len() as u64;
        // Align to 8-byte boundary for next write
        self.write_offset = (self.write_offset + 7) & !7;
        self.sequence += 1;

        Ok(())
    }

    /// Reset the write offset (for arena recycling).
    pub fn reset(&mut self) {
        self.write_offset = 0;
    }
}

impl Drop for ShmPublisher {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.shm_ptr as *mut libc::c_void, self.shm_size);
            libc::close(self.shm_fd);
            // Unlink the shared memory region
            libc::shm_unlink(ARENA_SHM_NAME.as_ptr() as *const libc::c_char);
        }
        info!("[ICEORYX2] Publisher shut down. Arena unlinked.");
    }
}
