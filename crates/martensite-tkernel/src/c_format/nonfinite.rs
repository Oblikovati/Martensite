//! One spelling of infinities and NaNs on every target (#201).
//!
//! The sign bit is printed as `-`, as glibc does. Windows UCRT would print the x86
//! default NaN as `-nan(ind)`; the kernel no longer copies that host spelling, so
//! text output is the same on every release target.

/// `inf`, `-inf`, `nan` or `-nan` by the sign bit; `nonfinite_text(f64::INFINITY)`.
pub(crate) fn nonfinite_text(value: f64) -> String {
    let word = if value.is_nan() { "nan" } else { "inf" };
    format!("{}{word}", if value.is_sign_negative() { "-" } else { "" })
}

#[cfg(test)]
mod tests {
    use super::nonfinite_text;
    #[test]
    fn special_values_are_spelled_by_sign_on_every_target() {
        assert_eq!(nonfinite_text(f64::INFINITY), "inf");
        assert_eq!(nonfinite_text(-f64::INFINITY), "-inf");
        assert_eq!(nonfinite_text(f64::NAN), "nan");
        // The x86-64 default NaN, which Windows UCRT spells `-nan(ind)`.
        assert_eq!(
            nonfinite_text(f64::from_bits(0xfff8_0000_0000_0000)),
            "-nan"
        );
        assert_eq!(nonfinite_text(f64::from_bits(0x7ff8_0000_0000_0001)), "nan");
    }
}
