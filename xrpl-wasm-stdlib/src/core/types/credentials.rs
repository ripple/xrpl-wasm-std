/// A 256-byte credential identifier on the XRP Ledger.
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Automatically derived with Copy for consistency
/// - `Copy`: Derived despite the 256-byte size to enable array initialization patterns
/// - `PartialEq, Eq`: Enable credential ID comparisons and use in collections
///
/// Note: `Copy` is derived here as an exception to the usual size guidelines (>32 bytes).
/// This is necessary because `CredentialID` is used in fixed-size arrays (e.g., `[CredentialID; 10]`)
/// which require `Copy` for array initialization syntax. While this means copies are expensive,
/// the usage pattern (storing in arrays, not frequently copied) makes this acceptable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct CredentialID(pub [u8; 256]);

/// A collection of up to 10 credential IDs (2560+ bytes total).
///
/// ## Derived Traits
///
/// - `Debug`: Useful for development and debugging
/// - `Clone`: Reasonable for this large struct when explicit copying is needed
/// - `PartialEq, Eq`: Enable credential ID collection comparisons
///
/// Note: `Copy` is intentionally not derived due to the struct's size (2560+ bytes).
/// Large `Copy` types can lead to accidental expensive copies and poor performance.
/// Use `.clone()` when you need to duplicate a credential ID collection.
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
