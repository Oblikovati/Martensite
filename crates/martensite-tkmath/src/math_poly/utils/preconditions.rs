//! Preconditions of the public `math_poly::utils` helpers. Raw pointers
//! and fixed `std::array` buffers would be read or written past when these do not hold
//! (undefined behaviour); out-of-range failure naming the offending sizes
//! and the expected shape is raised instead.
use crate::math_poly::MAX_POLY_DEGREE;
use martensite_tkernel::standard::Failure;

/// Refuses a degree outside `minimum..=MAX_POLY_DEGREE`, the range the fixed
/// `THE_MAX_POLY_DEGREE + 1` arrays hold; returns it as an index.
/// `fixed_degree("math_poly::utils::Evaluate", 2, 0)? == 2`.
pub(super) fn fixed_degree(member: &str, degree: i32, minimum: i32) -> Result<usize, Failure> {
    if degree < minimum || degree > MAX_POLY_DEGREE as i32 {
        return Err(Failure::out_of_range(&format!(
            "{member}: degree {degree}; expected {minimum} <= degree <= {MAX_POLY_DEGREE}"
        )));
    }
    Ok(degree as usize)
}

/// Refuses coefficients that do not hold `c[0..=degree]` for a degree of at least
/// `minimum` (without upper limit: the caller's pointer is all code reads).
/// `slice_degree("math_poly::utils::EvaluateComplex", 3, 2, 0)? == 2`.
pub(super) fn slice_degree(
    member: &str,
    len: usize,
    degree: i32,
    minimum: i32,
) -> Result<usize, Failure> {
    if degree < minimum || len <= degree as usize {
        return Err(Failure::out_of_range(&format!(
            "{member}: degree {degree} with {len} coefficients; expected degree >= {minimum} \
             and at least degree + 1 coefficients"
        )));
    }
    Ok(degree as usize)
}

/// Both limits: a caller slice copied into a fixed `THE_MAX_POLY_DEGREE + 1` array.
/// `bounded_degree("math_poly::utils::HasMultiplicity", 3, 2, 0)? == 2`.
pub(super) fn bounded_degree(
    member: &str,
    len: usize,
    degree: i32,
    minimum: i32,
) -> Result<usize, Failure> {
    fixed_degree(member, degree, minimum)?;
    slice_degree(member, len, degree, minimum)
}

/// Refuses a count that already fills a `capacity`-slot buffer about to take one more
/// item. `free_slot("math_poly::utils::AppendUnique", 3, 4, "roots")?`.
pub(super) fn free_slot(
    member: &str,
    count: usize,
    capacity: usize,
    items: &str,
) -> Result<(), Failure> {
    if count < capacity {
        return Ok(());
    }
    Err(Failure::out_of_range(&format!(
        "{member}: {items} {} of a {capacity}-slot buffer; expected at most {capacity} {items}",
        count + 1
    )))
}

/// Refuses a stored count above the buffer holding it.
/// `stored_count("math_poly::utils::ConsolidateRealRoots", 3, 20, "real roots")?`.
pub(super) fn stored_count(
    member: &str,
    count: usize,
    capacity: usize,
    items: &str,
) -> Result<(), Failure> {
    if count <= capacity {
        return Ok(());
    }
    Err(Failure::out_of_range(&format!(
        "{member}: {count} {items} in a {capacity}-slot buffer; expected at most {capacity}"
    )))
}
