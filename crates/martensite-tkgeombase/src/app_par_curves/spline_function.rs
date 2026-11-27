//! `AppParCurves::SplineFunction`: the B-spline basis matrix and its derivative at
//! parameters, with the span index of each parameter.
use martensite_tkernel::standard::Failure;
use martensite_tkmath::bsplclib;
use martensite_tkmath::math::{IntegerVector, Matrix, Vector};

// flatknots(i), checked (the retained file compiles the checks out).
#[inline]
fn knot(flatknots: &Vector<'_>, i: i32) -> Result<f64, Failure> {
    flatknots.value(i).copied()
}

// The deg + 1 non-null B-spline values (and derivatives) at u in span `kindex`, by
// the retained triangular recurrence, written into `poles` and `dpoles` (length >= deg + 2).
fn fill_local_basis(
    deg: i32,
    flatknots: &Vector<'_>,
    kindex: i32,
    u: f64,
    poles: &mut [f64],
    dpoles: &mut [f64],
) -> Result<(), Failure> {
    poles[1] = 1.;
    for qq in 2..=deg as usize {
        poles[qq] = 0.;
        for pp in 1..qq {
            let (left, right) = (
                knot(flatknots, kindex + pp as i32)?,
                knot(flatknots, kindex - qq as i32 + pp as i32 + 1)?,
            );
            let inverse: f64 = 1.0 / (left - right);
            let saved: f64 = (u - right) * inverse * poles[pp];
            poles[pp] *= (left - u) * inverse;
            poles[pp] += poles[qq];
            poles[qq] = saved;
        }
    }
    let len = deg as usize + 2;
    dpoles[..len].copy_from_slice(&poles[..len]);
    last_degree(deg, flatknots, kindex, u, [poles, dpoles])
}

// The last raise of the recurrence, which also yields the derivatives.
fn last_degree(
    deg: i32,
    flatknots: &Vector<'_>,
    kindex: i32,
    u: f64,
    [poles, dpoles]: [&mut [f64]; 2],
) -> Result<(), Failure> {
    let qq: usize = deg as usize + 1;
    let (mut locqq, mut locdqq): (f64, f64) = (0., 0.);
    for pp in 1..=deg as usize {
        let (left, right) = (
            knot(flatknots, kindex + pp as i32)?,
            knot(flatknots, kindex - qq as i32 + pp as i32 + 1)?,
        );
        let inverse: f64 = 1.0 / (left - right);
        let saved: f64 = (u - right) * inverse * poles[pp];
        poles[pp] *= (left - u) * inverse;
        poles[pp] += locqq;
        locqq = saved;
        let local_inverse: f64 = f64::from(deg) * inverse;
        let saved: f64 = local_inverse * dpoles[pp];
        dpoles[pp] *= -local_inverse;
        dpoles[pp] += locdqq;
        locdqq = saved;
    }
    poles[qq] = locqq;
    dpoles[qq] = locdqq;
    Ok(())
}

// A(i, j) = value and DA(i, j) = derivative (DA first, as the retained chained assignment).
#[inline]
fn put(
    a: &mut Matrix<'_>,
    da: &mut Matrix<'_>,
    [i, j]: [i32; 2],
    [value, derivative]: [f64; 2],
) -> Result<(), Failure> {
    *da.change_value(i, j)? = derivative;
    *a.change_value(i, j)? = value;
    Ok(())
}

/// `AppParCurves::SplineFunction(NbPoles, Degree, Parameters, FlatKnots, A, DA, Index)`:
/// row `i` of `A` and `DA` holds the B-spline functions and derivatives at
/// `Parameters(i)` (zero outside the span) and `Index(i)` the span start minus one;
/// the span search starts from the previous parameter's span.
/// `spline_function(4, 3, &parameters, &flat_knots, &mut a, &mut da, &mut index)?`.
pub fn spline_function(
    nb_poles: i32,
    degree: i32,
    parameters: &Vector<'_>,
    flatknots: &Vector<'_>,
    a: &mut Matrix<'_>,
    da: &mut Matrix<'_>,
    index: &mut IntegerVector<'_>,
) -> Result<(), Failure> {
    let deg1: i32 = degree + 1;
    let buf_len: usize = usize::try_from(deg1 + 1).map_err(|_| {
        Failure::out_of_range(&format!("AppParCurves::SplineFunction: degree {degree}"))
    })?;
    let mut stack_p: [f64; 32] = [0.0; 32];
    let mut stack_dp: [f64; 32] = [0.0; 32];
    let mut heap_p: Vec<f64>;
    let mut heap_dp: Vec<f64>;
    let (poles, dpoles): (&mut [f64], &mut [f64]) = if buf_len <= stack_p.len() {
        (&mut stack_p[..buf_len], &mut stack_dp[..buf_len])
    } else {
        heap_p = vec![0.0; buf_len];
        heap_dp = vec![0.0; buf_len];
        (&mut heap_p, &mut heap_dp)
    };
    let aflatknots = flatknots.array1();
    let mut oldkindex: i32 = 1;
    for i in parameters.lower()..=parameters.upper() {
        let u: f64 = parameters[i];
        let (mut kindex, mut new_u) = (oldkindex, u);
        bsplclib::locate_parameter_flat_knots(
            degree,
            &aflatknots,
            u,
            false,
            deg1,
            nb_poles + 1,
            &mut kindex,
            &mut new_u,
        )?;
        oldkindex = kindex;
        *index.change_value(i)? = kindex - degree - 1;
        poles.fill(0.0);
        fill_local_basis(degree, flatknots, kindex, u, poles, dpoles)?;
        for j in 1..=deg1 {
            put(
                a,
                da,
                [i, j + oldkindex - deg1],
                [poles[j as usize], dpoles[j as usize]],
            )?;
        }
        for j in (1..oldkindex - degree).chain(oldkindex + 1..=nb_poles) {
            put(a, da, [i, j], [0., 0.])?;
        }
    }
    Ok(())
}
