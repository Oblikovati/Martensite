//! Descriptor slots and push ranges preserve native interface ordering across OIT variants.
use super::runtime::{VulkanRuntime, vk_error};
use crate::RenderError;
use ash::vk;

#[allow(clippy::too_many_arguments)] // Each argument is a distinct Vulkan or behaviour input.
pub(super) fn create_layout(
    runtime: &VulkanRuntime,
    push: vk::PushConstantRange,
    sprite: Option<vk::DescriptorSetLayout>,
    lighting: Option<&super::storage_descriptor::StorageDescriptorLayout>,
    interpolation: bool,
    triangle_grid_coverage: bool,
    native_grid_v4: bool,
    text_grid_interpolation: bool,
    window_depth_status: Option<&super::storage_descriptor::StorageDescriptorLayout>,
    clipping: Option<&super::storage_descriptor::StorageDescriptorLayout>,
    [sphere, oit]: [Option<vk::DescriptorSetLayout>; 2],
    [pbr, maps, shadow]: [Option<vk::DescriptorSetLayout>; 3],
) -> Result<vk::PipelineLayout, RenderError> {
    // Fragment constants force redundant lavapipe state work on every surface draw.
    let mut ranges = interpolation_push_ranges(
        push,
        interpolation,
        triangle_grid_coverage,
        native_grid_v4,
        text_grid_interpolation,
    );
    if window_depth_status.is_some() {
        ranges.push(super::window_depth_record::push_range());
    }
    let mut layouts = descriptor_layout_handles(
        sprite,
        lighting.map(|s| s.handle),
        clipping.map(|s| s.handle),
    );
    if let Some(layout) = sphere {
        append_at(&mut layouts, 2, layout);
    }
    if let Some(layout) = oit {
        while layouts.len() < 3 {
            layouts.push(layout);
        }
        layouts.push(layout);
    }
    if let Some(layout) = pbr {
        append_pbr_environment(&mut layouts, layout);
    }
    if let Some(layout) = maps {
        append_at(&mut layouts, 5, layout);
    }
    if let Some(layout) = shadow {
        append_at(&mut layouts, 6, layout);
    }
    if let Some(status) = window_depth_status {
        append_at(&mut layouts, 7, status.handle);
    }
    let info = vk::PipelineLayoutCreateInfo::default()
        .push_constant_ranges(&ranges)
        .set_layouts(&layouts);
    // SAFETY: the create information borrows live values of this scope and the device stays valid for the runtime's lifetime.
    let layout = unsafe { runtime.device().create_pipeline_layout(&info, None) }
        .map_err(|e| vk_error("create pipeline layout", e))?;
    #[cfg(feature = "command-state-diagnostics")]
    super::replay_resource_diagnostic::pipeline_layout(layout, &info);
    Ok(layout)
}

/// The PBR environment owns set 4; earlier unused slots repeat set 0's compatible layout.
fn append_pbr_environment(
    layouts: &mut Vec<vk::DescriptorSetLayout>,
    pbr: vk::DescriptorSetLayout,
) {
    append_at(layouts, 4, pbr);
}

/// Place `layout` at set `index`; earlier unused slots repeat set 0's compatible layout.
fn append_at(
    layouts: &mut Vec<vk::DescriptorSetLayout>,
    index: usize,
    layout: vk::DescriptorSetLayout,
) {
    let filler = layouts.first().copied().unwrap_or(layout);
    layouts.resize(index, filler);
    layouts.push(layout);
}

pub(super) fn fixed_marker_push_range(
    line: bool,
    edges: bool,
    fixed_marker: bool,
) -> vk::PushConstantRange {
    let mut range = push_range(line, edges);
    if fixed_marker {
        range.size = 84;
    }
    range
}

pub(super) fn push_range(line: bool, edges: bool) -> vk::PushConstantRange {
    let stages = if edges {
        vk::ShaderStageFlags::VERTEX
            | vk::ShaderStageFlags::GEOMETRY
            | vk::ShaderStageFlags::FRAGMENT
    } else if line {
        vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT
    } else {
        vk::ShaderStageFlags::VERTEX
    };
    vk::PushConstantRange::default()
        .stage_flags(stages)
        .size(if edges {
            128
        } else if line {
            80
        } else {
            64
        })
}

pub(super) fn descriptor_layout_handles(
    sprite: Option<vk::DescriptorSetLayout>,
    lighting: Option<vk::DescriptorSetLayout>,
    clipping: Option<vk::DescriptorSetLayout>,
) -> Vec<vk::DescriptorSetLayout> {
    if let (Some(sprite), Some(lighting)) = (sprite, lighting) {
        // Keep clipping at set 1; an unused compatible layout fills its slot when absent.
        return vec![lighting, clipping.unwrap_or(lighting), sprite];
    }
    sprite
        .or(lighting)
        .or(clipping)
        .into_iter()
        .chain(clipping)
        .collect()
}

fn interpolation_push_ranges(
    push: vk::PushConstantRange,
    enabled: bool,
    triangle_grid_coverage: bool,
    native_grid_v4: bool,
    text_grid_interpolation: bool,
) -> Vec<vk::PushConstantRange> {
    if text_grid_interpolation {
        return vec![vk::PushConstantRange {
            size: push.size.max(96),
            ..push
        }];
    }
    let mut ranges = vec![push];
    if enabled {
        ranges.push(
            vk::PushConstantRange::default()
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                .offset(64)
                .size(if native_grid_v4 {
                    52
                } else if triangle_grid_coverage {
                    20
                } else {
                    16
                }),
        );
    }
    ranges
}
#[cfg(test)]
mod interpolation_tests {
    use super::*;
    #[test]
    fn hardware_layout_is_unchanged_and_v2_owns_only_fragment_viewport() {
        let push = push_range(false, false);
        assert_eq!(
            interpolation_push_ranges(push, false, false, false, false).len(),
            1
        );
        let ranges = interpolation_push_ranges(push, true, false, false, false);
        assert_eq!(
            (ranges[0].size, ranges[0].stage_flags),
            (64, vk::ShaderStageFlags::VERTEX)
        );
        assert_eq!(
            (ranges[1].offset, ranges[1].size, ranges[1].stage_flags),
            (64, 16, vk::ShaderStageFlags::FRAGMENT)
        );
    }

    #[test]
    fn grid_coverage_appends_one_origin_word_to_the_existing_v2_range() {
        let push = push_range(false, false);
        let ranges = interpolation_push_ranges(push, true, true, false, false);
        assert_eq!((ranges[1].offset, ranges[1].size), (64, 20));
    }

    #[test]
    fn v4_facing_words_extend_only_the_v4_fragment_range() {
        let ranges = interpolation_push_ranges(push_range(false, false), true, true, true, false);
        assert_eq!((ranges[1].offset, ranges[1].size), (64, 52));
    }

    #[test]
    fn text_grid_extends_its_existing_vertex_fragment_range_without_overlap() {
        let ranges = interpolation_push_ranges(push_range(true, false), true, true, false, true);
        assert_eq!(ranges.len(), 1);
        assert_eq!(
            (ranges[0].offset, ranges[0].size, ranges[0].stage_flags),
            (
                0,
                96,
                vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT
            )
        );
    }
}
