use super::*;

#[test]
fn six_significant_digits_round_like_a_default_stream() {
    assert_eq!(six_significant_digits(6.349831), 6.34983);
    assert_eq!(six_significant_digits(1038.6249), 1038.62);
    assert_eq!(six_significant_digits(0.), 0.);
}

#[test]
fn empty_shape_has_no_length() {
    let check = MassCheck::empty(MassKind::Length);
    assert!(check_mass(&Shape::default(), &check).is_ok());
}
