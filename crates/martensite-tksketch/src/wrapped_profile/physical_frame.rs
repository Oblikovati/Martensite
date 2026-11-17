//! Preserve one stored rigid-frame convention for cylindrical and conical publication.
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Axis3, Direction3, Point3, Transform3};

pub(super) fn stored_rigid_transform(
    origin: [f64; 3],
    axis: [f64; 3],
    radial: [f64; 3],
) -> Result<Transform3, Failure> {
    let [x, y, z] = origin;
    let axis = frame_direction(axis)?;
    let radial = frame_direction(radial)?;
    let target = Axis3::new(Point3::new(x, y, z), axis, radial)?;
    let transform = Transform3::displacement(Axis3::default(), target);
    let rows = std::array::from_fn(|i| {
        let [a, b, c] = transform.matrix().coefficients()[i];
        [a, b, c, transform.translation_part().coordinates()[i]]
    });
    Transform3::from_stored_rigid_coefficients(rows)
}
fn frame_direction([x, y, z]: [f64; 3]) -> Result<Direction3, Failure> {
    Direction3::new(x, y, z)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stored_frame_preserves_exact_axis_permutation_and_translation_bits() {
        let transform = stored_rigid_transform([2., 3., 4.], [1., 0., 0.], [0., 1., 0.]).unwrap();
        assert_eq!(
            *transform.matrix().coefficients(),
            [[0., 0., 1.], [1., 0., 0.], [0., 1., 0.]]
        );
        assert_eq!(
            transform.translation_part().coordinates().map(f64::to_bits),
            [2., 3., 4.].map(f64::to_bits)
        );
    }
    #[test]
    fn stored_frame_refuses_zero_axes_and_radial_directions() {
        assert!(stored_rigid_transform([0.; 3], [0.; 3], [1., 0., 0.]).is_err());
        assert!(stored_rigid_transform([0.; 3], [0., 0., 1.], [0.; 3]).is_err());
    }
}
