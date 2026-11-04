//! Bind the minimal subset request to the actual existing pipeline descriptions.
use super::super::{pipeline_state::PipelineState, pipeline_variants::PipelineVariant};
use ash::vk;
use std::collections::BTreeSet;

#[test]
fn actual_layouts_need_neither_triangle_fans_nor_attributes_beyond_stride() {
    let mut strides = BTreeSet::new();
    for index in 0..44 {
        for flags in 0..64 {
            inspect_variant(layout_variant(index, flags), &mut strides);
        }
    }
    assert_eq!(strides, BTreeSet::from([32, 40, 44, 48, 60, 88, 104]));
}

fn layout_variant(index: usize, flags: u32) -> PipelineVariant {
    let mut variant = PipelineVariant::from_index(index, false);
    variant.lighting = flags & 1 != 0;
    variant.fixed_lighting = flags & 2 != 0;
    variant.surface_texture = flags & 4 != 0;
    variant.surface_alpha = flags & 8 != 0;
    variant.mesh_edges = flags & 16 != 0;
    variant.fixed_line_strip = flags & 32 != 0;
    variant
}

fn inspect_variant(variant: PipelineVariant, strides: &mut BTreeSet<u32>) {
    let state = PipelineState::new(variant);
    let description = state.description(vk::PipelineLayout::null(), &[]);
    // PipelineState owns each pointed-to description; vertex arrays are static.
    // SAFETY: the pointer comes from a live Vulkan structure of this scope, and Vulkan's structure rules make it valid to read here.
    unsafe {
        assert_ne!(
            (*description.p_input_assembly_state).topology,
            vk::PrimitiveTopology::TRIANGLE_FAN
        );
        inspect_vertex_state(&*description.p_vertex_input_state, strides);
    }
}

fn inspect_vertex_state(
    vertex: &vk::PipelineVertexInputStateCreateInfo<'_>,
    strides: &mut BTreeSet<u32>,
) {
    // The caller borrows PipelineState and its static vertex tables throughout inspection.
    // SAFETY: the pointers and counts come from one live vertex input description of this scope.
    let bindings = unsafe { vertex_bindings(vertex) };
    // SAFETY: the pointers and counts come from one live vertex input description of this scope.
    let attributes = unsafe { vertex_attributes(vertex) };
    for binding in bindings {
        assert!(binding.stride.is_multiple_of(4));
        strides.insert(binding.stride);
    }
    for attribute in attributes {
        let binding = bindings
            .iter()
            .find(|entry| entry.binding == attribute.binding)
            .unwrap();
        assert!(attribute.offset + attribute_bytes(attribute.format) <= binding.stride);
    }
}

unsafe fn vertex_bindings<'a>(
    vertex: &'a vk::PipelineVertexInputStateCreateInfo<'_>,
) -> &'a [vk::VertexInputBindingDescription] {
    // The actual PipelineState producer supplies a live array of the reported length.
    // SAFETY: the pointer and length describe a live allocation or Vulkan output array of this scope, and the view does not outlive it.
    unsafe {
        std::slice::from_raw_parts(
            vertex.p_vertex_binding_descriptions,
            vertex.vertex_binding_description_count as usize,
        )
    }
}

unsafe fn vertex_attributes<'a>(
    vertex: &'a vk::PipelineVertexInputStateCreateInfo<'_>,
) -> &'a [vk::VertexInputAttributeDescription] {
    // The actual PipelineState producer supplies a live array of the reported length.
    // SAFETY: the pointer and length describe a live allocation or Vulkan output array of this scope, and the view does not outlive it.
    unsafe {
        std::slice::from_raw_parts(
            vertex.p_vertex_attribute_descriptions,
            vertex.vertex_attribute_description_count as usize,
        )
    }
}

fn attribute_bytes(format: vk::Format) -> u32 {
    match format {
        vk::Format::R32G32_SFLOAT => 8,
        vk::Format::R32G32B32_SFLOAT => 12,
        vk::Format::R32G32B32A32_SFLOAT => 16,
        unexpected => panic!("attribute format {unexpected:?}; expected audited float2/3/4 layout"),
    }
}
