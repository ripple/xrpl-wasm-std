//! XRPL Amount types and operations.
//!
//! The XRP Ledger supports three distinct amount types:
//!
//! ## 1. XRP
//! - Represented as 64-bit unsigned integers in "drops" (1 XRP = 1,000,000 drops)
//! - Format: `[0][1][0][61-bit drop amount]`
//! - Use standard integer operations
//!
//! ## 2. Fungible Tokens (IOUs)
//! - Custom 64-bit floating-point format for issued currencies
//! - Format: `[1][Sign][8-bit Exponent][54-bit Mantissa]`
//! - Operations MUST use host float functions (float_add, float_multiply, etc.)
//! - See [`opaque_float::OpaqueFloat`] for the representation
//! - See [`token_amount`] for higher-level types
//!
//! ## 3. Multi-Purpose Tokens (MPTs)
//! - 64-bit integer quantity with 192-bit issuance ID
//! - Format: `[0x60][64-bit quantity][192-bit MPT ID]`
//! - See [`mpt_id`] for MPT identifiers
//!
//! ## Float Operations
//!
//! All fungible token arithmetic uses rippled's Number class via FFI:
//! - Deterministic decimal arithmetic matching XRPL consensus
//! - Explicit rounding modes (ToNearest, TowardsZero, Downward, Upward)
//! - ~16 decimal digits of precision
//! - Exponent range: -96 to +80
//!
//! Never perform direct arithmetic on [`opaque_float::OpaqueFloat`] bytes.
//! Always use the host float functions (float_add, float_multiply, etc.) in [`crate::host`].

pub mod asset;
pub mod currency_code;
pub mod mpt_id;
pub mod opaque_float;
pub mod token_amount;
