//! The modelview inverse Mesa's `_math_matrix_analyse` derives for a fixed-function frame.
//!
//! The matrix is classified from its bits (`analyse_from_scratch`) and the class picks the
//! inversion, so a rigid view transposes while the mirrored frame of a capping plane goes through
//! cofactors. Rounding therefore depends on the class; each step below is one binary32 operation.
const NEAR: f32 = 1e-6;

fn entry(m: &[f32; 16], row: usize, column: usize) -> f32 {
    m[column * 4 + row]
}
fn dot3(a: &[f32], b: &[f32]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn square(x: f32) -> f32 {
    x * x
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Class {
    /// `MATRIX_2D_NO_ROT`, the identity included.
    Plane2dNoRotation,
    /// `MATRIX_3D_NO_ROT`.
    NoRotation,
    /// `MATRIX_PERSPECTIVE`.
    Perspective,
    /// `MATRIX_2D` and `MATRIX_3D`, which share `invert_matrix_3d`.
    Affine { rigid: bool, uniform_scale: bool },
}

fn classify(m: &[f32; 16]) -> Option<Class> {
    let zero = |indices: &[usize]| indices.iter().all(|&i| m[i] == 0.);
    let affine = zero(&[3, 7, 11]) && m[15] == 1.;
    if !affine {
        let frustum = zero(&[4, 12, 1, 13, 2, 6, 3, 7, 15]) && m[11] == -1.;
        return frustum.then_some(Class::Perspective);
    }
    let no_plane_rotation = zero(&[1, 4]);
    let flat = zero(&[2, 6, 8, 9, 14]) && m[10] == 1.;
    if no_plane_rotation && flat {
        return Some(Class::Plane2dNoRotation);
    }
    if flat {
        return Some(plane_class(m));
    }
    if no_plane_rotation && zero(&[2, 6, 8, 9]) {
        return Some(Class::NoRotation);
    }
    Some(space_class(m))
}
fn plane_class(m: &[f32; 16]) -> Class {
    let (a, b) = (&m[0..2], &m[4..6]);
    let scaled = square(a[0] * a[0] + a[1] * a[1] - 1.) > square(NEAR)
        || square(b[0] * b[0] + b[1] * b[1] - 1.) > square(NEAR);
    let sheared = square(a[0] * b[0] + a[1] * b[1]) > square(NEAR);
    Class::Affine {
        rigid: !sheared && !scaled,
        uniform_scale: false,
    }
}
fn space_class(m: &[f32; 16]) -> Class {
    let (c1, c2, c3) = (
        dot3(&m[0..3], &m[0..3]),
        dot3(&m[4..7], &m[4..7]),
        dot3(&m[8..11], &m[8..11]),
    );
    let equal_scale = square(c1 - c2) < square(NEAR) && square(c1 - c3) < square(NEAR);
    let uniform_scale = equal_scale && square(c1 - 1.) > square(NEAR);
    let rotation = square(dot3(&m[0..3], &m[4..7])) < square(NEAR) && {
        let cross = [
            m[1] * m[6] - m[2] * m[5],
            m[2] * m[4] - m[0] * m[6],
            m[0] * m[5] - m[1] * m[4],
        ];
        let rest = [cross[0] - m[8], cross[1] - m[9], cross[2] - m[10]];
        dot3(&rest, &rest) < square(NEAR)
    };
    Class::Affine {
        rigid: rotation && equal_scale,
        uniform_scale,
    }
}
/// The inverse Mesa keeps for a loaded matrix; `None` when its class is `MATRIX_GENERAL`, whose
/// elimination is not reproduced here, or when it is singular; `modelview_inverse(view)`.
pub(crate) fn modelview_inverse(m: [f32; 16]) -> Option<[f32; 16]> {
    match classify(&m)? {
        Class::Plane2dNoRotation => diagonal_inverse(&m, 2),
        Class::NoRotation => diagonal_inverse(&m, 3),
        Class::Perspective => perspective_inverse(&m),
        Class::Affine {
            rigid: true,
            uniform_scale,
        } => Some(rigid_inverse(&m, uniform_scale)),
        Class::Affine { rigid: false, .. } => cofactor_inverse(&m),
    }
}
fn diagonal_inverse(m: &[f32; 16], size: usize) -> Option<[f32; 16]> {
    let mut out = identity();
    for i in 0..size {
        let value = entry(m, i, i);
        if value == 0. {
            return None;
        }
        out[i * 4 + i] = 1. / value;
        out[12 + i] = -(m[12 + i] * out[i * 4 + i]);
    }
    Some(out)
}
fn perspective_inverse(m: &[f32; 16]) -> Option<[f32; 16]> {
    if m[14] == 0. {
        return None;
    }
    let mut out = identity();
    out[0] = 1. / m[0];
    out[5] = 1. / m[5];
    out[12] = m[8];
    out[13] = m[9];
    out[10] = 0.;
    out[14] = -1.;
    out[11] = 1. / m[14];
    out[15] = m[10] * out[11];
    Some(out)
}
fn rigid_inverse(m: &[f32; 16], uniform_scale: bool) -> [f32; 16] {
    let scale = uniform_scale.then(|| 1. / dot3(&[m[0], m[4], m[8]], &[m[0], m[4], m[8]]));
    let mut out = identity();
    for row in 0..3 {
        for column in 0..3 {
            let value = entry(m, column, row);
            out[column * 4 + row] = scale.map_or(value, |scale| scale * value);
        }
    }
    translate(&mut out, m);
    out
}
fn upper_determinant(e: impl Fn(usize, usize) -> f32) -> f32 {
    let (mut positive, mut negative) = (0_f32, 0_f32);
    for term in [
        e(0, 0) * e(1, 1) * e(2, 2),
        e(1, 0) * e(2, 1) * e(0, 2),
        e(2, 0) * e(0, 1) * e(1, 2),
        -e(2, 0) * e(1, 1) * e(0, 2),
        -e(1, 0) * e(0, 1) * e(2, 2),
        -e(0, 0) * e(2, 1) * e(1, 2),
    ] {
        if term >= 0. {
            positive += term;
        } else {
            negative += term;
        }
    }
    positive + negative
}
fn cofactor_inverse(m: &[f32; 16]) -> Option<[f32; 16]> {
    let e = |row, column| entry(m, row, column);
    let determinant = upper_determinant(e);
    if determinant.abs() < 1e-25 {
        return None;
    }
    let d = 1. / determinant;
    let mut out = identity();
    out[0] = (e(1, 1) * e(2, 2) - e(2, 1) * e(1, 2)) * d;
    out[4] = -(e(0, 1) * e(2, 2) - e(2, 1) * e(0, 2)) * d;
    out[8] = (e(0, 1) * e(1, 2) - e(1, 1) * e(0, 2)) * d;
    out[1] = -(e(1, 0) * e(2, 2) - e(2, 0) * e(1, 2)) * d;
    out[5] = (e(0, 0) * e(2, 2) - e(2, 0) * e(0, 2)) * d;
    out[9] = -(e(0, 0) * e(1, 2) - e(1, 0) * e(0, 2)) * d;
    out[2] = (e(1, 0) * e(2, 1) - e(2, 0) * e(1, 1)) * d;
    out[6] = -(e(0, 0) * e(2, 1) - e(2, 0) * e(0, 1)) * d;
    out[10] = (e(0, 0) * e(1, 1) - e(1, 0) * e(0, 1)) * d;
    translate(&mut out, m);
    Some(out)
}
/// `_mesa_transform_vector`: the row vector `plane` times `inverse`.
fn plane_through(plane: [f32; 4], inverse: &[f32; 16]) -> [f32; 4] {
    let mut out = [0.; 4];
    for (column, value) in out.iter_mut().enumerate() {
        let term = |row: usize| plane[row] * inverse[column * 4 + row];
        *value = term(0) + term(1) + term(2) + term(3);
    }
    out
}
/// The clip-space plane llvmpipe's draw module receives for a world-space `glClipPlane` issued
/// under the view matrix (`_ClipUserPlane`); `clip_space_plane(plane, view, projection)`.
pub(crate) fn clip_space_plane(
    plane: [f32; 4],
    view: [f32; 16],
    projection: [f32; 16],
) -> Option<[f32; 4]> {
    let eye = plane_through(plane, &modelview_inverse(view)?);
    Some(plane_through(eye, &modelview_inverse(projection)?))
}
fn translate(out: &mut [f32; 16], m: &[f32; 16]) {
    for row in 0..3 {
        let term = |column: usize| m[12 + column] * out[column * 4 + row];
        out[12 + row] = -(term(0) + term(1) + term(2));
    }
}
fn identity() -> [f32; 16] {
    [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    /// A capping plane's model-view at the camera of demo/samples/dimensions, as native loaded
    /// it, and the normal Mesa's vertex stage computed for the plane's (0,-1,0).
    #[test]
    fn traced_capping_frame_reproduces_the_native_normal_bits() {
        let m = [
            0x3f7490ee_u32,
            0xbdb2db37,
            0x3e908c69,
            0,
            0x3e974e6c,
            0x3e908c69,
            0xbf69a499,
            0,
            0x31fffffe,
            0xbf7490ee,
            0xbe974e6d,
            0,
            0x423e2d84,
            0xc2f2abf4,
            0xc375d9d4,
            0x3f800000,
        ]
        .map(f32::from_bits);
        let inverse = modelview_inverse(m).unwrap();
        let normal = [-inverse[1], -inverse[5], -inverse[9]].map(f32::to_bits);
        assert_eq!(normal, [0xbe974e6d, 0xbe908c6a, 0x3f69a49a]);
    }
    #[test]
    fn translated_rotation_transposes_without_cofactor_rounding() {
        let (c, s) = (0.6_f32, 0.8_f32);
        let m = [c, s, 0., 0., -s, c, 0., 0., 0., 0., 1., 0., 3., 4., 5., 1.];
        let inverse = modelview_inverse(m).unwrap();
        assert_eq!(inverse[1], m[4]);
        assert_eq!(inverse[4], m[1]);
        assert_eq!(inverse[14], -5.);
    }
    #[test]
    fn mirrored_frame_takes_the_cofactor_route() {
        let m = [
            1., 0., 0., 0., 0., 0., 1., 0., 0., 1., 0., 0., 0., 0., 0., 1.,
        ];
        let Some(Class::Affine { rigid: false, .. }) = classify(&m) else {
            panic!("a mirrored frame is not a rotation");
        };
        let inverse = modelview_inverse(m).unwrap();
        assert_eq!(inverse[6], 1.);
        assert_eq!(inverse[9], 1.);
    }
    #[test]
    fn general_projective_matrix_keeps_the_caller_route() {
        let mut m = identity();
        m[11] = -2.;
        assert_eq!(modelview_inverse(m), None);
    }
    #[test]
    fn frustum_inverse_follows_the_perspective_class() {
        let mut m = [0.; 16];
        (m[0], m[5], m[10], m[11], m[14]) = (2., 4., -3., -1., -8.);
        let inverse = modelview_inverse(m).unwrap();
        assert_eq!((inverse[0], inverse[5]), (0.5, 0.25));
        assert_eq!(
            (inverse[11], inverse[14], inverse[15]),
            (-0.125, -1., 0.375)
        );
    }
    #[test]
    fn plane_passes_through_the_view_then_the_projection() {
        // Moving the eye by 2 along z shifts the plane z = 0 by 2 in eye space.
        let mut view = identity();
        view[14] = -2.;
        let mut projection = identity();
        projection[10] = 0.5;
        let plane = clip_space_plane([0., 0., 1., 0.], view, projection).unwrap();
        assert_eq!(plane, [0., 0., 2., 2.]);
    }
}
