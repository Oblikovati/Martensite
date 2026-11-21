//! String storage: ASCII and UTF-16 strings and their shared handles.
mod ascii_string;
pub use ascii_string::AsciiString;
#[cfg(test)]
pub(crate) use ascii_string::CatOperand;
pub(crate) use ascii_string::c_string_prefix;
mod extended_string;
pub use extended_string::ExtendedString;
mod h_ascii_string;
pub use h_ascii_string::HAsciiString;
mod h_extended_string;
pub use h_extended_string::{HExtendedString, HExtendedStringHasher};
#[cfg(test)]
mod primes;
#[cfg(test)]
pub(crate) use primes::next_prime_for_map;

#[cfg(test)]
#[path = "../../tests/suites/tcollection_private.rs"]
mod tests;
