//! Complete native-qualified rows; parallel interior pulses remain diagnostic.
pub(super) const ROWS: &[(&str, &str)] = &[
    (
        include_str!("states0.expected"),
        include_str!("events0.expected"),
    ),
    (
        include_str!("states1.expected"),
        include_str!("events1.expected"),
    ),
];
pub(super) const FRAMES: &[&str] = &[include_str!("frames0.expected")];
