// File: /home/chege/nairobi-connector-open-source/crates/nairobi-hub/src/shm_subscriber.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-08

//! iceoryx2 Shared Memory Subscriber — The Data Plane Consumer.
//!
//! Reads analytical results directly from the POSIX shared memory arena
//! using iceoryx2 zero-copy headers. This eliminates all D-Bus GVariant
//! deserialization and kernel socket buffer copies from the read path.
//!
//! Latency: nanoseconds (direct pointer read from mapped region).

use iceoryx2::prelude::*;
use nairobi_protocol::arena::*;
use nairobi_protocol::{ImperialError, ImperialResult};
use tracing::{info, warn};

/// The Hub-side iceoryx2 subscriber + POSIX shm reader.
///
/// Architecture:
///   - iceoryx2 subscribe: receives fixed-size `ArenaHeader` (zero-copy)
///   - POSIX shm_open (RO): reads the variable-length bulk payload data
pub struct ShmSubscriber {
    subscriber: iceoryx2::port::subscriber::Subscriber<
        iceoryx2::service::zero_copy::Service,
        ArenaHeader,
    >,
    shm_ptr: *const u8,
    shm_fd: i32,
    shm_size: usize,
}

unsafe impl Send for ShmSubscriber {}
unsafe impl Sync for ShmSubscriber {}

impl ShmSubscriber {
    /// Initialize iceoryx2 subscriber + mmap the bulk POSIX shm arena (read-only).
    ///
    /// Returns `Err` if iceoryx2 or POSIX shm initialization fails.
    /// The caller should catch this and fall back to D-Bus GVariant.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let service_name = ServiceName::new(ARENA_SERVICE_NAME)
            .map_err(|e| format!("Invalid iceoryx2 service name: {:?}", e))?;

        let service = zero_copy::Service::new(&service_name)
            .publish_subscribe()
            .open_or_create::<ArenaHeader>()
            .map_err(|e| format!("iceoryx2 service open failed: {:?}", e))?;

        let subscriber = service
            .subscriber()
            .create()
            .map_err(|e| format!("iceoryx2 subscriber creation failed: {:?}", e))?;

        // Open existing POSIX shm (read-only)
        let shm_fd = unsafe {
            libc::shm_open(
                ARENA_SHM_NAME.as_ptr() as *const libc::c_char,
                libc::O_RDONLY,
                0,
            )
        };

        if shm_fd < 0 {
            return Err(format!(
                "shm_open (RO) failed: {}. Arena may not be created yet.",
                std::io::Error::last_os_error()
            )
            .into());
        }

        // mmap read-only
        let shm_ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                ARENA_MAX_SIZE,
                libc::PROT_READ,
                libc::MAP_SHARED,
                shm_fd,
                0,
            )
        };

        if shm_ptr == libc::MAP_FAILED {
            unsafe { libc::close(shm_fd) };
            return Err(format!("mmap (RO) failed: {}", std::io::Error::last_os_error()).into());
        }

        info!(
            "[ICEORYX2] Subscriber initialized. Arena mapped (RO): {} MB. Service: {}",
            ARENA_MAX_SIZE / (1024 * 1024),
            ARENA_SERVICE_NAME
        );

        Ok(Self {
            subscriber,
            shm_ptr: shm_ptr as *const u8,
            shm_fd,
            shm_size: ARENA_MAX_SIZE,
        })
    }

    /// Receive the latest result from shared memory.
    ///
    /// Returns `Some(bytes)` if a new result is available, `None` otherwise.
    /// The read is a direct pointer dereference — nanosecond latency.
    pub fn receive(&self) -> ImperialResult<Option<Vec<u8>>> {
        match self.subscriber.receive() {
            Ok(Some(sample)) => {
                let header: &ArenaHeader = &*sample;

                // Bounds check
                let end = header.offset as usize + header.length as usize;
                if end > self.shm_size {
                    return Err(ImperialError::SystemicSeizure(format!(
                        "Arena header out of bounds: offset={}, length={}, arena_size={}",
                        header.offset, header.length, self.shm_size
                    )));
                }

                // Direct pointer read — zero copies through the kernel
                let data = unsafe {
                    std::slice::from_raw_parts(
                        self.shm_ptr.add(header.offset as usize),
                        header.length as usize,
                    )
                };

                info!(
                    "[ICEORYX2] Received seq={}, len={}, type={:?}. Direct shm read.",
                    header.sequence_id, header.length, header.payload_type
                );

                Ok(Some(data.to_vec()))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(ImperialError::SystemicSeizure(format!(
                "iceoryx2 receive failed: {:?}",
                e
            ))),
        }
    }

    /// Drain all pending samples and return the latest one.
    /// Used after receiving a "SHM_READY" signal over D-Bus.
    pub fn receive_latest(&self) -> ImperialResult<Option<Vec<u8>>> {
        let mut latest: Option<Vec<u8>> = None;
        loop {
            match self.receive()? {
                Some(data) => latest = Some(data),
                None => break,
            }
        }
        Ok(latest)
    }
}

impl Drop for ShmSubscriber {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.shm_ptr as *mut libc::c_void, self.shm_size);
            libc::close(self.shm_fd);
        }
        info!("[ICEORYX2] Subscriber shut down. Arena unmapped.");
    }
}
