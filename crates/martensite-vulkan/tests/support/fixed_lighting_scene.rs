//! Genuine retained scene adapter for native GL lighting fixtures.
use super::lighting_scene::LightingScene;
use martensite_vulkan::{
    Graphic3dRenderDriver, RenderError, RenderLightingPipeline, RenderMeshEdgePipeline,
    RenderTextureCoordinatePipeline, VulkanOptions,
};
#[path = "fixed_lighting_arrays.rs"]
mod fixed_arrays;
#[path = "fixed_lighting_binding.rs"]
mod fixed_binding;
#[path = "fixed_lighting_markers.rs"]
mod fixed_markers;
use martensite_tkernel::standard::Handle;
#[path = "fixed_lighting_transform.rs"]
mod fixed_transform;
use fixed_transform::apply_fixed_fixture_transform;
use martensite_tkservice::graphic3d::*;

pub fn fixed_lighting_driver(
    options: VulkanOptions,
) -> Result<Handle<Graphic3dRenderDriver>, RenderError> {
    Graphic3dRenderDriver::with_render_pipelines(
        options,
        RenderTextureCoordinatePipeline::GeneratedCompatibility,
        RenderMeshEdgePipeline::Programmable,
        RenderLightingPipeline::FixedFunction,
    )
}
/// Build the fixed-lighting integration fixture with one explicit replay configuration.
/// Example: `fixed_lighting_driver_with_behavior(options, RenderBehaviorConfiguration::default())?`.
#[allow(dead_code)] // Only some targets that share this file use it.
pub fn fixed_lighting_driver_with_behavior(
    options: VulkanOptions,
    behavior: martensite_vulkan::RenderBehaviorConfiguration,
) -> Result<Handle<Graphic3dRenderDriver>, RenderError> {
    let selection = martensite_vulkan::RenderPipelinePolicies {
        texture_coordinates: RenderTextureCoordinatePipeline::GeneratedCompatibility,
        mesh_edges: RenderMeshEdgePipeline::Programmable,
        lighting: RenderLightingPipeline::FixedFunction,
        ..Default::default()
    };
    Graphic3dRenderDriver::with_render_behavior_configuration(
        options,
        selection,
        martensite_vulkan::RenderCoordinateConvention::default(),
        martensite_vulkan::RenderLineWidthPolicy::default(),
        martensite_vulkan::RenderLineClippingBehavior::default(),
        behavior,
    )
}
pub fn fixed_lighting_scene(driver: &Handle<Graphic3dRenderDriver>, mode: &str) -> LightingScene {
    let canonical = fixed_fixture_mode(mode);
    let mut fixture = LightingScene::new(driver, &canonical);
    configure_fixed_fixture_style(&fixture, mode);
    replace_fixed_fixture_vertices(&mut fixture, mode);
    if mode == "lit-perspective" {
        fixture
            .base
            .view
            .camera()
            .set_projection_type(Graphic3dCameraProjection::Perspective);
    }
    apply_fixed_fixture_transform(&fixture, mode);
    configure_clip_msaa_quad(&fixture, mode);
    fixed_binding::fixed_adjacent_groups(&fixture, mode);
    fixture
}

fn fixed_fixture_mode(mode: &str) -> String {
    if mode.starts_with("lit-clip-msaa-quad-") {
        return "lit-phong".into();
    }
    if let Some(suffix) = mode.strip_prefix("lit-unlit-") {
        return format!("lit-{suffix}");
    }
    if let Some(suffix) = mode.strip_prefix("lit-adjacent-") {
        return format!("lit-{suffix}");
    }
    if let Some(suffix) = mode.strip_prefix("lit-persist-") {
        return format!("lit-{suffix}");
    }
    if mode.starts_with("lit-facet-") {
        return "lit-facet".into();
    }
    if let Some(suffix) = mode.strip_prefix("lit-strip-") {
        return format!("lit-line-{suffix}");
    }
    mode.into()
}

fn configure_clip_msaa_quad(fixture: &LightingScene, mode: &str) {
    let Some(samples) = clip_msaa_sample_count(mode) else {
        return;
    };
    let plane = Handle::new(
        Graphic3dClipPlane::from_equation(martensite_tkernel::ncollection::Vec4::new(
            1., 0., 0., -0.25,
        ))
        .unwrap(),
    );
    let planes = Handle::new(Graphic3dSequenceOfHClipPlane::default());
    assert!(planes.append(plane));
    fixture.base.cview().set_clip_planes(&planes);
    fixture
        .base
        .cview()
        .change_rendering_params(&mut |params| params.nb_msaa_samples = samples);
}

fn clip_msaa_sample_count(mode: &str) -> Option<i32> {
    match mode {
        "lit-clip-msaa-quad-0" => Some(0),
        "lit-clip-msaa-quad-8" => Some(8),
        _ => None,
    }
}

fn configure_fixed_fixture_style(fixture: &LightingScene, mode: &str) {
    let shading = if mode.contains("pbr") {
        if mode.contains("facet") {
            Graphic3dTypeOfShadingModel::PbrFacet
        } else {
            Graphic3dTypeOfShadingModel::Pbr
        }
    } else if mode.starts_with("lit-unlit") || mode.contains("marker-") && mode.contains("unlit") {
        Graphic3dTypeOfShadingModel::Unlit
    } else if mode.starts_with("lit-marker-") && mode.contains("facet") {
        Graphic3dTypeOfShadingModel::PhongFacet
    } else {
        return;
    };
    fixture.aspect.set_shading_model(shading);
}
fn replace_fixed_fixture_vertices(fixture: &mut LightingScene, mode: &str) {
    let vertices = if mode.starts_with("lit-marker-") {
        fixed_markers::fixed_marker_aspect(&fixture.aspect, mode);
        Some(fixed_markers::fixed_marker_array(mode))
    } else {
        fixed_arrays::fixed_array_override(mode)
    };
    let Some(vertices) = vertices else {
        return;
    };
    fixture.base.group.clear(false);
    fixture
        .base
        .group
        .set_group_primitives_aspect(&fixture.aspect.cast());
    fixture.base.group.add_primitive_array(&vertices, true);
    fixture.vertices = vertices;
}
