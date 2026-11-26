//! Evaluation of a span cached as polynomial coefficients in the normalized
//! parameter `(U - CacheParameter) / SpanLength`.
use super::MAX_DEGREE;
use super::flat_points;
use super::point::{CurveD3, CurvePoint};
use crate::plib::{eval_polynomial, no_derivative_eval_polynomial, rational_derivatives};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;

const MAX_STACK_FLAT: usize = (MAX_DEGREE as usize + 1) * 3;

#[inline]
fn with_flat_points<P: CurvePoint, R>(poles: &Array1<'_, P>, f: impl FnOnce(&[f64]) -> R) -> R {
    let d = P::DIMENSION;
    let total = poles.len() * d;
    if total <= MAX_STACK_FLAT {
        let mut stack = [0.0_f64; MAX_STACK_FLAT];
        for (idx, p) in poles.as_slice().iter().enumerate() {
            let base = idx * d;
            for k in 0..d {
                stack[base + k] = p.pole_coordinate(k);
            }
        }
        return f(&stack[..total]);
    }
    let flat = flat_points(poles);
    f(&flat)
}

/// Degree of a cached span and its parametrization.
#[derive(Clone, Copy, Debug)]
pub struct CacheSpan {
    /// Degree of the cached polynomial.
    pub degree: i32,
    /// Parameter where the cache was built.
    pub cache_parameter: f64,
    /// Span length used to normalize the parameter.
    pub span_length: f64,
}

impl CacheSpan {
    /// The span of a Bezier coefficient array: degree `count-1` on `[0, 1]`
    /// (the CoefsD* shortcuts); `CacheSpan::bezier(4).degree == 3`.
    pub fn bezier(count: usize) -> Self {
        Self {
            degree: count as i32 - 1,
            cache_parameter: 0.,
            span_length: 1.,
        }
    }

    fn normalized(&self, u: f64) -> f64 {
        (u - self.cache_parameter) / self.span_length
    }
}

/// Point of a cached 2D/3D span (`bsplclib::cache_d0`); `CoefsD0` is
/// `cache_d0(u, CacheSpan::bezier(poles.len()), ..)`;
/// `bsplclib::cache_d0(0.5, span, &cache_poles, None)?`.
pub fn cache_d0<P: CurvePoint>(
    u: f64,
    span: CacheSpan,
    poles: &Array1<'_, P>,
    weights: Option<&Array1<'_, f64>>,
) -> Result<P, Failure> {
    let d: usize = P::DIMENSION;
    let new_parameter: f64 = span.normalized(u);
    let mut point_buf = [0.0_f64; 3];
    let point = &mut point_buf[..d];
    let degree: i32 = span.degree;
    with_flat_points(poles, |flat| {
        no_derivative_eval_polynomial(
            new_parameter,
            degree,
            d as i32,
            degree * d as i32,
            flat,
            point,
        )
    })?;
    if let Some(weights) = weights {
        let mut inverse: [f64; 1] = [0.0];
        no_derivative_eval_polynomial(
            new_parameter,
            degree,
            1,
            degree,
            weights.as_slice(),
            &mut inverse,
        )?;
        let inverse: f64 = 1.0e0 / inverse[0];
        for value in point.iter_mut() {
            *value *= inverse;
        }
    }
    Ok(P::point_from(point))
}

/// Point and first derivative of a cached span (`bsplclib::cache_d1`);
/// `bsplclib::cache_d1(0.5, span, &cache_poles, None)?`.
pub fn cache_d1<P: CurvePoint>(
    u: f64,
    span: CacheSpan,
    poles: &Array1<'_, P>,
    weights: Option<&Array1<'_, f64>>,
) -> Result<(P, P::Vector), Failure> {
    let d: usize = P::DIMENSION;
    let new_parameter: f64 = span.normalized(u);
    let mut pd_buf = [0.0_f64; 6];
    let pd = &mut pd_buf[..2 * d];
    with_flat_points(poles, |flat| {
        eval_polynomial(new_parameter, 1, span.degree, d as i32, flat, pd)
    })?;
    for value in &mut pd[d..] {
        *value /= span.span_length;
    }
    if let Some(weights) = weights {
        let mut wd: [f64; 2] = [0.0; 2];
        eval_polynomial(
            new_parameter,
            1,
            span.degree,
            1,
            weights.as_slice(),
            &mut wd,
        )?;
        wd[1] /= span.span_length;
        rational_in_place(1, d, pd, &wd)?;
    }
    Ok((P::point_from(pd), P::vector_from(&pd[d..])))
}

// plib::rational_derivatives called with the same array as input and output: each
// row is read before it is written, so a copy of the input is equivalent.
fn rational_in_place(request: i32, d: usize, pd: &mut [f64], wd: &[f64]) -> Result<(), Failure> {
    let mut input = [0.0_f64; 12];
    let len = pd.len();
    input[..len].copy_from_slice(pd);
    rational_derivatives(request, d as i32, &input[..len], wd, pd)
}

/// Point and two derivatives of a cached span (`bsplclib::cache_d2`);
/// `bsplclib::cache_d2(0.5, span, &cache_poles, None)?`.
pub fn cache_d2<P: CurvePoint>(
    u: f64,
    span: CacheSpan,
    poles: &Array1<'_, P>,
    weights: Option<&Array1<'_, f64>>,
) -> Result<(P, P::Vector, P::Vector), Failure> {
    let d: usize = P::DIMENSION;
    let new_parameter: f64 = span.normalized(u);
    let mut pd_buf = [0.0_f64; 9];
    let pd = &mut pd_buf[..3 * d];
    with_flat_points(poles, |flat| {
        eval_polynomial(new_parameter, 2, span.degree, d as i32, flat, pd)
    })?;
    let end_index: usize = span.degree.clamp(0, 2) as usize;
    scale_rows(pd, d, end_index, span.span_length);
    nullify_rows(pd, d, span.degree, 2);
    if let Some(weights) = weights {
        let mut wd: [f64; 3] = [0.0; 3];
        eval_polynomial(
            new_parameter,
            2,
            span.degree,
            1,
            weights.as_slice(),
            &mut wd,
        )?;
        nullify_rows(&mut wd, 1, span.degree, 2);
        scale_rows(&mut wd, 1, end_index, span.span_length);
        rational_in_place(2, d, pd, &wd)?;
    }
    Ok((
        P::point_from(pd),
        P::vector_from(&pd[d..]),
        P::vector_from(&pd[2 * d..]),
    ))
}

/// Point and three derivatives of a cached span (`bsplclib::cache_d3`);
/// `bsplclib::cache_d3(0.5, span, &cache_poles, None)?`.
pub fn cache_d3<P: CurvePoint>(
    u: f64,
    span: CacheSpan,
    poles: &Array1<'_, P>,
    weights: Option<&Array1<'_, f64>>,
) -> Result<CurveD3<P>, Failure> {
    let d: usize = P::DIMENSION;
    let new_parameter: f64 = span.normalized(u);
    let mut pd_buf = [0.0_f64; 12];
    let pd = &mut pd_buf[..4 * d];
    with_flat_points(poles, |flat| {
        eval_polynomial(new_parameter, 3, span.degree, d as i32, flat, pd)
    })?;
    // D3 clears the rows above the degree before scaling, D2 after.
    nullify_rows(pd, d, span.degree, 3);
    let end_index: usize = span.degree.clamp(0, 3) as usize;
    scale_rows(pd, d, end_index, span.span_length);
    if let Some(weights) = weights {
        let mut wd: [f64; 4] = [0.0; 4];
        eval_polynomial(
            new_parameter,
            3,
            span.degree,
            1,
            weights.as_slice(),
            &mut wd,
        )?;
        scale_rows(&mut wd, 1, end_index, span.span_length);
        nullify_rows(&mut wd, 1, span.degree, 3);
        rational_in_place(3, d, pd, &wd)?;
    }
    Ok((
        P::point_from(pd),
        P::vector_from(&pd[d..]),
        P::vector_from(&pd[2 * d..]),
        P::vector_from(&pd[3 * d..]),
    ))
}

// Row r (1..=end_index) *= 1/SpanLength^r, the factor divided by SpanLength each row.
fn scale_rows(values: &mut [f64], d: usize, end_index: usize, span_length: f64) {
    let mut factor: f64 = 1.0e0 / span_length;
    for row in 1..=end_index {
        for value in &mut values[row * d..(row + 1) * d] {
            *value *= factor;
        }
        factor /= span_length;
    }
}

// Clears the derivative rows Degree+1..=last.
fn nullify_rows(values: &mut [f64], d: usize, degree: i32, last: i32) {
    for row in degree + 1..=last {
        if row >= 0 {
            let row: usize = row as usize;
            values[row * d..(row + 1) * d].fill(0.);
        }
    }
}

/// [`cache_d0`] on Bezier coefficients from `poles_coefficients` (`bsplclib::coefs_d0`);
/// `bsplclib::coefs_d0(0.5, &coefficients, None)?`.
pub fn coefs_d0<P: CurvePoint>(
    u: f64,
    poles: &Array1<'_, P>,
    weights: Option<&Array1<'_, f64>>,
) -> Result<P, Failure> {
    cache_d0(u, CacheSpan::bezier(poles.len()), poles, weights)
}

/// [`cache_d1`] on Bezier coefficients (`bsplclib::coefs_d1`);
/// `bsplclib::coefs_d1(0.5, &coefficients, None)?`.
pub fn coefs_d1<P: CurvePoint>(
    u: f64,
    poles: &Array1<'_, P>,
    weights: Option<&Array1<'_, f64>>,
) -> Result<(P, P::Vector), Failure> {
    cache_d1(u, CacheSpan::bezier(poles.len()), poles, weights)
}

/// [`cache_d2`] on Bezier coefficients (`bsplclib::coefs_d2`);
/// `bsplclib::coefs_d2(0.5, &coefficients, None)?`.
pub fn coefs_d2<P: CurvePoint>(
    u: f64,
    poles: &Array1<'_, P>,
    weights: Option<&Array1<'_, f64>>,
) -> Result<(P, P::Vector, P::Vector), Failure> {
    cache_d2(u, CacheSpan::bezier(poles.len()), poles, weights)
}

/// [`cache_d3`] on Bezier coefficients (`bsplclib::coefs_d3`);
/// `bsplclib::coefs_d3(0.5, &coefficients, None)?`.
pub fn coefs_d3<P: CurvePoint>(
    u: f64,
    poles: &Array1<'_, P>,
    weights: Option<&Array1<'_, f64>>,
) -> Result<CurveD3<P>, Failure> {
    cache_d3(u, CacheSpan::bezier(poles.len()), poles, weights)
}
