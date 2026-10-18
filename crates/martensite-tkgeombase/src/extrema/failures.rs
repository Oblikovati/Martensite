//! The retained exceptions of the Extrema accessors, as `Failure` values that name the
//! class, the offending value and the expected shape.
use crate::native_failure::failure;
use martensite_tkernel::standard::Failure;

/// `not-done failure` raised by `class::member` on an algorithm that is not done.
pub(crate) fn not_done(member: &str) -> Failure {
    failure(
        "StdFail_NotDone",
        &format!("{member} - the algorithm is not done; expected IsDone() == true"),
    )
}

/// `index-range failure` raised for the index `n` outside `[1, count]`.
pub(crate) fn out_of_range(member: &str, n: i32, count: i32) -> Failure {
    failure(
        "Standard_OutOfRange",
        &format!("{member} - index {n} is outside [1, {count}]"),
    )
}

/// `NotImplemented` of a retained member without a body.
pub(crate) fn not_implemented(member: &str) -> Failure {
    failure(
        "Standard_NotImplemented",
        &format!("{member} - this configuration is not implemented natively"),
    )
}

/// `null-object failure` for a member called before its curve or surface is set
/// (a null pointer dereference natively).
pub(crate) fn null_object(member: &str) -> Failure {
    failure(
        "Standard_NullObject",
        &format!("{member} - no curve or surface is set; expected an initialized algorithm"),
    )
}

/// `TypeMismatch` of a function object whose point or curve is not set.
pub(crate) fn type_mismatch(member: &str) -> Failure {
    failure(
        "Standard_TypeMismatch",
        &format!("{member} - No init; expected the point and the curve or surface to be set"),
    )
}

/// Checks `1 <= n <= count` as the retained `if (N < 1 || N > NbExt())` tests do.
pub(crate) fn check_index(member: &str, n: i32, count: i32) -> Result<usize, Failure> {
    if n < 1 || n > count {
        return Err(out_of_range(member, n, count));
    }
    Ok((n - 1) as usize)
}

/// The 0-based index of `n` for the retained `if (N < 1 || N > NbExt())` test: a null
/// or negative index raises index-range failure before `NbExt()` is called (and can
/// raise not-done failure).
pub(crate) fn extremum_index(
    member: &str,
    n: i32,
    nb_ext: impl FnOnce() -> Result<i32, Failure>,
) -> Result<usize, Failure> {
    let count: i32 = if n < 1 { 0 } else { nb_ext()? };
    check_index(member, n, count)
}

/// The 1-based element `n` of a retained sequence or array, `index-range failure`
/// outside it (the checked `Sequence::Value`).
pub(crate) fn sequence_value<T: Copy>(member: &str, values: &[T], n: i32) -> Result<T, Failure> {
    let index: usize = check_index(member, n, values.len() as i32)?;
    Ok(values[index])
}
