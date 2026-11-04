//! Additional actual fixed pipeline operands used by generic replay consumers.
use ash::vk;

pub(super) fn fixed_json(info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    format!("{},{}", vertex_json(info), blend_json(info))
}
fn vertex_json(info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    let Some(vertex) = (unsafe { info.p_vertex_input_state.as_ref() }) else {
        return "\"vertex_input\":null".to_owned();
    };
    let bindings = binding_rows(vertex);
    let attributes = attribute_rows(vertex);
    format!(
        "\"vertex_input\":{{\"binding_stride_rate\":{bindings:?},\"location_binding_format_offset\":{attributes:?}}}"
    )
}
fn binding_rows(vertex: &vk::PipelineVertexInputStateCreateInfo<'_>) -> Vec<[u32; 3]> {
    if vertex.vertex_binding_description_count == 0 {
        return Vec::new();
    }
    // SAFETY: final creation info's caller retains its successful create-call arrays.
    let rows = unsafe {
        std::slice::from_raw_parts(
            vertex.p_vertex_binding_descriptions,
            vertex.vertex_binding_description_count as usize,
        )
    };
    rows.iter()
        .map(|row| [row.binding, row.stride, row.input_rate.as_raw() as u32])
        .collect()
}
fn attribute_rows(vertex: &vk::PipelineVertexInputStateCreateInfo<'_>) -> Vec<[u32; 4]> {
    if vertex.vertex_attribute_description_count == 0 {
        return Vec::new();
    }
    // SAFETY: same live creation-info ownership as binding_rows.
    let rows = unsafe {
        std::slice::from_raw_parts(
            vertex.p_vertex_attribute_descriptions,
            vertex.vertex_attribute_description_count as usize,
        )
    };
    rows.iter()
        .map(|row| {
            [
                row.location,
                row.binding,
                row.format.as_raw() as u32,
                row.offset,
            ]
        })
        .collect()
}
fn blend_json(info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    let Some(blend) = (unsafe { info.p_color_blend_state.as_ref() }) else {
        return "\"color_blend\":null".to_owned();
    };
    let rows = blend_rows(blend);
    format!(
        "\"color_blend\":{{\"logic_enable\":{},\"logic_op\":{},\"attachments\":{rows:?},\"constant_bits\":{:?}}}",
        blend.logic_op_enable,
        blend.logic_op.as_raw(),
        blend.blend_constants.map(f32::to_bits)
    )
}
fn blend_rows(blend: &vk::PipelineColorBlendStateCreateInfo<'_>) -> Vec<[u32; 8]> {
    if blend.attachment_count == 0 {
        return Vec::new();
    }
    // SAFETY: same live successful creation-info ownership, including zero-count null handling.
    let rows =
        unsafe { std::slice::from_raw_parts(blend.p_attachments, blend.attachment_count as usize) };
    rows.iter().map(blend_words).collect()
}
fn blend_words(row: &vk::PipelineColorBlendAttachmentState) -> [u32; 8] {
    [
        row.blend_enable,
        row.src_color_blend_factor.as_raw() as u32,
        row.dst_color_blend_factor.as_raw() as u32,
        row.color_blend_op.as_raw() as u32,
        row.src_alpha_blend_factor.as_raw() as u32,
        row.dst_alpha_blend_factor.as_raw() as u32,
        row.alpha_blend_op.as_raw() as u32,
        row.color_write_mask.as_raw(),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn absent_fixed_descriptions_remain_unknown_not_defaults() {
        assert_eq!(
            fixed_json(&vk::GraphicsPipelineCreateInfo::default()),
            "\"vertex_input\":null,\"color_blend\":null"
        );
    }
    #[test]
    fn vertex_stride_and_attribute_offsets_are_actual_api_values() {
        let bindings = [vk::VertexInputBindingDescription::default().stride(44)];
        let attributes = [vk::VertexInputAttributeDescription::default()
            .location(2)
            .offset(32)
            .format(vk::Format::R32G32B32_SFLOAT)];
        let vertex = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);
        assert_eq!(binding_rows(&vertex), [[0, 44, 0]]);
        assert_eq!(attribute_rows(&vertex), [[2, 0, 106, 32]]);
    }
    #[test]
    fn alpha_blend_masks_are_not_inferred_from_shader_name() {
        let row = vk::PipelineColorBlendAttachmentState::default()
            .blend_enable(true)
            .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .color_write_mask(vk::ColorComponentFlags::from_raw(7));
        let words = blend_words(&row);
        assert_eq!(words[0], 1);
        assert_eq!(words[1], vk::BlendFactor::SRC_ALPHA.as_raw() as u32);
        assert_eq!(words[7], 7);
    }
}
