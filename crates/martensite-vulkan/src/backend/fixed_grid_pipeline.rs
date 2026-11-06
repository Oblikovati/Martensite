//! Instance binding and push extensions preserve every original attribute offset.
use ash::vk;
const INSTANCE_BINDINGS: [vk::VertexInputBindingDescription; 6] = [
    binding(32),
    binding(44),
    binding(48),
    binding(60),
    binding(88),
    binding(104),
];
const fn binding(stride: u32) -> vk::VertexInputBindingDescription {
    vk::VertexInputBindingDescription {
        binding: 0,
        stride,
        input_rate: vk::VertexInputRate::INSTANCE,
    }
}
pub(super) fn fixed_grid_instance_input(
    mut input: vk::PipelineVertexInputStateCreateInfo<'static>,
) -> vk::PipelineVertexInputStateCreateInfo<'static> {
    // SAFETY: the pointer was set by this scope's live Vulkan structure chain, and the structure type names what it points at.
    let original = unsafe { &*input.p_vertex_binding_descriptions };
    let selected = INSTANCE_BINDINGS
        .iter()
        .find(|binding| binding.stride == original.stride)
        .expect("the admitted built-in point vertex stride");
    input.p_vertex_binding_descriptions = selected;
    input
}
pub(super) fn fixed_grid_push_range(
    mut range: vk::PushConstantRange,
    selected: bool,
) -> vk::PushConstantRange {
    if selected {
        range.size = 128;
    }
    range
}

pub(super) fn create_fixed_grid_pipeline(
    runtime: &super::runtime::VulkanRuntime,
    info: vk::GraphicsPipelineCreateInfo<'_>,
    raster: &vk::PipelineRasterizationStateCreateInfo<'_>,
    clamp: bool,
) -> Result<vk::Pipeline, crate::RenderError> {
    let values = [u32::from(!runtime.depth.clip_control), u32::from(clamp)];
    let bytes: Vec<u8> = values.into_iter().flat_map(u32::to_ne_bytes).collect();
    let entries = [specialization_entry(7, 0), specialization_entry(8, 4)];
    let constants = vk::SpecializationInfo::default()
        .map_entries(&entries)
        .data(&bytes);
    // SAFETY: the pointer and length describe a live allocation or Vulkan output array of this scope, and the view does not outlive it.
    let original = unsafe { std::slice::from_raw_parts(info.p_stages, info.stage_count as usize) };
    let stages: Vec<_> = original
        .iter()
        .map(|stage| {
            if stage.stage == vk::ShaderStageFlags::VERTEX {
                stage.specialization_info(&constants)
            } else {
                *stage
            }
        })
        .collect();
    super::raster_pipeline_policy::create_pipeline(runtime, info.stages(&stages), raster)
}

const fn specialization_entry(id: u32, offset: u32) -> vk::SpecializationMapEntry {
    vk::SpecializationMapEntry {
        constant_id: id,
        offset,
        size: 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_grid_instance_bindings_preserve_all_builtin_strides() {
        for stride in [32, 44, 48, 60, 88, 104] {
            let binding = [vk::VertexInputBindingDescription {
                binding: 0,
                stride,
                input_rate: vk::VertexInputRate::VERTEX,
            }];
            let input = vk::PipelineVertexInputStateCreateInfo::default()
                .vertex_binding_descriptions(&binding);
            // The input description is consumed synchronously; only copied attribute metadata survives.
            let owned = vk::PipelineVertexInputStateCreateInfo {
                vertex_binding_description_count: input.vertex_binding_description_count,
                p_vertex_binding_descriptions: input.p_vertex_binding_descriptions,
                ..Default::default()
            };
            let selected = fixed_grid_instance_input(owned);
            assert_eq!(
                // SAFETY: the pointer comes from a live Vulkan structure of this scope, and Vulkan's structure rules make it valid to read here.
                unsafe { (*selected.p_vertex_binding_descriptions).input_rate },
                vk::VertexInputRate::INSTANCE
            );
            assert_eq!(
                // SAFETY: the pointer comes from a live Vulkan structure of this scope, and Vulkan's structure rules make it valid to read here.
                unsafe { (*selected.p_vertex_binding_descriptions).stride },
                stride
            );
        }
    }
    #[test]
    fn fixed_grid_push_and_specialization_do_not_alias_existing_fields() {
        let original = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
            offset: 0,
            size: 84,
        };
        assert_eq!(fixed_grid_push_range(original, false).size, 84);
        assert_eq!(fixed_grid_push_range(original, true).size, 128);
        assert_eq!(
            fixed_grid_push_range(original, true).stage_flags,
            original.stage_flags
        );
        assert_eq!(specialization_entry(7, 0).constant_id, 7);
        assert_eq!(specialization_entry(8, 4).offset, 4);
    }
}
