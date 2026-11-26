//! `AppParCurves::BernsteinMatrix`, `Bernstein` and `SecondDerivativeBernstein`:
//! the Bernstein basis of degree NbPoles - 1 and its derivatives at parameters.
use martensite_tkernel::standard::Failure;
use martensite_tkmath::math::{Matrix, Vector};

// The Bernstein values of degree NbPoles - 2 at u, B(0..n), by the retained
// de Casteljau recurrence.
#[inline]
fn fill_lower_basis(u0: f64, b: &mut [f64]) {
    let n = b.len();
    if n == 0 {
        return;
    }
    b[0] = 1.;
    for id in 1..n {
        let y0: f64 = b[0];
        let mut y1: f64 = u0 * y0;
        b[0] = y0 - y1;
        for slot in &mut b[1..id] {
            let xs: f64 = y1;
            let y0: f64 = *slot;
            y1 = u0 * y0;
            *slot = y0 - y1 + xs;
        }
        b[id] = y1;
    }
}

fn with_lower_basis_buf<R>(
    nb_poles: i32,
    f: impl FnOnce(&mut [f64]) -> Result<R, Failure>,
) -> Result<R, Failure> {
    if nb_poles < 2 {
        return Err(Failure::out_of_range(&format!(
            "AppParCurves::Bernstein: NbPoles = {nb_poles}; expected >= 2"
        )));
    }
    let n = (nb_poles - 1) as usize;
    let mut stack = [0.0_f64; 32];
    if n <= stack.len() {
        f(&mut stack[..n])
    } else {
        let mut heap = vec![0.0_f64; n];
        f(&mut heap)
    }
}

// A(i, j) = value, with the checked access the retained file compiles out (an index
// outside A is undefined natively, index-range failure here).
#[inline]
fn put(a: &mut Matrix<'_>, i: i32, j: i32, value: f64) -> Result<(), Failure> {
    *a.change_value(i, j)? = value;
    Ok(())
}

/// `AppParCurves::BernsteinMatrix(NbPoles, U, A)`: `A(i, j)` is the Bernstein
/// function j of degree NbPoles - 1 at `U(i)` (rows as the indexes of `u`, columns
/// from 1); `bernstein_matrix(4, &u, &mut a)?`.
pub fn bernstein_matrix(nb_poles: i32, u: &Vector<'_>, a: &mut Matrix<'_>) -> Result<(), Failure> {
    with_lower_basis_buf(nb_poles, |b| {
        let n = b.len();
        for i in u.lower()..=u.upper() {
            let u0: f64 = u[i];
            let u1: f64 = 1. - u0;
            fill_lower_basis(u0, b);
            put(a, i, 1, u1 * b[0])?;
            put(a, i, nb_poles, u0 * b[n - 1])?;
            for j in 2..=nb_poles - 1 {
                let idx = (j - 1) as usize;
                put(a, i, j, u1 * b[idx] + u0 * b[idx - 1])?;
            }
        }
        Ok(())
    })
}

/// `AppParCurves::Bernstein(NbPoles, U, A, DA)`: the Bernstein matrix and its
/// derivative matrix; `bernstein(4, &u, &mut a, &mut da)?`.
pub fn bernstein(
    nb_poles: i32,
    u: &Vector<'_>,
    a: &mut Matrix<'_>,
    da: &mut Matrix<'_>,
) -> Result<(), Failure> {
    let ndeg: f64 = f64::from(nb_poles - 1);
    with_lower_basis_buf(nb_poles, |b| {
        let n = b.len();
        for i in u.lower()..=u.upper() {
            let u0: f64 = u[i];
            let u1: f64 = 1. - u0;
            fill_lower_basis(u0, b);
            put(da, i, 1, -ndeg * b[0])?;
            put(da, i, nb_poles, ndeg * b[n - 1])?;
            put(a, i, 1, u1 * b[0])?;
            put(a, i, nb_poles, u0 * b[n - 1])?;
            for j in 2..=nb_poles - 1 {
                let idx = (j - 1) as usize;
                let (bj, bj1) = (b[idx], b[idx - 1]);
                put(da, i, j, ndeg * (bj1 - bj))?;
                put(a, i, j, u1 * bj + u0 * bj1)?;
            }
        }
        Ok(())
    })
}

/// `AppParCurves::SecondDerivativeBernstein(U, DDA)`: the second derivatives of the
/// Bernstein functions of degree `DDA.Length() - 1` at `u`, written from index 1 (the
/// retained code assumes DDA starts at 1); `second_derivative_bernstein(0.5, &mut dda)?`.
pub fn second_derivative_bernstein(u: f64, dda: &mut Vector<'_>) -> Result<(), Failure> {
    let deg: i32 = dda.length() - 1;
    if deg == 1 {
        *dda.change_value(1)? = 0.;
        *dda.change_value(2)? = 0.;
        return Ok(());
    }
    if deg == 2 {
        (
            *dda.change_value(1)?,
            *dda.change_value(2)?,
            *dda.change_value(3)?,
        ) = (2., -4., 2.);
        return Ok(());
    }
    let n4: f64 = f64::from(deg * (deg - 1));
    with_lower_basis_buf(deg, |b| {
        fill_lower_basis(u, b);
        let n = b.len();
        *dda.change_value(1)? = n4 * b[0];
        *dda.change_value(2)? = n4 * (-2. * b[0] + b[1]);
        *dda.change_value(deg)? = n4 * (b[n - 2] - 2. * b[n - 1]);
        *dda.change_value(deg + 1)? = n4 * b[n - 1];
        for j in 2..=deg - 2 {
            let idx = (j - 1) as usize;
            *dda.change_value(j + 1)? = n4 * (b[idx - 1] - 2. * b[idx] + b[idx + 1]);
        }
        Ok(())
    })
}
