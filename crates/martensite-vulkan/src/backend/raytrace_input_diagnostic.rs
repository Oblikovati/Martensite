//! Capture bytes and handles at the actual upload, creation and command-recording seams.
use super::{
    frame::FrameResources,
    raytrace_pipeline::{RaytracePipeline, RaytraceProgram},
    raytrace_resources::RaytraceResources,
};
use crate::raytrace_input_observation::{
    RayCommandInputs, RayDispatchInputs, RayInputPhase, RayPipelineInputs, RayUploadInputs,
};
use ash::{vk, vk::Handle};
pub(super) fn pipeline(
    location: (vk::Pipeline, vk::PipelineLayout, vk::DescriptorSetLayout),
    modules: &[vk::ShaderModule],
    program: RaytraceProgram,
    constants: (&[u8], &[vk::SpecializationMapEntry]),
    sources: (&[u8], &[u8]),
) -> RayPipelineInputs {
    RayPipelineInputs {
        pipeline: location.0.as_raw(),
        layout: location.1.as_raw(),
        modules: modules.iter().map(|module| module.as_raw()).collect(),
        scene_layout: location.2.as_raw(),
        specialization_entries: specialization_entries(constants.1),
        program: format!("{program:?}"),
        specialization: constants.0.to_vec(),
        vertex: sources.0.to_vec(),
        fragment: sources.1.to_vec(),
    }
}
fn specialization_entries(entries: &[vk::SpecializationMapEntry]) -> Vec<[u64; 3]> {
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

pub(super) fn uploaded(
    mut resources: RaytraceResources,
    frame: &crate::RenderFrame,
) -> RaytraceResources {
    let Some(token) = &frame.raytrace_observation else {
        return resources;
    };
    #[cfg(feature = "raytrace-hit-diagnostics")]
    super::raytrace_hit_scene::uploaded(&resources, token);
    token.uploaded(RayUploadInputs {
        buffer: resources.uniform_handle.as_raw(),
        set: resources.set.as_raw(),
        layout: resources.uniform_layout.as_raw(),
        bytes: resources.uniform_bytes.clone(),
    });
    resources.observation = Some(token.clone());
    resources
}
pub(super) fn frame(
    command: vk::CommandBuffer,
    resources: &FrameResources,
    frame: &crate::RenderFrame,
) {
    let Some(token) = &frame.raytrace_observation else {
        return;
    };
    token.command(RayCommandInputs {
        command: command.as_raw(),
        logical: [resources.extent.width, resources.extent.height],
        attachment: [
            resources.attachment_extent.width,
            resources.attachment_extent.height,
        ],
    });
    eprintln!("{}", token.marker(RayInputPhase::Frame, command.as_raw()));
}
pub(super) fn dispatched(
    command: vk::CommandBuffer,
    pipeline: &RaytracePipeline,
    sets: &[vk::DescriptorSet],
    resources: &RaytraceResources,
) {
    let Some(token) = &resources.observation else {
        return;
    };
    let Some(pipeline) = &pipeline.input_observation else {
        token.refuse("missing actual created pipeline inputs");
        return;
    };
    token.dispatched(dispatch_inputs(command, sets, pipeline));
    eprintln!(
        "{}",
        token.marker(RayInputPhase::Dispatch, command.as_raw())
    );
}

fn dispatch_inputs(
    command: vk::CommandBuffer,
    sets: &[vk::DescriptorSet],
    pipeline: &RayPipelineInputs,
) -> RayDispatchInputs {
    RayDispatchInputs {
        command: command.as_raw(),
        sets: sets.iter().map(|set| set.as_raw()).collect(),
        pipeline: pipeline.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn observed_pipeline_fixture() -> RayPipelineInputs {
        let location = (
            vk::Pipeline::from_raw(31),
            vk::PipelineLayout::from_raw(37),
            vk::DescriptorSetLayout::from_raw(41),
        );
        let modules = [
            vk::ShaderModule::from_raw(43),
            vk::ShaderModule::from_raw(47),
        ];
        pipeline(
            location,
            &modules,
            RaytraceProgram::Render,
            (&[11, 22], &[observed_entry_fixture()]),
            (&[33, 44], &[55, 66]),
        )
    }
    fn observed_entry_fixture() -> vk::SpecializationMapEntry {
        vk::SpecializationMapEntry::default()
            .constant_id(7)
            .offset(12)
            .size(4)
    }
    #[test]
    fn pipeline_metadata_retains_actual_maps_handles_and_input_bytes() {
        let observed = observed_pipeline_fixture();
        assert_eq!(observed.specialization_entries, [[7, 12, 4]]);
        assert_eq!(observed.modules, [43, 47]);
        assert_eq!(
            (observed.pipeline, observed.layout, observed.scene_layout),
            (31, 37, 41)
        );
        assert_eq!(observed.specialization, [11, 22]);
        assert_eq!(observed.vertex, [33, 44]);
        assert_eq!(observed.fragment, [55, 66]);
    }
    #[test]
    fn dispatch_metadata_retains_actual_command_and_ordered_bound_sets() {
        let command = vk::CommandBuffer::from_raw(29);
        let sets = [
            vk::DescriptorSet::from_raw(31),
            vk::DescriptorSet::from_raw(37),
        ];
        let observed = dispatch_inputs(command, &sets, &observed_pipeline_fixture());
        assert_eq!(observed.command, 29);
        assert_eq!(observed.sets, [31, 37]);
        assert_eq!(observed.pipeline.pipeline, 31);
    }
}
