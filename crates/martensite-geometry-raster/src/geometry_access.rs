//! Access to the geometry handles of the geometry-command drawables.
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};

/// Usage: `let geometry = checked_geometry(&handle)?;`.
/// The object of `handle`; Standard_NullObject when it is null (a drawable restored
/// from an unreadable record, which the C++ code would dereference).
pub fn checked_geometry<T: ?Sized + RuntimeTyped>(handle: &Handle<T>) -> Result<&T, Failure> {
    handle
        .get()
        .ok_or_else(|| Failure::with_message("Standard_NullObject", "null geometry in a drawable"))
}
