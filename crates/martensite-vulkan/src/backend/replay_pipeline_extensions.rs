//! Only recognized live extension payloads are interpreted; unknown types remain explicit.
use ash::vk;
use serde_json::{Value, json};

pub(super) fn types(pointer: *const std::ffi::c_void) -> Vec<i32> {
    nodes(pointer)
        .iter()
        .map(|row| unsafe { (**row).s_type.as_raw() })
        .collect()
}
fn nodes(pointer: *const std::ffi::c_void) -> Vec<*const vk::BaseInStructure<'static>> {
    let mut result = Vec::new();
    let mut current = pointer.cast::<vk::BaseInStructure<'static>>();
    // SAFETY: caller supplies the live Vulkan create-info chain consumed synchronously.
    while let Some(row) = unsafe { current.as_ref() } {
        result.push(current);
        current = row.p_next;
    }
    result
}
pub(super) fn rendering(pointer: *const std::ffi::c_void) -> Vec<Value> {
    nodes(pointer).into_iter().map(rendering_node).collect()
}
fn rendering_node(pointer: *const vk::BaseInStructure<'static>) -> Value {
    let kind = unsafe { (*pointer).s_type };
    if kind != vk::StructureType::PIPELINE_RENDERING_CREATE_INFO {
        return json!({"unsupported_type":kind.as_raw()});
    }
    let row = unsafe { &*pointer.cast::<vk::PipelineRenderingCreateInfo<'_>>() };
    let colors = if row.color_attachment_count == 0 {
        &[][..]
    } else {
        unsafe {
            std::slice::from_raw_parts(
                row.p_color_attachment_formats,
                row.color_attachment_count as usize,
            )
        }
    };
    json!({"type":kind.as_raw(),"view_mask":row.view_mask,"colors":colors.iter().map(|format|format.as_raw()).collect::<Vec<_>>(),
        "depth":row.depth_attachment_format.as_raw(),"stencil":row.stencil_attachment_format.as_raw()})
}
pub(super) fn raster(pointer: *const std::ffi::c_void) -> Vec<Value> {
    nodes(pointer).into_iter().map(raster_node).collect()
}
fn raster_node(pointer: *const vk::BaseInStructure<'static>) -> Value {
    let kind = unsafe { (*pointer).s_type };
    match kind {
        vk::StructureType::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT => {
            let row = unsafe {
                &*pointer.cast::<vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT<'_>>()
            };
            json!({"type":kind.as_raw(),"provoking_mode":row.provoking_vertex_mode.as_raw()})
        }
        vk::StructureType::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT => {
            let row = unsafe {
                &*pointer.cast::<vk::PipelineRasterizationDepthClipStateCreateInfoEXT<'_>>()
            };
            json!({"type":kind.as_raw(),"depth_clip_enable":row.depth_clip_enable,"flags":row.flags.as_raw()})
        }
        vk::StructureType::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR => line_state(pointer),
        _ => json!({"unsupported_type":kind.as_raw()}),
    }
}

fn line_state(pointer: *const vk::BaseInStructure<'static>) -> Value {
    let row = unsafe { &*pointer.cast::<vk::PipelineRasterizationLineStateCreateInfoKHR<'_>>() };
    json!({"type":row.s_type.as_raw(),"mode":row.line_rasterization_mode.as_raw(),
        "stipple_enabled":row.stippled_line_enable,"stipple_factor":row.line_stipple_factor,"stipple_pattern":row.line_stipple_pattern})
}

pub(in crate::backend) fn image(pointer: *const std::ffi::c_void) -> Vec<Value> {
    nodes(pointer).into_iter().map(image_node).collect()
}
fn image_node(pointer: *const vk::BaseInStructure<'static>) -> Value {
    let kind = unsafe { (*pointer).s_type };
    if kind != vk::StructureType::IMAGE_FORMAT_LIST_CREATE_INFO {
        return json!({"unsupported_type":kind.as_raw()});
    }
    let row = unsafe { &*pointer.cast::<vk::ImageFormatListCreateInfo<'_>>() };
    let formats = if row.view_format_count == 0 {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(row.p_view_formats, row.view_format_count as usize) }
    };
    json!({"type":kind.as_raw(),"formats":formats.iter().map(|format|format.as_raw()).collect::<Vec<_>>()})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_rendering_formats_and_view_mask_are_bound() {
        let formats = [vk::Format::R8G8B8A8_SRGB];
        let mut rendering = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&formats)
            .depth_attachment_format(vk::Format::D32_SFLOAT)
            .view_mask(2);
        let info = vk::GraphicsPipelineCreateInfo::default().push_next(&mut rendering);
        let rows = super::rendering(info.p_next);
        assert_eq!(rows[0]["colors"], json!([43]));
        assert_eq!(rows[0]["depth"], 126);
        assert_eq!(rows[0]["view_mask"], 2);
    }
    #[test]
    fn actual_provoking_mode_is_not_inferred_from_extension_type() {
        let mut provoking = vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT::default()
            .provoking_vertex_mode(vk::ProvokingVertexModeEXT::LAST_VERTEX);
        let raster = vk::PipelineRasterizationStateCreateInfo::default().push_next(&mut provoking);
        assert_eq!(super::raster(raster.p_next)[0]["provoking_mode"], 1);
    }
    #[test]
    fn line_state_payload_remains_observed_even_for_filled_triangles() {
        let mut line = vk::PipelineRasterizationLineStateCreateInfoKHR::default()
            .line_rasterization_mode(vk::LineRasterizationModeKHR::BRESENHAM)
            .stippled_line_enable(true)
            .line_stipple_factor(3)
            .line_stipple_pattern(0x55aa);
        let raster = vk::PipelineRasterizationStateCreateInfo::default().push_next(&mut line);
        let value = super::raster(raster.p_next);
        assert_eq!(value[0]["mode"], 2);
        assert_eq!(value[0]["stipple_enabled"], 1);
        assert_eq!(value[0]["stipple_pattern"], 0x55aa);
    }
    #[test]
    fn unknown_extension_remains_explicit_for_consumer_refusal() {
        let unknown = vk::BaseInStructure {
            s_type: vk::StructureType::APPLICATION_INFO,
            ..Default::default()
        };
        let pointer = std::ptr::from_ref(&unknown).cast();
        assert_eq!(super::rendering(pointer)[0], json!({"unsupported_type":0}));
    }
}
