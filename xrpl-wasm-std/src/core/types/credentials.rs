#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct CredentialID(pub [u8; 256]);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
        // Using `enumerate` gives us the index `i`.
        // Since CredentialID is Copy, `*id` performs a cheap copy.
        for (i, &id) in ids.iter().enumerate() {
            credential_ids_array[i] = id;
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
        for (i, &id) in ids.iter().enumerate() {
            credential_ids_array[i] = id;
        }

        Ok(CredentialIDs {
            credential_ids: credential_ids_array,
            num_credential_ids: ids.len() as u8,
        })
    }
}

pub const EMPTY_CREDENTIAL_ID: CredentialID = CredentialID([0x00; 256]);
