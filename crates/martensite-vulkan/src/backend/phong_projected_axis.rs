//! Homogeneous decisions use exact f32 products, never a rounded screen-area zero.
use num_rational::BigRational;

pub(super) fn constant_axis(clips: [[f32; 4]; 3]) -> bool {
    [0, 1].into_iter().any(|axis| {
        [1, 2].into_iter().all(|i| {
            f64::from(clips[0][axis]) * f64::from(clips[i][3])
                == f64::from(clips[i][axis]) * f64::from(clips[0][3])
        })
    })
}
pub(super) fn inside_clip(clips: [[f32; 4]; 3]) -> bool {
    clips.into_iter().all(|p| {
        p.into_iter().all(f32::is_finite)
            && p[3] > 0.
            && p[..3].iter().all(|x| x.abs() <= p[3])
            // The unchanged VS may remap Z by (z+w)/2 on zero-to-one devices.
            && (p[2] + p[3]).is_finite()
    })
}
pub(super) fn nonzero_determinant(clips: [[f32; 4]; 3]) -> bool {
    let terms = determinant_terms(clips);
    let det = (terms[0] + terms[1] + terms[2]) - (terms[3] + terms[4] + terms[5]);
    let maximum = terms.into_iter().map(f64::abs).fold(0., f64::max);
    // Six terms each have two products, then five additions: 192 eps times the
    // largest rounded term exceeds the total worst-case rounding error (no f64
    // overflow/underflow is possible for three finite f32 factors).
    let bound = (maximum * (192. * f64::EPSILON)).next_up();
    if det.abs() > bound {
        return true;
    }
    exact_determinant(clips).is_some_and(|value| value != BigRational::from_integer(0.into()))
}
fn determinant_terms(clips: [[f32; 4]; 3]) -> [f64; 6] {
    let [a, b, c] = clips.map(|p| [f64::from(p[0]), f64::from(p[1]), f64::from(p[3])]);
    [
        a[0] * b[1] * c[2],
        b[0] * c[1] * a[2],
        c[0] * a[1] * b[2],
        c[0] * b[1] * a[2],
        b[0] * a[1] * c[2],
        a[0] * c[1] * b[2],
    ]
}
fn exact_determinant(clips: [[f32; 4]; 3]) -> Option<BigRational> {
    let values = clips
        .into_iter()
        .flat_map(|p| [p[0], p[1], p[3]])
        .map(|v| BigRational::from_float(f64::from(v)))
        .collect::<Option<Vec<_>>>()?;
    let [a, b, c] = [&values[0..3], &values[3..6], &values[6..9]];
    Some(
        &a[0] * &b[1] * &c[2] + &b[0] * &c[1] * &a[2] + &c[0] * &a[1] * &b[2]
            - &c[0] * &b[1] * &a[2]
            - &b[0] * &a[1] * &c[2]
            - &a[0] * &c[1] * &b[2],
    )
}

#[cfg(test)]
#[path = "phong_projected_axis_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "phong_projected_product_tests.rs"]
mod shader_product_tests;
