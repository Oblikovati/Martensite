//! The version of this crate, as the kernel reports it.
//!
//! Every value comes from the package version Cargo records at build time, so the
//! reported version always matches the released crate. Callers that persist or compare
//! versions read them through these functions.

const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const COMPLETE: &str = concat!(
    env!("CARGO_PKG_VERSION_MAJOR"),
    ".",
    env!("CARGO_PKG_VERSION_MINOR"),
    ".",
    env!("CARGO_PKG_VERSION_PATCH")
);

/// The pre-release tag of the package version ("" for a release).
///
/// ```
/// let tag = martensite_tkernel::standard::version_info::kernel_development_version();
/// assert_eq!(tag, env!("CARGO_PKG_VERSION_PRE"));
/// ```
pub const fn kernel_development_version() -> &'static str {
    env!("CARGO_PKG_VERSION_PRE")
}

/// "major.minor" as a double.
///
/// ```
/// let version = martensite_tkernel::standard::version_info::kernel_version_double();
/// assert!(version > 0.0);
/// ```
pub const fn kernel_version_double() -> f64 {
    let minor = decimal_value(MINOR);
    let mut scale = 1.0;
    let mut digits = MINOR.len();
    while digits > 0 {
        scale *= 10.0;
        digits -= 1;
    }
    decimal_value(MAJOR) + minor / scale
}

/// "major.minor".
///
/// ```
/// let version = martensite_tkernel::standard::version_info::kernel_version_string();
/// assert_eq!(version.split('.').count(), 2);
/// ```
pub const fn kernel_version_string() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION_MAJOR"),
        ".",
        env!("CARGO_PKG_VERSION_MINOR")
    )
}

/// "major.minor.patch".
///
/// ```
/// let version = martensite_tkernel::standard::version_info::kernel_version_string_complete();
/// assert_eq!(version.split('.').count(), 3);
/// ```
pub const fn kernel_version_string_complete() -> &'static str {
    COMPLETE
}

/// The complete package version, including its pre-release tag when there is one.
///
/// ```
/// use martensite_tkernel::standard::version_info::kernel_version_string_extended;
/// assert_eq!(kernel_version_string_extended(), env!("CARGO_PKG_VERSION"));
/// ```
pub const fn kernel_version_string_extended() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// The value of a string of ASCII decimal digits, as `f64`.
const fn decimal_value(digits: &str) -> f64 {
    let bytes = digits.as_bytes();
    let mut value = 0.0;
    let mut index = 0;
    while index < bytes.len() {
        value = value * 10.0 + (bytes[index] - b'0') as f64;
        index += 1;
    }
    value
}
