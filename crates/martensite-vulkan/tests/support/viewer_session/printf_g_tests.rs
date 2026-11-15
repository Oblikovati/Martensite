use super::printf_g;

#[test]
fn six_significant_digits_without_trailing_zeros() {
    assert_eq!(printf_g(1.807106781), "1.80711");
    assert_eq!(printf_g(0.), "0");
    assert_eq!(printf_g(46.), "46");
    assert_eq!(printf_g(-2.5), "-2.5");
    assert_eq!(printf_g(1234567.), "1.23457e+06");
    assert_eq!(printf_g(0.0000123), "1.23e-05");
    assert_eq!(printf_g(999999.7), "1e+06");
}
