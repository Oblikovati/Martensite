//! Specialize the stock Phong equations when neither the lighting nor interpolation needs world position.
use super::runtime::VulkanRuntime;
use crate::{PrimitiveDraw, PrimitiveTopology, RenderLighting, RenderShading};
use martensite_tkservice::graphic3d::Graphic3dTypeOfLightSource;

// Compile lighting.{vert,frag} with glslangValidator -V --target-env vulkan1.3
// and -DMARTENSITE_ORTHOGRAPHIC_DIRECTIONAL_PHONG=1. The fragment module is byte-identical
// with or without MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES: it does not use derivatives.
pub(super) const SHADERS: [&[u8]; 2] = [
    include_bytes!("../../shaders/lighting.directional-phong.vert.spv"),
    include_bytes!("../../shaders/lighting.directional-phong.frag.spv"),
];

// Amortize the extra pipeline for large resident streams. The transfer runs with a
// 51,471-vertex mesh regressed when it selected this variant (#594).
pub(crate) const MIN_VERTICES: usize =
    (4 << 20) / super::pipeline_state::LIT_VERTEX_STRIDE as usize;

#[cfg(test)]
pub(super) fn admitted(runtime: &VulkanRuntime, draw: &PrimitiveDraw) -> bool {
    admitted_for_vertices(runtime, draw, draw.vertices.len())
}

pub(super) fn admitted_for_vertices(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
    vertex_count: usize,
) -> bool {
    vertex_count >= MIN_VERTICES && supported_behavior(runtime) && directional_phong_draw(draw)
}

pub(crate) fn directional_phong_draw(draw: &PrimitiveDraw) -> bool {
    ordinary_surface(draw) && draw.lighting.as_ref().is_some_and(directional_material)
}

pub(super) fn supported_behavior(runtime: &VulkanRuntime) -> bool {
    !cfg!(any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics",
        feature = "lit-only-command-diagnostics"
    )) && runtime.light_dot_behavior == crate::RenderLightDotBehavior::DriverNativeV1
        && runtime.phong_interpolation == crate::RenderPhongInterpolationBehavior::HardwareV1
        && runtime.triangle_coverage == crate::RenderTriangleCoverageBehavior::HardwareV1
        && runtime.projection_behavior == crate::RenderProjectionBehavior::PrecomposedV1
}

pub(super) fn ordinary_surface(draw: &PrimitiveDraw) -> bool {
    draw.topology() == PrimitiveTopology::Triangles
        && !draw.annotation
        && draw.clipping.is_none()
        && draw.hatch_rows.is_none()
        && draw.surface_texture.is_none()
        && draw.surface_texture_unavailable.is_none()
        && draw.surface_cutoff.is_none()
        && draw.unlit_back_color.is_none()
        && !draw.surface_blend
        && !draw.surface_forced_blend
        && draw.mesh_edges.is_none()
        && draw.polygon_edge_style.is_none()
        && !draw.polygon_points
        && draw.textured_style.is_none()
        && draw.user_program.is_none()
        && draw.capping_pass == crate::RenderCappingPass::None
        && draw.depth_pass == Default::default()
        && !draw.fixed_function_lighting
}

fn directional_material(state: &RenderLighting) -> bool {
    state.shading == RenderShading::Phong
        && state.fixed.is_none()
        && !state.pbr
        && !state.vertex_colors
        && state.eye[3] == 0.
        && directional_lights(&state.lights)
}

pub(crate) fn directional_lights(lights: &crate::RenderLightSet) -> bool {
    lights.sources.iter().all(|source| {
        source.kind == Graphic3dTypeOfLightSource::Directional as u32 && !source.cast_shadows
    })
}

#[cfg(all(
    test,
    not(any(
        feature = "command-state-diagnostics",
        feature = "vertex-output-diagnostics",
        feature = "lit-only-command-diagnostics"
    ))
))]
mod tests {
    use super::*;
    use crate::backend::directional_phong_fixture::lit_surface;
    use crate::backend::resource_lifecycle_test_support::create_resource_renderer;
    use crate::{RenderBackend, RenderFrame};

    #[test]
    fn directional_phong_matches_the_general_shader_at_every_color_and_depth_pixel() {
        let mut renderer = create_resource_renderer();
        let stock = lit_surface(6);
        let large = lit_surface(MIN_VERTICES);
        assert!(!admitted(&renderer.runtime, &stock));
        assert!(admitted(&renderer.runtime, &large));
        let general = renderer
            .pipeline
            .select(&stock, false, None, false)
            .unwrap();
        let specialized = renderer
            .pipeline
            .select(&large, false, None, false)
            .unwrap();
        assert_ne!(general.handle, specialized.handle);
        let reference = renderer.render(&frame(stock)).unwrap();
        let result = renderer.render(&frame(large)).unwrap();
        assert!(
            reference
                .rgba
                .as_chunks::<4>()
                .0
                .iter()
                .any(|pixel| *pixel != [0, 0, 0, 255])
        );
        assert_eq!(reference.rgba, result.rgba);
        assert_eq!(reference.depth, result.depth);
        assert!(renderer.validation_errors().unwrap().is_empty());
    }

    #[test]
    fn shader_admission_tracks_live_lighting_inputs() {
        let renderer = create_resource_renderer();
        let mut draw = lit_surface(MIN_VERTICES);
        for change in [
            |state: &mut RenderLighting| state.vertex_colors = true,
            |state: &mut RenderLighting| state.eye[3] = 1.,
            |state: &mut RenderLighting| state.lights.sources.make_mut()[0].cast_shadows = true,
            |state: &mut RenderLighting| {
                state.lights.sources.make_mut()[0].kind =
                    Graphic3dTypeOfLightSource::Positional as u32
            },
            |state: &mut RenderLighting| state.shading = RenderShading::Gouraud,
        ] as [fn(&mut RenderLighting); 5]
        {
            let original = draw.lighting.clone();
            change(draw.lighting.as_mut().unwrap());
            assert!(!admitted(&renderer.runtime, &draw));
            draw.lighting = original;
        }
    }

    fn frame(draw: PrimitiveDraw) -> RenderFrame {
        RenderFrame::new(48, 32, [0., 0., 0., 1.], vec![draw]).unwrap()
    }
}
