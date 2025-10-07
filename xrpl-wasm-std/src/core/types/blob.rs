#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct Blob {
    pub data: [u8; 1024],

    /// The actual length of this blob, if less than data.len()
    pub len: usize,
}

pub const EMPTY_BLOB: Blob = Blob {
    data: [0u8; 1024], // TODO: Consider an optional?
    len: 0usize,
};
