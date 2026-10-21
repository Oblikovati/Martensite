//! The text of a double written by `std::cout << value` in the pinned C locale.
use martensite_tkernel::tcollection::AsciiString;

/// `operator<<(double)`: `%g` with six significant digits; `general(0.5) == "0.5"`.
pub(crate) fn general(value: f64) -> String {
    String::from_utf8_lossy(AsciiString::from_real(value).as_bytes()).into_owned()
}
