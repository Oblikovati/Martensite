//! Independent UTF-16 expectations for the seven frozen container scenarios.
pub const EXPECTED_UNITS: [&[u16]; 7] = [
    &[72, 101, 97, 100, 101, 114, 70, 111, 114, 109, 97, 116],
    &[70, 105, 114, 115, 116],
    &[84, 121, 112, 101, 70, 97, 108, 108, 98, 97, 99, 107],
    &[],
    &[70, 73, 76, 69, 95, 70, 79, 82, 77, 65, 84, 58],
    &[243],
    &[],
];
