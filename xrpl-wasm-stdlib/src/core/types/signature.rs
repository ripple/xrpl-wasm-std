//! Transaction signature type for XRPL.
//!
//! Signatures in XRPL can be either:
//! - 64 bytes for EdDSA (Ed25519) signatures
//! - 70-72 bytes for ECDSA (secp256k1) signatures
//!
//! This module provides a type-safe wrapper around signature data that can accommodate
//! both signature types without needing to know which algorithm was used.
//!
//! # Design: Struct Wrapper vs Type Alias
//!
//! This module uses a struct wrapper (`struct Signature(Blob<72>)`) rather than a type alias
//! (`type Signature = Blob<72>`) for several important reasons:
//!
//! ## Type Safety
//! A struct wrapper creates a distinct type that prevents accidental mixing with other
//! 72-byte blobs. This ensures that only actual signature data can be passed to functions
//! expecting signatures, preventing bugs where arbitrary binary data might be mistaken
//! for a signature.
//!
//! ## Semantic Meaning
//! The struct provides clear semantic meaning in APIs. When you see `Signature` in a
//! function signature or struct field, it's immediately clear that this represents a
//! cryptographic signature, not just any 72-byte blob.
//!
//! ## Future Extensibility
//! The struct wrapper allows adding signature-specific methods and validation in the future
//! without breaking changes. For example, we could add methods to detect signature type
//! (EdDSA vs ECDSA), validate signature length, or even perform signature verification.
//!
//! ## Trait Implementations
//! We can implement traits specifically for `Signature` that wouldn't make sense for all
//! `Blob<72>` instances, allowing signature-specific behavior while keeping the general
//! `Blob` type flexible.

use crate::core::types::blob::Blob;

/// Maximum size of a signature in bytes.
///
/// ECDSA signatures can be up to 72 bytes, which is the maximum signature size in XRPL.
/// EdDSA signatures are always 64 bytes.
pub const SIGNATURE_MAX_SIZE: usize = 72;

/// A transaction signature that can hold either EdDSA or ECDSA signatures.
///
/// This type is a **struct wrapper** around `Blob<72>` (not a type alias) that provides
/// type safety and semantic meaning for signature data. Signatures in XRPL can vary in length:
///
/// - **EdDSA (Ed25519)**: Always 64 bytes
/// - **ECDSA (secp256k1)**: 70-72 bytes (DER-encoded)
///
/// When loading signature data from the ledger, the actual signature type
/// may not be known, so this type can accommodate both.
///
/// // ## Derived Traits
///
/// - `Clone`: Reasonable for this 72-byte struct when explicit copying is needed
/// - `PartialEq, Eq`: Enable signature comparisons and use in collections
/// - `Debug`: Useful for development and debugging
///
/// Note: `Copy` is intentionally not derived due to the struct's size (72 bytes).
/// Large `Copy` types can lead to accidental expensive copies and poor performance.
/// Use `.clone()` when you need to duplicate a signature.
///
/// # Why a Struct Instead of a Type Alias?
///
/// This is implemented as `struct Signature(Blob<72>)` rather than `type Signature = Blob<72>`
/// to provide:
///
/// - **Type safety**: Prevents accidentally using arbitrary 72-byte blobs as signatures
/// - **Clear semantics**: Makes it obvious when working with signature data
/// - **Future extensibility**: Allows adding signature-specific methods without breaking changes
/// - **Targeted trait implementations**: Enables signature-specific behavior
///
/// See the module-level documentation for more details on this design decision.
///
/// # Examples
///
/// ```
/// use xrpl_wasm_stdlib::core::types::signature::Signature;
///
/// // Create a new empty signature
/// let sig = Signature::new();
/// assert_eq!(sig.len(), 0);
/// assert_eq!(sig.capacity(), 72);
///
/// // Create from a byte slice (e.g., 64-byte EdDSA signature)
/// let ed25519_sig_data = [0u8; 64];
/// let sig = Signature::from_slice(&ed25519_sig_data);
/// assert_eq!(sig.len(), 64);
///
/// // Create from a byte slice (e.g., 71-byte ECDSA signature)
/// let ecdsa_sig_data = [0u8; 71];
/// let sig = Signature::from_slice(&ecdsa_sig_data);
/// assert_eq!(sig.len(), 71);
///
/// // Access the underlying data
/// let bytes: &[u8] = sig.as_slice();
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Signature(pub Blob<SIGNATURE_MAX_SIZE>);

impl Signature {
    /// Creates a new empty signature.
    #[inline]
    pub const fn new() -> Self {
        Self(Blob::new())
    }

    /// Creates a signature from a byte slice, copying up to 72 bytes.
    ///
    /// If the slice is longer than 72 bytes, only the first 72 bytes are copied.
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Self {
        Self(Blob::from_slice(slice))
    }

    /// Returns the actual length of the signature data.
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the signature contains no data.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the maximum capacity of the signature (always 72 bytes).
    #[inline]
    pub const fn capacity(&self) -> usize {
        SIGNATURE_MAX_SIZE
    }

    /// Returns a slice of the actual signature data (not including unused capacity).
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    /// Returns a reference to the underlying blob.
    #[inline]
    pub const fn as_blob(&self) -> &Blob<SIGNATURE_MAX_SIZE> {
        &self.0
    }
}

impl From<Blob<SIGNATURE_MAX_SIZE>> for Signature {
    fn from(blob: Blob<SIGNATURE_MAX_SIZE>) -> Self {
        Self(blob)
    }
}

impl From<Signature> for Blob<SIGNATURE_MAX_SIZE> {
    fn from(sig: Signature) -> Self {
        sig.0
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_signature() {
        let sig = Signature::new();
        assert_eq!(sig.len(), 0);
        assert!(sig.is_empty());
        assert_eq!(sig.capacity(), 72);
        assert_eq!(sig.as_slice(), &[]);
    }

    #[test]
    fn test_from_slice_with_ed25519_signature() {
        // EdDSA signatures are 64 bytes
        let data = [0x42u8; 64];
        let sig = Signature::from_slice(&data);

        assert_eq!(sig.len(), 64);
        assert!(!sig.is_empty());
        assert_eq!(sig.as_slice().len(), 64);
        assert!(sig.as_slice().iter().all(|&b| b == 0x42));
    }

    #[test]
    fn test_from_slice_with_ecdsa_signature_70_bytes() {
        // ECDSA signatures can be 70 bytes
        let data = [0xABu8; 70];
        let sig = Signature::from_slice(&data);

        assert_eq!(sig.len(), 70);
        assert!(!sig.is_empty());
        assert_eq!(sig.as_slice().len(), 70);
        assert!(sig.as_slice().iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn test_from_slice_with_ecdsa_signature_71_bytes() {
        // ECDSA signatures can be 71 bytes
        let data = [0xCDu8; 71];
        let sig = Signature::from_slice(&data);

        assert_eq!(sig.len(), 71);
        assert_eq!(sig.as_slice().len(), 71);
        assert!(sig.as_slice().iter().all(|&b| b == 0xCD));
    }

    #[test]
    fn test_from_slice_with_ecdsa_signature_72_bytes() {
        // ECDSA signatures can be up to 72 bytes
        let data = [0xEFu8; 72];
        let sig = Signature::from_slice(&data);

        assert_eq!(sig.len(), 72);
        assert_eq!(sig.as_slice().len(), 72);
        assert!(sig.as_slice().iter().all(|&b| b == 0xEF));
    }

    #[test]
    fn test_from_slice_truncates_oversized_data() {
        // If someone provides more than 72 bytes, it should truncate
        let data = [0x99u8; 100];
        let sig = Signature::from_slice(&data);

        assert_eq!(sig.len(), 72);
        assert_eq!(sig.as_slice().len(), 72);
    }

    #[test]
    fn test_from_slice_with_empty_slice() {
        let data: &[u8] = &[];
        let sig = Signature::from_slice(data);

        assert_eq!(sig.len(), 0);
        assert!(sig.is_empty());
        assert_eq!(sig.as_slice(), &[]);
    }

    #[test]
    fn test_default_creates_empty_signature() {
        let sig: Signature = Signature::default();

        assert_eq!(sig.len(), 0);
        assert!(sig.is_empty());
        assert_eq!(sig.capacity(), 72);
    }

    #[test]
    fn test_capacity_is_always_72() {
        let sig1 = Signature::new();
        let sig2 = Signature::from_slice(&[1, 2, 3]);
        let sig3 = Signature::from_slice(&[0u8; 64]);

        assert_eq!(sig1.capacity(), 72);
        assert_eq!(sig2.capacity(), 72);
        assert_eq!(sig3.capacity(), 72);
    }

    #[test]
    fn test_from_blob() {
        let blob: Blob<72> = Blob::from_slice(&[0x11, 0x22, 0x33]);
        let sig = Signature::from(blob);

        assert_eq!(sig.len(), 3);
        assert_eq!(sig.as_slice(), &[0x11, 0x22, 0x33]);
    }

    #[test]
    fn test_into_blob() {
        let sig = Signature::from_slice(&[0xAA, 0xBB, 0xCC]);
        let blob: Blob<72> = sig.into();

        assert_eq!(blob.len(), 3);
        assert_eq!(blob.as_slice(), &[0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn test_as_blob() {
        let sig = Signature::from_slice(&[0x01, 0x02, 0x03, 0x04]);
        let blob_ref = sig.as_blob();

        assert_eq!(blob_ref.len(), 4);
        assert_eq!(blob_ref.as_slice(), &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_clone() {
        let sig1 = Signature::from_slice(&[1, 2, 3, 4, 5]);
        let sig2 = sig1.clone();

        // Both should have the same data
        assert_eq!(sig1.as_slice(), sig2.as_slice());
        assert_eq!(sig1.len(), sig2.len());
    }

    #[test]
    fn test_equality() {
        let sig1 = Signature::from_slice(&[0xAA, 0xBB, 0xCC]);
        let sig2 = Signature::from_slice(&[0xAA, 0xBB, 0xCC]);
        let sig3 = Signature::from_slice(&[0xAA, 0xBB, 0xDD]);

        assert_eq!(sig1, sig2);
        assert_ne!(sig1, sig3);
    }

    #[test]
    fn test_real_world_ecdsa_signature() {
        // This is the actual ECDSA signature from the test file
        let ecdsa_sig: [u8; 71] = [
            0x30, 0x45, 0x02, 0x21, 0x00, 0x8A, 0xD5, 0xEE, 0x48, 0xF7, 0xF1, 0x04, 0x78, 0x13,
            0xE7, 0x9C, 0x17, 0x4F, 0xE4, 0x01, 0xD0, 0x23, 0xA4, 0xB4, 0xA7, 0xB9, 0x9A, 0xF8,
            0x26, 0xE0, 0x81, 0xDB, 0x1D, 0xFF, 0x7B, 0x9C, 0x51, 0x02, 0x20, 0x13, 0x3F, 0x05,
            0xB7, 0xFD, 0x3D, 0x7D, 0x7F, 0x16, 0x3E, 0x8C, 0x77, 0xEE, 0x0A, 0x49, 0xD0, 0x26,
            0x19, 0xAB, 0x6C, 0x77, 0xCC, 0x34, 0x87, 0xD0, 0x09, 0x5C, 0x9B, 0x34, 0x03, 0x3C,
            0x1C,
        ];

        let sig = Signature::from_slice(&ecdsa_sig);
        assert_eq!(sig.len(), 71);
        assert_eq!(sig.as_slice(), &ecdsa_sig);
    }
}
