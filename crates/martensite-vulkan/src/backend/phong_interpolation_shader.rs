//! Narrow registered interfaces avoid silently selecting a different shading program.
use super::fragment_variant_registry::FragmentVariant;
use crate::{RenderError, RenderPhongInterpolationFailure as Failure};
macro_rules! interpolation_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $name)) as &[u8],
            include_bytes!(concat!("../../shaders/unsnapped-v2.", $name)) as &[u8],
        )
    };
}
const VERTICES: &[FragmentVariant] = &[
    interpolation_pair!("lighting.vert.spv"),
    interpolation_pair!("numeric-max-v2.lighting.vert.spv"),
    interpolation_pair!("lighting.clipped.vert.spv"),
    interpolation_pair!("numeric-max-v2.lighting.clipped.vert.spv"),
];
const FRAGMENTS: &[FragmentVariant] = &[
    interpolation_pair!("lighting.frag.spv"),
    interpolation_pair!("lighting.coordinates-v2.frag.spv"),
    interpolation_pair!("numeric-max-v2.lighting.frag.spv"),
    interpolation_pair!("numeric-max-v2.lighting.coordinates-v2.frag.spv"),
    interpolation_pair!("lighting.clipped.frag.spv"),
    interpolation_pair!("lighting.clipped.coordinates-v2.frag.spv"),
    interpolation_pair!("numeric-max-v2.lighting.clipped.frag.spv"),
    interpolation_pair!("numeric-max-v2.lighting.clipped.coordinates-v2.frag.spv"),
];
pub(super) fn triangle_kind(kind: super::pipeline_variants::PipelineVariant) -> bool {
    kind.lighting
        && !kind.fixed_lighting
        && !kind.pbr
        && !kind.line
        && !kind.point
        && !kind.text
        && !kind.polygon_points
        && !kind.polygon_edges
        && !kind.mesh_edges
}
pub(super) fn vertex(stock: &[u8]) -> Option<&'static [u8]> {
    VERTICES
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, after)| *after)
}
pub(super) fn fragment(stock: &[u8]) -> Result<&'static [u8], RenderError> {
    FRAGMENTS
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, after)| *after)
        .ok_or_else(|| {
            Failure::UnsupportedInterface {
                actual: format!("fragment {} bytes", stock.len()),
            }
            .into()
        })
}
pub(super) fn fragment_for(
    behavior: crate::RenderPhongInterpolationBehavior,
    stock: &[u8],
) -> Result<&'static [u8], RenderError> {
    if behavior != crate::RenderPhongInterpolationBehavior::UnsnappedV3 {
        return fragment(stock);
    }
    FRAGMENTS_V3
        .iter()
        .find(|(before, _)| *before == stock)
        .map(|(_, after)| *after)
        .ok_or_else(|| {
            Failure::UnsupportedInterface {
                actual: format!("V3 fragment {} bytes", stock.len()),
            }
            .into()
        })
}
macro_rules! projected_pair {
    ($name:literal) => {
        (
            include_bytes!(concat!("../../shaders/", $name)) as &[u8],
            include_bytes!(concat!("../../shaders/unsnapped-v3.", $name)) as &[u8],
        )
    };
}
const FRAGMENTS_V3: &[FragmentVariant] = &[
    projected_pair!("lighting.frag.spv"),
    projected_pair!("lighting.coordinates-v2.frag.spv"),
    projected_pair!("numeric-max-v2.lighting.frag.spv"),
    projected_pair!("numeric-max-v2.lighting.coordinates-v2.frag.spv"),
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn v3_fragment_registry_is_distinct_and_v2_replay_is_unchanged() {
        use crate::RenderPhongInterpolationBehavior as Behavior;
        for ((stock, v2), (v3_stock, v3)) in FRAGMENTS.iter().zip(FRAGMENTS_V3) {
            assert_eq!(stock, v3_stock);
            assert_ne!(v2, v3);
            assert_eq!(fragment_for(Behavior::UnsnappedV2, stock).unwrap(), *v2);
            assert_eq!(fragment_for(Behavior::UnsnappedV3, stock).unwrap(), *v3);
        }
        assert!(fragment_for(Behavior::UnsnappedV3, &[0]).is_err());
    }
    #[test]
    fn only_programmable_triangle_interfaces_can_select_v2() {
        let mut kind = super::super::pipeline_variants::PipelineVariant::from_index(1, false);
        kind.lighting = true;
        assert!(triangle_kind(kind));
        for other in [
            super::super::pipeline_variants::PipelineVariant {
                point: true,
                ..kind
            },
            super::super::pipeline_variants::PipelineVariant { line: true, ..kind },
            super::super::pipeline_variants::PipelineVariant {
                fixed_lighting: true,
                ..kind
            },
            super::super::pipeline_variants::PipelineVariant { pbr: true, ..kind },
        ] {
            assert!(!triangle_kind(other));
        }
    }
    #[test]
    fn all_registered_variants_preserve_distinct_module_identity() {
        for (before, after) in VERTICES {
            assert_eq!(vertex(before), Some(*after));
            assert_ne!(before, after);
        }
        for (before, after) in FRAGMENTS {
            assert_eq!(fragment(before).unwrap(), *after);
            assert_ne!(before, after);
        }
        assert!(vertex(&[0]).is_none());
        assert!(
            fragment(&[0])
                .unwrap_err()
                .phong_interpolation_failure()
                .is_some()
        );
    }
}
