//! Select the recorded output; every platform computes the same bits.

/// Select the recorded asset; `native_boolean_snapshot!("tkbo_cells_builder")`.
macro_rules! native_boolean_snapshot {
    ($probe:literal) => {{ include_str!(concat!("fixtures/", $probe, ".expected")) }};
}

pub(crate) use native_boolean_snapshot;
