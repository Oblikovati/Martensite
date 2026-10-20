//! How the case harness prints a real into the script result: `Draw_Interpretor::Append(double)` uses
//! `Sprintf(s, "%.17g", r)`; the cases then cut that text with script string commands.

/// C `printf("%.17g", value)` for a finite value; `g17(0.5) == "0.5"`.
pub fn g17(value: f64) -> String {
    if value == 0.0 {
        return if value.is_sign_negative() { "-0" } else { "0" }.into();
    }
    let scientific = format!("{value:.16e}");
    let (mantissa, exponent) = scientific.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    if (-4..17).contains(&exponent) {
        let decimals = (16 - exponent) as usize;
        return trim_fraction(&format!("{value:.decimals$}"));
    }
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{}e{sign}{:02}", trim_fraction(mantissa), exponent.abs())
}

/// `%g` drops trailing zeros of the fraction and a bare decimal point.
fn trim_fraction(text: &str) -> String {
    if !text.contains('.') {
        return text.into();
    }
    text.trim_end_matches('0').trim_end_matches('.').into()
}

#[test]
fn g17_follows_printf() {
    assert_eq!(g17(0.5), "0.5");
    assert_eq!(g17(1.0), "1");
    assert_eq!(g17(std::f64::consts::FRAC_PI_2), "1.5707963267948966");
    assert_eq!(g17(2.6097941390190726e-6), "2.6097941390190726e-06");
    assert_eq!(g17(1e20), "1e+20");
    assert_eq!(g17(-0.0001), "-0.0001");
}
