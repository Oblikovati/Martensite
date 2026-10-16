use super::AliasedArray;
use crate::standard::Failure;

impl AliasedArray<'_> {
    /// Apply positive signed-length validation; `AliasedArray::checked_size(3)?`.
    pub fn checked_size(count: i32) -> Result<usize, Failure> {
        if count <= 0 {
            return Err(Failure::with_message(
                "Standard_RangeError",
                "NCollection_AliasedArray, length should be positive",
            ));
        }
        Ok(count as usize)
    }

    /// Validate before `u32` narrowing; `AliasedArray::checked_stride(4)?`.
    /// Positive inputs above `u32::MAX` wrap, including to zero.
    pub fn checked_stride(stride: usize) -> Result<u32, Failure> {
        if stride == 0 {
            return Err(Failure::with_message(
                "Standard_RangeError",
                "NCollection_AliasedArray, stride should be positive",
            ));
        }
        Ok(stride as u32)
    }
}

// The operation name and message-only error are part of the observable failure.
pub(super) fn access_failure(method: &str, detail: &str, kind: &'static str) -> Failure {
    Failure::with_message(
        kind,
        &format!("NCollection_AliasedArray::{method}(), {detail}"),
    )
}
