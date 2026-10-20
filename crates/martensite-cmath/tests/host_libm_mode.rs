//! Both result modes on the inputs where glibc 2.39 differs from the correctly rounded value.
//!
//! Portable mode (the default) returns the reference; the opt-in `host-libm` feature
//! returns the platform C library result instead (behavior version 1 replay), which
//! for these inputs is a different word.
#![cfg(all(target_os = "linux", target_env = "gnu"))]
use crate::reference_values::UNARY;

/// The input and reference words of each `UNARY` row its comment marks `glibc`.
fn glibc_marked_rows() -> Vec<(String, u64, u64)> {
    let source = include_str!("reference_values.rs");
    let marked: Vec<&str> = source
        .lines()
        .filter(|line| line.contains("// glibc:"))
        .collect();
    UNARY
        .iter()
        .filter(|(name, input, _)| {
            let spelled = format!("(\"{name}\", {input:#018x},");
            marked
                .iter()
                .any(|line| line.trim_start().starts_with(&spelled))
        })
        .map(|&(name, input, expected)| (name.to_owned(), input, expected))
        .collect()
}

fn adapter(name: &str) -> fn(f64) -> f64 {
    use martensite_cmath::*;
    match name {
        "acos" => acos,
        "acosh" => acosh,
        "asin" => asin,
        "asinh" => asinh,
        "atan" => atan,
        "atanh" => atanh,
        "cos" => cos,
        "cosh" => cosh,
        "exp" => exp,
        "exp2" => exp2,
        "expm1" => expm1,
        "log" => log,
        "log10" => log10,
        "log1p" => log1p,
        "log2" => log2,
        "sin" => sin,
        "sinh" => sinh,
        "tan" => tan,
        "tanh" => tanh,
        other => panic!("no glibc-marked unary function {other:?}; expected one listed in UNARY"),
    }
}

/// The platform C library entry point of the same function, as Rust's std calls it.
#[allow(clippy::disallowed_methods)] // The platform result is what this test compares.
fn platform(name: &str, x: f64) -> f64 {
    match name {
        "acos" => x.acos(),
        "acosh" => x.acosh(),
        "asin" => x.asin(),
        "asinh" => x.asinh(),
        "atan" => x.atan(),
        "atanh" => x.atanh(),
        "cos" => x.cos(),
        "cosh" => x.cosh(),
        "exp" => x.exp(),
        "exp2" => x.exp2(),
        "expm1" => x.exp_m1(),
        "log" => x.ln(),
        "log10" => x.log10(),
        "log1p" => x.ln_1p(),
        "log2" => x.log2(),
        "sin" => x.sin(),
        "sinh" => x.sinh(),
        "tan" => x.tan(),
        "tanh" => x.tanh(),
        other => panic!("no platform entry point {other:?}; expected one listed in UNARY"),
    }
}

#[test]
fn glibc_marked_rows_cover_every_listed_function() {
    assert!(
        glibc_marked_rows().len() >= 30,
        "expected the glibc-marked reference rows"
    );
}

#[cfg(not(feature = "host-libm"))]
#[test]
fn portable_mode_returns_the_reference_where_glibc_differs() {
    for (name, input, expected) in glibc_marked_rows() {
        let x = f64::from_bits(input);
        let actual = adapter(&name)(x).to_bits();
        assert_eq!(actual, expected, "{name}({input:#018x})");
        assert_ne!(
            actual,
            platform(&name, x).to_bits(),
            "{name}({input:#018x}) matches glibc"
        );
    }
}

/// The inverse hyperbolic functions replay the C symbols the kernel declared before #201,
/// which the link may resolve to a non-glibc implementation, while Rust's std computes
/// them from `ln_1p` and `sqrt`; neither is a glibc witness, so the host test skips them.
#[cfg(all(feature = "host-libm", not(feature = "libm-attribution")))]
const C_SYMBOL_ONLY: [&str; 3] = ["asinh", "acosh", "atanh"];

#[cfg(all(feature = "host-libm", not(feature = "libm-attribution")))]
#[test]
fn host_libm_mode_returns_the_platform_result_where_glibc_differs() {
    let rows = glibc_marked_rows();
    let checked = rows
        .iter()
        .filter(|(name, _, _)| !C_SYMBOL_ONLY.contains(&name.as_str()));
    for (name, input, expected) in checked {
        let x = f64::from_bits(*input);
        let actual = adapter(name)(x).to_bits();
        assert_eq!(actual, platform(name, x).to_bits(), "{name}({input:#018x})");
        assert_ne!(
            actual, *expected,
            "{name}({input:#018x}) is the portable result"
        );
    }
}
