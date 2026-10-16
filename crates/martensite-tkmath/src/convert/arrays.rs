//! Array allocation and copy helpers shared by the Convert classes.
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;

/// `Array1(1, count)`; a negative size is the allocation failure;
/// `one_based::<f64>(3)?`.
pub(super) fn one_based<T: Default>(count: i32) -> Result<Array1<'static, T>, Failure> {
    Array1::with_bounds(1, count).map_err(|error| {
        Failure::with_type(
            "Standard_OutOfMemory",
            &format!("Convert: cannot allocate an array 1..{count}: {error}"),
            "",
        )
    })
}

/// Owned copy keeping the bounds (the Array1 copy constructor);
/// `owned_copy(&knots).lower() == knots.lower()`.
pub(super) fn owned_copy<T: Copy + Default>(array: &Array1<'_, T>) -> Array1<'static, T> {
    let mut copy: Array1<'static, T> = Array1::with_size(array.len());
    copy.update_lower_bound(array.lower());
    copy.as_mut_slice().copy_from_slice(array.as_slice());
    copy
}
