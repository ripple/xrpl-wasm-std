#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Blob {
    /// The actual length of this blob, if less than data.len()
    pub len: usize,

    /// Blob data - positioned after len for better cache locality during length checks
    pub data: [u8; 1024],
}

pub const EMPTY_BLOB: Blob = Blob {
    len: 0usize,
    data: [0u8; 1024], // TODO: Consider an optional?
};
