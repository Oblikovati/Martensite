//! The PreserveStoredV1 constructor retains bits or refuses the entire frame.
use martensite_tkmath::gp::{Axis3, Direction3, Point3};
fn oblique_frame() -> Axis3 {
    Axis3::new(
        Point3::new(7., -11., 5.),
        Direction3::new(2., 3., 4.).unwrap(),
        Direction3::new(4., -2., 1.).unwrap(),
    )
    .unwrap()
}
fn coefficients(frame: Axis3) -> [[f64; 3]; 3] {
    [
        *frame.direction().coordinates(),
        *frame.x_direction().coordinates(),
        *frame.y_direction().coordinates(),
    ]
}
#[test]
fn all_oblique_frame_bits_survive_both_handednesses() {
    for indirect in [false, true] {
        let mut original = oblique_frame();
        if indirect {
            original.y_reverse();
        }
        let [main, x, y] = coefficients(original);
        let restored = Axis3::from_stored_coefficients(original.location(), main, x, y).unwrap();
        assert_eq!(
            coefficients(restored).map(|v| v.map(f64::to_bits)),
            coefficients(original).map(|v| v.map(f64::to_bits))
        );
        assert_eq!(
            restored.location().coordinates().map(f64::to_bits),
            original.location().coordinates().map(f64::to_bits)
        );
    }
}
#[test]
fn signed_zero_and_smallest_subnormal_are_retained_without_repair() {
    let tiny = f64::from_bits(1);
    let frame = Axis3::from_stored_coefficients(
        Point3::new(-0., tiny, 0.),
        [-0., 0., 1.],
        [1., tiny, -0.],
        [-0., 1., 0.],
    )
    .unwrap();
    assert_eq!(frame.direction().x().to_bits(), (-0.0_f64).to_bits());
    assert_eq!(frame.x_direction().y().to_bits(), tiny.to_bits());
    assert_eq!(frame.location().x().to_bits(), (-0.0_f64).to_bits());
}
#[test]
fn adjacent_unit_coefficient_is_preserved_but_scaled_or_skew_frames_are_refused() {
    let adjacent = 1.0_f64.next_up();
    assert!(
        Axis3::from_stored_coefficients(
            Point3::default(),
            [0., 0., 1.],
            [adjacent, 0., 0.],
            [0., 1., 0.]
        )
        .is_ok()
    );
    for x in [
        [0., 0., 0.],
        [2., 0., 0.],
        [1.01, 0., 0.],
        [1., 0., 0.01],
        [0., 0., 1.],
        [f64::NAN, 0., 0.],
        [f64::INFINITY, 0., 0.],
    ] {
        assert!(
            Axis3::from_stored_coefficients(Point3::default(), [0., 0., 1.], x, [0., 1., 0.])
                .is_err(),
            "accepted {x:?}"
        );
    }
    assert!(
        Axis3::from_stored_coefficients(
            Point3::new(f64::INFINITY, 0., 0.),
            [0., 0., 1.],
            [1., 0., 0.],
            [0., 1., 0.]
        )
        .is_err()
    );
    assert!(
        Axis3::from_stored_coefficients(
            Point3::default(),
            [0., 0., 1.],
            [1., 0., 0.],
            [1., 0., 0.]
        )
        .is_err()
    );
}
