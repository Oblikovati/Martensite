//! Every selected source triangle is admitted before allocation or command recording.
use super::{
    pipeline_variants::PipelineVariant, runtime::VulkanRuntime,
    window_depth_parameters::WindowDepthPipelineParameters as Parameters,
};
use crate::{
    PrimitiveDraw, PrimitiveTopology, RenderError, RenderWindowDepthBehavior as Behavior,
    RenderWindowDepthBoundary as Boundary, RenderWindowDepthFailure as Failure,
    window_depth_closure::{
        WindowDepthClosureFailure, prove_fragment_window_depth_closure, prove_window_depth_closure,
    },
};
use ash::vk;

pub(super) fn enabled(behavior: Behavior, draw: &PrimitiveDraw) -> bool {
    if super::clipped_depth_device::selected(behavior) {
        return super::clipped_depth_upload::applies(draw);
    }
    behavior != Behavior::HardwareV1 && draw.topology() == PrimitiveTopology::Triangles
}

pub(super) fn prepare(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
    extent: vk::Extent2D,
    samples: vk::SampleCountFlags,
) -> Result<Option<Parameters>, RenderError> {
    if !enabled(runtime.window_depth_behavior, draw) {
        return Ok(None);
    }
    require_interface(runtime, draw, samples)?;
    let viewport =
        super::fixed_grid_viewport::viewport_pair(runtime.attachment_coordinates, extent)[0];
    let parameters = Parameters::selected(
        runtime.window_depth_behavior,
        viewport,
        runtime.depth.clip_control,
    )?
    .expect("explicitly selected depth behavior");
    admit_triangles(runtime, draw, parameters)?;
    Ok(Some(parameters))
}

fn require_interface(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
    samples: vk::SampleCountFlags,
) -> Result<(), RenderError> {
    if super::clipped_depth_device::selected(runtime.window_depth_behavior) {
        return super::clipped_depth_upload::require_interface(runtime, draw, samples);
    }
    let kind = super::pipeline_variants::special_variant(draw, false);
    let rejected = [
        (
            "V3 requires independently selected Native24UnitsV2",
            matches!(
                runtime.window_depth_behavior,
                Behavior::ReferenceWindowDepthV3 { .. }
                    | Behavior::Float64ReferenceWindowDepthV5 { .. }
            ) && runtime.depth_bias_behavior != crate::RenderDepthBiasBehavior::Native24UnitsV2,
        ),
        ("multisampling", samples != vk::SampleCountFlags::TYPE_1),
        (
            "alternate triangle program",
            !super::factorized_projection::kind_supported(kind),
        ),
        (
            "custom/compatibility program",
            draw.user_program.is_some() || draw.compatibility_color_target,
        ),
        (
            "fixed lighting or depth clamp",
            draw.fixed_function_lighting || draw.depth_clamp,
        ),
        (
            "nondefault depth pass",
            draw.depth_pass != Default::default(),
        ),
        (
            "world clipping",
            runtime.world_clipping != crate::RenderWorldClippingBehavior::FragmentV1,
        ),
        (
            "Phong interpolation",
            runtime.phong_interpolation != crate::RenderPhongInterpolationBehavior::HardwareV1,
        ),
        (
            "conservative coverage",
            runtime.triangle_coverage != crate::RenderTriangleCoverageBehavior::HardwareV1,
        ),
    ]
    .into_iter()
    .find_map(|(name, active)| active.then_some(name));
    if let Some(actual) = rejected {
        return Err(Failure::UnsupportedInterface {
            actual: actual.into(),
        }
        .into());
    }
    Ok(())
}

fn admit_triangles(
    runtime: &VulkanRuntime,
    draw: &PrimitiveDraw,
    parameters: Parameters,
) -> Result<(), RenderError> {
    if parameters.clipped_fan() {
        return super::clipped_depth_upload::admit_vertices(runtime, draw, parameters);
    }
    let projected = super::window_depth_transforms::projected_bounds(
        draw,
        runtime.projection_behavior,
        parameters.float64() || parameters.negative_one_to_one(),
    )
    .ok_or_else(|| Failure::UnsupportedInterface {
        actual: "nonfinite or unavailable source projection bounds".into(),
    })?;
    let viewport = parameters.viewport();
    let rectangle = [viewport.x, viewport.y, viewport.width, viewport.height];
    let [reference, raster] = parameters.grids();
    for (triangle, indices) in draw.indices.as_chunks::<3>().0.iter().enumerate() {
        let source = [indices[0], indices[1], indices[2]];
        let clips = source.map(|index| projected[index as usize]);
        let proof = if parameters.fragment_depth() {
            prove_fragment_window_depth_closure(
                clips,
                rectangle,
                parameters.float64() || parameters.negative_one_to_one(),
                reference,
                raster,
                identical_source_pair(draw, source),
            )
        } else {
            prove_window_depth_closure(
                clips,
                rectangle,
                parameters.float64() || parameters.negative_one_to_one(),
                reference,
                raster,
            )
        };
        proof.map_err(|failure| Failure::Triangle {
            triangle,
            vertex_indices: source,
            boundary: boundary(failure),
        })?;
    }
    Ok(())
}

fn identical_source_pair(draw: &PrimitiveDraw, indices: [u32; 3]) -> bool {
    let keys = indices.map(|index| {
        let vertex = draw.vertices[index as usize];
        let [x, y, z] = vertex.position;
        [x, y, z, vertex.weight].map(f32::to_bits)
    });
    keys[0] == keys[1] || keys[0] == keys[2] || keys[1] == keys[2]
}

fn boundary(failure: WindowDepthClosureFailure) -> Boundary {
    match failure {
        WindowDepthClosureFailure::OriginalClip => Boundary::OriginalClip,
        WindowDepthClosureFailure::WindowRange => Boundary::WindowRange,
        WindowDepthClosureFailure::ReferenceGrid => Boundary::ReferenceGrid,
        WindowDepthClosureFailure::Arithmetic => Boundary::ShaderArithmetic,
        WindowDepthClosureFailure::CorrectedClip => Boundary::CorrectedClip,
    }
}

pub(super) fn require_physical_source(
    parameters: Option<Parameters>,
    semantic: &PrimitiveDraw,
    physical: &PrimitiveDraw,
) -> Result<(), RenderError> {
    if parameters.is_none() || std::ptr::eq(semantic, physical) {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface {
        actual: "private replacement triangle geometry".into(),
    }
    .into())
}

pub(super) fn require_pipeline(
    parameters: Option<Parameters>,
    actual: PipelineVariant,
) -> Result<(), RenderError> {
    if parameters == actual.window_depth {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface {
        actual: "actual pipeline does not carry admitted window-depth parameters".into(),
    }
    .into())
}

pub(super) fn require_scene_viewports(
    runtime: &VulkanRuntime,
    draws: &[super::frame::UploadedDraw],
    extent: vk::Extent2D,
) -> Result<(), RenderError> {
    // All public scene recording paths use this exact full-region viewport (including 0..1 depth).
    let actual =
        super::fixed_grid_viewport::viewport_pair(runtime.attachment_coordinates, extent)[0];
    for draw in draws {
        if let Some(parameters) = draw.pipeline.kind.window_depth {
            parameters.require_actual_viewport(actual)?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "window_depth_upload_tests.rs"]
mod tests;
