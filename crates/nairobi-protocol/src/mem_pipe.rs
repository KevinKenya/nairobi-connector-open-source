// File: /home/KevinKenya/nairobi-connector-open-source/crates/nairobi-protocol/src/mem_pipe.rs
// Author: Kevin Chege. Location: Nairobi
// Date: 2026-05-06

// nairobi-open-source-release/crates/nairobi-protocol/src/mem_pipe.rs
//! `MemoryPipe` — Zero-copy anonymous memory via `memfd_create`.
//!
//! Implements the "Forge and Lock" pattern:
//! 1. **Allocate:** Create anonymous RAM via `memfd_create`.
//! 2. **Forge:** Map writable, write data, unmap.
//! 3. **Lock:** Apply seals (`F_SEAL_WRITE`, `F_SEAL_SHRINK`).
//! 4. **Distribute:** Expose as shared read-only slice or raw FD for D-Bus transmission.
//!
//! This is the Constitutional memory primitive shared across all microservices.

use crate::error::{ImperialError, ImperialResult};
use memfd::{FileSeal, Memfd, MemfdOptions};
use memmap2::Mmap;
use std::os::unix::io::AsRawFd;

/// Handles anonymous memory-mapped files via `memfd_create`.
pub struct MemoryPipe {
    memfd: Memfd,
    size: usize,
    read_mapping: Option<Mmap>,
}

impl MemoryPipe {
    /// Allocates a new `MemoryPipe` with the given size.
    pub fn new(size: usize) -> ImperialResult<Self> {
        let memfd = MemfdOptions::default()
            .allow_sealing(true)
            .create("sovereign_pipe")
            .map_err(|e| ImperialError::SystemicSeizure(format!("Failed to create memfd: {}", e)))?;

        memfd
            .as_file()
            .set_len(size as u64)
            .map_err(|e| {
                ImperialError::SystemicSeizure(format!("Failed to set memfd length: {}", e))
            })?;

        Ok(Self {
            memfd,
            size,
            read_mapping: None,
        })
    }

    /// Writes data to the `MemoryPipe` and seals it.
    ///
    /// Uses `mmap` to write directly to the anonymous RAM.
    /// Once written, the memory is sealed against further writes and shrinking.
    /// Returns the virtual address of the Forge mapping for verification.
    pub fn write_and_seal(&mut self, data: &[u8]) -> ImperialResult<*const u8> {
        if data.len() > self.size {
            return Err(ImperialError::SystemicSeizure(format!(
                "Data size ({}) exceeds pipe capacity ({})",
                data.len(),
                self.size
            )));
        }

        let forge_ptr: *const u8;

        // 1. Forge: Map RW and write data
        {
            let mut mmap = unsafe {
                memmap2::MmapMut::map_mut(self.memfd.as_file()).map_err(|e| {
                    ImperialError::SystemicSeizure(format!("Failed to mmap memfd (RW): {}", e))
                })?
            };

            forge_ptr = mmap.as_ptr();
            mmap[..data.len()].copy_from_slice(data);

            mmap.flush().map_err(|e| {
                ImperialError::SystemicSeizure(format!("Failed to flush mmap: {}", e))
            })?;
        } // mmap is dropped here (munmap)

        // 2. Lock: Apply seals
        self.memfd
            .add_seals(&[FileSeal::SealWrite, FileSeal::SealShrink])
            .map_err(|e| {
                ImperialError::SystemicSeizure(format!("Failed to seal memfd: {}", e))
            })?;

        Ok(forge_ptr)
    }

    /// Returns a shared read-only slice of the memory.
    ///
    /// Maps the memfd if it hasn't been mapped yet.
    /// Shared Read-Only mapping ensuring zero-copy access to physical silicon.
    pub fn as_slice(&mut self) -> ImperialResult<&[u8]> {
        if self.read_mapping.is_none() {
            let mmap = unsafe {
                Mmap::map(self.memfd.as_file()).map_err(|e| {
                    ImperialError::SystemicSeizure(format!("Failed to mmap memfd (RO): {}", e))
                })?
            };
            self.read_mapping = Some(mmap);
        }

        Ok(&self.read_mapping.as_ref().unwrap()[..])
    }

    /// Static method to map a raw File Descriptor into a read-only slice.
    ///
    /// This is the "Zero-Copy Mapper" consumer method for receiving FDs
    /// across D-Bus. The caller must ensure the FD is a valid, sealed memfd.
    ///
    /// # Safety
    /// The caller must guarantee that `fd` is a valid, open file descriptor
    /// pointing to a sealed memfd.
    pub unsafe fn map_fd(fd: i32) -> ImperialResult<Mmap> {
        use std::os::unix::io::FromRawFd;
        let file = std::fs::File::from_raw_fd(fd);

        let mmap = Mmap::map(&file).map_err(|e| {
            ImperialError::SystemicSeizure(format!("Failed to map FD {}: {}", fd, e))
        })?;

        Ok(mmap)
    }

    /// Returns the raw file descriptor for D-Bus transmission (`h` type).
    pub fn get_fd(&self) -> i32 {
        self.memfd.as_raw_fd()
    }

    /// Returns the size of the buffer.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Manually seals the MemoryPipe.
    ///
    /// This is used when the data has been written to the memfd via
    /// external means (e.g., libc::copy_file_range) instead of the
    /// internal `write_and_seal` method.
    pub fn seal(&self) -> ImperialResult<()> {
        self.memfd
            .add_seals(&[FileSeal::SealWrite, FileSeal::SealShrink])
            .map_err(|e| {
                ImperialError::SystemicSeizure(format!("Failed to seal memfd: {}", e))
            })?;
        Ok(())
    }
}
