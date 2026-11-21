use super::parametric_with_speed;

#[test]
fn parametric_tolerance_divides_by_curve_speed() {
    assert_eq!(parametric_with_speed(3.0, 7.0), 3.0 / 7.0);
}
