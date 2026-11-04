//! Registered clipped Phong fragments with projected-grid world-plane interpolation.
use super::triangle_grid_coverage_shader::lookup_fragment;
use crate::{RenderError, RenderWorldClipInterpolationFailure as Failure};

macro_rules! world_clip_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/unsnapped-v2.", $name)).as_slice(),
            include_bytes!(concat!(
                "../../shaders/world-clip-grid12-v1.unsnapped-v2.",
                $name
            ))
            .as_slice(),
        )
    };
}

const FRAGMENTS: &[(&[u8], &[u8])] = &[
    world_clip_pair!("lighting.clipped.frag.spv"),
    world_clip_pair!("lighting.clipped.coordinates-v2.frag.spv"),
    world_clip_pair!("numeric-max-v2.lighting.clipped.frag.spv"),
    world_clip_pair!("numeric-max-v2.lighting.clipped.coordinates-v2.frag.spv"),
];
macro_rules! world_clip_pair_v3 {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $name)).as_slice(),
            include_bytes!(concat!(
                "../../shaders/world-clip-grid12-v2.unsnapped-v3.",
                $name
            ))
            .as_slice(),
        )
    };
}
#[cfg(test)]
macro_rules! world_clip_v4_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $name)).as_slice(),
            include_bytes!(concat!(
                "../../shaders/world-clip-grid12-v2.triangle-grid-v4.unsnapped-v3.",
                $name
            ))
            .as_slice(),
        )
    };
}

macro_rules! world_clip_v4_compact_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $name)).as_slice(),
            include_bytes!(concat!(
                "../../shaders/world-clip-grid12-v2.triangle-grid-v4-compact.unsnapped-v3.",
                $name
            ))
            .as_slice(),
        )
    };
}
const FRAGMENTS_V3: &[(&[u8], &[u8])] = &[
    world_clip_pair_v3!("lighting.clipped.frag.spv"),
    world_clip_pair_v3!("lighting.clipped.coordinates-v2.frag.spv"),
    world_clip_pair_v3!("numeric-max-v2.lighting.clipped.frag.spv"),
    world_clip_pair_v3!("numeric-max-v2.lighting.clipped.coordinates-v2.frag.spv"),
];

#[cfg(test)]
const FRAGMENTS_V4: &[(&[u8], &[u8])] = &[
    world_clip_v4_pair!("lighting.clipped.frag.spv"),
    world_clip_v4_pair!("lighting.clipped.coordinates-v2.frag.spv"),
    world_clip_v4_pair!("numeric-max-v2.lighting.clipped.frag.spv"),
    world_clip_v4_pair!("numeric-max-v2.lighting.clipped.coordinates-v2.frag.spv"),
];

const FRAGMENTS_V4_COMPACT: &[(&[u8], &[u8])] = &[
    world_clip_v4_compact_pair!("lighting.clipped.frag.spv"),
    world_clip_v4_compact_pair!("lighting.clipped.coordinates-v2.frag.spv"),
    world_clip_v4_compact_pair!("numeric-max-v2.lighting.clipped.frag.spv"),
    world_clip_v4_compact_pair!("numeric-max-v2.lighting.clipped.coordinates-v2.frag.spv"),
];

pub(super) fn fragment(v2: &'static [u8]) -> Result<&'static [u8], RenderError> {
    FRAGMENTS
        .iter()
        .find(|(before, _)| *before == v2)
        .map(|(_, selected)| *selected)
        .ok_or_else(|| {
            Failure::UnsupportedDraw {
                actual: format!(
                    "unregistered clipped V2 fragment module of {} bytes",
                    v2.len()
                ),
            }
            .into()
        })
}

pub(super) fn fragment_v3(v3: &'static [u8]) -> Result<&'static [u8], RenderError> {
    FRAGMENTS_V3
        .iter()
        .find(|(before, _)| *before == v3)
        .map(|(_, selected)| *selected)
        .ok_or_else(|| {
            Failure::UnsupportedDraw {
                actual: format!(
                    "unregistered clipped V3 fragment module of {} bytes",
                    v3.len()
                ),
            }
            .into()
        })
}

#[cfg(test)]
pub(super) fn fragment_v4(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    FRAGMENTS_V4
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, selected)| *selected)
        .ok_or_else(|| {
            Failure::UnsupportedDraw {
                actual: format!(
                    "unregistered clipped V4 fragment module of {} bytes",
                    stock.len()
                ),
            }
            .into()
        })
}

pub(super) fn fragment_v4_compact(stock: &'static [u8]) -> Result<&'static [u8], RenderError> {
    lookup_fragment(FRAGMENTS_V4_COMPACT, stock, "V4 compact")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_clipped_v2_fragment_has_one_grid12_world_clip_partner() {
        assert_eq!(FRAGMENTS.len(), 4);
        for (before, selected) in FRAGMENTS {
            assert_ne!(before, selected);
            assert_eq!(fragment(before).unwrap(), *selected);
        }
        assert!(fragment(&[0]).is_err());
    }

    #[test]
    fn each_stock_clipped_fragment_maps_to_combined_v3_world_clip_module() {
        assert_eq!(FRAGMENTS_V3.len(), 4);
        for (before, selected) in FRAGMENTS_V3 {
            assert_ne!(before, selected);
            assert_eq!(fragment_v3(before).unwrap(), *selected);
        }
        assert!(fragment_v3(&[0]).is_err());
    }

    #[test]
    fn each_clipped_v4_fragment_maps_only_to_combined_coverage_clip_module() {
        assert_eq!(FRAGMENTS_V4.len(), 4);
        for (before, selected) in FRAGMENTS_V4 {
            assert_ne!(before, selected);
            assert_eq!(fragment_v4(before).unwrap(), *selected);
        }
        assert!(fragment_v4(&[0]).is_err());
    }

    #[test]
    fn compact_v4_clip_partners_are_selected_only_from_registered_clipped_stock() {
        assert_eq!(FRAGMENTS_V4_COMPACT.len(), 4);
        for (before, selected) in FRAGMENTS_V4_COMPACT {
            assert_eq!(fragment_v4_compact(before).unwrap(), *selected);
            assert_ne!(before, selected);
        }
        assert!(fragment_v4_compact(&[0]).is_err());
    }
}
