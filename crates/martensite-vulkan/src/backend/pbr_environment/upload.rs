//! Initial environment contents: the shared EnvLUT texels and GlPBREnvironment::clear.
use super::super::{
    buffer::DeviceBuffer, memory_selection::HostBufferAccess, submission::CommandSubmission,
};
use super::image::EnvironmentImage;
use crate::RenderError;
use ash::vk;

/// Textures_EnvLUT (generated ahead of time): 128x128 RG32F
/// little-endian texels, row 0 first, uploaded unchanged as GlView does.
const ENV_LUT_TEXELS: &[u8] = include_bytes!("env_lut.bin");

/// `GlPBREnvironment::clear` with colour (1, 1, 1): the diffuse SH map keeps only the
/// constant coefficient (texel 0 cleared to the colour, texels 1-8 to (0, 0, 0, 1)).
pub(super) fn cleared_diffuse_sh(color: [f32; 3]) -> Vec<u8> {
    let first = [color[0], color[1], color[2], 1.];
    std::iter::once(first)
        .chain(std::iter::repeat_n([0., 0., 0., 1.], 8))
        .flatten()
        .flat_map(f32::to_le_bytes)
        .collect()
}

/// The images to fill and the cleared colour; `EnvironmentUpload { .. }.record(..)`.
pub(super) struct EnvironmentUpload<'a> {
    pub lut: &'a EnvironmentImage,
    pub diffuse_sh: &'a EnvironmentImage,
    pub specular: &'a EnvironmentImage,
    pub color: [f32; 3],
}
impl EnvironmentUpload<'_> {
    /// Stage, copy and clear, then publish every image to fragment sampling; the staging
    /// buffers are returned so they outlive the submission. Example: `upload.submit(&runtime)?`.
    pub fn submit(
        &self,
        runtime: &std::sync::Arc<super::super::runtime::VulkanRuntime>,
    ) -> Result<(CommandSubmission, [DeviceBuffer; 2]), RenderError> {
        let lut = staging(runtime, ENV_LUT_TEXELS)?;
        let sh = staging(runtime, &cleared_diffuse_sh(self.color))?;
        let submission = CommandSubmission::new(runtime)?;
        self.record(runtime.device(), submission.command, lut.handle, sh.handle);
        Ok((submission, [lut, sh]))
    }
    fn record(
        &self,
        device: &ash::Device,
        command: vk::CommandBuffer,
        lut: vk::Buffer,
        sh: vk::Buffer,
    ) {
        let images = [self.lut, self.diffuse_sh, self.specular];
        barrier(
            device,
            command,
            &images,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        );
        copy_level0(device, command, lut, self.lut);
        copy_level0(device, command, sh, self.diffuse_sh);
        let clear = vk::ClearColorValue {
            float32: [self.color[0], self.color[1], self.color[2], 1.],
        };
        let range = [self.specular.spec.range()];
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            device.cmd_clear_color_image(
                command,
                self.specular.handle,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &clear,
                &range,
            );
        }
        barrier(
            device,
            command,
            &images,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );
    }
}
fn staging(
    runtime: &std::sync::Arc<super::super::runtime::VulkanRuntime>,
    bytes: &[u8],
) -> Result<DeviceBuffer, RenderError> {
    let buffer = DeviceBuffer::new(
        runtime,
        bytes.len() as u64,
        vk::BufferUsageFlags::TRANSFER_SRC,
        HostBufferAccess::Upload,
    )?;
    buffer.write(bytes)?;
    Ok(buffer)
}
fn copy_level0(
    device: &ash::Device,
    command: vk::CommandBuffer,
    source: vk::Buffer,
    image: &EnvironmentImage,
) {
    let region = vk::BufferImageCopy::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
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
fn barrier(
    device: &ash::Device,
    command: vk::CommandBuffer,
    images: &[&EnvironmentImage],
    old: vk::ImageLayout,
    new: vk::ImageLayout,
) {
    let barriers: Vec<_> = images
        .iter()
        .map(|image| layout_barrier(image, old, new))
        .collect();
    let dependency = vk::DependencyInfo::default().image_memory_barriers(&barriers);
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe { device.cmd_pipeline_barrier2(command, &dependency) };
}
fn layout_barrier(
    image: &EnvironmentImage,
    old: vk::ImageLayout,
    new: vk::ImageLayout,
) -> vk::ImageMemoryBarrier2<'static> {
    let (source_stage, source_access) = layout_access(old);
    let (destination_stage, destination_access) = layout_access(new);
    vk::ImageMemoryBarrier2::default()
        .image(image.handle)
        .subresource_range(image.spec.range())
        .old_layout(old)
        .new_layout(new)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .src_stage_mask(source_stage)
        .src_access_mask(source_access)
        .dst_stage_mask(destination_stage)
        .dst_access_mask(destination_access)
}
fn layout_access(layout: vk::ImageLayout) -> (vk::PipelineStageFlags2, vk::AccessFlags2) {
    match layout {
        vk::ImageLayout::UNDEFINED => (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL => (
            vk::PipelineStageFlags2::FRAGMENT_SHADER,
            vk::AccessFlags2::SHADER_SAMPLED_READ,
        ),
        _ => (
            vk::PipelineStageFlags2::TRANSFER,
            vk::AccessFlags2::TRANSFER_WRITE,
        ),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cleared_sh_keeps_only_the_constant_coefficient() {
        let bytes = cleared_diffuse_sh([1., 1., 1.]);
        let texels: Vec<f32> = bytes
            .chunks(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        assert_eq!(texels.len(), 36);
        assert_eq!(&texels[..4], &[1., 1., 1., 1.]);
        assert!(texels[4..].chunks(4).all(|t| t == [0., 0., 0., 1.]));
    }
    #[test]
    fn env_lut_holds_the_native_table() {
        assert_eq!(ENV_LUT_TEXELS.len(), 128 * 128 * 8);
        let value =
            |i: usize| f32::from_le_bytes(ENV_LUT_TEXELS[4 * i..4 * i + 4].try_into().unwrap());
        // First texels of Textures_EnvLUT: 0, 0, 0.0382714, 0.949235.
        assert_eq!(
            [value(0), value(1), value(2), value(3)],
            [0., 0., 0.0382714, 0.949235]
        );
    }
}
