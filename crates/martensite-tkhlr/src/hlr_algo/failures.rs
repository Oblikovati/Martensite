//! The named failures that stand for the native exceptions and undefined reads of
//! HLRAlgo.
use martensite_tkernel::ncollection::ArrayError;
use martensite_tkernel::standard::Failure;

/// The exception a native `NCollection_Array1` access raises (Standard_OutOfRange for
/// an index, Standard_RangeError for bounds), prefixed with the HLRAlgo member;
/// `array.value(i).map_err(array_failure("HLRAlgo_EdgesBlock::Edge"))`.
pub(crate) fn array_failure(member: &'static str) -> impl Fn(ArrayError) -> Failure {
    move |error: ArrayError| error.into_failure(member)
}

/// Standard_NullObject for a member that dereferences a null handle, which crashes
/// natively; `null_handle("HLRAlgo_PolyData::Nodes", "myHNodes")`.
pub(crate) fn null_handle(member: &str, field: &str) -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        &format!("{member}: the handle {field} is null, expected a set handle"),
    )
}

/// The x86-64 truncating conversion `(int)value` (`cvttsd2si`): NaN and values outside
/// the `int` range give the "integer indefinite" `INT_MIN`, where Rust `as` saturates;
/// `c_int_truncation(2.9) == 2`.
pub(crate) fn c_int_truncation(value: f64) -> i32 {
    // 2^31 is exact; every double in (-2^31 - 1, 2^31) truncates into the int range.
    let limit: f64 = 2147483648.0;
    if value.is_nan() || value >= limit || value <= -limit - 1.0 {
        return i32::MIN;
    }
    value as i32
}

#[cfg(test)]
mod tests {
    use super::c_int_truncation;

    #[test]
    fn truncation_gives_int_min_outside_the_int_range() {
        assert_eq!(c_int_truncation(2.9), 2);
        assert_eq!(c_int_truncation(-2.9), -2);
        assert_eq!(c_int_truncation(f64::NAN), i32::MIN);
        assert_eq!(c_int_truncation(3.0e10), i32::MIN);
        assert_eq!(c_int_truncation(-2147483648.5), -2147483648);
        assert_eq!(c_int_truncation(-2147483649.0), i32::MIN);
    }
}
