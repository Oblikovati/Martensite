//! Copy the rendered image entirely on the GPU, preserving sRGB and top-left orientation.
use super::{frame::FrameResources, runtime::VulkanRuntime, window_chain::WindowChain};
use ash::vk;
pub(super) fn record(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    resources: &FrameResources,
    (chain, index): (&WindowChain, u32),
    flip_output: bool,
) {
    let image = chain.images[index as usize];
    transition(runtime, command, image, false);
    let offsets = runtime
        .attachment_coordinates
        .presentation_offsets(resources.extent, chain.config.extent);
    let region = blit_region(flipped_destination(offsets, flip_output));
    record_blit(runtime, command, [resources.color.handle, image], region);
    transition(runtime, command, image, true);
}
/// Swap the destination rows for an upside-down presentation; `flipped_destination(o, true)`.
fn flipped_destination(
    mut offsets: super::attachment_coordinates::PresentationOffsets,
    flip_output: bool,
) -> super::attachment_coordinates::PresentationOffsets {
    if flip_output {
        let [top, bottom] = offsets.destination;
        offsets.destination = [
            ash::vk::Offset3D { y: bottom.y, ..top },
            ash::vk::Offset3D { y: top.y, ..bottom },
        ];
    }
    offsets
}
fn blit_region(offsets: super::attachment_coordinates::PresentationOffsets) -> vk::ImageBlit {
    let layers = vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .layer_count(1);
    vk::ImageBlit::default()
        .src_subresource(layers)
        .dst_subresource(layers)
        .src_offsets(offsets.source)
        .dst_offsets(offsets.destination)
}
fn record_blit(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    images: [vk::Image; 2],
    region: vk::ImageBlit,
) {
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_blit_image(
            command,
            images[0],
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            images[1],
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[region],
            vk::Filter::NEAREST,
        )
    };
}
fn transition(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    image: vk::Image,
    present: bool,
) {
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(1)
        .layer_count(1);
    let barrier = vk::ImageMemoryBarrier2::default()
        .image(image)
        .subresource_range(range)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED);
    let barrier = if present {
        barrier
            .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
    } else {
        barrier
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .dst_stage_mask(vk::PipelineStageFlags2::TRANSFER)
            .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
    };
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_pipeline_barrier2(
            command,
            &vk::DependencyInfo::default().image_memory_barriers(&[barrier]),
        )
    };
}
#[cfg(test)]
mod tests {
    use super::super::attachment_coordinates::AttachmentCoordinates;
    #[test]
    fn native_rows_flip_once_at_window_presentation() {
        let extent = ash::vk::Extent2D {
            width: 31,
            height: 17,
        };
        let offsets = AttachmentCoordinates::LegacyBottomUpV1.presentation_offsets(extent, extent);
        let (source, destination) = (offsets.source, offsets.destination);
        assert_eq!(
            (source[0].x, source[0].y, source[1].x, source[1].y),
            (0, 17, 31, 0)
        );
        assert_eq!((destination[0].y, destination[1].y), (0, 17));
    }
    #[test]
    fn flipped_output_swaps_only_the_destination_rows() {
        let extent = ash::vk::Extent2D {
            width: 32,
            height: 16,
        };
        let plain = AttachmentCoordinates::default().presentation_offsets(extent, extent);
        let flipped = super::flipped_destination(
            AttachmentCoordinates::default().presentation_offsets(extent, extent),
            true,
        );
        assert_eq!(flipped.source, plain.source);
        let rows = |o: [ash::vk::Offset3D; 2]| (o[0].x, o[0].y, o[1].x, o[1].y, o[1].z);
        assert_eq!(rows(plain.destination), (0, 0, 32, 16, 1));
        assert_eq!(rows(flipped.destination), (0, 16, 32, 0, 1));
    }
    #[test]
    fn blit_covers_the_top_left_extent() {
        let extent = ash::vk::Extent2D {
            width: 32,
            height: 16,
        };
        let result = AttachmentCoordinates::default()
            .presentation_offsets(ash::vk::Extent2D::default(), extent)
            .destination;
        assert_eq!(
            (
                result[0].x,
                result[0].y,
                result[1].x,
                result[1].y,
                result[1].z
            ),
            (0, 0, 32, 16, 1)
        );
    }
}
