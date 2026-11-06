//! Every graphics pipeline applies the same admitted runtime policy.
use super::mirrored_sample_locations::places_mirrored;
use super::{
    attachment_coordinates::AttachmentCoordinates,
    runtime::{VulkanRuntime, vk_error},
};
use crate::RenderError;
use ash::vk;

/// Apply one runtime policy to all graphics pipelines; `create_pipeline(runtime, info, raster)?`.
pub(super) fn create_pipeline(
    runtime: &VulkanRuntime,
    info: vk::GraphicsPipelineCreateInfo<'_>,
    raster: &vk::PipelineRasterizationStateCreateInfo<'_>,
) -> Result<vk::Pipeline, RenderError> {
    if !runtime.depth.clip_control {
        return create_with_raster_policy(runtime, info, raster);
    }
    // OpenGL clip depth through the viewport transform: negativeOneToOne, and the vertex
    // stages skip their zero-to-one remap (specialization constant 7, kRemapClipDepth). The
    // mesh-edge geometry stage reads the same constant, because its altitudes use window depth.
    let remap = 0u32.to_ne_bytes();
    let entries = [vk::SpecializationMapEntry::default()
        .constant_id(7)
        .offset(0)
        .size(4)];
    let specialization = vk::SpecializationInfo::default()
        .map_entries(&entries)
        .data(&remap);
    // The caller's stage array and viewport state outlive this call.
    // SAFETY: the pointer and length describe a live allocation or Vulkan output array of this scope, and the view does not outlive it.
    let original = unsafe { std::slice::from_raw_parts(info.p_stages, info.stage_count as usize) };
    let stages: Vec<_> = original
        .iter()
        .map(|stage| {
            if (stage.stage == vk::ShaderStageFlags::VERTEX
                || stage.stage == vk::ShaderStageFlags::GEOMETRY)
                && stage.p_specialization_info.is_null()
            {
                stage.specialization_info(&specialization)
            } else {
                *stage
            }
        })
        .collect();
    let mut clip =
        vk::PipelineViewportDepthClipControlCreateInfoEXT::default().negative_one_to_one(true);
    // SAFETY: the pointer comes from a live Vulkan structure of this scope, and Vulkan's structure rules make it valid to read here.
    let viewport = unsafe { info.p_viewport_state.as_ref() }.map(|state| {
        // Same viewport and scissor description, chained to the clip-control node.
        let mut linked = vk::PipelineViewportStateCreateInfo::default().push_next(&mut clip);
        linked.flags = state.flags;
        linked.viewport_count = state.viewport_count;
        linked.p_viewports = state.p_viewports;
        linked.scissor_count = state.scissor_count;
        linked.p_scissors = state.p_scissors;
        linked
    });
    let mut info = info.stages(&stages);
    if let Some(viewport) = &viewport {
        info = info.viewport_state(viewport);
    }
    create_with_raster_policy(runtime, info, raster)
}
fn create_with_raster_policy(
    runtime: &VulkanRuntime,
    info: vk::GraphicsPipelineCreateInfo<'_>,
    raster: &vk::PipelineRasterizationStateCreateInfo<'_>,
) -> Result<vk::Pipeline, RenderError> {
    let coordinates = runtime.attachment_coordinates;
    let legacy = coordinates == AttachmentCoordinates::LegacyBottomUpV1;
    if legacy && !runtime.raster_policy.requires_last_vertex() {
        return create_unmodified_pipeline(runtime, info);
    }
    let mut provoking = vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT::default()
        .provoking_vertex_mode(vk::ProvokingVertexModeEXT::LAST_VERTEX);
    let selected = select_raster(
        raster,
        coordinates,
        runtime.raster_policy.requires_last_vertex(),
        &mut provoking,
    );
    let info = info.rasterization_state(&selected);
    create_unmodified_pipeline(runtime, info)
}
pub(super) fn select_raster<'a>(
    raster: &'a vk::PipelineRasterizationStateCreateInfo<'a>,
    coordinates: AttachmentCoordinates,
    requires_last_vertex: bool,
    provoking: &'a mut vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT<'static>,
) -> vk::PipelineRasterizationStateCreateInfo<'a> {
    let mut selected = *raster;
    selected.front_face = coordinates.pipeline_front_face(selected.front_face);
    if requires_last_vertex {
        selected = selected.push_next(provoking);
    }
    selected
}
/// Multisampled pipelines place OpenGL's mirrored standard positions where the device can.
fn create_unmodified_pipeline(
    runtime: &VulkanRuntime,
    info: vk::GraphicsPipelineCreateInfo<'_>,
) -> Result<vk::Pipeline, RenderError> {
    // The caller's multisample state outlives this call.
    // SAFETY: the pointer comes from a live Vulkan structure of this scope, and Vulkan's structure rules make it valid to read here.
    let Some(samples) = (unsafe { info.p_multisample_state.as_ref() }) else {
        return create_located_pipeline(runtime, info);
    };
    let count = samples.rasterization_samples;
    let positions = super::mirrored_sample_locations::mirrored_positions(count)
        .filter(|_| places_mirrored(runtime.mirrored_samples, count));
    let Some(positions) = positions else {
        return create_located_pipeline(runtime, info);
    };
    let mut locations = vk::PipelineSampleLocationsStateCreateInfoEXT::default()
        .sample_locations_enable(true)
        .sample_locations_info(super::mirrored_sample_locations::locations_info(positions));
    let linked = samples.push_next(&mut locations);
    create_located_pipeline(runtime, info.multisample_state(&linked))
}
fn create_located_pipeline(
    runtime: &VulkanRuntime,
    info: vk::GraphicsPipelineCreateInfo<'_>,
) -> Result<vk::Pipeline, RenderError> {
    #[cfg(test)]
    let info = executable_creation_info(runtime, info);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    match unsafe {
        runtime
            .device()
            .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)
    } {
        Ok(pipelines) => {
            #[cfg(any(
                feature = "vertex-output-diagnostics",
                feature = "command-state-diagnostics"
            ))]
            super::vertex_pipeline_diagnostic::created(runtime, &info, pipelines[0]);
            Ok(pipelines[0])
        }
        Err((partial, error)) => {
            for handle in partial {
                // SAFETY: the handle belongs to this device and its owner releases it only after the GPU work that used it has completed.
                unsafe { runtime.device().destroy_pipeline(handle, None) };
            }
            Err(vk_error("create graphics pipeline", error))
        }
    }
}

#[cfg(test)]
#[path = "raster_pipeline_policy_tests.rs"]
mod tests;

#[cfg(test)]
fn executable_creation_info<'a>(
    runtime: &VulkanRuntime,
    info: vk::GraphicsPipelineCreateInfo<'a>,
) -> vk::GraphicsPipelineCreateInfo<'a> {
    let flags = super::diagnostic_executable::creation_flags(
        runtime.executable_profile.is_some(),
        info.flags,
    );
    info.flags(flags)
}
