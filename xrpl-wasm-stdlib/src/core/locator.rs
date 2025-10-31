//! Builder for nested field access locators.
//!
//! Locators encode a path to a nested field (sfields and array indices) in a compact
//! binary format understood by the host. Use it to access fields like `Memos[0].MemoType`.
//!
//! Example
//! ```no_run
//! use xrpl_wasm_stdlib::core::locator::Locator;
//! use xrpl_wasm_stdlib::sfield;
//! let mut l = Locator::new();
//! l.pack(sfield::Memos);
//! l.pack(0);
//! l.pack(sfield::MemoType);
//! # let _ = (l.len() >= 3);
//! ```

use core::mem::MaybeUninit;

/// The size of the buffer, in bytes, to use for any new locator
const LOCATOR_BUFFER_SIZE: usize = 64;

// /// A Locator may only pack this many levels deep in an object hierarchy (inclusive of the first
// /// field)
// const MAX_DEPTH: u8 = 12; // 1 byte for slot; 5 bytes for each packed object.

/// A Locator allows a WASM developer located any field in any object (even nested fields) by
/// specifying a `slot_num` (1 byte); a `locator_field_type` (1 byte); then one of an `sfield` (4
/// bytes) or an `index` (4 bytes).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Locator {
    // The first packed value is 6 bytes; All nested/packed values are 5 bytes; so 64 bytes allow
    // 12 nested levels of access.
    buffer: [u8; LOCATOR_BUFFER_SIZE],

    /// An index into `buffer` where the next packing operation can be stored.
    cur_buffer_index: usize,
}

impl Default for Locator {
    fn default() -> Self {
        Self::new()
    }
}

impl Locator {
    /// Create a new Locator using an unsigned 8-bit slot number. Valid slots are 0 to 255.
    pub fn new_with_slot(slot_num: u8) -> Locator {
        let mut buffer = MaybeUninit::<[u8; 64]>::uninit();
        unsafe {
            buffer.as_mut_ptr().cast::<u8>().write(slot_num);
        }
        Self {
            buffer: unsafe { buffer.assume_init() },
            cur_buffer_index: 1,
        }
    }

    /// Create a new Locator. Valid slots are 0 to 255.
    pub fn new() -> Locator {
        let mut buffer = MaybeUninit::<[u8; 64]>::uninit();
        // Initialize only the first byte to 0 for safety
        unsafe {
            buffer.as_mut_ptr().cast::<u8>().write(0);
        }
        Self {
            buffer: unsafe { buffer.assume_init() },
            cur_buffer_index: 0,
        }
    }

    pub fn pack(&mut self, sfield_or_index: i32) -> bool {
        if self.cur_buffer_index + 4 > LOCATOR_BUFFER_SIZE {
            return false;
        }

        let value_bytes: [u8; 4] = sfield_or_index.to_le_bytes();
        self.buffer[self.cur_buffer_index..self.cur_buffer_index + 4].copy_from_slice(&value_bytes);
        self.cur_buffer_index += 4;

        true
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.buffer.as_ptr()
    }

    pub fn num_packed_bytes(&self) -> usize {
        self.cur_buffer_index
    }

    pub fn len(&self) -> usize {
        self.cur_buffer_index
    }

    pub fn is_empty(&self) -> bool {
        self.cur_buffer_index == 0
    }

    pub fn repack_last(&mut self, sfield_or_index: i32) -> bool {
        self.cur_buffer_index -= 4;

        let value_bytes: [u8; 4] = sfield_or_index.to_le_bytes();
        self.buffer[self.cur_buffer_index..self.cur_buffer_index + 4].copy_from_slice(&value_bytes);
        self.cur_buffer_index += 4;

        true
    }
}
