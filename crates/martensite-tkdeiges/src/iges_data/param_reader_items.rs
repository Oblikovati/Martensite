//! Reads of one item of a shared array.
use super::{IgesDataParamReader, iges_array1_set};
use martensite_tkernel::{
    ncollection::HArray1,
    standard::{Failure, Handle},
    tcollection::HAsciiString,
};

/// Reads an integer into a rank of an array; one that does not read leaves the rank as it is.
///
/// # Errors
/// A failure of the read or a rank out of range.
pub(crate) fn iges_read_integer_item(
    pr: &mut IgesDataParamReader,
    message: &str,
    array: &Handle<HArray1<i32>>,
    rank: i32,
    where_called: &str,
) -> Result<(), Failure> {
    let mut value = 0;
    if pr.read_integer_text(&pr.current(), message, &mut value)? {
        iges_array1_set(array, rank, value, where_called)?;
    }
    Ok(())
}

/// Reads a real into a rank of an array; one that does not read leaves the rank as it is.
///
/// # Errors
/// A failure of the read or a rank out of range.
pub(crate) fn iges_read_real_item(
    pr: &mut IgesDataParamReader,
    message: &str,
    array: &Handle<HArray1<f64>>,
    rank: i32,
    where_called: &str,
) -> Result<(), Failure> {
    let mut value = 0.0;
    if pr.read_real_text(&pr.current(), message, &mut value)? {
        iges_array1_set(array, rank, value, where_called)?;
    }
    Ok(())
}

/// Reads a text into a rank of an array; one that does not read leaves the rank as it is.
///
/// # Errors
/// A failure of the read or a rank out of range.
pub(crate) fn iges_read_text_item(
    pr: &mut IgesDataParamReader,
    message: &str,
    array: &Handle<HArray1<Handle<HAsciiString>>>,
    rank: i32,
    where_called: &str,
) -> Result<(), Failure> {
    let mut value = Handle::default();
    if pr.read_text(&pr.current(), message, &mut value)? {
        iges_array1_set(array, rank, value, where_called)?;
    }
    Ok(())
}
