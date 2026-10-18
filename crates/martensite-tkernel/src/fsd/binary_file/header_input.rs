//! Scalar publication and header traversal shared by input adapters.
use super::FileHeader;

pub(super) fn read_integer<E>(
    value: &mut i32,
    read: impl FnOnce(&mut [u8; 4]) -> Result<(), E>,
) -> Result<(), E> {
    let mut bytes = value.to_ne_bytes();
    let outcome = read(&mut bytes);
    *value = i32::from_ne_bytes(bytes);
    outcome
}

pub(super) fn read_fields<E>(
    header: &mut FileHeader,
    mut read: impl FnMut(&mut i32) -> Result<(), E>,
) -> Result<(), E> {
    for field in header.fields_mut() {
        read(field)?;
    }
    Ok(())
}
