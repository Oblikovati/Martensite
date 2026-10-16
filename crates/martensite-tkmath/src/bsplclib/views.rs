//! Views `Array(Lower..Upper')` over the leading part of an array, as the
//! code builds with `Array1(Array(Lower), Lower, Upper')`.
//!
//! A view reaching past the underlying storage would address foreign memory;
//! here it is rejected with out-of-range failure naming the requested bound.
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;

fn prefix_length<T>(function: &str, array: &Array1<'_, T>, upper: i32) -> Result<usize, Failure> {
    let length: i64 = i64::from(upper) - i64::from(array.lower()) + 1;
    if length < 0 || length > array.len() as i64 {
        return Err(Failure::out_of_range(&format!(
            "{function}: view {}..{upper} exceeds the array bounds {}..{}",
            array.lower(),
            array.lower(),
            array.upper()
        )));
    }
    Ok(length as usize)
}

/// Read-only view `array(Lower..upper)`; `prefix_view("f", &poles, 4)?`.
pub(crate) fn prefix_view<'s, T: Clone>(
    function: &str,
    array: &'s Array1<'_, T>,
    upper: i32,
) -> Result<Array1<'s, T>, Failure> {
    let length: usize = prefix_length(function, array, upper)?;
    Ok(Array1::from_shared(
        &array.as_slice()[..length],
        array.lower(),
    ))
}

/// Writable view `array(Lower..upper)` sharing the storage; `prefix_view_mut("f", &mut poles, 4)?`.
pub(crate) fn prefix_view_mut<'s, T>(
    function: &str,
    array: &'s mut Array1<'_, T>,
    upper: i32,
) -> Result<Array1<'s, T>, Failure> {
    let length: usize = prefix_length(function, array, upper)?;
    let lower: i32 = array.lower();
    let mut view: Array1<'s, T> = Array1::from_slice(&mut array.as_mut_slice()[..length]);
    view.update_lower_bound(lower);
    Ok(view)
}

/// Owned array with bounds 1..count, as `Array1<double>(1, count)`;
/// negative counts give the "bad range" failure; `one_based(3)?`.
pub(crate) fn one_based<T: Default>(
    function: &str,
    count: i32,
) -> Result<Array1<'static, T>, Failure> {
    Array1::with_bounds(1, count).map_err(|error| {
        Failure::with_type(
            "Standard_RangeError",
            &format!("{function}: cannot allocate 1..{count}: {error}"),
            "",
        )
    })
}
