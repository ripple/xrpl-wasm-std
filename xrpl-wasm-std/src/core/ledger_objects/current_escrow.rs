use crate::core::ledger_objects::traits::{CurrentEscrowFields, CurrentLedgerObjectCommonFields};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub struct CurrentEscrow;

impl CurrentLedgerObjectCommonFields for CurrentEscrow {}

impl CurrentEscrowFields for CurrentEscrow {}

#[inline]
pub fn get_current_escrow() -> CurrentEscrow {
    CurrentEscrow
}
