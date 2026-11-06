//! Registered geometry interfaces preserve original interpolation varyings and vertex pushes.
use super::pipeline_variants::PipelineVariant;
use crate::{RenderError, RenderWindowDepthFailure as Failure};
use ash::vk;

pub(super) fn seed_samples(kind: PipelineVariant) -> vk::SampleCountFlags {
    if kind
        .window_depth
        .is_some_and(|parameters| parameters.clipped_fan())
    {
        return vk::SampleCountFlags::TYPE_8;
    }
    vk::SampleCountFlags::TYPE_1
}

pub(super) fn geometry(kind: PipelineVariant) -> Option<&'static [u8]> {
    kind.window_depth.map(|parameters| {
        if parameters.shared_incidence() {
            return include_bytes!("../../shaders/clipped.incidence.geom.spv").as_slice();
        }
        if parameters.clipped_fan() {
            return include_bytes!("../../shaders/clipped.depth.geom.spv").as_slice();
        }
        if parameters.float64() {
            return if kind.lighting {
                include_bytes!("../../shaders/window.depth.v5.lighting.geom.spv").as_slice()
            } else {
                include_bytes!("../../shaders/window.depth.v5.unlit.geom.spv").as_slice()
            };
        }
        if parameters.fragment_depth() {
            return if kind.lighting {
                include_bytes!("../../shaders/window.depth.v3.lighting.geom.spv").as_slice()
            } else {
                include_bytes!("../../shaders/window.depth.v3.unlit.geom.spv").as_slice()
            };
        }
        if kind.lighting {
            include_bytes!("../../shaders/window.depth.lighting.geom.spv").as_slice()
        } else {
            include_bytes!("../../shaders/window.depth.unlit.geom.spv").as_slice()
        }
    })
}

pub(super) fn vertex(
    kind: PipelineVariant,
    stock: &'static [u8],
) -> Result<&'static [u8], RenderError> {
    if kind
        .window_depth
        .is_some_and(|parameters| parameters.float64())
    {
        return super::window_depth_fragment::float64_vertex(stock);
    }
    if kind.window_depth.is_none() || !kind.lighting {
        return Ok(stock);
    }
    for (original, compact) in COMPACT_VERTICES {
        if stock == *original {
            return Ok(compact);
        }
    }
    Err(Failure::UnsupportedInterface {
        actual: "unregistered window-depth lighting vertex".into(),
    }
    .into())
}

const COMPACT_VERTICES: &[(&[u8], &[u8])] = &[
    (
        include_bytes!("../../shaders/lighting.vert.spv"),
        include_bytes!("../../shaders/window.depth.lighting.vert.spv"),
    ),
    (
        include_bytes!("../../shaders/numeric-max-v2.lighting.vert.spv"),
        include_bytes!("../../shaders/window.depth.numeric-max-v2.lighting.vert.spv"),
    ),
    (
        include_bytes!("../../shaders/factorized-v2.lighting.vert.spv"),
        include_bytes!("../../shaders/window.depth.factorized-v2.lighting.vert.spv"),
    ),
    (
        include_bytes!("../../shaders/factorized-v2.numeric-max-v2.lighting.vert.spv"),
        include_bytes!("../../shaders/window.depth.factorized-v2.numeric-max-v2.lighting.vert.spv"),
    ),
];

pub(super) fn admit_kind(
    kind: PipelineVariant,
    oit: bool,
    samples: vk::SampleCountFlags,
) -> Result<(), RenderError> {
    if kind.window_depth.is_none() {
        return Ok(());
    }
    if kind.window_depth.is_some_and(|p| p.clipped_fan()) {
        return super::clipped_depth_upload::require_pipeline(kind, oit, samples);
    }
    if !oit
        && samples == vk::SampleCountFlags::TYPE_1
        && super::factorized_projection::kind_supported(kind)
        && !kind.clamp
        && !kind.triangle_grid_coverage
    {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface {
        actual: format!("actual pipeline {kind:?}, OIT={oit}, samples={samples:?}"),
    }
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_pipeline_has_no_new_geometry_or_capability_refusal() {
        let kind = super::super::pipeline_variants::PipelineVariant::from_index(0, false);
        assert!(geometry(kind).is_none());
        assert!(admit_kind(kind, true, vk::SampleCountFlags::TYPE_4).is_ok());
    }
    #[test]
    fn compact_lighting_registry_preserves_every_declared_vertex_variant() {
        let mut kind = super::super::pipeline_variants::PipelineVariant::from_index(0, false);
        for (original, _) in COMPACT_VERTICES {
            assert_eq!(vertex(kind, original).unwrap(), *original);
        }
        kind.lighting = true;
        kind.window_depth =
            super::super::window_depth_parameters::WindowDepthPipelineParameters::selected(
                crate::RenderWindowDepthBehavior::UnsnappedWindowDepthV2 {
                    reference_grid: crate::RenderViewportGridPrecision::Fractional12,
                    raster_grid: crate::RenderViewportGridPrecision::Fractional8,
                },
                vk::Viewport {
                    width: 409.,
                    height: 409.,
                    max_depth: 1.,
                    ..Default::default()
                },
                false,
            )
            .unwrap();
        for (original, compact) in COMPACT_VERTICES {
            assert_eq!(vertex(kind, original).unwrap(), *compact);
        }
        assert!(vertex(kind, b"unknown").is_err());
    }
}

#[cfg(test)]
#[path = "window_depth_seed_tests.rs"]
mod seed_tests;
