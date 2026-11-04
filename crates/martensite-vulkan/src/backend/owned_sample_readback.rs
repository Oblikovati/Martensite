//! Owned v2 transfers retain every IEEE f32 depth bit; v1 D24 decoding stays separately tested.
use super::{buffer::DeviceBuffer, frame::FrameResources, runtime::VulkanRuntime};
use ash::vk;
use std::{path::Path, sync::Arc};
pub(super) type OwnedResult<T> = Result<T, Box<dyn std::error::Error>>;

pub(super) fn stencil_buffer(runtime: &Arc<VulkanRuntime>) -> OwnedResult<DeviceBuffer> {
    let buffer = DeviceBuffer::new(
        runtime,
        4096,
        vk::BufferUsageFlags::TRANSFER_DST,
        super::memory_selection::HostBufferAccess::Readback,
    )?;
    Ok(buffer)
}
pub(super) fn copy_stencil(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    buffer: &DeviceBuffer,
) {
    let region = stencil_region();
    // record_frame already transitioned BOTH aspects; this additional copy precedes the fence.
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_copy_image_to_buffer(
            command,
            resources.depth.handle,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            buffer.handle,
            &[region],
        );
    }
    host_barrier(runtime, command);
}
fn stencil_region() -> vk::BufferImageCopy {
    let aspect = vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::STENCIL)
        .layer_count(1);
    let extent = vk::Extent3D {
        width: 64,
        height: 64,
        depth: 1,
    };
    vk::BufferImageCopy::default()
        .image_subresource(aspect)
        .image_extent(extent)
}
fn host_barrier(runtime: &VulkanRuntime, command: vk::CommandBuffer) {
    let barrier = vk::MemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
        .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::HOST)
        .dst_access_mask(vk::AccessFlags2::HOST_READ);
    // Coherent host-visible memory still requires execution and availability synchronization.
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().memory_barriers(&[barrier]),
        );
    }
}
pub(super) fn write_images(
    directory: &Path,
    resources: &FrameResources,
    stencil: &DeviceBuffer,
) -> OwnedResult<()> {
    std::fs::create_dir(directory)?;
    let raw_color = resources.color_read.read(16384)?;
    let raw_depth = resources.depth_read.read(16384)?;
    std::fs::write(directory.join("color.rgba"), raw_color)?;
    std::fs::write(
        directory.join("depth.f32bits.u32le"),
        logical_f32_depth(&raw_depth)?,
    )?;
    std::fs::write(directory.join("depth.raw"), raw_depth)?;
    std::fs::write(directory.join("stencil.u8"), stencil.read(4096)?)?;
    Ok(())
}
pub(super) fn logical_f32_depth(raw: &[u8]) -> OwnedResult<Vec<u8>> {
    if raw.len() != 16384 {
        return Err(format!(
            "f32 depth bytes={}; expected16384 exact D32 bits",
            raw.len()
        )
        .into());
    }
    Ok(raw.to_vec())
}
pub(super) fn logical_depth(raw: &[u8]) -> OwnedResult<Vec<u8>> {
    if raw.len() != 16384 {
        return Err(format!("depth bytes={}; expected 16384 D24/X8 bytes", raw.len()).into());
    }
    // Vulkan buffer-image depth-only D24 uses the low24 bits. X8 padding is unspecified:
    // retain raw bytes separately, compare exact logical24 bits without claiming padding equivalence.
    let logical = raw
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|word| [word[0], word[1], word[2], 0])
        .collect();
    Ok(logical)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn v2_depth_preserves_every_bit_without_masking_or_quantization() {
        let raw = [0x12, 0x34, 0x56, 0xAB].repeat(4096);
        assert_eq!(logical_f32_depth(&raw).unwrap(), raw);
        assert!(logical_f32_depth(&[0; 4]).is_err());
    }
    #[test]
    fn logical_depth_preserves_each_depth_bit_and_labels_only_padding_as_nonlogical() {
        let raw = [0x12, 0x34, 0x56, 0xAB].repeat(4096);
        assert_eq!(
            logical_depth(&raw).unwrap(),
            [0x12, 0x34, 0x56, 0].repeat(4096)
        );
        assert_eq!(raw[3], 0xAB);
    }
    #[test]
    fn cropped_readback_refuses_instead_of_filling_missing_samples() {
        assert!(
            logical_depth(&[0; 4])
                .unwrap_err()
                .to_string()
                .contains("expected 16384")
        );
    }
}
