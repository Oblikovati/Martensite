//! The `HArray1` allocations of the AdvApp2Var classes: 1-based arrays
//! whose failures name the member that allocated them.
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;

/// A zero-filled array `1..=size` (`new HArray1<T>(1, size)`, which is
/// uninitialised natively for doubles); `one_based(3, "Patch")?`.
pub(super) fn one_based<T: Default>(
    size: i32,
    member: &str,
) -> Result<Array1<'static, T>, Failure> {
    Array1::with_bounds(1, size).map_err(|error| error.into_failure(member))
}

/// The array `1..=values.len()` holding `values`; `one_based_from(&[-1.0, 1.0])`.
pub(super) fn one_based_from<T: Default + Copy>(values: &[T]) -> Array1<'static, T> {
    let mut array: Array1<'static, T> = Array1::with_size(values.len());
    array.update_lower_bound(1);
    array.as_mut_slice().copy_from_slice(values);
    array
}

/// The copy of an optional table (`HArray->Array1()`), or a null-object failure naming
/// what is missing; `required(iso.som_tab(), "Patch: iso sums")?`.
pub(super) fn required<T>(value: Option<T>, what: &str) -> Result<T, Failure> {
    value.ok_or_else(|| {
        Failure::with_message("Standard_NullObject", &format!("{what}: null handle"))
    })
}
