//! Truncation of Legendre and Jacobi curves to a tolerance: `mmtrpjj_` and the
//! file-static `mmtrpj0_`, `mmtrpj2_`, `mmtrpj4_`, `mmtrpj6_`.
use super::TruncationPrecisionAccess;
use super::approx_errors::{XMAXJ_ORDER2, XMAXJ_ORDER4, XMAXJ_ORDER6};
use super::mmeps1_;
use super::truncation_access::{BorrowedTruncationReals, TruncationRealAccess};
use super::truncation_integers::{BorrowedTruncationIntegers, TruncationIntegerAccess};
use super::truncation_precision::BorrowedTruncationPrecision;
use martensite_tkernel::standard::Failure;

/// Finds how many coefficients of a Jacobi curve of continuity order `iordre` keep
/// the truncation error within `epsi3d` (`mmtrpjj_`): `ncfnew` receives the count,
/// `errmax` the error of the dropped tail and `ycvmax` its bound per dimension.
/// Orders beyond 1 all use the order-2 (`mmtrpj6_`) tables; returns 0.
/// `mmtrpjj_(6, 3, 6, 1e-6, 0, &crvlgd, &mut ycvmax, &mut errmax, &mut ncfnew)`.
pub fn mmtrpjj_(
    ncofmx: i32,
    ndimen: i32,
    ncoeff: i32,
    epsi3d: f64,
    iordre: i32,
    crvlgd: &[f64],
    ycvmax: &mut [f64],
    errmax: &mut f64,
    ncfnew: &mut i32,
) -> i32 {
    let mut access = BorrowedTruncationPrecision {
        precision: epsi3d,
        error: errmax,
    };
    mmtrpjj_with_precision_access(
        ncofmx,
        ndimen,
        ncoeff,
        iordre,
        crvlgd,
        ycvmax,
        &mut access,
        ncfnew,
    )
}

pub(crate) fn mmtrpjj_with_precision_access(
    ncofmx: i32,
    ndimen: i32,
    ncoeff: i32,
    iordre: i32,
    crvlgd: &[f64],
    ycvmax: &mut [f64],
    access: &mut impl TruncationPrecisionAccess,
    ncfnew: &mut i32,
) -> i32 {
    let mut reals = BorrowedTruncationReals::new(crvlgd, ycvmax, access);
    mmtrpjj_with_real_access(ncofmx, ndimen, ncoeff, iordre, &mut reals, ncfnew)
        .expect("borrowed truncation real access has no fallible storage")
}

pub(crate) fn mmtrpjj_with_real_access(
    ncofmx: i32,
    ndimen: i32,
    ncoeff: i32,
    iordre: i32,
    access: &mut impl TruncationRealAccess,
    ncfnew: &mut i32,
) -> Result<i32, Failure> {
    let mut integers = BorrowedTruncationIntegers::new(ncofmx, ndimen, ncoeff, iordre, ncfnew);
    mmtrpjj_with_caller_access(&mut integers, access)
}

pub(super) fn mmtrpjj_with_caller_access(
    integers: &mut impl TruncationIntegerAccess,
    access: &mut impl TruncationRealAccess,
) -> Result<i32, Failure> {
    let stride = integers.stride()?;
    let order = integers.order()?;
    let mut curve = JacobiCurve { stride, integers };
    match (order + 1) << 1 {
        0 => mmtrpj0_(&mut curve, access)?,
        2 => mmtrpj_(&mut curve, 2, &XMAXJ_ORDER2, access)?,
        4 => mmtrpj_(&mut curve, 4, &XMAXJ_ORDER4, access)?,
        _ => mmtrpj_(&mut curve, 6, &XMAXJ_ORDER6, access)?,
    }
    Ok(0)
}

/// A `(stride, dimension)` coefficient array with a live coefficient count.
/// Only coefficient stride is captured before output-count initialization.
struct JacobiCurve<'a, Integers> {
    stride: i32,
    integers: &'a mut Integers,
}

fn initialize_bounds(
    curve: &mut JacobiCurve<'_, impl TruncationIntegerAccess>,
    minimum: i32,
    access: &mut impl TruncationRealAccess,
) -> Result<(), Failure> {
    curve.integers.set_retained(minimum)?;
    for component in 0..curve.integers.dimension()? {
        access.set_bound(component, 0.)?;
    }
    access.set_error(0.)
}

fn accumulate_bound(
    curve: &JacobiCurve<'_, impl TruncationIntegerAccess>,
    coefficient: i32,
    basis: f64,
    access: &mut impl TruncationRealAccess,
) -> Result<f64, Failure> {
    for component in 0..curve.integers.dimension()? {
        let value = access.coefficient(coefficient - 1 + component * curve.stride)?;
        let bound = access.bound(component)? + value.abs() * basis;
        access.set_bound(component, bound)?;
    }
    access.bound_norm(curve.integers.dimension()?)
}

/// `mmtrpj0_`: drop Legendre coefficients from the highest down while the
/// accumulated bound stays within precision. Bounds are written before the live
/// precision read; the same slots may be shared.
fn mmtrpj0_(
    curve: &mut JacobiCurve<'_, impl TruncationIntegerAccess>,
    access: &mut impl TruncationRealAccess,
) -> Result<(), Failure> {
    initialize_bounds(curve, 1, access)?;
    for index in (2..=curve.integers.count()?).rev() {
        let basis = ((f64::from(index - 1) * 2. + 1.) / 2.).sqrt();
        let error = accumulate_bound(curve, index, basis, access)?;
        if error > access.precision()? {
            curve.integers.set_retained(index)?;
            return Ok(());
        }
        access.set_error(error)?;
    }
    Ok(())
}

/// `mmtrpj2_`, `mmtrpj4_` and `mmtrpj6_` keep at least the constraint
/// coefficients; if every tail coefficient goes, label L400 also drops
/// constraint coefficients whose sum remains below `eps1`.
fn mmtrpj_(
    curve: &mut JacobiCurve<'_, impl TruncationIntegerAccess>,
    minimum: i32,
    maxima: &[f64],
    access: &mut impl TruncationRealAccess,
) -> Result<(), Failure> {
    initialize_bounds(curve, minimum, access)?;
    for index in (minimum + 1..=curve.integers.count()?).rev() {
        let basis = maxima[(index - minimum - 1) as usize];
        let error = accumulate_bound(curve, index, basis, access)?;
        if error > access.precision()? {
            curve.integers.set_retained(index)?;
            break;
        }
        access.set_error(error)?;
    }
    if curve.integers.retained()? == minimum {
        let retained = significant_constraints(curve, minimum, access)?;
        curve.integers.set_retained(retained)?;
    }
    Ok(())
}

/// The L400 search finds the highest constraint coefficient `2..=minimum`
/// exceeding `eps1` in sum, or 1. It reads current coefficients after all
/// bound/error stores.
fn significant_constraints(
    curve: &JacobiCurve<'_, impl TruncationIntegerAccess>,
    minimum: i32,
    access: &impl TruncationRealAccess,
) -> Result<i32, Failure> {
    let mut epsilon = 0.;
    mmeps1_(&mut epsilon);
    for index in (2..=minimum).rev() {
        let mut bound = 0.;
        for component in 0..curve.integers.dimension()? {
            bound += access
                .coefficient(index - 1 + component * curve.stride)?
                .abs();
        }
        if bound > epsilon {
            return Ok(index);
        }
    }
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legendre_truncation_stops_at_the_first_large_tail() {
        let crvlgd: [f64; 4] = [1.0, 0.5, 1e-9, 1e-10];
        let (mut ycvmax, mut errmax, mut ncfnew) = ([0.0; 1], 0.0, 0);
        mmtrpjj_(
            4,
            1,
            4,
            1e-6,
            -1,
            &crvlgd,
            &mut ycvmax,
            &mut errmax,
            &mut ncfnew,
        );
        assert_eq!(ncfnew, 2);
        assert!(errmax > 0.0 && errmax < 1e-6);
    }

    #[test]
    fn jacobi_truncation_drops_negligible_constraint_coefficients() {
        let crvlgd: [f64; 5] = [1.0, 0.0, 0.0, 0.0, 0.0];
        let (mut ycvmax, mut errmax, mut ncfnew) = ([0.0; 1], 0.0, 0);
        mmtrpjj_(
            5,
            1,
            5,
            1e-6,
            0,
            &crvlgd,
            &mut ycvmax,
            &mut errmax,
            &mut ncfnew,
        );
        assert_eq!((ncfnew, errmax), (1, 0.0));
        let crvlgd: [f64; 5] = [1.0, 1.0, 1.0, 0.0, 1.0];
        mmtrpjj_(
            5,
            1,
            5,
            1e-6,
            0,
            &crvlgd,
            &mut ycvmax,
            &mut errmax,
            &mut ncfnew,
        );
        assert_eq!(ncfnew, 5);
    }
}
