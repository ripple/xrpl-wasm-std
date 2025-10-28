// pub fn authorize(recipient: &AccountID, amount: &Amount) -> i32 {
//     unsafe {
//         // Build Payment transaction
//         let txn_index = build_txn(TT_PAYMENT);
//         if txn_index < 0 {
//             return -100; // Build error
//         }
        
//         // Get the encoded amount from Amount
//         let amount_bytes = amount.to_bytes();
        
//         // Add Amount field
//         if add_txn_field(
//             txn_index,
//             sfield::Amount,
//             amount_bytes.as_ptr(),
//             amount_bytes.len()
//         ) < 0 {
//             return -101; // Field error
//         }
        
//         // Add Destination field (21 bytes: 1 byte prefix + 20 byte account)
//         let mut dest_buffer = [0u8; 21];
//         dest_buffer[0] = 0x14; // Account ID type prefix
//         dest_buffer[1..21].copy_from_slice(&recipient.0);
        
//         if add_txn_field(
//             txn_index,
//             sfield::Destination,
//             dest_buffer.as_ptr(),
//             dest_buffer.len()
//         ) < 0 {
//             return -102; // Field error
//         }
        
//         // Emit the transaction
//         emit_built_txn(txn_index)
//     }
// }