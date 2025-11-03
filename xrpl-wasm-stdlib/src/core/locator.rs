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
        let mut buffer: [u8; 64] = [0; 64];
        buffer[0] = slot_num;
        Self {
            buffer,
            cur_buffer_index: 1,
        }
    }

    /// Create a new Locator. Valid slots are 0 to 255.
    pub fn new() -> Locator {
        Self {
            buffer: [0; 64],
            cur_buffer_index: 0,
        }
    }

    pub fn pack(&mut self, sfield_or_index: i32) -> bool {
        if self.cur_buffer_index + 4 > LOCATOR_BUFFER_SIZE {
            return false;
        }

        let value_bytes: [u8; 4] = sfield_or_index.to_le_bytes();

        for byte in value_bytes.iter() {
            match self.buffer.get_mut(self.cur_buffer_index) {
                Some(b) => *b = *byte,
                None => return false,
            }
            self.cur_buffer_index += 1;
        }

        true
    }

    pub fn get_addr(&self) -> *const u8 {
        self.buffer.as_ptr()
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

        for byte in value_bytes.iter() {
            match self.buffer.get_mut(self.cur_buffer_index) {
                Some(b) => *b = *byte,
                None => return false,
            }
            self.cur_buffer_index += 1;
        }

        true
    }
}
