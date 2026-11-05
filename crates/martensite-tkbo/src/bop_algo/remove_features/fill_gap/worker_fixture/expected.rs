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
    (
        include_str!("states2.expected"),
        include_str!("events2.expected"),
    ),
    (
        include_str!("states3.expected"),
        include_str!("events3.expected"),
    ),
    (
        include_str!("states4.expected"),
        include_str!("events4.expected"),
    ),
];
pub(super) const FRAMES: &[&str] = &[
    include_str!("frames0.expected"),
    include_str!("frames1.expected"),
    include_str!("frames2.expected"),
    include_str!("frames3.expected"),
    include_str!("frames4.expected"),
    include_str!("frames5.expected"),
];
