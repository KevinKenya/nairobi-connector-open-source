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

// File: crates/nairobi-axum-refinery/src/ingest.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-21

// nairobi-open-source-release/crates/nairobi-axum-refinery/src/ingest.rs
use io_uring::IoUring;
use libc::{c_void, MAP_ANONYMOUS, MAP_HUGETLB, MAP_HUGE_1GB, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use nairobi_protocol::{ImperialError, ImperialResult, MemoryPipe};
use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::ptr;
use tracing::{info, warn};
use zbus::zvariant::OwnedFd;

/// Allocates a 1GB Huge Page or falls back to standard pages with THP hint.
pub unsafe fn allocate_huge_page(size: usize) -> ImperialResult<*mut c_void> {
    let addr = libc::mmap(
        ptr::null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB | MAP_HUGE_1GB,
        -1,
        0,
    );

    if addr != libc::MAP_FAILED {
        info!("[MEMORY] Successfully allocated 1GB Huge Page.");
        return Ok(addr);
    }

    warn!("[MEMORY] 1GB Huge Page allocation failed. Falling back to THP.");

    let addr = libc::mmap(
        ptr::null_mut(),
        size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    );

    if addr == libc::MAP_FAILED {
        return Err(ImperialError::Ingestion(
            "Failed to allocate fallback memory".into(),
        ));
    }

    libc::madvise(addr, size, libc::MADV_HUGEPAGE);

    Ok(addr)
}

/// The Dirac Ingestion Engine — Hardware-accelerated zero-copy ingestion.
///
/// 3-Tier Ingestion Strategy:
///   Tier 1: io_uring Read → Huge Page → write to memfd (hardware DMA path)
///   Tier 2: copy_file_range (kernel splice)
///   Tier 3: mmap fallback
pub struct DiracEngine {
    ring: IoUring,
    buffer_ptr: *mut c_void,
    buffer_size: usize,
}

unsafe impl Send for DiracEngine {}
unsafe impl Sync for DiracEngine {}

impl DiracEngine {
    pub fn new(buffer_size: usize) -> ImperialResult<Self> {
        let ring = match IoUring::builder()
            .setup_sqpoll(2000)
            .build(256)
        {
            Ok(r) => {
                info!("[DIRAC] io_uring initialized with SQPOLL.");
                r
            }
            Err(e) => {
                warn!(
                    "[WARNING] SQPOLL requires elevated privileges. Falling back to standard io_uring. ({})",
                    e
                );
                IoUring::new(256).map_err(|e| {
                    ImperialError::Ingestion(format!("Failed to init io_uring: {}", e))
                })?
            }
        };

        let buffer_ptr = unsafe { allocate_huge_page(buffer_size)? };

        Ok(Self {
            ring,
            buffer_ptr,
            buffer_size,
        })
    }

/// Ingests a file using the 3-Tier strategy:
///   Tier 1: io_uring Read into Huge Page buffer → write to memfd
///   Tier 2: copy_file_range kernel splice → memfd
///   Tier 3: mmap fallback → memfd
pub async fn ingest(&mut self, file_path: &str, _delimiter: &str, _encoding: &str) -> ImperialResult<OwnedFd> {
        let file = File::open(file_path).map_err(|e| {
            ImperialError::Ingestion(format!("Failed to open {}: {}", file_path, e))
        })?;
        let src_fd = file.as_raw_fd();
        let metadata = file
            .metadata()
            .map_err(|e| ImperialError::Ingestion(format!("Failed to get metadata: {}", e)))?;
        let file_size = metadata.len() as usize;

        if file_size > self.buffer_size {
            return Err(ImperialError::Ingestion(format!(
                "File too large ({} > {})",
                file_size, self.buffer_size
            )));
        }

        info!("[DIRAC] Initializing 3-Tier Ingestion for {} (Size: {})", file_path, file_size);

        let mut pipe = MemoryPipe::new(file_size)?;
        let dst_fd = pipe.get_fd();

        // === TIER 1: io_uring Read into Huge Page ===
        let uring_success = unsafe {
            let read_e = io_uring::opcode::Read::new(
                io_uring::types::Fd(src_fd),
                self.buffer_ptr as *mut u8,
                file_size as u32,
            )
            .build()
            .user_data(0x42);

            let mut success = false;
            if self.ring.submission().push(&read_e).is_ok() {
                match self.ring.submit_and_wait(1) {
                    Ok(_) => {
                        if let Some(cqe) = self.ring.completion().next() {
                            let bytes_read = cqe.result();
                            if bytes_read >= 0 && bytes_read as usize == file_size {
                                info!(
                                    "[DIRAC] io_uring ingestion complete. Huge Page mapped at: {:p}. {} bytes DMA'd.",
                                    self.buffer_ptr, file_size
                                );
                                // Write from Huge Page buffer into memfd
                                let buf = std::slice::from_raw_parts(
                                    self.buffer_ptr as *const u8,
                                    file_size,
                                );
                                pipe.write_and_seal(buf)?;
                                success = true;
                            } else {
                                warn!(
                                    "[DIRAC] io_uring partial/failed read: result={}, expected={}. Falling to Tier 2.",
                                    bytes_read, file_size
                                );
                            }
                        }
                    }
                    Err(e) => {
                        warn!("[DIRAC] io_uring submit_and_wait failed: {}. Falling to Tier 2.", e);
                    }
                }
            } else {
                warn!("[DIRAC] io_uring SQ full. Falling to Tier 2.");
            }
            success
        };

        if !uring_success {
            // === TIER 2: copy_file_range kernel splice ===
            let mut off_in: libc::loff_t = 0;
            let mut off_out: libc::loff_t = 0;

            let ret = unsafe {
                libc::copy_file_range(src_fd, &mut off_in, dst_fd, &mut off_out, file_size, 0)
            };

            if ret < 0 {
                let err = std::io::Error::last_os_error();
                warn!("[DIRAC] Kernel Splice failed ({}). Falling to Tier 3 (Mmap).", err);

                // === TIER 3: Mmap Zero-Copy fallback ===
                let mmap = unsafe {
                    memmap2::Mmap::map(&file).map_err(|e| {
                        ImperialError::Ingestion(format!("Mmap fallback failed: {}", e))
                    })?
                };
                pipe.write_and_seal(&mmap)?;
            } else {
                let mut total_copied = ret as usize;
                while total_copied < file_size {
                    let bytes_to_copy = std::cmp::min(1024 * 1024 * 1024, file_size - total_copied);
                    let ret = unsafe {
                        libc::copy_file_range(
                            src_fd,
                            &mut off_in,
                            dst_fd,
                            &mut off_out,
                            bytes_to_copy,
                            0,
                        )
                    };
                    if ret <= 0 {
                        if ret < 0 {
                            warn!("[DIRAC] Mid-splice failure: {}", std::io::Error::last_os_error());
                        }
                        break;
                    }
                    total_copied += ret as usize;
                }
                pipe.seal()?;
                info!("[DIRAC] Kernel Splice complete (Tier 2). Total bytes: {}", total_copied);
            }
        }

        let duped = unsafe { libc::dup(dst_fd) };
        if duped < 0 {
            return Err(ImperialError::Ingestion("Failed to dup FD".into()));
        }
        
        Ok(unsafe { OwnedFd::from_raw_fd(duped) })
    }
}

impl Drop for DiracEngine {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.buffer_ptr, self.buffer_size);
        }
    }
}

