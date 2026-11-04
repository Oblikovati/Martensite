//! Serialize only final live pipeline descriptions, before their owners leave scope.
use ash::{vk, vk::Handle};

pub(super) fn pipeline_json(info: &vk::GraphicsPipelineCreateInfo<'_>, pipeline: u64) -> String {
    format!(
        "{{\"kind\":\"vertex-output-pipeline-created\",\"pipeline\":{pipeline},\"stages\":{:?},{},{},\"dynamic_states\":{:?},\"specializations\":[{}],\"viewport_clip\":{},{} }}",
        stage_identity(info),
        raster_json(info),
        assembly_json(info),
        dynamic_states(info),
        stage_specializations(info),
        viewport_clip(info),
        super::super::replay_pipeline_diagnostic::fixed_json(info),
    )
}

fn raster_json(info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    // All linked creation descriptions remain alive through the synchronous create call.
    let raster = unsafe { info.p_rasterization_state.as_ref() }.unwrap();
    format!(
        "\"polygon_mode\":{},\"cull_mode\":{},\"front_face\":{},\"rasterizer_discard\":{},\"depth_clamp\":{},\"raster_extensions\":{:?}",
        raster.polygon_mode.as_raw(),
        raster.cull_mode.as_raw(),
        raster.front_face.as_raw(),
        raster.rasterizer_discard_enable,
        raster.depth_clamp_enable,
        extension_types(raster.p_next),
    )
}

fn assembly_json(info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    let assembly = unsafe { info.p_input_assembly_state.as_ref() }.unwrap();
    let samples = unsafe { info.p_multisample_state.as_ref() }.unwrap();
    format!(
        "\"topology\":{},\"primitive_restart\":{},\"samples\":{}",
        assembly.topology.as_raw(),
        assembly.primitive_restart_enable,
        samples.rasterization_samples.as_raw()
    )
}

fn stage_identity(info: &vk::GraphicsPipelineCreateInfo<'_>) -> Vec<[u64; 2]> {
    // Vulkan already consumed this exact live array; no stage list is reconstructed from policy.
    let stages = unsafe { std::slice::from_raw_parts(info.p_stages, info.stage_count as usize) };
    stages
        .iter()
        .map(|stage| [u64::from(stage.stage.as_raw()), stage.module.as_raw()])
        .collect()
}

fn stage_specializations(info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    // Same live stage array as the synchronous Vulkan call above.
    let stages = unsafe { std::slice::from_raw_parts(info.p_stages, info.stage_count as usize) };
    stages
        .iter()
        .map(|stage| specialization_json(stage.p_specialization_info))
        .collect::<Vec<_>>()
        .join(",")
}

fn specialization_json(pointer: *const vk::SpecializationInfo<'_>) -> String {
    let Some(spec) = (unsafe { pointer.as_ref() }) else {
        return "null".to_owned();
    };
    let maps = specialization_entries(spec);
    let bytes = specialization_bytes(spec);
    format!("{{\"map\":{maps:?},\"bytes\":{bytes:?}}}")
}

fn specialization_entries(spec: &vk::SpecializationInfo<'_>) -> Vec<[u64; 3]> {
    if spec.map_entry_count == 0 {
        return Vec::new();
    }
    let entries =
        unsafe { std::slice::from_raw_parts(spec.p_map_entries, spec.map_entry_count as usize) };
    entries
        .iter()
        .map(|entry| {
            [
                u64::from(entry.constant_id),
                u64::from(entry.offset),
                entry.size as u64,
            ]
        })
        .collect()
}

fn specialization_bytes<'a>(spec: &'a vk::SpecializationInfo<'_>) -> &'a [u8] {
    if spec.data_size == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(spec.p_data.cast::<u8>(), spec.data_size) }
}

fn viewport_clip(info: &vk::GraphicsPipelineCreateInfo<'_>) -> String {
    let Some(viewport) = (unsafe { info.p_viewport_state.as_ref() }) else {
        return "null".to_owned();
    };
    let mut current = viewport.p_next.cast::<vk::BaseInStructure<'_>>();
    while let Some(extension) = unsafe { current.as_ref() } {
        if extension.s_type
            == vk::StructureType::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT
        {
            let clip = unsafe {
                &*current.cast::<vk::PipelineViewportDepthClipControlCreateInfoEXT<'_>>()
            };
            return format!(
                "{{\"negative_one_to_one\":{}}}",
                clip.negative_one_to_one != 0
            );
        }
        current = extension.p_next;
    }
    "{\"negative_one_to_one\":false}".to_owned()
}

fn dynamic_states(info: &vk::GraphicsPipelineCreateInfo<'_>) -> Vec<i32> {
    let Some(state) = (unsafe { info.p_dynamic_state.as_ref() }) else {
        return Vec::new();
    };
    if state.dynamic_state_count == 0 {
        return Vec::new();
    }
    let states = unsafe {
        std::slice::from_raw_parts(state.p_dynamic_states, state.dynamic_state_count as usize)
    };
    states.iter().map(|state| state.as_raw()).collect()
}

fn extension_types(pointer: *const std::ffi::c_void) -> Vec<i32> {
    let mut current = pointer.cast::<vk::BaseInStructure<'_>>();
    let mut types = Vec::new();
    while let Some(extension) = unsafe { current.as_ref() } {
        types.push(extension.s_type.as_raw());
        current = extension.p_next;
    }
    types
}

pub(super) fn facing_json(command: u64, culling: Option<u32>, front: i32) -> String {
    let culling = culling.map_or_else(|| "null".to_owned(), |mode| mode.to_string());
    format!(
        "{{\"kind\":\"vertex-output-facing-command\",\"command\":{command},\"cull_mode\":{culling},\"front_face\":{front}}}"
    )
}

pub(super) fn depth_json(command: u64, test: bool, write: bool, compare: i32) -> String {
    format!(
        "{{\"kind\":\"vertex-output-depth-command\",\"command\":{command},\"test\":{test},\"write\":{write},\"compare\":{compare}}}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn command_receipts_distinguish_unchanged_cull_mode_and_record_depth_comparison() {
        assert_eq!(
            facing_json(5, Some(2), 1),
            "{\"kind\":\"vertex-output-facing-command\",\"command\":5,\"cull_mode\":2,\"front_face\":1}"
        );
        assert!(facing_json(5, None, 0).contains("\"cull_mode\":null"));
        assert_eq!(
            depth_json(5, true, false, 3),
            "{\"kind\":\"vertex-output-depth-command\",\"command\":5,\"test\":true,\"write\":false,\"compare\":3}"
        );
    }
    #[test]
    fn final_specialization_maps_and_bytes_are_recorded_without_policy_inference() {
        let entries = [vk::SpecializationMapEntry {
            constant_id: 7,
            offset: 0,
            size: 4,
        }];
        let bytes = 0_u32.to_ne_bytes();
        let specialization = vk::SpecializationInfo::default()
            .map_entries(&entries)
            .data(&bytes);
        let stages =
            [vk::PipelineShaderStageCreateInfo::default().specialization_info(&specialization)];
        let info = vk::GraphicsPipelineCreateInfo::default().stages(&stages);
        assert_eq!(
            stage_specializations(&info),
            "{\"map\":[[7, 0, 4]],\"bytes\":[0, 0, 0, 0]}"
        );
        assert_eq!(specialization_json(std::ptr::null()), "null");
    }
    #[test]
    fn depth_clip_record_reads_live_viewport_extension() {
        let mut clip =
            vk::PipelineViewportDepthClipControlCreateInfoEXT::default().negative_one_to_one(true);
        let viewport = vk::PipelineViewportStateCreateInfo::default().push_next(&mut clip);
        let info = vk::GraphicsPipelineCreateInfo::default().viewport_state(&viewport);
        assert_eq!(viewport_clip(&info), "{\"negative_one_to_one\":true}");
        assert_eq!(
            viewport_clip(&vk::GraphicsPipelineCreateInfo::default()),
            "null"
        );
    }
    #[test]
    fn dynamic_states_record_only_the_live_declared_array() {
        let states = [vk::DynamicState::VIEWPORT];
        let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&states);
        let info = vk::GraphicsPipelineCreateInfo::default().dynamic_state(&dynamic);
        assert_eq!(
            dynamic_states(&info),
            vec![vk::DynamicState::VIEWPORT.as_raw()]
        );
        assert!(dynamic_states(&vk::GraphicsPipelineCreateInfo::default()).is_empty());
    }
    #[test]
    fn raster_receipt_reads_the_final_extension_chain() {
        let mut provoking = vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT::default();
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .push_next(&mut provoking);
        let info = vk::GraphicsPipelineCreateInfo::default().rasterization_state(&raster);
        let receipt = raster_json(&info);
        assert!(receipt.contains(&format!(
                "\"raster_extensions\":[{}]",
                vk::StructureType::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT
                    .as_raw()
            )));
        assert!(receipt.contains("\"rasterizer_discard\":0"));
    }
    #[test]
    fn pipeline_receipt_joins_the_actual_pipeline_and_stage_handles() {
        let raster = vk::PipelineRasterizationStateCreateInfo::default();
        let assembly = vk::PipelineInputAssemblyStateCreateInfo::default();
        let samples = vk::PipelineMultisampleStateCreateInfo::default();
        let info = vk::GraphicsPipelineCreateInfo::default()
            .rasterization_state(&raster)
            .input_assembly_state(&assembly)
            .multisample_state(&samples);
        assert_identity_receipt(info);
    }
    fn assert_identity_receipt(info: vk::GraphicsPipelineCreateInfo<'_>) {
        let stages = [vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vk::ShaderModule::from_raw(12))];
        let receipt = pipeline_json(&info.stages(&stages), 37);
        assert!(receipt.contains("\"pipeline\":37"));
        assert!(receipt.contains("\"stages\":[[1, 12]]"));
        assert!(receipt.contains("\"dynamic_states\":[]"));
    }
}
