use super::{assert_receipt, canonical_nan_tokens};

#[test]
fn only_isolated_decimal_nan_bits_are_canonicalized() {
    let positive = 0x7ff8_0000_0000_0001_u64;
    let negative = 0xfff8_0000_0000_0002_u64;
    assert_eq!(
        canonical_nan_tokens(&format!("{positive} {negative}\n")),
        "NaN NaN\n"
    );
    assert_eq!(
        canonical_nan_tokens(&format!("x{positive} {positive}_ {positive}x")),
        format!("x{positive} {positive}_ {positive}x")
    );
    assert_receipt("nan", &format!("{positive}\n"), &format!("{negative}\n"));
}

#[test]
fn finite_infinite_zero_and_literal_changes_are_rejected() {
    for (expected, actual) in [
        ("0\n", "9223372036854775808\n"),
        ("4607182418800017408", "4607182418800017409"),
        ("9218868437227405312", "18442240474082181120"),
        ("kind ", "kind"),
        ("1\n", "1\n\n"),
    ] {
        assert!(std::panic::catch_unwind(|| assert_receipt("boundary", expected, actual)).is_err());
    }
}
