//! API-consumed replay operands; these records do not claim GPU readback.
use ash::{vk, vk::Handle};

pub(super) fn push(
    command: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    stages: vk::ShaderStageFlags,
    offset: u32,
    bytes: &[u8],
) {
    eprintln!(
        "{}",
        push_json(
            [command.as_raw(), layout.as_raw()],
            stages.as_raw(),
            offset,
            bytes
        )
    );
}
fn push_json(identity: [u64; 2], stages: u32, offset: u32, bytes: &[u8]) -> String {
    format!(
        "{{\"kind\":\"command-push-constants\",\"command_layout\":{identity:?},\"stages\":{stages},\"offset\":{offset},\"bytes\":{bytes:?},\"gpu_readback\":false}}"
    )
}
pub(super) fn storage(set: vk::DescriptorSet, buffer: vk::Buffer, bytes: &[u8], readback: bool) {
    let identity = [set.as_raw(), buffer.as_raw()];
    eprintln!("{}", storage_json(identity, bytes, readback));
}
fn storage_json(identity: [u64; 2], bytes: &[u8], readback: bool) -> String {
    format!(
        "{{\"kind\":\"storage-descriptor-bytes\",\"set_buffer\":{identity:?},\"binding\":0,\"offset\":0,\"bytes\":{bytes:?},\"gpu_readback\":{readback}}}"
    )
}
pub(super) fn completed(command: vk::CommandBuffer, fence: vk::Fence) {
    let identity = [command.as_raw(), fence.as_raw()];
    eprintln!("{{\"kind\":\"submission-fence-completed\",\"command_fence\":{identity:?}}}");
}
pub(super) fn descriptor_bound(
    command: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    index: u32,
    set: vk::DescriptorSet,
    dynamic_offsets: &[u32],
) {
    let identity = [command.as_raw(), layout.as_raw(), set.as_raw()];
    eprintln!(
        "{{\"kind\":\"command-descriptor-bound\",\"command_layout_set\":{identity:?},\"set_index\":{index},\"dynamic_offsets\":{dynamic_offsets:?},\"bind_point\":0}}"
    );
}
pub(super) fn buffer_written(buffer: vk::Buffer, bytes: &[u8]) {
    let buffer = buffer.as_raw();
    eprintln!(
        "{{\"kind\":\"buffer-host-write-input\",\"buffer\":{buffer},\"bytes\":{bytes:?},\"gpu_readback\":false}}"
    );
}
pub(super) fn descriptor_writes(writes: &[vk::WriteDescriptorSet<'_>]) {
    for write in writes {
        eprintln!("{}", descriptor_write_json(write));
    }
}
fn descriptor_write_json(write: &vk::WriteDescriptorSet<'_>) -> serde_json::Value {
    serde_json::json!({"kind":"descriptor-set-write","set":write.dst_set.as_raw(),"binding":write.dst_binding,
        "array_element":write.dst_array_element,"descriptor_type":write.descriptor_type.as_raw(),"count":write.descriptor_count,
        "buffers":buffer_infos(write),"images":image_infos(write),"texel_views_present":!write.p_texel_buffer_view.is_null(),
        "extensions_present":!write.p_next.is_null()})
}
fn buffer_infos(write: &vk::WriteDescriptorSet<'_>) -> Option<Vec<[u64; 3]>> {
    if !matches!(
        write.descriptor_type,
        vk::DescriptorType::UNIFORM_BUFFER
            | vk::DescriptorType::STORAGE_BUFFER
            | vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC
            | vk::DescriptorType::STORAGE_BUFFER_DYNAMIC
    ) || write.p_buffer_info.is_null()
    {
        return None;
    }
    let rows =
        unsafe { std::slice::from_raw_parts(write.p_buffer_info, write.descriptor_count as usize) };
    Some(
        rows.iter()
            .map(|row| [row.buffer.as_raw(), row.offset, row.range])
            .collect(),
    )
}
fn image_infos(write: &vk::WriteDescriptorSet<'_>) -> Option<Vec<[u64; 3]>> {
    if !image_descriptor(write.descriptor_type) || write.p_image_info.is_null() {
        return None;
    }
    let rows =
        unsafe { std::slice::from_raw_parts(write.p_image_info, write.descriptor_count as usize) };
    Some(rows.iter().map(image_info_words).collect())
}
fn image_descriptor(kind: vk::DescriptorType) -> bool {
    matches!(
        kind,
        vk::DescriptorType::SAMPLER
            | vk::DescriptorType::COMBINED_IMAGE_SAMPLER
            | vk::DescriptorType::SAMPLED_IMAGE
            | vk::DescriptorType::STORAGE_IMAGE
            | vk::DescriptorType::INPUT_ATTACHMENT
    )
}
fn image_info_words(row: &vk::DescriptorImageInfo) -> [u64; 3] {
    [
        row.sampler.as_raw(),
        row.image_view.as_raw(),
        row.image_layout.as_raw() as u64,
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actual_descriptor_range_type_and_array_element_are_bound() {
        let rows = [vk::DescriptorBufferInfo::default()
            .buffer(vk::Buffer::from_raw(8))
            .offset(12)
            .range(16)];
        let write = vk::WriteDescriptorSet::default()
            .dst_set(vk::DescriptorSet::from_raw(7))
            .dst_binding(3)
            .dst_array_element(2)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&rows);
        let value = descriptor_write_json(&write);
        assert_eq!(value["buffers"], serde_json::json!([[8, 12, 16]]));
        assert_eq!(value["array_element"], 2);
        assert_eq!(value["descriptor_type"], 7);
    }
    #[test]
    fn inactive_buffer_pointer_is_not_dereferenced_for_image_write() {
        let write = vk::WriteDescriptorSet {
            p_buffer_info: std::ptr::dangling(),
            descriptor_type: vk::DescriptorType::SAMPLER,
            ..Default::default()
        };
        assert_eq!(buffer_infos(&write), None);
    }
    #[test]
    fn storage_upload_and_fenced_readback_have_distinct_authority() {
        let upload = storage_json([3, 4], &[5, 6], false);
        let readback = storage_json([3, 4], &[7, 8], true);
        assert!(upload.contains("\"gpu_readback\":false"));
        assert!(readback.contains("\"gpu_readback\":true"));
        assert!(readback.contains("\"bytes\":[7, 8]"));
    }
    #[test]
    fn push_records_disjoint_bias_offset_and_exact_operand_bytes() {
        let json = push_json([1, 2], 24, 128, &[0, 255]);
        assert!(json.contains("\"offset\":128,\"bytes\":[0, 255]"));
        assert!(json.contains("\"gpu_readback\":false"));
    }
}
