//! Actual sampled-resource API inputs remain owned scalar/byte records for replay.
use ash::{vk, vk::Handle};
use serde_json::{Value, json};

pub(in crate::backend) fn image(image: vk::Image, info: &vk::ImageCreateInfo<'_>) {
    eprintln!("{}", image_json(image.as_raw(), info));
}
fn image_json(image: u64, info: &vk::ImageCreateInfo<'_>) -> Value {
    json!({"kind":"sampled-image-create", "image":image,
        "flags":info.flags.as_raw(), "image_type":info.image_type.as_raw(),
        "format":info.format.as_raw(), "extent":[info.extent.width,info.extent.height,info.extent.depth],
        "mip_levels":info.mip_levels, "array_layers":info.array_layers,
        "samples":info.samples.as_raw(), "tiling":info.tiling.as_raw(),
        "usage":info.usage.as_raw(), "sharing_mode":info.sharing_mode.as_raw(),
        "queue_family_count":info.queue_family_index_count,
        "queue_family_indices_observed":info.queue_family_index_count == 0,
        "initial_layout":info.initial_layout.as_raw(), "pnext_present":!info.p_next.is_null()})
}
pub(in crate::backend) fn view(view: vk::ImageView, info: &vk::ImageViewCreateInfo<'_>) {
    eprintln!("{}", view_json(view.as_raw(), info));
}
fn view_json(view: u64, info: &vk::ImageViewCreateInfo<'_>) -> Value {
    let range = info.subresource_range;
    let components = info.components;
    json!({"kind":"sampled-image-view-create", "view":view, "image":info.image.as_raw(),
        "flags":info.flags.as_raw(), "view_type":info.view_type.as_raw(), "format":info.format.as_raw(),
        "components":[components.r.as_raw(),components.g.as_raw(),components.b.as_raw(),components.a.as_raw()],
        "subresource":[range.aspect_mask.as_raw(),range.base_mip_level,range.level_count,range.base_array_layer,range.layer_count],
        "pnext_present":!info.p_next.is_null()})
}
pub(in crate::backend) fn staging(image: vk::Image, buffer: vk::Buffer, bytes: &[u8]) {
    eprintln!("{}", staging_json(image.as_raw(), buffer.as_raw(), bytes));
}
fn staging_json(image: u64, buffer: u64, bytes: &[u8]) -> Value {
    json!({"kind":"sampled-image-staging-write", "image":image, "buffer":buffer,
        "byte_count":bytes.len(), "bytes":bytes})
}
pub(in crate::backend) fn copy(
    command: vk::CommandBuffer,
    buffer: vk::Buffer,
    image: vk::Image,
    layout: vk::ImageLayout,
    regions: &[vk::BufferImageCopy],
) {
    eprintln!(
        "{}",
        copy_json(
            command.as_raw(),
            buffer.as_raw(),
            image.as_raw(),
            layout,
            regions
        )
    );
}
fn copy_json(
    command: u64,
    buffer: u64,
    image: u64,
    layout: vk::ImageLayout,
    regions: &[vk::BufferImageCopy],
) -> Value {
    json!({"kind":"command-copy-buffer-to-sampled-image", "command":command,
        "buffer":buffer, "image":image, "layout":layout.as_raw(),
        "regions":regions.iter().map(copy_region_json).collect::<Vec<_>>()})
}
pub(in crate::backend) fn copy_region_json(region: &vk::BufferImageCopy) -> Value {
    let sub = region.image_subresource;
    let offset = region.image_offset;
    let extent = region.image_extent;
    json!({"buffer_offset":region.buffer_offset, "buffer_row_length":region.buffer_row_length,
        "buffer_image_height":region.buffer_image_height,
        "subresource":[sub.aspect_mask.as_raw(),sub.mip_level,sub.base_array_layer,sub.layer_count],
        "offset":[offset.x,offset.y,offset.z], "extent":[extent.width,extent.height,extent.depth]})
}
pub(in crate::backend) fn sampler(sampler: vk::Sampler, info: &vk::SamplerCreateInfo<'_>) {
    eprintln!("{}", sampler_json(sampler.as_raw(), info));
}
fn sampler_json(sampler: u64, info: &vk::SamplerCreateInfo<'_>) -> Value {
    json!({"kind":"sampled-image-sampler-create", "sampler":sampler, "flags":info.flags.as_raw(),
        "filters":[info.mag_filter.as_raw(),info.min_filter.as_raw(),info.mipmap_mode.as_raw()],
        "address_modes":[info.address_mode_u.as_raw(),info.address_mode_v.as_raw(),info.address_mode_w.as_raw()],
        "lod_bias_bits":info.mip_lod_bias.to_bits(), "anisotropy_enable":info.anisotropy_enable,
        "max_anisotropy_bits":info.max_anisotropy.to_bits(), "compare_enable":info.compare_enable,
        "compare_op":info.compare_op.as_raw(), "lod_range_bits":[info.min_lod.to_bits(),info.max_lod.to_bits()],
        "border_color":info.border_color.as_raw(), "unnormalized_coordinates":info.unnormalized_coordinates,
        "pnext_present":!info.p_next.is_null()})
}
pub(in crate::backend) fn descriptor(
    set: vk::DescriptorSet,
    binding: u32,
    info: &vk::DescriptorImageInfo,
) {
    eprintln!("{}", descriptor_json(set.as_raw(), binding, info));
}
fn descriptor_json(set: u64, binding: u32, info: &vk::DescriptorImageInfo) -> Value {
    json!({"kind":"sampled-image-descriptor-write", "set":set, "binding":binding,
        "descriptor_type":vk::DescriptorType::COMBINED_IMAGE_SAMPLER.as_raw(), "array_element":0,
        "descriptor_count":1, "view":info.image_view.as_raw(), "sampler":info.sampler.as_raw(),
        "layout":info.image_layout.as_raw()})
}

#[cfg(test)]
#[path = "replay_sampled_image_diagnostic_tests.rs"]
mod tests;

pub(in crate::backend) fn readback(
    command: vk::CommandBuffer,
    image: vk::Image,
    buffer: vk::Buffer,
    regions: &[vk::BufferImageCopy],
) {
    eprintln!(
        "{}",
        json!({"kind":"command-copy-image-to-buffer","command":command.as_raw(),"image":image.as_raw(),
        "buffer":buffer.as_raw(),"layout":vk::ImageLayout::TRANSFER_SRC_OPTIMAL.as_raw(),
        "regions":regions.iter().map(copy_region_json).collect::<Vec<_>>()})
    );
}
