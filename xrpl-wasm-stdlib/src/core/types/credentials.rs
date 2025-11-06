//! Credential types for XRPL credentials
//!
//! This module provides types for working with XRPL credentials.
//! A CredentialID is a 32-byte (256-bit) hash that uniquely identifies a credential.

use crate::core::types::uint::Hash256;

/// A credential identifier (32-byte hash)
///
/// CredentialIDs are 256-bit hashes that uniquely identify credentials on the XRPL.
/// They are computed from the credential's issuer, subject, and credential type.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct CredentialID(pub [u8; 32]);

impl CredentialID {
    /// Creates a new CredentialID from a 32-byte array
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the inner 32 bytes as a reference
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<[u8; 32]> for CredentialID {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl From<Hash256> for CredentialID {
    fn from(hash: Hash256) -> Self {
        Self(*hash.as_bytes())
    }
}

impl From<CredentialID> for Hash256 {
    fn from(cred_id: CredentialID) -> Self {
        Hash256::from(cred_id.0)
    }
}

/// Empty credential ID constant (all zeros)
pub const EMPTY_CREDENTIAL_ID: CredentialID = CredentialID([0x00; 32]);
