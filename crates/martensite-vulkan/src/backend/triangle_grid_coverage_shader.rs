//! Exact-grid fragments are paired with the existing registered V2 vertex interface.
use crate::{RenderError, RenderTriangleCoverageFailure as Failure};

type ShaderPair = (&'static [u8], &'static [u8]);

macro_rules! coverage_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/unsnapped-v2.", $name)).as_slice(),
            include_bytes!(concat!(
                "../../shaders/triangle-grid-v2.unsnapped-v2.",
                $name
            ))
            .as_slice(),
        )
    };
}

const FRAGMENTS: &[ShaderPair] = &[
    coverage_pair!("lighting.frag.spv"),
    coverage_pair!("lighting.coordinates-v2.frag.spv"),
    coverage_pair!("numeric-max-v2.lighting.frag.spv"),
    coverage_pair!("numeric-max-v2.lighting.coordinates-v2.frag.spv"),
];
macro_rules! coverage_v4_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/unsnapped-v2.", $name)).as_slice(),
            include_bytes!(concat!(
                "../../shaders/triangle-grid-v4.unsnapped-v2.",
                $name
            ))
            .as_slice(),
        )
    };
}
const FRAGMENTS_V4: &[ShaderPair] = &[
    coverage_v4_pair!("lighting.frag.spv"),
    coverage_v4_pair!("lighting.coordinates-v2.frag.spv"),
    coverage_v4_pair!("numeric-max-v2.lighting.frag.spv"),
    coverage_v4_pair!("numeric-max-v2.lighting.coordinates-v2.frag.spv"),
];

pub(super) fn fragment_v4(v2: &'static [u8]) -> Result<&'static [u8], RenderError> {
    lookup_fragment(FRAGMENTS_V4, v2, "V4")
}

pub(super) fn fragment_v4_v3(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    FRAGMENTS_V4_V3
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, selected)| *selected)
        .ok_or_else(|| {
            Failure::UnsupportedInterface {
                actual: format!(
                    "unregistered V4/V3 fragment module of {} bytes",
                    stock.len()
                ),
            }
            .into()
        })
}

const FRAGMENTS_V3: &[ShaderPair] = &[
    (
        include_bytes!("../../shaders/lighting.frag.spv"),
        include_bytes!("../../shaders/triangle-grid-v3.unsnapped-v3.lighting.frag.spv"),
    ),
    (
        include_bytes!("../../shaders/lighting.coordinates-v2.frag.spv"),
        include_bytes!(
            "../../shaders/triangle-grid-v3.unsnapped-v3.lighting.coordinates-v2.frag.spv"
        ),
    ),
    (
        include_bytes!("../../shaders/numeric-max-v2.lighting.frag.spv"),
        include_bytes!(
            "../../shaders/triangle-grid-v3.unsnapped-v3.numeric-max-v2.lighting.frag.spv"
        ),
    ),
    (
        include_bytes!("../../shaders/numeric-max-v2.lighting.coordinates-v2.frag.spv"),
        include_bytes!(
            "../../shaders/triangle-grid-v3.unsnapped-v3.numeric-max-v2.lighting.coordinates-v2.frag.spv"
        ),
    ),
    (
        include_bytes!("../../shaders/lighting.clipped.frag.spv"),
        include_bytes!("../../shaders/triangle-grid-v3.unsnapped-v3.lighting.clipped.frag.spv"),
    ),
    (
        include_bytes!("../../shaders/lighting.clipped.coordinates-v2.frag.spv"),
        include_bytes!(
            "../../shaders/triangle-grid-v3.unsnapped-v3.lighting.clipped.coordinates-v2.frag.spv"
        ),
    ),
    (
        include_bytes!("../../shaders/numeric-max-v2.lighting.clipped.frag.spv"),
        include_bytes!(
            "../../shaders/triangle-grid-v3.unsnapped-v3.numeric-max-v2.lighting.clipped.frag.spv"
        ),
    ),
    (
        include_bytes!("../../shaders/numeric-max-v2.lighting.clipped.coordinates-v2.frag.spv"),
        include_bytes!(
            "../../shaders/triangle-grid-v3.unsnapped-v3.numeric-max-v2.lighting.clipped.coordinates-v2.frag.spv"
        ),
    ),
];

macro_rules! v4_v3_pair {
    ($stock:literal, $selected:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $stock)).as_slice(),
            include_bytes!(concat!("../../shaders/", $selected)).as_slice(),
        )
    };
}

const FRAGMENTS_V4_V3: &[ShaderPair] = &[
    v4_v3_pair!(
        "lighting.frag.spv",
        "triangle-grid-v4.unsnapped-v3.lighting.frag.spv"
    ),
    v4_v3_pair!(
        "lighting.coordinates-v2.frag.spv",
        "triangle-grid-v4.unsnapped-v3.lighting.coordinates-v2.frag.spv"
    ),
    v4_v3_pair!(
        "numeric-max-v2.lighting.frag.spv",
        "triangle-grid-v4.unsnapped-v3.numeric-max-v2.lighting.frag.spv"
    ),
    v4_v3_pair!(
        "numeric-max-v2.lighting.coordinates-v2.frag.spv",
        "triangle-grid-v4.unsnapped-v3.numeric-max-v2.lighting.coordinates-v2.frag.spv"
    ),
    v4_v3_pair!(
        "lighting.clipped.frag.spv",
        "triangle-grid-v4.unsnapped-v3.lighting.clipped.frag.spv"
    ),
    v4_v3_pair!(
        "lighting.clipped.coordinates-v2.frag.spv",
        "triangle-grid-v4.unsnapped-v3.lighting.clipped.coordinates-v2.frag.spv"
    ),
    v4_v3_pair!(
        "numeric-max-v2.lighting.clipped.frag.spv",
        "triangle-grid-v4.unsnapped-v3.numeric-max-v2.lighting.clipped.frag.spv"
    ),
    v4_v3_pair!(
        "numeric-max-v2.lighting.clipped.coordinates-v2.frag.spv",
        "triangle-grid-v4.unsnapped-v3.numeric-max-v2.lighting.clipped.coordinates-v2.frag.spv"
    ),
];

pub(super) fn fragment(v2: &'static [u8]) -> Result<&'static [u8], RenderError> {
    lookup_fragment(FRAGMENTS, v2, "V2")
}

pub(super) fn lookup_fragment(
    variants: &[ShaderPair],
    source: &'static [u8],
    version: &str,
) -> Result<&'static [u8], RenderError> {
    variants
        .iter()
        .find(|(before, _)| *before == source)
        .map(|(_, coverage)| *coverage)
        .ok_or_else(|| {
            Failure::UnsupportedInterface {
                actual: format!(
                    "{version} unsnapped fragment module of {} bytes",
                    source.len()
                ),
            }
            .into()
        })
}

pub(super) fn fragment_v3(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    FRAGMENTS_V3
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, selected)| *selected)
        .ok_or_else(|| {
            Failure::UnsupportedInterface {
                actual: format!("stock fragment module of {} bytes", stock.len()),
            }
            .into()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_v4_fragment_has_a_distinct_logical_semantics_partner() {
        for ((source, _), (v4_source, v4)) in FRAGMENTS.iter().zip(FRAGMENTS_V4) {
            assert_eq!(source, v4_source);
            assert_ne!(fragment_v4(source).unwrap(), *v4_source);
            assert_ne!(*v4, fragment(source).unwrap());
        }
        assert!(
            fragment_v4(&[0])
                .unwrap_err()
                .triangle_coverage_failure()
                .is_some()
        );
    }

    #[test]
    fn every_v2_fragment_has_one_distinct_grid_coverage_partner() {
        for (v2, selected) in FRAGMENTS {
            assert_eq!(fragment(v2).unwrap(), *selected);
            assert_ne!(v2, selected);
        }
        assert!(
            fragment(&[0])
                .unwrap_err()
                .triangle_coverage_failure()
                .is_some()
        );
    }

    #[test]
    fn every_stock_fragment_has_a_v3_unsnapped_grid_partner() {
        assert_eq!(FRAGMENTS_V3.len(), 8);
        for (stock, selected) in FRAGMENTS_V3 {
            assert_ne!(stock, selected);
            assert_eq!(fragment_v3(stock).unwrap(), *selected);
        }
        assert!(fragment_v3(&[0]).is_err());
    }

    #[test]
    fn v4_v3_fragment_registry_preserves_clipped_and_unclipped_stock_routes() {
        assert_eq!(FRAGMENTS_V4_V3.len(), 8);
        for (stock, selected) in FRAGMENTS_V4_V3 {
            assert_eq!(fragment_v4_v3(stock).unwrap(), *selected);
            assert_ne!(stock, selected);
        }
        assert!(fragment_v4_v3(&[0]).is_err());
    }
}
