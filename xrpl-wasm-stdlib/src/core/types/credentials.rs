/// A 256-byte credential identifier on the XRP Ledger.
///
/// ## Derived Traits
///
/// - `Copy`: Derived despite 256-byte size to enable array initialization patterns
/// - `PartialEq, Eq`: Enable comparisons and use in collections
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// Note: `Copy` is an exception to usual size guidelines, required for array initialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct CredentialID(pub [u8; 256]);

/// A collection of up to 10 credential IDs (2560+ bytes total).
///
/// ## Derived Traits
///
/// - `PartialEq, Eq`: Enable comparisons
/// - `Debug, Clone`: Standard traits for development and consistency
///
/// Note: `Copy` is intentionally not derived due to the struct's size (2560+ bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct CredentialIDs {
    pub credential_ids: [CredentialID; 10],
    pub num_credential_ids: u8, // Max of 10!
}

#[derive(Debug, PartialEq, Eq)]
pub struct TooManyIdsError; // Simple error type

impl CredentialIDs {
    // Using a slice allows accepting various inputs like arrays or Vecs
    pub fn new(ids: &[CredentialID]) -> Self {
        // Runtime check for the maximum number of IDs.
        // Consider using TryFrom (see option 3) for fallible creation.
        assert!(
            ids.len() <= 10,
            "Cannot create CredentialIDs with more than 10 IDs."
        );

        let mut credential_ids_array = [EMPTY_CREDENTIAL_ID; 10];

        // Copy the provided IDs into the start of the array.
        // Since CredentialID is Copy, this is a simple assignment.
        for (i, &id) in ids.iter().enumerate() {
            credential_ids_array[i] = id; // <-- Copy
        }

        CredentialIDs {
            credential_ids: credential_ids_array,
            // Convert usize from len() to u8 safely.
            // The assert above ensures this won't overflow.
            num_credential_ids: ids.len() as u8,
        }
    }
}

impl TryFrom<&[CredentialID]> for CredentialIDs {
    type Error = TooManyIdsError;

    fn try_from(ids: &[CredentialID]) -> Result<Self, Self::Error> {
        if ids.len() > 10 {
            return Err(TooManyIdsError);
        }

        let mut credential_ids_array = [EMPTY_CREDENTIAL_ID; 10];

        // Copy the provided IDs into the start of the array.
        // Since CredentialID is Copy, this is a simple assignment.
        for (i, &id) in ids.iter().enumerate() {
            credential_ids_array[i] = id; // <-- Copy
        }

        Ok(CredentialIDs {
            credential_ids: credential_ids_array,
            num_credential_ids: ids.len() as u8,
        })
    }
}

pub const EMPTY_CREDENTIAL_ID: CredentialID = CredentialID([0x00; 256]);
