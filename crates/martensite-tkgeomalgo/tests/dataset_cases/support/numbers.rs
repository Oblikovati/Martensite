//! How the case harness prints reals: `Draw_Interpretor << double` is `%.17g`, `xdistcs` uses `%lg`,
//! and a `std::ostream` at precision `p` in scientific mode is `%.{p}e`; the cases cut
//! that text with script string commands and compare numbers through script `expr`.

/// C `printf("%.{precision}g", value)` for a finite value; `g(0.5, 17) == "0.5"`.
pub fn g(value: f64, precision: usize) -> String {
    if value == 0.0 {
        return if value.is_sign_negative() { "-0" } else { "0" }.into();
    }
    let precision = precision.max(1);
    let scientific = format!("{value:.*e}", precision - 1);
    let (mantissa, exponent) = scientific.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    if exponent >= -4 && exponent < precision as i32 {
        let decimals = (precision as i32 - 1 - exponent) as usize;
        return trim_fraction(&format!("{value:.decimals$}"));
    }
    c_exponent(&trim_fraction(mantissa), exponent)
}

/// `Draw_Interpretor << double`: `%.17g`.
pub fn g17(value: f64) -> String {
    g(value, 17)
}

/// C `printf("%.{precision}e", value)`; `e(12.7, 15) == "1.270000000000000e+01"`.
pub fn e(value: f64, precision: usize) -> String {
    let scientific = format!("{value:.precision$e}");
    let (mantissa, exponent) = scientific.split_once('e').unwrap();
    c_exponent(mantissa, exponent.parse().unwrap())
}

/// `std::ostream << double` at `precision` in the default float field: `%.{precision}g`.
pub fn stream(value: f64, precision: usize) -> String {
    g(value, precision)
}

fn c_exponent(mantissa: &str, exponent: i32) -> String {
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{mantissa}e{sign}{:02}", exponent.abs())
}

/// `%g` drops trailing zeros of the fraction and a bare decimal point.
fn trim_fraction(text: &str) -> String {
    if !text.contains('.') {
        return text.into();
    }
    text.trim_end_matches('0').trim_end_matches('.').into()
}

/// script reads a number the way `strtod` does; the cases parse the printed text back.
pub fn script_real(text: &str) -> f64 {
    text.trim()
        .parse()
        .unwrap_or_else(|_| panic!("expected a ScriptStep real, got {text:?}"))
}

#[test]
fn numbers_follow_printf() {
    assert_eq!(g17(0.5), "0.5");
    assert_eq!(g17(1.0), "1");
    assert_eq!(g17(std::f64::consts::FRAC_PI_2), "1.5707963267948966");
    assert_eq!(g17(2.6097941390190726e-6), "2.6097941390190726e-06");
    assert_eq!(g17(1e20), "1e+20");
    assert_eq!(g(1.0e-7, 6), "1e-07");
    assert_eq!(g(0.123456789, 6), "0.123457");
    assert_eq!(g(123456789.0, 6), "1.23457e+08");
    assert_eq!(e(12.7, 15), "1.270000000000000e+01");
    assert_eq!(e(-0.0005, 3), "-5.000e-04");
}
