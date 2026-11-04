use super::*;

#[test]
fn image_creation_retains_format_extent_and_unknown_chain_marker() {
    let info = varied_image_creation_info();
    let value = image_json(29, &info);
    assert_eq!(value["image"], 29);
    assert_eq!(value["extent"], json!([7, 11, 13]));
    assert_eq!(value["mip_levels"], 4);
    assert_eq!(value["samples"], 4);
    assert_eq!(value["pnext_present"], false);
    assert_eq!(value["queue_family_indices_observed"], true);
}

#[test]
fn unobserved_image_extension_or_queue_operands_are_explicit() {
    let mut info = vk::ImageCreateInfo::default();
    info.p_next = std::ptr::NonNull::<u8>::dangling().as_ptr().cast();
    info.queue_family_index_count = 2;
    let value = image_json(31, &info);
    assert_eq!(value["pnext_present"], true);
    assert_eq!(value["queue_family_indices_observed"], false);
    assert_eq!(value["queue_family_count"], 2);
}

#[test]
fn actual_view_components_and_subresource_ranges_are_not_defaulted() {
    let mut info = vk::ImageViewCreateInfo::default().image(vk::Image::from_raw(37));
    info.components.r = vk::ComponentSwizzle::B;
    info.subresource_range = vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        base_mip_level: 2,
        level_count: 3,
        base_array_layer: 4,
        layer_count: 5,
    };
    let value = view_json(41, &info);
    assert_eq!(value["image"], 37);
    assert_eq!(value["components"][0], vk::ComponentSwizzle::B.as_raw());
    assert_eq!(value["subresource"], json!([1, 2, 3, 4, 5]));
}

#[test]
fn staging_payload_is_owned_and_preserves_zero_and_full_alpha_bytes() {
    let mut bytes = vec![255, 255, 255, 0, 255, 255, 255, 255];
    let value = staging_json(43, 47, &bytes);
    bytes.fill(17);
    assert_eq!(value["byte_count"], 8);
    assert_eq!(
        value["bytes"],
        json!([255, 255, 255, 0, 255, 255, 255, 255])
    );
    assert_eq!(value["buffer"], 47);
}

#[test]
fn copy_operands_preserve_stride_offset_and_region_order() {
    let regions = strided_image_copy_regions();
    let value = copy_json(53, 59, 61, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &regions);
    assert_eq!(value["regions"][0]["buffer_offset"], 23);
    assert_eq!(value["regions"][0]["buffer_row_length"], 7);
    assert_eq!(value["regions"][0]["buffer_image_height"], 11);
    assert_eq!(value["regions"][0]["offset"], json!([-3, 5, -7]));
    assert_eq!(value["regions"][1]["buffer_offset"], 97);
}

#[test]
fn sampler_float_bits_and_enable_operands_remain_literal() {
    let info = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .mip_lod_bias(-0.0)
        .min_lod(f32::from_bits(1))
        .max_lod(3.5)
        .compare_enable(true)
        .compare_op(vk::CompareOp::GREATER);
    let value = sampler_json(67, &info);
    assert_eq!(value["lod_bias_bits"], (-0.0f32).to_bits());
    assert_eq!(value["lod_range_bits"], json!([1, 3.5f32.to_bits()]));
    assert_eq!(value["filters"][0], vk::Filter::LINEAR.as_raw());
    assert_eq!(value["compare_enable"], 1);
    assert_eq!(value["compare_op"], vk::CompareOp::GREATER.as_raw());
}

#[test]
fn descriptor_write_binds_actual_distinct_resource_handles() {
    let info = vk::DescriptorImageInfo::default()
        .image_view(vk::ImageView::from_raw(71))
        .sampler(vk::Sampler::from_raw(73))
        .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
    let value = descriptor_json(79, 5, &info);
    assert_eq!(value["set"], 79);
    assert_eq!(value["binding"], 5);
    assert_eq!(value["view"], 71);
    assert_eq!(value["sampler"], 73);
    assert_eq!(value["descriptor_count"], 1);
    assert_eq!(value["array_element"], 0);
}

fn varied_image_creation_info() -> vk::ImageCreateInfo<'static> {
    vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_3D)
        .format(vk::Format::R8G8B8A8_UNORM)
        .extent(vk::Extent3D {
            width: 7,
            height: 11,
            depth: 13,
        })
        .mip_levels(4)
        .array_layers(2)
        .samples(vk::SampleCountFlags::TYPE_4)
}

fn strided_image_copy_regions() -> [vk::BufferImageCopy; 2] {
    let first = vk::BufferImageCopy::default()
        .buffer_offset(23)
        .buffer_row_length(7)
        .buffer_image_height(11)
        .image_offset(vk::Offset3D { x: -3, y: 5, z: -7 });
    let second = vk::BufferImageCopy::default().buffer_offset(97);
    [first, second]
}
