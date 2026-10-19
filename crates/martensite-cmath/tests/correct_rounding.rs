//! Every function returns the high-precision reference rounded once to nearest (#201).
use super::reference_values::{BINARY, UNARY};

fn unary(name: &str) -> fn(f64) -> f64 {
    use martensite_cmath::*;
    match name {
        "sin" => sin,
        "cos" => cos,
        "tan" => tan,
        "asin" => asin,
        "acos" => acos,
        "atan" => atan,
        "sinh" => sinh,
        "cosh" => cosh,
        "tanh" => tanh,
        "asinh" => asinh,
        "acosh" => acosh,
        "atanh" => atanh,
        "exp" => exp,
        "exp2" => exp2,
        "expm1" => expm1,
        "log" => log,
        "log1p" => log1p,
        "log2" => log2,
        "log10" => log10,
        "cbrt" => cbrt,
        other => panic!("no unary function {other:?}; expected a martensite_cmath name"),
    }
}

fn binary(name: &str) -> fn(f64, f64) -> f64 {
    use martensite_cmath::*;
    match name {
        "atan2" => atan2,
        "pow" => pow,
        "hypot" => hypot,
        other => panic!("no binary function {other:?}; expected atan2, pow or hypot"),
    }
}

#[test]
fn unary_functions_return_the_correctly_rounded_reference() {
    for &(name, input, expected) in UNARY {
        let actual = unary(name)(f64::from_bits(input)).to_bits();
        assert_eq!(actual, expected, "{name}({input:#018x})");
    }
}

#[test]
fn binary_functions_return_the_correctly_rounded_reference() {
    for &(name, first, second, expected) in BINARY {
        let actual = binary(name)(f64::from_bits(first), f64::from_bits(second)).to_bits();
        assert_eq!(actual, expected, "{name}({first:#018x}, {second:#018x})");
    }
}

#[test]
fn single_precision_pow_returns_the_correctly_rounded_reference() {
    // mpmath references for the sRGB transfer exponents.
    let cases: [(u32, u32, u32); 4] = [
        (0x3f00_0000, 0x400c_cccd, 0x3e5e_dc66),
        (0x3f00_0000, 0x3ee8_ba2f, 0x3f3a_d03e),
        (0x3f33_3333, 0x4019_999a, 0x3ed9_85e5),
        (0x3e99_999a, 0x3ed5_5555, 0x3f1b_03da),
    ];
    for (x, y, expected) in cases {
        let actual = martensite_cmath::powf(f32::from_bits(x), f32::from_bits(y));
        assert_eq!(actual.to_bits(), expected, "powf({x:#010x}, {y:#010x})");
    }
}

#[test]
fn sincos_matches_the_separate_functions() {
    for x in [-1.0e6, -3.0, -0.0, 0.0, 1.0e-300, 0.5, 2.0, 1.0e22] {
        let (sine, cosine) = martensite_cmath::sincos(x);
        assert_eq!(sine.to_bits(), martensite_cmath::sin(x).to_bits());
        assert_eq!(cosine.to_bits(), martensite_cmath::cos(x).to_bits());
    }
}

#[test]
fn powi_multiplies_by_repeated_squaring() {
    let x: f64 = 1.1;
    let squared = x * x;
    // n = 5 is x * x^4: one product of x with the second square.
    assert_eq!(martensite_cmath::powi(x, 5), x * (squared * squared));
    assert_eq!(martensite_cmath::powi(x, -3), 1.0 / (x * squared));
    assert_eq!(martensite_cmath::powi(f64::NAN, 0), 1.0);
    assert_eq!(martensite_cmath::powi(-0.0, 3).to_bits(), (-0.0f64).to_bits());
    assert_eq!(martensite_cmath::powi(2.0, i32::MIN), 0.0);
}
