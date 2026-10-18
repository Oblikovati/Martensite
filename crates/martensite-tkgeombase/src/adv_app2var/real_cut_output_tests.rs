//! Real cut output tests behavior and supporting operations.
use super::*;
fn frame() -> AdvApp2VarIsoRealFrame {
    AdvApp2VarIsoRealFrame::new(
        (0..9).map(f64::from).collect(),
        AdvApp2VarRealBindings {
            domain: [0, 1, 2, 3],
            constant_parameter: 8,
        },
    )
    .unwrap()
}
#[test]
fn signed_output_access_preserves_bits_and_checks_initialized_prefix_bounds() {
    let mut frame = frame();
    assert_eq!(frame.output_value(4, -1).unwrap(), 3.);
    frame.set_output_value(4, -1, -0.).unwrap();
    assert_eq!(
        frame.real(AdvApp2VarRealField::VEnd).to_bits(),
        (-0_f64).to_bits()
    );
    assert!(frame.output_value(4, i32::MIN).is_err());
    assert!(frame.output_value(usize::MAX, 1).is_err());
    assert!(frame.set_output_value(4, 5, 1.).is_err());
    frame.shift_output_values(4, -1, 0, 16).unwrap();
    assert_eq!(
        [
            frame.output_value(4, 0).unwrap().to_bits(),
            frame.output_value(4, 1).unwrap().to_bits()
        ],
        [(-0_f64).to_bits(), 4_f64.to_bits()]
    );
    assert_eq!(frame.output_value(4, 2).unwrap(), 6.);
    frame.shift_output_values(4, 5, 5, 0).unwrap();
    assert!(frame.shift_output_values(4, 5, 5, 8).is_err());
    assert!(frame.shift_output_values(4, 0, 0, -8).is_err());
    assert!(frame.shift_output_values(4, 0, 0, 7).is_err());
}
#[test]
fn cut_access_and_existing_curve_output_share_actual_owner_storage() {
    use crate::adv_app2var::approx_f2var::curve_output::CurveCoefficientStorage;
    let mut frame = frame();
    let mut curve = frame.configure_scalar_curve_region(0, 9).unwrap();
    frame.set_output_value(4, -1, 0.25).unwrap();
    assert_eq!(curve.with_curve_values(|values| values[3]), 0.25);
    curve.with_curve_values(|values| values[3] = 0.75);
    assert_eq!(frame.output_value(4, -1).unwrap(), 0.75);
    frame.shift_output_values(4, -1, 0, 16).unwrap();
    assert_eq!(
        curve.with_curve_values(|values| [values[4], values[5]]),
        [0.75, 4.]
    );
}
