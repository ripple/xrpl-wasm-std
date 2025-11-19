//TODO add docs after discussing the interface
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_TO_NEAREST: i32 = 0;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_TOWARDS_ZERO: i32 = 1;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_DOWNWARD: i32 = 2;
#[allow(unused)]
pub const FLOAT_ROUNDING_MODES_UPWARD: i32 = 3;

// pub enum RippledRoundingModes{
//     ToNearest = 0,
//     TowardsZero = 1,
//     DOWNWARD = 2,
//     UPWARD = 3
// }

#[allow(unused)]
#[link(wasm_import_module = "host_lib")]
unsafe extern "C" {
    // ###############################
    // Host Function Category: getters
    // ###############################

    /// Retrieves the current ledger sequence number.
    ///
    /// This function populates a provided buffer with the ledger sequence number.
    ///
    /// # Returns
    ///
    /// - Returns the current ledger sequence number on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_ledger_sqn() -> i32;

    /// Retrieves the parent ledger time.
    ///
    /// This function is used to obtain the parent ledger's timestamp as a byte array.
    /// The timestamp is written into a provided output buffer.
    ///
    /// # Returns
    ///
    /// - Returns the parent ledger time on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_parent_ledger_time() -> i32;

    /// Retrieves the hash of the parent ledger.
    ///
    /// This function fetches the hash of the parent ledger and stores it in the buffer provided.
    /// The hash is expected to be written to the memory location pointed by `out_buff_ptr`,
    /// and its length should not exceed the `out_buff_len`.
    ///
    /// # Parameters
    /// - `out_buff_ptr`: A mutable pointer to a buffer where the parent ledger hash will be written.
    ///   The buffer must be allocated and managed by the caller.
    /// - `out_buff_len`: The maximum length of the buffer in bytes. This indicates the size of the
    ///   buffer and ensures that the function does not write beyond the allowed length.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_parent_ledger_hash(out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    /// Retrieves the current transaction base fee.
    ///
    /// # Returns
    ///
    /// - Returns a positive transaction base fee on success.
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_base_fee() -> i32;

    /// Retrieves the state of an amendment and whether it's enabled or not.
    ///
    /// # Parameters
    ///
    /// - `amendment_ptr`: A raw pointer to the amendment. This can be either the uint256 that
    ///   represents the hash of an amendment, or the string name of the
    ///   amendment.
    /// - `amendment_len`: The length of the amendment specified by `amendment_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a boolean 0 or 1 (whether the amendment is enabled or not) on success.
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn amendment_enabled(amendment_ptr: *const u8, amendment_len: usize) -> i32;

    /// Fetch a ledger entry pointed by the given keylet.
    ///
    /// This function uses the keylet to locate a ledger entry. If found, add it to the
    /// cache. The cache can have up to 255 ledger entries. If `cache_num` is 0, the
    /// new ledger entry will put in the next available cache space. If `cache_num` is not 0,
    /// the new ledger entry will replace an existing ledger entry in the catch.
    ///
    /// # Parameters
    ///
    /// - `keylet_ptr`: A raw pointer to the keylet, which is a unique identifier used to
    ///   locate or store data in the ledger.
    /// - `keylet_len`: The length of the keylet specified by `keylet_ptr`.
    /// - `cache_num`: The cache number to which the keylet will be placed in.
    ///   If 0, the host will assign a new cache space.
    ///
    /// # Returns
    ///
    /// - Returns a positive cache number
    /// - Returns a negative error code on failure
    pub fn cache_ledger_obj(keylet_ptr: *const u8, keylet_len: usize, cache_num: i32) -> i32;

    /// Retrieves a specific transaction field and writes it into the provided output buffer.
    ///
    /// # Parameters
    ///
    /// * `field` - An integer value representing the specific transaction field to retrieve.
    /// * `out_buff_ptr` - A mutable pointer to a buffer where the output data will be written.
    /// * `out_buff_len` - The size (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_tx_field(field: i32, out_buff_ptr: *mut u8, out_buff_len: usize) -> i32;

    /// Retrieves a specific field from the current ledger object and writes it into the provided buffer.
    ///
    /// # Parameters
    /// - `field` (`i32`): The integer identifier for the desired field in the ledger object.
    /// - `out_buff_ptr` (`*mut u8`): A mutable pointer to the memory location where the field data
    ///   will be written. This should point to a pre-allocated buffer.
    /// - `out_buff_len` (`usize`): The size (in bytes) of the buffer provided by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_current_ledger_obj_field(
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific field from a ledger object based on the given parameters.
    ///
    /// # Parameters
    ///
    /// - `cache_num`: An integer representing the cache index of the ledger object.
    /// - `field`: An integer representing the specific field to retrieve from the ledger object.
    /// - `out_buff_ptr`: A mutable pointer to a buffer where the retrieved field data will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_ledger_obj_field(
        cache_num: i32,
        field: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a nested field from the current ledger object and writes it into the provided buffer.
    ///
    /// # Parameters
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    /// - `out_buff_ptr`: A pointer to a mutable byte array where the resulting field data will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_tx_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a specific nested field from the current ledger object.
    ///
    /// This function is designed to access a nested field within the ledger object
    /// specified by the `locator`. The `locator` acts as a path or identifier to
    /// the desired field. The resulting data is written to the `out_buff` buffer.
    /// The function returns a status code indicating success or failure of the operation.
    ///
    /// # Parameters
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    /// - `out_buff_ptr`: A pointer to a mutable byte array where the resulting field data will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_current_ledger_obj_nested_field(
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves a nested field from a ledger object in a specific cache_num and writes the result into an output buffer.
    ///
    /// # Parameters
    /// - `cache_num`: The cache index of the ledger object to access.
    /// - `locator_ptr`: A pointer to the memory location containing the locator string data
    ///   (used to identify the nested field in the ledger object).
    /// - `locator_len`: The length of the locator string.
    /// - `out_buff_ptr`: A pointer to the buffer where the retrieved nested field value will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_ledger_obj_nested_field(
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves the length of an array based on the provided field value.
    ///
    /// # Parameters
    /// - `field` (i32): The integer identifier for the desired field.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_tx_array_len(field: i32) -> i32;

    /// Retrieves the length of an array based on the provided field value.
    ///
    /// # Parameters
    /// - `field` (i32): The integer identifier for the desired field.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_current_ledger_obj_array_len(field: i32) -> i32;

    /// Retrieves the length of an array based on the provided cache number and field value.
    ///
    /// # Parameters
    /// - `cache_num`: The cache index of the ledger object to access.
    /// - `field` (i32): The integer identifier for the desired field.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_ledger_obj_array_len(cache_num: i32, field: i32) -> i32;

    /// Retrieves the length of an array based on the provided locator.
    ///
    /// # Parameters
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_tx_nested_array_len(locator_ptr: *const u8, locator_len: usize) -> i32;

    /// Retrieves the length of an array based on the provided locator.
    ///
    /// # Parameters
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_current_ledger_obj_nested_array_len(
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;

    /// Retrieves the length of an array based on the provided locator.
    ///
    /// # Parameters
    /// - `cache_num`: The cache index of the ledger object to access.
    /// - `locator_ptr`: A pointer to a byte array containing the locator for the nested field.
    /// - `locator_len`: The length of the locator data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of array length on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_ledger_obj_nested_array_len(
        cache_num: i32,
        locator_ptr: *const u8,
        locator_len: usize,
    ) -> i32;

    // ###################################################
    // Host Function Category: update current ledger entry
    // ###################################################
    /// Updates a data field of the current ledger entry
    ///
    /// # Parameters
    ///
    /// - `data_ptr`: A pointer to the data to be written.
    /// - `data_len`: The size of the data.
    ///
    /// # Returns
    ///
    /// - 0 on success
    /// - negative for an error
    pub fn update_data(data_ptr: *const u8, data_len: usize) -> i32;

    // ###################################################
    // Host Function Category: hash and keylet computation
    // ###################################################

    /// Computes the first 32 bytes (half) of the SHA-512 hash for the given input data.
    ///
    /// # Parameters
    ///
    /// - `data_ptr`: A pointer to the input data to be hashed.
    /// - `data_len`: The length, in bytes, of the input data.
    /// - `out_buff_ptr`: A pointer to the buffer where the resulting 32-byte hash will be written.
    /// - `out_buff_len`: The length, in bytes, of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn compute_sha512_half(
        data_ptr: *const u8,
        data_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Checks a key signature when provided the message and public key.
    ///
    /// # Parameters
    /// - `message_ptr`: A pointer to the message data to be verified.
    /// - `message_len`: The length, in bytes, of the message data.
    /// - `signature_ptr`: A pointer to the signature data.
    /// - `signature_len`: The length, in bytes, of the signature data.
    /// - `pubkey_ptr`: A pointer to the public key data.
    /// - `pubkey_len`: The length, in bytes, of the public key data.
    ///
    /// # Returns
    ///
    /// - Returns 1 if the signature is valid.
    /// - Returns 0 if the signature is invalid.
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn check_sig(
        message_ptr: *const u8,
        message_len: usize,
        signature_ptr: *const u8,
        signature_len: usize,
        pubkey_ptr: *const u8,
        pubkey_len: usize,
    ) -> i32;

    /// Generates the keylet (key identifier) for a specific account.
    ///
    /// This function is used to calculate the account keylet in a cryptographic or
    /// blockchain-based system. A keylet is typically used to identify an account or entity
    /// in a secure and deterministic way.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory of the account identifier.
    /// - `account_len`: The size (in bytes) of the data pointed to by `account_ptr`.
    /// - `out_buff_ptr`: A pointer to the memory where the generated keylet will be stored.
    /// - `out_buff_len`: The length (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn account_keylet(
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Generates the keylet (key identifier) for a specific AMM.
    ///
    /// This function is used to calculate the AMM keylet in a cryptographic or
    /// blockchain-based system. A keylet is typically used to identify an AMM or entity
    /// in a secure and deterministic way.
    ///
    /// # Parameters
    ///
    /// - `issue1_ptr`: A pointer to the memory of the issue1 identifier.
    /// - `issue1_len`: The size (in bytes) of the data pointed to by `issue1_ptr`.
    /// - `issue2_ptr`: A pointer to the memory of the issue2 identifier.
    /// - `issue2_len`: The size (in bytes) of the data pointed to by `issue2_ptr`.
    /// - `out_buff_ptr`: A pointer to the memory where the generated keylet will be stored.
    /// - `out_buff_len`: The length (in bytes) of the buffer pointed to by `out_buff_ptr`.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn amm_keylet(
        issue1_ptr: *const u8,
        issue1_len: usize,
        issue2_ptr: *const u8,
        issue2_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a check entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `sequence`: The account sequence number associated with the check entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn check_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Generates a keylet for a credential.
    ///
    /// # Parameters
    ///
    /// * `subject_ptr`: A pointer to the memory location where the subject data begins.
    /// * `subject_len`: The length of the subject data in bytes.
    /// * `issuer_ptr`: A pointer to the memory location where the issuer data begins.
    /// * `issuer_len`: The length of the issuer data in bytes.
    /// * `cred_type_ptr`: A pointer to the memory location where the credential type data begins.
    /// * `cred_type_len`: The length of the credential type data in bytes.
    /// * `out_buff_ptr`: A pointer to the buffer where the generated keylet will be written.
    /// * `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn credential_keylet(
        subject_ptr: *const u8,
        subject_len: usize,
        issuer_ptr: *const u8,
        issuer_len: usize,
        cred_type_ptr: *const u8,
        cred_type_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a delegate entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `authorize_ptr`: A pointer to the memory location of the authorized account.
    /// - `authorize_len`: The length of the authorized account.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn delegate_keylet(
        account_ptr: *const u8,
        account_len: usize,
        authorize_ptr: *const u8,
        authorize_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a deposit preauth entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `authorize_ptr`: A pointer to the memory location of the authorized account.
    /// - `authorize_len`: The length of the authorized account.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn deposit_preauth_keylet(
        account_ptr: *const u8,
        account_len: usize,
        authorize_ptr: *const u8,
        authorize_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a DID entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn did_keylet(
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for an escrow entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `sequence`: The account sequence number associated with the escrow entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn escrow_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a trustline entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account1_ptr`: A pointer to the memory location of the first accountID.
    /// - `account1_len`: The length of the first accountID.
    /// - `account2_ptr`: A pointer to the memory location of the second accountID.
    /// - `account2_len`: The length of the second accountID.
    /// - `currency_ptr`: A pointer to the memory location of the currency.
    /// - `currency_len`: The length of the currency.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn line_keylet(
        account1_ptr: *const u8,
        account1_len: usize,
        account2_ptr: *const u8,
        account2_len: usize,
        currency_ptr: *const u8,
        currency_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for an MPT issuance entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `issuer_ptr`: A pointer to the memory location of the accountID.
    /// - `issuer_len`: The length of the accountID.
    /// - `sequence`: The account sequence number associated with the MPT issuance entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn mpt_issuance_keylet(
        issuer_ptr: *const u8,
        issuer_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for an MPToken entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `mptid_ptr`: A pointer to the memory location of the MPTID.
    /// - `mptid_len`: The length of the MPTID.
    /// - `holder_ptr`: A pointer to the memory location of the holder account.
    /// - `holder_len`: The length of the holder account.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn mptoken_keylet(
        mptid_ptr: *const u8,
        mptid_len: usize,
        holder_ptr: *const u8,
        holder_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for an NFT offer entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `sequence`: The account sequence number associated with the NFT offer entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn nft_offer_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for an offer entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `sequence`: The account sequence number associated with the offer entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn offer_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Generates a keylet associated with an oracle's account and document ID.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `document_id`: An integer representing the ID of the document associated with the oracle.
    /// - `out_buff_ptr`: A pointer to a pre-allocated buffer where the resulting keylet will be
    ///   written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn oracle_keylet(
        account_ptr: *const u8,
        account_len: usize,
        document_id: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a payment channel entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `destination_ptr`: A pointer to the memory location of the destination.
    /// - `destination_len`: The length of the destination.
    /// - `sequence`: The account sequence number associated with the payment channel entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn paychan_keylet(
        account_ptr: *const u8,
        account_len: usize,
        destination_ptr: *const u8,
        destination_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a permissioned domain entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `sequence`: The account sequence number associated with the permissioned domain entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn permissioned_domain_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a signer entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn signers_keylet(
        account_ptr: *const u8,
        account_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a ticket entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `sequence`: The account sequence number associated with the ticket entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn ticket_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Computes the Keylet for a vault entry in a ledger.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `sequence`: The account sequence number associated with the vault entry.
    /// - `out_buff_ptr`: A pointer to the output buffer where the derived keylet will be stored.
    /// - `out_buff_len`: The length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn vault_keylet(
        account_ptr: *const u8,
        account_len: usize,
        sequence: i32,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    // #############################
    // Host Function Category: NFT
    // #############################

    /// Retrieves the URI details of a specific NFT (Non-Fungible Token) associated with a given account.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the memory location of the accountID.
    /// - `account_len`: The length of the accountID.
    /// - `nft_id_ptr`: A pointer to the memory location containing the NFT identifier.
    /// - `nft_id_len`: The length of the NFT identifier in bytes.
    /// - `out_buff_ptr`: A mutable pointer to the memory location where the retrieved NFT URI
    ///   will be written.
    /// - `out_buff_len`: The maximum length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_nft(
        account_ptr: *const u8,
        account_len: usize,
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves the issuer of a specific NFT (Non-Fungible Token).
    ///
    /// # Parameters
    ///
    /// - `nft_id_ptr`: A pointer to the memory location containing the NFT identifier.
    /// - `nft_id_len`: The length of the NFT identifier in bytes.
    /// - `out_buff_ptr`: A mutable pointer to the memory location where the retrieved issuer
    ///   account will be written.
    /// - `out_buff_len`: The maximum length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_nft_issuer(
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves the taxon of a specific NFT (Non-Fungible Token).
    ///
    /// # Parameters
    ///
    /// - `nft_id_ptr`: A pointer to the memory location containing the NFT identifier.
    /// - `nft_id_len`: The length of the NFT identifier in bytes.
    /// - `out_buff_ptr`: A mutable pointer to the memory location where the retrieved taxon
    ///   will be written.
    /// - `out_buff_len`: The maximum length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_nft_taxon(
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    /// Retrieves the flags of a specific NFT (Non-Fungible Token).
    ///
    /// # Parameters
    ///
    /// - `nft_id_ptr`: A pointer to the memory location containing the NFT identifier.
    /// - `nft_id_len`: The length of the NFT identifier in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive flags value on success, which is a bitmask representing the NFT's flags
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_nft_flags(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;

    /// Retrieves the transfer fee of a specific NFT (Non-Fungible Token).
    ///
    /// # Parameters
    ///
    /// - `nft_id_ptr`: A pointer to the memory location containing the NFT identifier.
    /// - `nft_id_len`: The length of the NFT identifier in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive transfer fee value on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_nft_transfer_fee(nft_id_ptr: *const u8, nft_id_len: usize) -> i32;

    /// Retrieves the serial number of a specific NFT (Non-Fungible Token).
    ///
    /// # Parameters
    ///
    /// - `nft_id_ptr`: A pointer to the memory location containing the NFT identifier.
    /// - `nft_id_len`: The length of the NFT identifier in bytes.
    /// - `out_buff_ptr`: A mutable pointer to the memory location where the retrieved serial
    ///   number will be written.
    /// - `out_buff_len`: The maximum length of the output buffer.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes wrote to an output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   ../core/error_codes.rs
    pub fn get_nft_serial(
        nft_id_ptr: *const u8,
        nft_id_len: usize,
        out_buff_ptr: *mut u8,
        out_buff_len: usize,
    ) -> i32;

    // #############################
    // Host Function Category: FLOAT
    // #############################
    // Float operations for fungible token (IOU) arithmetic.
    // These functions use rippled's Number class via FFI for exact compatibility.
    //
    // ## Architecture
    // Float computations use the rippled Number class:
    // WASM Module -> Host Function -> XRPLD Number (rippled via FFI) -> Result
    //
    // ## XRPL Amount Types
    // The XRPL has three amount types:
    // 1. XRP - 64-bit integer (drops)
    // 2. Fungible Tokens (IOUs) - Custom 64-bit float format (these functions)
    // 3. MPTs - 64-bit integer quantity with issuance ID
    //
    // ## Float Format (IOUs)
    // 64-bit custom encoding: [Type:1][Sign:1][Exponent:8][Mantissa:54]
    // - Type bit: Always 1 for fungible tokens
    // - Sign bit: 1=positive, 0=negative
    // - Exponent: 8 bits, biased by 97 (range -96 to +80)
    // - Mantissa: 54 bits (16 decimal digits precision)
    // - Zero: Special encoding 0x8000000000000000
    //
    // ## Rounding Modes
    // All functions accept a rounding_mode parameter:
    // - 0: ToNearest (ties to even)
    // - 1: TowardsZero (truncate)
    // - 2: Downward (towards -∞)
    // - 3: Upward (towards +∞)

    /// Converts a signed 64-bit integer to an opaque float representation
    /// # Parameters
    /// * `in_int` - The input integer to convert
    /// * `out_buff` - Pointer to output buffer where the float will be written
    /// * `rounding_mode` - Rounding mode to use for the conversion
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_from_int(
        in_int: i64,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    /// Converts an unsigned integer to an opaque float representation
    /// # Parameters
    /// * `in_uint_ptr` - Pointer to the input unsigned integer
    /// * `out_buff` - Pointer to output buffer where the float will be written
    /// * `rounding_mode` - Rounding mode to use for the conversion
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_from_uint(
        in_uint_ptr: *const u8,
        in_uint_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    /// Creates a float from explicit exponent and mantissa values
    /// # Parameters
    /// * `exponent` - The exponent value
    /// * `mantissa` - The mantissa value
    /// * `out_buff` - Pointer to output buffer where the float will be written
    /// * `rounding_mode` - Rounding mode to use for the operation
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_set(
        exponent: i32,
        mantissa: i64,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    /// Compares two opaque float values
    /// # Parameters
    /// * `in_buff1` - Pointer to first float value
    /// * `in_buff2` - Pointer to second float value
    /// # Returns
    /// 0 if equal, 1 if first > second, 2 if first < second,
    pub fn float_compare(
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
    ) -> i32;

    /// Adds two opaque float values
    /// # Parameters
    /// * `in_buff1` - Pointer to first float value
    /// * `in_buff2` - Pointer to second float value
    /// * `out_buff` - Pointer to output buffer where result will be written
    /// * `rounding_mode` - Rounding mode to use for the addition
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_add(
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    /// Subtracts two opaque float values
    /// # Parameters
    /// * `in_buff1` - Pointer to first float value
    /// * `in_buff2` - Pointer to second float value
    /// * `out_buff` - Pointer to output buffer where result will be written
    /// * `rounding_mode` - Rounding mode to use for the subtraction
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_subtract(
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    /// Multiplies two opaque float values
    /// # Parameters
    /// * `in_buff1` - Pointer to first float value
    /// * `in_buff2` - Pointer to second float value
    /// * `out_buff` - Pointer to output buffer where result will be written
    /// * `rounding_mode` - Rounding mode to use for the multiplication
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_multiply(
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    /// Divides two opaque float values
    /// # Parameters
    /// * `in_buff1` - Pointer to dividend float value
    /// * `in_buff2` - Pointer to divisor float value
    /// * `out_buff` - Pointer to output buffer where result will be written
    /// * `rounding_mode` - Rounding mode to use for the division
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_divide(
        in_buff1: *const u8,
        in_buff1_len: usize,
        in_buff2: *const u8,
        in_buff2_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    /// Calculates the nth power of an opaque float value
    /// # Parameters
    /// * `in_buff` - Pointer to input float value
    /// * `in_int` - The power to calculate (e.g., 2 for square)
    /// * `out_buff` - Pointer to output buffer where result will be written
    /// * `rounding_mode` - Rounding mode to use for the operation
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_pow(
        in_buff: *const u8,
        in_buff_len: usize,
        in_int: i32,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    /// Calculates the nth root of an opaque float value
    /// # Parameters
    /// * `in_buff` - Pointer to input float value
    /// * `in_int` - The root to calculate (e.g., 2 for square root)
    /// * `out_buff` - Pointer to output buffer where result will be written
    /// * `rounding_mode` - Rounding mode to use for the operation
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_root(
        in_buff: *const u8,
        in_buff_len: usize,
        in_int: i32,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    /// Calculates the natural logarithm of an opaque float value
    /// # Arguments
    /// * `in_buff` - Pointer to input float value
    /// * `out_buff` - Pointer to output buffer where result will be written
    /// * `rounding_mode` - Rounding mode to use for the operation
    /// # Returns
    /// 8 on success, error code otherwise
    pub fn float_log(
        in_buff: *const u8,
        in_buff_len: usize,
        out_buff: *mut u8,
        out_buff_len: usize,
        rounding_mode: i32,
    ) -> i32;

    // #############################
    // Host Function Category: TRACE
    // #############################

    /// Print to the trace log on XRPLd. Any XRPLd instance set to \"trace\" log level will see this.
    ///
    /// # Parameters
    /// - `msg_read_ptr`: A pointer to an array containing text characters (in either utf8).
    /// - `msg_read_len`: The byte length of the text to send to the trace log.
    /// - `data_read_ptr`: A pointer to an array of bytes containing arbitrary data.
    /// - `data_read_len`: The byte length of the data to send to the trace log.
    /// - `as_hex`: If 0 treat the data_read_ptr as pointing at a string of text, otherwise treat it
    ///   as data and print hex.
    ///
    /// # Returns
    ///
    /// Returns an integer representing the result of the operation. A value of `0` or higher
    /// signifies the number of message bytes that were written to the trace function. Non-zero
    /// values indicate an error that corresponds to a known error code (e.g., incorrect buffer
    /// sizes).
    pub fn trace(
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        data_read_ptr: *const u8,
        data_read_len: usize,
        as_hex: i32,
    ) -> i32;

    /// Print a number to the trace log on XRPLd. Any XRPLd instance set to \"trace\" log level will
    /// see this.
    ///
    /// # Parameters
    /// * `msg_read_ptr`: A pointer to an array containing text characters (in either utf8).
    /// * `msg_read_len`: The byte length of the text to send to the trace log.
    /// * `number`: Any integer you wish to display after the text.
    ///
    /// # Returns
    ///
    /// Returns an integer representing the result of the operation. A value of `0` or higher
    /// signifies the number of message bytes that were written to the trace function. Non-zero
    /// values indicate an error that corresponds to a known error code (e.g., incorrect buffer
    /// sizes).
    pub fn trace_num(msg_read_ptr: *const u8, msg_read_len: usize, number: i64) -> i32;

    /// Print an account to the trace log on XRPLd. Any XRPLd instance set to \"trace\" log level will
    /// see this.
    ///
    /// # Parameters
    /// * `msg_read_ptr`: A pointer to an array containing text characters (in either utf8).
    /// * `msg_read_len`: The byte length of the text to send to the trace log.
    /// * `account_ptr`: A pointer to an account.
    /// * `account_len`: The byte length of the account.
    ///
    /// # Returns
    ///
    /// Returns an integer representing the result of the operation. A value of `0` or higher
    /// signifies the number of message bytes that were written to the trace function. Non-zero
    /// values indicate an error that corresponds to a known error code (e.g., incorrect buffer
    /// sizes).
    pub fn trace_account(
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        account_ptr: *const u8,
        account_len: usize,
    ) -> i32;

    /// Print an OpaqueFloat number to the trace log on XRPLd. Any XRPLd instance set to \"trace\"
    /// log level will see this.
    ///
    /// # Parameters
    /// * `msg_read_ptr`: A pointer to an array containing text characters (in either utf8).
    /// * `msg_read_len`: The byte length of the text to send to the trace log.
    /// * `opaque_float_ptr`: A pointer to an array of 8 bytes containing the u64 opaque pointer value.
    ///
    /// # Returns
    ///
    /// Returns an integer representing the result of the operation. A value of `0` or higher
    /// signifies the number of message bytes that were written to the trace function. Non-zero
    /// values indicate an error that corresponds to a known error code (e.g., incorrect buffer
    /// sizes).
    pub fn trace_opaque_float(
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        opaque_float_ptr: *const u8,
        opaque_float_len: usize,
    ) -> i32;

    /// Print an amount to the trace log on XRPLd. Any XRPLd instance set to \"trace\" log level will
    /// see this.
    ///
    /// # Parameters
    /// * `msg_read_ptr`: A pointer to an array containing text characters (in either utf8).
    /// * `msg_read_len`: The byte length of the text to send to the trace log.
    /// * `amount_ptr`: A pointer to an amount.
    /// * `amount_len`: The byte length of the amount.
    ///
    /// # Returns
    ///
    /// Returns an integer representing the result of the operation. A value of `0` or higher
    /// signifies the number of message bytes that were written to the trace function. Non-zero
    /// values indicate an error that corresponds to a known error code (e.g., incorrect buffer
    /// sizes).
    pub fn trace_amount(
        msg_read_ptr: *const u8,
        msg_read_len: usize,
        amount_ptr: *const u8,
        amount_len: usize,
    ) -> i32;

    // #####################################
    // Host Function Category: SMART CONTRACT
    // #####################################

    /// Retrieves a parameter passed to the smart contract instance at initialization.
    ///
    /// This function allows the smart contract to access parameters that were provided when
    /// the contract instance was created or deployed.
    ///
    /// # Parameters
    ///
    /// - `index`: The zero-based index of the parameter to retrieve.
    /// - `st_type_id`: The serialization type ID expected for this parameter, used for validation.
    /// - `out_buff_ptr`: A mutable pointer to the buffer where the parameter data will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes written to the output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn instance_param(
        index: i32, 
        st_type_id: i32, 
        out_buff_ptr: *mut u8, 
        out_buff_len: usize
    ) -> i32;

    /// Retrieves a parameter passed to the currently executing smart contract function.
    ///
    /// This function allows the smart contract to access parameters that were provided
    /// in the current function invocation.
    ///
    /// # Parameters
    ///
    /// - `index`: The zero-based index of the function parameter to retrieve.
    /// - `st_type_id`: The serialization type ID expected for this parameter, used for validation.
    /// - `out_buff_ptr`: A mutable pointer to the buffer where the parameter data will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes written to the output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn function_param(
        index: i32, 
        st_type_id: i32, 
        out_buff_ptr: *mut u8, 
        out_buff_len: usize
    ) -> i32;

    // #####################################
    // Host Function Category: DATA STORAGE
    // #####################################

    /// Retrieves a field value from a data object stored under a specific account and key.
    ///
    /// This function accesses the smart contract's persistent storage to read a field
    /// from a data object.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the account identifier that owns the data object.
    /// - `account_len`: The length of the account identifier in bytes.
    /// - `key_ptr`: A pointer to the key that identifies the data object.
    /// - `key_len`: The length of the key in bytes.
    /// - `out_buff_ptr`: A pointer to the buffer where the field value will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes written to the output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_data_object_field(
        account_ptr: *const u8, 
        account_len: usize, 
        key_ptr: *const u8, 
        key_len: usize, 
        out_buff_ptr: *const u8, 
        out_buff_len: usize
    ) -> i32;

    /// Retrieves a nested field value from a data object stored under a specific account and key.
    ///
    /// This function accesses nested fields within a data object using a path or nested locator.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the account identifier that owns the data object.
    /// - `account_len`: The length of the account identifier in bytes.
    /// - `key_ptr`: A pointer to the key that identifies the data object.
    /// - `key_len`: The length of the key in bytes.
    /// - `nst_ptr`: A pointer to the nested path or locator that identifies the nested field.
    /// - `nst_len`: The length of the nested path in bytes.
    /// - `out_buff_ptr`: A pointer to the buffer where the nested field value will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes written to the output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_data_nested_object_field(
        account_ptr: *const u8, 
        account_len: usize, 
        key_ptr: *const u8, 
        key_len: usize, 
        nst_ptr: *const u8, 
        nst_len: usize, 
        out_buff_ptr: *const u8, 
        out_buff_len: usize
    ) -> i32;

    /// Retrieves an element from an array field in a data object.
    ///
    /// This function accesses a specific element by index from an array stored in a data object.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the account identifier that owns the data object.
    /// - `account_len`: The length of the account identifier in bytes.
    /// - `key_ptr`: A pointer to the key that identifies the data object.
    /// - `key_len`: The length of the key in bytes.
    /// - `index`: The zero-based index of the array element to retrieve.
    /// - `out_buff_ptr`: A pointer to the buffer where the array element value will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes written to the output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_data_array_element_field(
        account_ptr: *const u8, 
        account_len: usize, 
        key_ptr: *const u8, 
        key_len: usize, 
        index: i32, 
        out_buff_ptr: *const u8, 
        out_buff_len: usize
    ) -> i32;

    /// Retrieves an element from a nested array field in a data object.
    ///
    /// This function accesses a specific element by index from a nested array within a data object.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the account identifier that owns the data object.
    /// - `account_len`: The length of the account identifier in bytes.
    /// - `key_ptr`: A pointer to the key that identifies the data object.
    /// - `key_len`: The length of the key in bytes.
    /// - `index`: The zero-based index of the array element to retrieve.
    /// - `nst_ptr`: A pointer to the nested path or locator that identifies the nested array.
    /// - `nst_len`: The length of the nested path in bytes.
    /// - `out_buff_ptr`: A pointer to the buffer where the array element value will be written.
    /// - `out_buff_len`: The size of the output buffer in bytes.
    ///
    /// # Returns
    ///
    /// - Returns a positive number of bytes written to the output buffer on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn get_data_nested_array_element_field(
        account_ptr: *const u8, 
        account_len: usize, 
        key_ptr: *const u8, 
        key_len: usize, 
        index: i32, 
        nst_ptr: *const u8, 
        nst_len: usize, 
        out_buff_ptr: *const u8, 
        out_buff_len: usize
    ) -> i32;

    /// Sets or updates a field value in a data object.
    ///
    /// This function writes data to the smart contract's persistent storage.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the account identifier that owns the data object.
    /// - `account_len`: The length of the account identifier in bytes.
    /// - `key_ptr`: A pointer to the key that identifies the data object.
    /// - `key_len`: The length of the key in bytes.
    /// - `data_ptr`: A pointer to the data to be written to the field.
    /// - `data_len`: The length of the data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns 0 on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn set_data_object_field(
        account_ptr: *const u8, 
        account_len: usize, 
        key_ptr: *const u8, 
        key_len: usize, 
        data_ptr: *const u8, 
        data_len: usize
    ) -> i32;

    /// Sets or updates a nested field value in a data object.
    ///
    /// This function writes data to a nested field within a data object in persistent storage.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the account identifier that owns the data object.
    /// - `account_len`: The length of the account identifier in bytes.
    /// - `key_ptr`: A pointer to the key that identifies the data object.
    /// - `key_len`: The length of the key in bytes.
    /// - `nst_ptr`: A pointer to the nested path or locator that identifies the nested field.
    /// - `nst_len`: The length of the nested path in bytes.
    /// - `data_ptr`: A pointer to the data to be written to the nested field.
    /// - `data_len`: The length of the data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns 0 on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn set_data_nested_object_field(
        account_ptr: *const u8, 
        account_len: usize, 
        key_ptr: *const u8, 
        key_len: usize, 
        nst_ptr: *const u8, 
        nst_len: usize, 
        data_ptr: *const u8, 
        data_len: usize
    ) -> i32;

    /// Sets or updates an element in an array field of a data object.
    ///
    /// This function writes data to a specific array element by index in persistent storage.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the account identifier that owns the data object.
    /// - `account_len`: The length of the account identifier in bytes.
    /// - `key_ptr`: A pointer to the key that identifies the data object.
    /// - `key_len`: The length of the key in bytes.
    /// - `index`: The zero-based index of the array element to set.
    /// - `data_ptr`: A pointer to the data to be written to the array element.
    /// - `data_len`: The length of the data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns 0 on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn set_data_array_element_field(
        account_ptr: *const u8, 
        account_len: usize, 
        key_ptr: *const u8, 
        key_len: usize, 
        index: i32, 
        data_ptr: *const u8, 
        data_len: usize
    ) -> i32;

    /// Sets or updates an element in a nested array field of a data object.
    ///
    /// This function writes data to a specific element in a nested array in persistent storage.
    ///
    /// # Parameters
    ///
    /// - `account_ptr`: A pointer to the account identifier that owns the data object.
    /// - `account_len`: The length of the account identifier in bytes.
    /// - `key_ptr`: A pointer to the key that identifies the data object.
    /// - `key_len`: The length of the key in bytes.
    /// - `index`: The zero-based index of the array element to set.
    /// - `nst_ptr`: A pointer to the nested path or locator that identifies the nested array.
    /// - `nst_len`: The length of the nested path in bytes.
    /// - `data_ptr`: A pointer to the data to be written to the array element.
    /// - `data_len`: The length of the data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns 0 on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn set_data_nested_array_element_field(
        account_ptr: *const u8, 
        account_len: usize, 
        key_ptr: *const u8, 
        key_len: usize, 
        index: i32, 
        nst_ptr: *const u8, 
        nst_len: usize, 
        data_ptr: *const u8, 
        data_len: usize
    ) -> i32;

    // #############################################
    // Host Function Category: TRANSACTION BUILDING
    // #############################################

    /// Begins building a new transaction of the specified type.
    ///
    /// This function initializes a transaction builder and returns an index that can be used
    /// to add fields to the transaction before emitting it.
    ///
    /// # Parameters
    ///
    /// - `txn_type`: The transaction type identifier (e.g., Payment, TrustSet, etc.).
    ///
    /// # Returns
    ///
    /// - Returns a positive transaction builder index on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn build_txn(txn_type: i32) -> i32;

    /// Adds a field to a transaction being built.
    ///
    /// This function adds a specific field with its value to a transaction that was
    /// previously initialized with `build_txn`.
    ///
    /// # Parameters
    ///
    /// - `index`: The transaction builder index returned by `build_txn`.
    /// - `field`: The field identifier to add to the transaction.
    /// - `write_ptr`: A pointer to the data for this field.
    /// - `write_len`: The length of the field data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns 0 on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn add_txn_field(
        index: i32, 
        field: i32, 
        write_ptr: *const u8, 
        write_len: usize
    ) -> i32;

    /// Emits a transaction that was built using the transaction builder.
    ///
    /// This function finalizes and submits a transaction that was constructed with
    /// `build_txn` and `add_txn_field`.
    ///
    /// # Parameters
    ///
    /// - `index`: The transaction builder index returned by `build_txn`.
    ///
    /// # Returns
    ///
    /// - Returns 0 on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn emit_built_txn(index: i32) -> i32;

    /// Emits a pre-serialized transaction directly.
    ///
    /// This function submits a complete, serialized transaction without using the
    /// transaction builder.
    ///
    /// # Parameters
    ///
    /// - `txn_read_ptr`: A pointer to the serialized transaction data.
    /// - `txn_read_len`: The length of the serialized transaction in bytes.
    ///
    /// # Returns
    ///
    /// - Returns 0 on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn emit_txn(txn_read_ptr: *const u8, txn_read_len: usize) -> i32;

    /// Emits a custom event from the smart contract.
    ///
    /// This function allows the smart contract to emit named events with associated data,
    /// which can be observed by external systems monitoring the blockchain.
    ///
    /// # Parameters
    ///
    /// - `name_ptr`: A pointer to the event name string.
    /// - `name_len`: The length of the event name in bytes.
    /// - `data_ptr`: A pointer to the event data payload.
    /// - `data_len`: The length of the event data in bytes.
    ///
    /// # Returns
    ///
    /// - Returns 0 on success
    /// - Returns a negative error code on failure. The list of error codes is defined in
    ///   `../core/error_codes.rs`
    pub fn emit_event(
        name_ptr: *const u8, 
        name_len: usize, 
        data_ptr: *const u8, 
        data_len: usize
    ) -> i32;
}
