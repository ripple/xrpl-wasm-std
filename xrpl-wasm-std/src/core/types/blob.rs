// Maximum size for blob fields, including FinishFunction WASM bytecode
// WASM contracts can be up to 100KB, so we use a generous buffer size
pub const BLOB_BUFFER_SIZE: usize = 102400; // 100KB

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Blob {
    pub data: [u8; BLOB_BUFFER_SIZE],

    /// The actual length of this blob, if less than data.len()
    pub len: usize,
}

pub const EMPTY_BLOB: Blob = Blob {
    data: [0u8; BLOB_BUFFER_SIZE],
    len: 0usize,
};
