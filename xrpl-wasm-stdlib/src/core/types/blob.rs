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

impl Blob {
    /// Creates a new Blob from a buffer and length.
    pub fn new(data: [u8; 1024], len: usize) -> Self {
        Blob { data, len }
    }

    /// Returns a slice of the actual data (only the valid bytes).
    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }
}
