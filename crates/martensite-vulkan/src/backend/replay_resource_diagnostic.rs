//! Resource and layout API descriptions preserve handles, offsets and initialized bytes.
use ash::{vk, vk::Handle};
use serde_json::json;

pub(super) fn pipeline_layout(layout: vk::PipelineLayout, info: &vk::PipelineLayoutCreateInfo<'_>) {
    eprintln!("{}", pipeline_layout_json(layout, info));
}
fn pipeline_layout_json(
    layout: vk::PipelineLayout,
    info: &vk::PipelineLayoutCreateInfo<'_>,
) -> serde_json::Value {
    json!({"kind":"pipeline-layout-create-input","layout":layout.as_raw(),
        "sets":set_layouts(info),"push_ranges":push_ranges(info),
        "extensions_present":!info.p_next.is_null(),"flags":info.flags.as_raw()})
}
fn set_layouts(info: &vk::PipelineLayoutCreateInfo<'_>) -> Vec<u64> {
    if info.set_layout_count == 0 {
        return Vec::new();
    }
    let rows =
        unsafe { std::slice::from_raw_parts(info.p_set_layouts, info.set_layout_count as usize) };
    rows.iter().map(|set| set.as_raw()).collect()
}
fn push_ranges(info: &vk::PipelineLayoutCreateInfo<'_>) -> Vec<[u32; 3]> {
    if info.push_constant_range_count == 0 {
        return Vec::new();
    }
    let rows = unsafe {
        std::slice::from_raw_parts(
            info.p_push_constant_ranges,
            info.push_constant_range_count as usize,
        )
    };
    rows.iter()
        .map(|row| [row.stage_flags.as_raw(), row.offset, row.size])
        .collect()
}
pub(super) fn descriptor_layout(
    layout: vk::DescriptorSetLayout,
    info: &vk::DescriptorSetLayoutCreateInfo<'_>,
) {
    let bindings = if info.binding_count == 0 {
        &[][..]
    } else {
        unsafe { std::slice::from_raw_parts(info.p_bindings, info.binding_count as usize) }
    };
    let rows: Vec<_> = bindings.iter().map(|row|json!({"binding":row.binding,"type":row.descriptor_type.as_raw(),
        "count":row.descriptor_count,"stages":row.stage_flags.as_raw(),"immutable_samplers_present":!row.p_immutable_samplers.is_null()})).collect();
    eprintln!(
        "{}",
        json!({"kind":"descriptor-layout-bindings","layout":layout.as_raw(),"bindings":rows,
        "flags":info.flags.as_raw(),"extensions_present":!info.p_next.is_null()})
    );
}
pub(super) fn allocated(set: vk::DescriptorSet, layout: vk::DescriptorSetLayout) {
    eprintln!(
        "{}",
        json!({"kind":"descriptor-set-allocated","set":set.as_raw(),"layout":layout.as_raw()})
    );
}
pub(super) fn status_accepted(set: vk::DescriptorSet, indices: &[u32], expected: u64) {
    eprintln!(
        "{}",
        json!({"kind":"window-depth-status-accepted","set":set.as_raw(),"indices":indices,
        "expected_invocations":expected,"validator":"window_depth_status::WindowDepthStatus::validate","accepted":true})
    );
}

pub(super) fn target_extensions(image: vk::Image, info: &vk::ImageCreateInfo<'_>) {
    eprintln!(
        "{}",
        json!({"kind":"target-image-extensions","image":image.as_raw(),
        "extensions":super::replay_pipeline_state::extensions::image(info.p_next)})
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    struct FakeApiLayout {
        sets: [vk::DescriptorSetLayout; 2],
        ranges: [vk::PushConstantRange; 2],
    }
    impl FakeApiLayout {
        fn new() -> Self {
            Self {
                sets: [
                    vk::DescriptorSetLayout::from_raw(3),
                    vk::DescriptorSetLayout::from_raw(4),
                ],
                ranges: [
                    vk::PushConstantRange::default()
                        .stage_flags(vk::ShaderStageFlags::VERTEX)
                        .size(128),
                    vk::PushConstantRange::default()
                        .stage_flags(
                            vk::ShaderStageFlags::GEOMETRY | vk::ShaderStageFlags::FRAGMENT,
                        )
                        .offset(128)
                        .size(16),
                ],
            }
        }
        fn info(&self) -> vk::PipelineLayoutCreateInfo<'_> {
            vk::PipelineLayoutCreateInfo::default()
                .set_layouts(&self.sets)
                .push_constant_ranges(&self.ranges)
        }
    }
    #[test]
    fn actual_layout_set_order_and_disjoint_stage_ranges_are_retained() {
        let fixture = FakeApiLayout::new();
        let value = pipeline_layout_json(vk::PipelineLayout::from_raw(9), &fixture.info());
        assert_eq!(value["sets"], json!([3, 4]));
        assert_eq!(value["push_ranges"], json!([[1, 0, 128], [24, 128, 16]]));
        assert_eq!(value["layout"], 9);
    }
    #[test]
    fn zero_count_null_layout_arrays_are_not_dereferenced() {
        let info = vk::PipelineLayoutCreateInfo::default();
        assert!(set_layouts(&info).is_empty());
        assert!(push_ranges(&info).is_empty());
    }
}
