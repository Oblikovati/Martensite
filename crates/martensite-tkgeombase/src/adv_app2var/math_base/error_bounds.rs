//! The same ordered bounds arithmetic accepts borrowed or original-owner scalars.
use super::approx_errors::{XMAXJ_ORDER2, XMAXJ_ORDER4, XMAXJ_ORDER6};
use super::truncation_access::TruncationRealAccess;
use martensite_tkernel::standard::Failure;

struct TruncatedCurve {
    stride: i32,
    dimensions: i32,
    coefficients: i32,
    retained: i32,
}
pub(crate) fn mmaperx_with_real_access(
    stride: i32,
    dimensions: i32,
    coefficients: i32,
    order: i32,
    retained: i32,
    access: &mut impl TruncationRealAccess,
    code: &mut i32,
) -> Result<i32, Failure> {
    *code = 0;
    let curve = TruncatedCurve {
        stride,
        dimensions,
        coefficients,
        retained,
    };
    match (order + 1) << 1 {
        0 => mmaper0_(&curve, access)?,
        2 => mmaperj_(&curve, 3, &XMAXJ_ORDER2, access)?,
        4 => mmaperj_(&curve, 5, &XMAXJ_ORDER4, access)?,
        6 => mmaperj_(&curve, 7, &XMAXJ_ORDER6, access)?,
        _ => *code = 1,
    }
    Ok(0)
}
fn reset_bounds(
    curve: &TruncatedCurve,
    access: &mut impl TruncationRealAccess,
) -> Result<(), Failure> {
    for component in 0..curve.dimensions {
        access.set_bound(component, 0.)?;
    }
    Ok(())
}
fn add_coefficient_bound(
    curve: &TruncatedCurve,
    index: i32,
    basis: f64,
    access: &mut impl TruncationRealAccess,
) -> Result<(), Failure> {
    for component in 0..curve.dimensions {
        let coefficient = access.coefficient(index - 1 + component * curve.stride)?;
        let bound = access.bound(component)? + coefficient.abs() * basis;
        access.set_bound(component, bound)?;
    }
    Ok(())
}
fn mmaper0_(curve: &TruncatedCurve, access: &mut impl TruncationRealAccess) -> Result<(), Failure> {
    reset_bounds(curve, access)?;
    let mut first = 1;
    if curve.retained >= first {
        first = curve.retained + 1;
    }
    for index in first..=curve.coefficients {
        let basis = ((f64::from(index - 1) * 2. + 1.) / 2.).sqrt();
        add_coefficient_bound(curve, index, basis, access)?;
    }
    let error = access.bound_norm(curve.dimensions)?;
    access.set_error(error)
}
fn mmaperj_(
    curve: &TruncatedCurve,
    first_basis: i32,
    maxima: &[f64],
    access: &mut impl TruncationRealAccess,
) -> Result<(), Failure> {
    reset_bounds(curve, access)?;
    let first = first_basis.max(curve.retained + 1);
    for index in first..=curve.coefficients {
        let basis = maxima[(index - first_basis) as usize];
        add_coefficient_bound(curve, index, basis, access)?;
    }
    let error = access.bound_norm(curve.dimensions)?;
    access.set_error(error)
}
