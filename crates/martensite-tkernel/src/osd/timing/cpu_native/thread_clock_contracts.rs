//! Conversion fixtures before replacing the native thread clock (#89).
use super::thread_cpu_seconds;

#[test]
fn thread_clock_conversion_preserves_fallback_fractions_and_rounding() {
    for (stamp, expected) in [
        (None, 0.0_f64),
        (Some((0.0, 0.0)), 0.0),
        (Some((2.0, 250_000_000.0)), 2.25),
        (Some((3.0, 750_000_000.0)), 3.75),
        (
            Some((9_007_199_254_740_992.0, 500_000_000.0)),
            9_007_199_254_740_992.0,
        ),
    ] {
        let actual = thread_cpu_seconds(stamp);
        assert_eq!(actual.0.to_bits(), expected.to_bits(), "stamp {stamp:?}");
        assert_eq!(actual.1.to_bits(), 0.0_f64.to_bits(), "stamp {stamp:?}");
    }
}
