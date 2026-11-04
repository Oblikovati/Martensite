//! Cube map upload: glTexImage2D of each side's memory rows, then glGenerateMipmap as linear
//! half-size blits of all six sides, level by level; or, for compressed (DDS) sides,
//! glCompressedTexImage2D of every supplied level of every side and no generated levels.
use super::super::{
    buffer::DeviceBuffer, memory_selection::HostBufferAccess, runtime::VulkanRuntime,
    submission::CommandSubmission,
};
use super::EnvironmentImage;
use crate::environment_frame::CubeMapPixels;
use crate::{RenderError, error::rejected};
use ash::vk;
use std::mem::ManuallyDrop;
use std::sync::Arc;

/// Fill `image` from `pixels` and leave every level shader-readable.
/// Example: `let image = upload_faces(&runtime, image, &pixels)?`.
pub(super) fn upload_faces(
    runtime: &Arc<VulkanRuntime>,
    image: EnvironmentImage,
    pixels: &CubeMapPixels,
) -> Result<EnvironmentImage, RenderError> {
    let bytes = packed_faces(pixels)?;
    let staging = DeviceBuffer::new(
        runtime,
        bytes.len() as u64,
        vk::BufferUsageFlags::TRANSFER_SRC,
        HostBufferAccess::Upload,
    )?;
    staging.write(&bytes)?;
    let submission = CommandSubmission::new(runtime)?;
    if pixels.levels.is_empty() {
        record_upload(runtime.device(), submission.command, staging.handle, &image);
    } else {
        let regions = compressed_regions(pixels);
        record_compressed_upload(
            runtime.device(),
            submission.command,
            staging.handle,
            &image,
            &regions,
        );
    }
    finish_owned(submission, (image, staging)).map(|(image, _)| image)
}

/// Wait for a one-off submission and hand back what it used; a submission that may still run
/// keeps its resources alive forever instead. Example: `let image = finish_owned(s, image)?`.
pub(in super::super) fn finish_owned<T>(
    submission: CommandSubmission,
    owned: T,
) -> Result<T, RenderError> {
    let outcome = submission.finish();
    if submission.retirement_incomplete() {
        // The device may still read or write these resources: never free them.
        let _never_freed = (ManuallyDrop::new(owned), ManuallyDrop::new(submission));
        return Err(outcome.err().unwrap_or_else(|| {
            rejected("incomplete retirement", "a retired environment submission")
        }));
    }
    outcome.map(|()| owned)
}

/// The six sides back to back; every side must hold the same texel count (or, compressed, the
/// sum of its supplied levels).
fn packed_faces(pixels: &CubeMapPixels) -> Result<Vec<u8>, RenderError> {
    let texels = pixels.size as usize * pixels.size as usize;
    let side = pixels.faces[0].len();
    let whole = match pixels.levels.is_empty() {
        true => side.is_multiple_of(texels),
        false => side == pixels.levels.iter().sum::<usize>(),
    };
    if side == 0 || !whole || pixels.faces.iter().any(|face| face.len() != side) {
        return Err(rejected(
            pixels.faces.each_ref().map(Vec::len),
            "six cube map sides of equal whole-texel size",
        ));
    }
    Ok(pixels.faces.concat())
}

fn record_upload(
    device: &ash::Device,
    command: vk::CommandBuffer,
    staging: vk::Buffer,
    image: &EnvironmentImage,
) {
    let levels = image.spec.levels;
    let dst = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
    let src = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
    cube_barrier(
        device,
        command,
        image,
        0..levels,
        vk::ImageLayout::UNDEFINED,
        dst,
    );
    copy_level0(device, command, staging, image);
    for level in 1..levels {
        cube_barrier(device, command, image, level - 1..level, dst, src);
        blit_level(device, command, image, level);
    }
    let read = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
    cube_barrier(device, command, image, 0..levels - 1, src, read);
    cube_barrier(device, command, image, levels - 1..levels, dst, read);
}

/// One copy per side and supplied level, at that level's offset in the packed sides.
fn compressed_regions(pixels: &CubeMapPixels) -> Vec<vk::BufferImageCopy> {
    let side: usize = pixels.levels.iter().sum();
    let mut regions = Vec::with_capacity(6 * pixels.levels.len());
    for face in 0..6_u32 {
        let mut offset = face as usize * side;
        for (level, length) in pixels.levels.iter().enumerate() {
            let size = (pixels.size >> level).max(1);
            let layers = vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(level as u32)
                .base_array_layer(face)
                .layer_count(1);
            regions.push(
                vk::BufferImageCopy::default()
                    .buffer_offset(offset as u64)
                    .image_subresource(layers)
                    .image_extent(vk::Extent3D {
                        width: size,
                        height: size,
                        depth: 1,
                    }),
            );
            offset += length;
        }
    }
    regions
}

fn record_compressed_upload(
    device: &ash::Device,
    command: vk::CommandBuffer,
    staging: vk::Buffer,
    image: &EnvironmentImage,
    regions: &[vk::BufferImageCopy],
) {
    let levels = image.spec.levels;
    let dst = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
    cube_barrier(
        device,
        command,
        image,
        0..levels,
        vk::ImageLayout::UNDEFINED,
        dst,
    );
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { device.cmd_copy_buffer_to_image(command, staging, image.handle, dst, regions) };
    let read = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
    cube_barrier(device, command, image, 0..levels, dst, read);
}

fn copy_level0(
    device: &ash::Device,
    command: vk::CommandBuffer,
    source: vk::Buffer,
    image: &EnvironmentImage,
) {
    let region = vk::BufferImageCopy::default()
        .image_subresource(layers(0))
        .image_extent(vk::Extent3D {
            width: image.spec.width,
            height: image.spec.height,
            depth: 1,
        });
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        device.cmd_copy_buffer_to_image(
            command,
            source,
            image.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[region],
        );
    }
}

fn blit_level(
    device: &ash::Device,
    command: vk::CommandBuffer,
    image: &EnvironmentImage,
    level: u32,
) {
    let corner = |level: u32| vk::Offset3D {
        x: (image.spec.width >> level).max(1) as i32,
        y: (image.spec.height >> level).max(1) as i32,
        z: 1,
    };
    let blit = vk::ImageBlit::default()
        .src_subresource(layers(level - 1))
        .src_offsets([vk::Offset3D::default(), corner(level - 1)])
        .dst_subresource(layers(level))
        .dst_offsets([vk::Offset3D::default(), corner(level)]);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        device.cmd_blit_image(
            command,
            image.handle,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            image.handle,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[blit],
            vk::Filter::LINEAR,
        );
    }
}

fn layers(level: u32) -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .mip_level(level)
        .layer_count(6)
}

/// Move `levels` of every side of `image` between layouts, waiting for the stage that last
/// touched the old layout. Example: `cube_barrier(device, cmd, &image, 0..1, old, new)`.
pub(in super::super) fn cube_barrier(
    device: &ash::Device,
    command: vk::CommandBuffer,
    image: &EnvironmentImage,
    levels: std::ops::Range<u32>,
    old: vk::ImageLayout,
    new: vk::ImageLayout,
) {
    if levels.is_empty() {
        return;
    }
    let (src_stage, src_access) = layout_access(old);
    let (dst_stage, dst_access) = layout_access(new);
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(levels.start)
        .level_count(levels.end - levels.start)
        .layer_count(image.spec.layers());
    let barriers = [vk::ImageMemoryBarrier2::default()
        .image(image.handle)
        .subresource_range(range)
        .old_layout(old)
        .new_layout(new)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .src_stage_mask(src_stage)
        .src_access_mask(src_access)
        .dst_stage_mask(dst_stage)
        .dst_access_mask(dst_access)];
    let dependency = vk::DependencyInfo::default().image_memory_barriers(&barriers);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { device.cmd_pipeline_barrier2(command, &dependency) };
}

fn layout_access(layout: vk::ImageLayout) -> (vk::PipelineStageFlags2, vk::AccessFlags2) {
    use vk::{AccessFlags2 as A, ImageLayout as L, PipelineStageFlags2 as S};
    match layout {
        L::UNDEFINED => (S::NONE, A::NONE),
        L::SHADER_READ_ONLY_OPTIMAL => (S::FRAGMENT_SHADER, A::SHADER_SAMPLED_READ),
        L::COLOR_ATTACHMENT_OPTIMAL => (
            S::COLOR_ATTACHMENT_OUTPUT,
            A::COLOR_ATTACHMENT_WRITE | A::COLOR_ATTACHMENT_READ,
        ),
        L::TRANSFER_SRC_OPTIMAL => (S::BLIT, A::TRANSFER_READ),
        _ => (S::COPY | S::BLIT, A::TRANSFER_WRITE),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface_texture::SurfaceImageEncoding;

    fn pixels(sides: [usize; 6]) -> CubeMapPixels {
        CubeMapPixels {
            id: vec![],
            revision: 0,
            size: 2,
            encoding: SurfaceImageEncoding::Srgb8,
            faces: sides.map(|len| vec![7; len]),
            levels: Vec::new(),
            top_down: true,
            z_inverted: false,
        }
    }

    #[test]
    fn sides_pack_in_native_side_order() {
        let mut cube = pixels([16; 6]);
        cube.faces[1] = vec![9; 16];
        let packed = packed_faces(&cube).unwrap();
        assert_eq!(packed.len(), 96);
        assert_eq!(packed[16], 9);
        assert_eq!(packed[32], 7);
    }

    #[test]
    fn unequal_or_partial_sides_are_rejected() {
        assert!(packed_faces(&pixels([16, 16, 16, 16, 16, 12])).is_err());
        assert!(packed_faces(&pixels([15; 6])).is_err());
        assert!(packed_faces(&pixels([0; 6])).is_err());
    }
}
