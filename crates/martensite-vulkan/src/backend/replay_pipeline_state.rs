//! Final fixed-function operands supplement the existing stage/vertex/blend receipt.
use ash::{vk, vk::Handle};
use serde_json::{Value, json};
#[path = "replay_pipeline_extensions.rs"]
pub(super) mod extensions;

pub(super) fn created(
    device: vk::Device,
    info: &vk::GraphicsPipelineCreateInfo<'_>,
    pipeline: vk::Pipeline,
) {
    eprintln!(
        "{}",
        json!({"kind":"pipeline-fixed-create-input", "device":device.as_raw(),
        "pipeline":pipeline.as_raw(), "layout":info.layout.as_raw(), "flags":info.flags.as_raw(),
        "stages":stages(info), "rendering":extensions::rendering(info.p_next),
        "raster":raster(info), "depth_stencil":depth(info), "multisample":samples(info),
        "viewport_counts":viewport_counts(info),"state_metadata":state_metadata(info)})
    );
}
fn state_metadata(info: &vk::GraphicsPipelineCreateInfo<'_>) -> Value {
    json!({
        "vertex_input":unsafe {info.p_vertex_input_state.as_ref()}.map(|row|metadata(row.flags.as_raw(),row.p_next)),
        "input_assembly":unsafe {info.p_input_assembly_state.as_ref()}.map(|row|metadata(row.flags.as_raw(),row.p_next)),
        "color_blend":unsafe {info.p_color_blend_state.as_ref()}.map(|row|metadata(row.flags.as_raw(),row.p_next)),
        "dynamic_state":unsafe {info.p_dynamic_state.as_ref()}.map(|row|metadata(row.flags.as_raw(),row.p_next)),
        "viewport_state":unsafe {info.p_viewport_state.as_ref()}.map(|row|metadata(row.flags.as_raw(),row.p_next))
    })
}
fn metadata(flags: u32, pointer: *const std::ffi::c_void) -> Value {
    json!({"flags":flags,"extensions":extensions::types(pointer)})
}
fn stages(info: &vk::GraphicsPipelineCreateInfo<'_>) -> Vec<Value> {
    // SAFETY: caller retains the stage array and entrypoint strings through successful creation.
    let rows = unsafe { std::slice::from_raw_parts(info.p_stages, info.stage_count as usize) };
    rows.iter().map(|row| json!({"stage":row.stage.as_raw(), "module":row.module.as_raw(),
        "flags":row.flags.as_raw(), "entrypoint_bytes":unsafe { std::ffi::CStr::from_ptr(row.p_name) }.to_bytes(),
        "extensions":extensions::types(row.p_next)})).collect()
}
fn viewport_counts(info: &vk::GraphicsPipelineCreateInfo<'_>) -> Value {
    let Some(row) = (unsafe { info.p_viewport_state.as_ref() }) else {
        return Value::Null;
    };
    json!({"viewports":row.viewport_count, "scissors":row.scissor_count})
}
fn raster(info: &vk::GraphicsPipelineCreateInfo<'_>) -> Value {
    let Some(row) = (unsafe { info.p_rasterization_state.as_ref() }) else {
        return Value::Null;
    };
    json!({"flags":row.flags.as_raw(), "depth_bias_enable":row.depth_bias_enable,
        "depth_bias_bits":[row.depth_bias_constant_factor.to_bits(),row.depth_bias_clamp.to_bits(),row.depth_bias_slope_factor.to_bits()],
        "line_width_bits":row.line_width.to_bits(), "extensions":extensions::raster(row.p_next)})
}
fn depth(info: &vk::GraphicsPipelineCreateInfo<'_>) -> Value {
    let Some(row) = (unsafe { info.p_depth_stencil_state.as_ref() }) else {
        return Value::Null;
    };
    json!({"flags":row.flags.as_raw(), "test":row.depth_test_enable,"write":row.depth_write_enable,
        "compare":row.depth_compare_op.as_raw(),"bounds_enable":row.depth_bounds_test_enable,
        "bounds_bits":[row.min_depth_bounds.to_bits(),row.max_depth_bounds.to_bits()],
        "stencil_enable":row.stencil_test_enable,"front":stencil(row.front),"back":stencil(row.back),
        "extensions":extensions::types(row.p_next)})
}
fn stencil(row: vk::StencilOpState) -> [u32; 7] {
    [
        row.fail_op.as_raw() as u32,
        row.pass_op.as_raw() as u32,
        row.depth_fail_op.as_raw() as u32,
        row.compare_op.as_raw() as u32,
        row.compare_mask,
        row.write_mask,
        row.reference,
    ]
}
fn samples(info: &vk::GraphicsPipelineCreateInfo<'_>) -> Value {
    let Some(row) = (unsafe { info.p_multisample_state.as_ref() }) else {
        return Value::Null;
    };
    json!({"flags":row.flags.as_raw(),"samples":row.rasterization_samples.as_raw(),
        "sample_shading":row.sample_shading_enable,"min_sample_shading_bits":row.min_sample_shading.to_bits(),
        "mask":sample_mask(row),"alpha_to_coverage":row.alpha_to_coverage_enable,"alpha_to_one":row.alpha_to_one_enable,
        "extensions":extensions::types(row.p_next)})
}
fn sample_mask(row: &vk::PipelineMultisampleStateCreateInfo<'_>) -> Option<Vec<u32>> {
    if row.p_sample_mask.is_null() {
        return None;
    }
    let count = row.rasterization_samples.as_raw().div_ceil(32) as usize;
    // SAFETY: VkSampleCountFlagBits describes the caller-owned sample mask array length.
    Some(unsafe { std::slice::from_raw_parts(row.p_sample_mask, count) }.to_vec())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn auxiliary_state_extensions_are_visible_and_absence_is_distinct() {
        let unknown = vk::BaseInStructure {
            s_type: vk::StructureType::APPLICATION_INFO,
            ..Default::default()
        };
        let viewport = vk::PipelineViewportStateCreateInfo {
            p_next: std::ptr::from_ref(&unknown).cast(),
            ..Default::default()
        };
        let vertex = vk::PipelineVertexInputStateCreateInfo::default();
        let info = vk::GraphicsPipelineCreateInfo::default()
            .viewport_state(&viewport)
            .vertex_input_state(&vertex);
        let value = state_metadata(&info);
        assert_eq!(value["viewport_state"]["extensions"], json!([0]));
        assert_eq!(value["vertex_input"]["extensions"], json!([]));
        assert_eq!(value["input_assembly"], Value::Null);
    }
    #[test]
    fn null_mask_and_explicit_mask_are_distinct() {
        let mask = [0x55];
        let base = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);
        assert_eq!(sample_mask(&base), None);
        assert_eq!(sample_mask(&base.sample_mask(&mask)), Some(vec![0x55]));
    }
    #[test]
    fn missing_depth_description_is_unknown_and_bounds_bits_are_exact() {
        assert_eq!(
            depth(&vk::GraphicsPipelineCreateInfo::default()),
            Value::Null
        );
        let state = vk::PipelineDepthStencilStateCreateInfo::default()
            .min_depth_bounds(-0.0)
            .max_depth_bounds(0.75);
        let value = depth(&vk::GraphicsPipelineCreateInfo::default().depth_stencil_state(&state));
        assert_eq!(
            value["bounds_bits"],
            json!([(-0.0f32).to_bits(), 0.75f32.to_bits()])
        );
    }
    #[test]
    fn stencil_order_keeps_failure_and_depth_failure_distinct() {
        let row = vk::StencilOpState::default()
            .fail_op(vk::StencilOp::REPLACE)
            .depth_fail_op(vk::StencilOp::INVERT)
            .reference(7);
        assert_eq!(stencil(row), [2, 0, 5, 0, 0, 0, 7]);
    }
}
