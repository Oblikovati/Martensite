//! Operation-local creation inputs, read while their Vulkan backing descriptions are live.
use super::*;

pub(crate) struct OwnedPipelineCapture {
    pub specialization: [u8; 4],
    pub json: String,
}
pub(crate) fn snapshot(
    runtime: &VulkanRuntime,
    info: &vk::GraphicsPipelineCreateInfo<'_>,
    specialization: [u8; 4],
    depth: vk::Format,
    stencil: vk::Format,
) -> OwnedPipelineCapture {
    let depth = depth.as_raw();
    let stencil = stencil.as_raw();
    let clip = runtime.depth.clip_control;
    let (fixed, raster) = (fixed_fields(info), raster_fields(runtime, info));
    let vertex = vertex_fields(info);
    let json = format!(
        "{{{fixed},{raster},{vertex},\"depth_format\":{depth},\"stencil_format\":{stencil},\"clip_negative_one_to_one\":{clip}}}"
    );
    OwnedPipelineCapture {
        specialization,
        json,
    }
}
fn fixed_fields(info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    // The caller owns every pointed-to description until this synchronous snapshot returns.
    // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
    let assembly = unsafe { &*info.p_input_assembly_state };
    // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
    let depth = unsafe { &*info.p_depth_stencil_state };
    // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
    let samples = unsafe { &*info.p_multisample_state };
    // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
    let blend = unsafe { &*info.p_color_blend_state };
    let topology = assembly.topology.as_raw();
    let (restart, compare) = (
        assembly.primitive_restart_enable,
        depth.depth_compare_op.as_raw(),
    );
    let bounds = depth.depth_bounds_test_enable;
    let blending = blend_fields(blend);
    let sampling = sample_fields(samples);
    let logic = blend.logic_op_enable;
    format!(
        "\"topology\":{topology},\"restart\":{restart},\"depth_compare\":{compare},\"depth_bounds\":{bounds},\"logic_op\":{logic},{sampling},{blending}"
    )
}
fn sample_fields(samples: &vk::PipelineMultisampleStateCreateInfo<'_>) -> String {
    let count = samples.rasterization_samples.as_raw();
    let shading = samples.sample_shading_enable;
    let coverage = samples.alpha_to_coverage_enable;
    let one = samples.alpha_to_one_enable;
    let mask = samples.p_sample_mask.is_null();
    format!(
        "\"samples\":{count},\"sample_shading\":{shading},\"alpha_to_coverage\":{coverage},\"alpha_to_one\":{one},\"sample_mask_default_all\":{mask}"
    )
}
fn blend_fields(blend: &vk::PipelineColorBlendStateCreateInfo<'_>) -> String {
    // The current user pipeline has one attachment. Null+zero never reaches from_raw_parts.
    if blend.attachment_count == 0 {
        return "\"color_blends\":[]".to_owned();
    }
    let attachments =
        // SAFETY: the pointer and length describe a live allocation or Vulkan output array of this scope, and the view does not outlive it.
        unsafe { std::slice::from_raw_parts(blend.p_attachments, blend.attachment_count as usize) };
    let rows: Vec<String> = attachments
        .iter()
        .map(|attachment| {
            format!(
                "[{},{}]",
                attachment.blend_enable,
                attachment.color_write_mask.as_raw()
            )
        })
        .collect();
    format!("\"color_blends\":[{}]", rows.join(","))
}
fn vertex_fields(info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    // The user pipeline's nonempty vertex description outlives the synchronous snapshot.
    // SAFETY: the pointers and counts come from one live vertex input description of this scope.
    let vertex = unsafe { &*info.p_vertex_input_state };
    let bindings = binding_fields(vertex);
    let attributes = attribute_fields(vertex);
    format!("\"vertex_bindings\":{bindings},\"vertex_attributes\":{attributes}")
}
fn binding_fields(vertex: &vk::PipelineVertexInputStateCreateInfo<'_>) -> String {
    // Caller retains the actual nonempty binding slice through pipeline creation.
    // SAFETY: the pointer and length describe a live allocation or Vulkan output array of this scope, and the view does not outlive it.
    let bindings = unsafe {
        std::slice::from_raw_parts(
            vertex.p_vertex_binding_descriptions,
            vertex.vertex_binding_description_count as usize,
        )
    };
    let rows: Vec<[u32; 3]> = bindings
        .iter()
        .map(|item| [item.binding, item.stride, item.input_rate.as_raw() as u32])
        .collect();
    format!("{rows:?}")
}
fn attribute_fields(vertex: &vk::PipelineVertexInputStateCreateInfo<'_>) -> String {
    // Caller retains the actual nonempty attribute slice through pipeline creation.
    // SAFETY: the pointer and length describe a live allocation or Vulkan output array of this scope, and the view does not outlive it.
    let attributes = unsafe {
        std::slice::from_raw_parts(
            vertex.p_vertex_attribute_descriptions,
            vertex.vertex_attribute_description_count as usize,
        )
    };
    let rows: Vec<[u32; 4]> = attributes.iter().map(attribute_row).collect();
    format!("{rows:?}")
}
fn attribute_row(item: &vk::VertexInputAttributeDescription) -> [u32; 4] {
    let format = item.format.as_raw() as u32;
    let offsets = [item.binding, item.offset];
    [item.location, offsets[0], format, offsets[1]]
}
fn raster_fields(runtime: &VulkanRuntime, info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    // Authored fields are live inputs. Effective facing is explicitly derived from the same
    // immutable runtime consumed by raster_pipeline_policy, not called a driver query.
    // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
    let raster = unsafe { &*info.p_rasterization_state };
    let polygon = raster.polygon_mode.as_raw();
    let clamp = raster.depth_clamp_enable;
    let discard = raster.rasterizer_discard_enable;
    let width = raster.line_width;
    let front = raster.front_face.as_raw();
    let policy = runtime
        .attachment_coordinates
        .pipeline_front_face(raster.front_face)
        .as_raw();
    format!(
        "\"polygon_mode\":{polygon},\"depth_clamp\":{clamp},\"raster_discard\":{discard},\"line_width\":{width},\"authored_front_face\":{front},\"policy_front_face\":{policy},\"policy_derivation\":\"raster_pipeline_policy.rs\""
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn attachment_snapshot_keeps_actual_write_masks_and_blend_enable() {
        let rows = [vk::PipelineColorBlendAttachmentState::default()
            .blend_enable(true)
            .color_write_mask(vk::ColorComponentFlags::from_raw(7))];
        let blend = vk::PipelineColorBlendStateCreateInfo::default().attachments(&rows);
        assert_eq!(blend_fields(&blend), "\"color_blends\":[[1,7]]");
        assert_eq!(blend_fields(&Default::default()), "\"color_blends\":[]");
    }
    #[test]
    fn fixed_snapshot_reads_changed_sample_and_depth_state() {
        let state = PipelineState::new(
            super::super::super::pipeline_variants::PipelineVariant::from_index(0, false),
        );
        let info = state.description(vk::PipelineLayout::null(), &[]);
        let captured = fixed_fields(&info);
        assert!(captured.contains("\"samples\":1"));
        assert!(captured.contains("\"depth_compare\":3"));
        assert!(captured.contains("\"sample_mask_default_all\":true"));
    }
    #[test]
    fn vertex_input_snapshot_retains_binding_stride_format_and_offset() {
        let state = PipelineState::new(
            super::super::super::pipeline_variants::PipelineVariant::from_index(0, false),
        );
        let info = state.description(vk::PipelineLayout::null(), &[]);
        assert!(vertex_fields(&info).contains("[0, 32, 0]"));
        assert!(vertex_fields(&info).contains("[1, 0, 109, 16]"));
    }
}
