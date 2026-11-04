//! Opt-in raw varying diagnostic; no scene, intersection or qualification authority.
use super::{
    runtime::{VulkanOptions, VulkanRuntime},
    storage_descriptor::{StorageDescriptorLayout, UploadedStorage},
    submission::CommandSubmission,
};
use ash::vk;
use std::{path::Path, sync::Arc};
#[path = "raytrace_pixel_pipeline.rs"]
mod pipeline;
pub(super) const EXTENT: u32 = 409;
pub(super) const BYTE_COUNT: usize = 409 * 32;
#[test]
#[ignore = "requires source-reviewed canonical GPU lease and actual AMD RADV"]
fn raw_center_column() {
    let request = std::env::var("MARTENSITE_RAYTRACE_PIXEL_OUT").unwrap();
    let directory = Path::new(&request);
    assert!(directory.is_absolute());
    std::fs::create_dir(directory).unwrap();
    let runtime = selected_runtime();
    capture(&runtime, directory);
    assert!(runtime.validation.errors().unwrap().is_empty());
}
fn selected_runtime() -> Arc<VulkanRuntime> {
    let runtime = VulkanRuntime::create_with_raster_policy(
        VulkanOptions {
            validation: true,
            device_index: Some(0),
        },
        Default::default(),
    )
    .unwrap();
    assert_eq!(
        runtime.attachment_coordinates,
        super::attachment_coordinates::AttachmentCoordinates::TopLeftLogicalV2
    );
    require_device(&runtime);
    runtime
}
fn require_device(runtime: &VulkanRuntime) {
    let driver = super::driver_identity::query_driver_identity(&runtime.instance, runtime.physical);
    assert_eq!(
        driver.vendor_id, 0x1002,
        "expected actual AMD, got {driver:?}"
    );
    assert_eq!(driver.driver_id, vk::DriverId::MESA_RADV.as_raw() as u32);
    assert!(
        runtime.fragment_stores,
        "expected actual fragmentStoresAndAtomics"
    );
}
fn capture(runtime: &Arc<VulkanRuntime>, directory: &Path) {
    let layout = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::FRAGMENT).unwrap();
    let output = UploadedStorage::new(runtime, &layout, &vec![0; BYTE_COUNT]).unwrap();
    let pipeline = pipeline::PixelPipeline::new(runtime, layout.handle);
    let commands = CommandSubmission::new(runtime).unwrap();
    record(runtime, &commands, &pipeline, output.set);
    commands.finish().unwrap();
    let bytes = output.test_readback(BYTE_COUNT).unwrap();
    require_complete(&bytes, runtime.attachment_coordinates.leading_rows(EXTENT));
    std::fs::write(directory.join("column.bin"), bytes).unwrap();
    metadata(runtime, directory);
}
fn record(
    runtime: &VulkanRuntime,
    submission: &CommandSubmission,
    pipeline: &pipeline::PixelPipeline,
    descriptor: vk::DescriptorSet,
) {
    let extent = vk::Extent2D {
        width: EXTENT,
        height: EXTENT,
    };
    begin(runtime, submission.command, extent);
    super::record::set_viewport(runtime, submission.command, extent);
    pipeline.draw(submission.command, descriptor);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_end_rendering(submission.command) };
    host_barrier(runtime, submission.command);
}
fn begin(runtime: &VulkanRuntime, command: vk::CommandBuffer, extent: vk::Extent2D) {
    let area = vk::Rect2D {
        offset: Default::default(),
        extent: runtime.attachment_coordinates.attachment_extent(extent),
    };
    let info = vk::RenderingInfo::default()
        .render_area(area)
        .layer_count(1);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { runtime.device().cmd_begin_rendering(command, &info) };
}
fn host_barrier(runtime: &VulkanRuntime, command: vk::CommandBuffer) {
    let barrier = [vk::MemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::SHADER_WRITE)
        .dst_access_mask(vk::AccessFlags::HOST_READ)];
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_pipeline_barrier(
            command,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &barrier,
            &[],
            &[],
        )
    };
}
fn require_complete(bytes: &[u8], leading: u32) {
    assert_eq!(bytes.len(), BYTE_COUNT);
    for (index, row) in bytes.as_chunks::<32>().0.iter().enumerate() {
        assert_eq!(
            u32::from_ne_bytes(row[28..32].try_into().unwrap()),
            index as u32 + leading
        );
        assert_eq!(u32::from_ne_bytes(row[16..20].try_into().unwrap()), 1);
        assert_eq!(
            u32::from_ne_bytes(row[8..12].try_into().unwrap()),
            0.5f32.to_bits()
        );
    }
}
fn metadata(runtime: &VulkanRuntime, directory: &Path) {
    let driver = super::owned_sample_metadata::device_identity(runtime);
    let ids = super::owned_sample_metadata::id_properties(runtime);
    let coordinates = format!("{:?}", runtime.attachment_coordinates);
    let text = format!(
        r#"{{"driver":{driver},"identifiers":{ids},"coordinates":"{coordinates}","width":409,"logical_height":409,"attachment_height":410,"leading_rows":1,"samples":1,"column":204,"vertices":6,"qualification":false,"reference":"counterfactual gl_FragCoord, not observed vPixel"}}"#
    );
    std::fs::write(directory.join("capture.json"), text).unwrap();
}
#[test]
fn column_layout_requires_one_actual_fragment_and_exact_reference() {
    let mut bytes = vec![0; BYTE_COUNT];
    for (index, row) in bytes.as_chunks_mut::<32>().0.iter_mut().enumerate() {
        row[28..32].copy_from_slice(&(index as u32 + 1).to_ne_bytes());
        row[16..20].copy_from_slice(&1u32.to_ne_bytes());
        row[8..12].copy_from_slice(&0.5f32.to_bits().to_ne_bytes());
    }
    require_complete(&bytes, 1);
    bytes[16] = 0;
    assert!(std::panic::catch_unwind(|| require_complete(&bytes, 1)).is_err());
}
#[test]
fn leading_row_matches_production_odd_height_raster_contract() {
    use super::attachment_coordinates::AttachmentCoordinates;
    let policy = AttachmentCoordinates::TopLeftLogicalV2;
    let logical = vk::Extent2D {
        width: EXTENT,
        height: EXTENT,
    };
    let full = vk::Rect2D {
        offset: Default::default(),
        extent: logical,
    };
    let actual = policy.raster_regions(logical, full, full);
    assert_eq!(policy.attachment_extent(logical).height, 410);
    assert_eq!(actual.scissor.offset.y, 1);
    assert_eq!(actual.scissor.extent.height, EXTENT);
    assert_eq!((actual.viewport.y, actual.viewport.height), (410., -409.));
}

#[path = "raytrace_generate_ray_control.rs"]
mod generate_ray;
