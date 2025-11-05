//! Placeholder types for array and object SFields.
//!
//! These types are used as placeholders in SField definitions for array and object types
//! that cannot be directly retrieved from ledger objects. They are primarily used within
//! `Location` for navigating nested structures.
//!
//! **Important**: These types intentionally do NOT implement `FieldGetter`. Attempting to use
//! them with `get_field()` or `get_field_optional()` will result in a compile-time error,
//! preventing accidental misuse at runtime.

/// Placeholder type for array SFields.
///
/// Array types in XRPL (like Signers, Memos, etc.) cannot be directly retrieved
/// as complete values. Instead, they are used within `Location` to navigate to
/// specific array elements.
///
/// This type intentionally does NOT implement `FieldGetter` to prevent compile-time
/// misuse. If you need to access array elements, use `Location` to navigate to
/// specific fields within the array.
///
/// There are purposefully no FieldGetters for this type, since it cannot be retrieved directly. Instead one must use the `Locator` to fetch information from it.
///
/// TODO: explore using the Locator under the hood here
#[derive(Debug, Eq, PartialEq)]
pub struct Array;
