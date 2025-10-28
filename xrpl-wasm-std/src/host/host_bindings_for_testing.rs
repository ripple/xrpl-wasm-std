// This file exists as a host_binding stand-in for non-WASM targets. For example, this file will
// be used during unit tests.

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_sqn() -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_parent_ledger_time() -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_parent_ledger_hash(_out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_base_fee() -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn amendment_enabled(_amendment_ptr: *const u8, _amendment_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn cache_ledger_obj(_keylet_ptr: *const u8, _keylet_len: usize, _cache_num: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_field(_field: i32, _out_buff_ptr: *mut u8, _out_buff_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_field(
    _field: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_field(
    _cache_num: i32,
    _field: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_nested_field(
    _locator_ptr: *const u8,
    _locator_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_nested_field(
    _locator_ptr: *const u8,
    _locator_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_nested_field(
    _cache_num: i32,
    _locator_ptr: *const u8,
    _locator_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_array_len(_field: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_array_len(_field: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_array_len(_cache_num: i32, _field: i32) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_tx_nested_array_len(_locator_ptr: *const u8, _locator_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_current_ledger_obj_nested_array_len(
    _locator_ptr: *const u8,
    _locator_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ledger_obj_nested_array_len(
    _cache_num: i32,
    _locator_ptr: *const u8,
    _locator_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn update_data(_data_ptr: *const u8, _data_len: usize) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn compute_sha512_half(
    _data_ptr: *const u8,
    _data_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn check_sig(
    _message_ptr: *const u8,
    _message_len: usize,
    _signature_ptr: *const u8,
    _signature_len: usize,
    _pubkey_ptr: *const u8,
    _pubkey_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn account_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn amm_keylet(
    _issue1_ptr: *const u8,
    _issue1_len: usize,
    _issue2_ptr: *const u8,
    _issue2_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn check_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn credential_keylet(
    _subject_ptr: *const u8,
    _subject_len: usize,
    _issuer_ptr: *const u8,
    _issuer_len: usize,
    _cred_type_ptr: *const u8,
    _cred_type_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn delegate_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _authorize_ptr: *const u8,
    _authorize_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn deposit_preauth_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _authorize_ptr: *const u8,
    _authorize_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn did_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn escrow_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn line_keylet(
    _account1_ptr: *const u8,
    _account1_len: usize,
    _account2_ptr: *const u8,
    _account2_len: usize,
    _currency_ptr: *const u8,
    _currency_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn mpt_issuance_keylet(
    _issuer_ptr: *const u8,
    _issuer_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn mptoken_keylet(
    _mptid_ptr: *const u8,
    _mptid_len: usize,
    _holder_ptr: *const u8,
    _holder_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn nft_offer_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn offer_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn oracle_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _document_id: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn paychan_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _destination_ptr: *const u8,
    _destination_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn permissioned_domain_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn signers_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn ticket_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn vault_keylet(
    _account_ptr: *const u8,
    _account_len: usize,
    _sequence: i32,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft(
    _account_ptr: *const u8,
    _account_len: usize,
    _nft_id_ptr: *const u8,
    _nft_id_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_issuer(
    _nft_id_ptr: *const u8,
    _nft_id_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    32
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_taxon(
    _nft_id_ptr: *const u8,
    _nft_id_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    4
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_flags(_nft_id_ptr: *const u8, _nft_id_len: usize) -> i32 {
    0
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_transfer_fee(_nft_id_ptr: *const u8, _nft_id_len: usize) -> i32 {
    0
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_nft_serial(
    _nft_id_ptr: *const u8,
    _nft_id_len: usize,
    _out_buff_ptr: *mut u8,
    _out_buff_len: usize,
) -> i32 {
    4
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_from_int(
    _in_int: i64,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_from_uint(
    _in_uint_ptr: *const u8,
    _in_uint_len: usize,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_set(
    _exponent: i32,
    _mantissa: i64,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_compare(
    _in_buff1: *const u8,
    _in_buff1_len: usize,
    _in_buff2: *const u8,
    _in_buff2_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_add(
    _in_buff1: *const u8,
    _in_buff1_len: usize,
    _in_buff2: *const u8,
    _in_buff2_len: usize,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_subtract(
    _in_buff1: *const u8,
    _in_buff1_len: usize,
    _in_buff2: *const u8,
    _in_buff2_len: usize,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_multiply(
    _in_buff1: *const u8,
    _in_buff1_len: usize,
    _in_buff2: *const u8,
    _in_buff2_len: usize,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_divide(
    _in_buff1: *const u8,
    _in_buff1_len: usize,
    _in_buff2: *const u8,
    _in_buff2_len: usize,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_pow(
    _in_buff: *const u8,
    _in_buff_len: usize,
    _in_int: i32,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_root(
    _in_buff: *const u8,
    _in_buff_len: usize,
    _in_int: i32,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn float_log(
    _in_buff: *const u8,
    _in_buff_len: usize,
    _out_buff: *mut u8,
    _out_buff_len: usize,
    _rounding_mode: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace(
    _msg_read_ptr: *const u8,
    _msg_read_len: usize,
    _data_read_ptr: *const u8,
    _data_read_len: usize,
    _as_hex: i32,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace_num(_msg_read_ptr: *const u8, _msg_read_len: usize, _number: i64) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace_account(
    _msg_read_ptr: *const u8,
    _msg_read_len: usize,
    _account_ptr: *const u8,
    _account_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace_opaque_float(
    _msg_read_ptr: *const u8,
    _msg_read_len: usize,
    _opaque_float_ptr: *const u8,
    _opaque_float_len: usize,
) -> i32 {
    -1
}

#[allow(unused)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn trace_amount(
    _msg_read_ptr: *const u8,
    _msg_read_len: usize,
    _amount_ptr: *const u8,
    _amount_len: usize,
) -> i32 {
    0 // Success for testing
}
