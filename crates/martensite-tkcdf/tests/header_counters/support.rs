//! Owned counter input locations and independent expected integers.
#[path = "printer.rs"]
mod printer;
use martensite_tkernel::tcollection::ExtendedString;
pub use printer::recorder;

pub fn fixture(kind: &str, scenario: usize) -> ExtendedString {
    let path = format!(
        "{}/tests/header_counters/fixtures/{kind}-{scenario}.fsd",
        env!("CARGO_MANIFEST_DIR")
    );
    ExtendedString::from_c_string(path.as_bytes(), true)
}

pub const EXPECTED_COUNTERS: [(i32, i32, bool); 9] = [
    (17, 23, false),
    (-5, -8, false),
    (9, -7, true),
    (0, -1, true),
    (0, -1, true),
    (0, -1, false),
    (5, 2, false),
    (i32::MIN, i32::MIN, false),
    (0, -1, true),
];
