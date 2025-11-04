/// Placeholder type for object SFields.
///
/// Object types in XRPL (like Memo, SignerEntry, etc.) cannot be directly retrieved
/// as complete values. Instead, they are used within `Location` to navigate to
/// specific object fields.
///
/// This type intentionally does NOT implement `FieldGetter` to prevent compile-time
/// misuse. If you need to access object fields, use `Location` to navigate to
/// specific fields within the object.
///
/// There are purposefully no FieldGetters for this type, since it cannot be retrieved directly. Instead one must use the `Locator` to fetch information from it.
///
/// TODO: explore using the Locator under the hood here
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Object;
