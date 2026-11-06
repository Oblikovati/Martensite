//! Subview colour copies into a presented frame (`GlView::blitSubviews`).
//!
//! Native copies every active subview into the parent's draw framebuffer before the swap, so
//! the window shows them. A presented Vulkan frame stages the same clipped subview rows
//! (`RenderFrame::subview_blits`) and copies them into its colour attachment after the
//! read-back copy and before the window blit; the bytes are the subview's captured sRGB texels,
//! so the copy is exact, like `glBlitFramebuffer` between equal formats.
use super::{buffer::DeviceBuffer, memory_selection::HostBufferAccess, runtime::VulkanRuntime};
use crate::{RenderError, RenderFrame, subview_composition::RenderSubviewBlit};
use ash::vk;
use std::sync::Arc;

/// The staged subview rows of one presented frame and their attachment regions.
pub(super) struct SubviewUpload {
    buffer: DeviceBuffer,
    regions: Vec<vk::BufferImageCopy>,
}

impl SubviewUpload {
    /// Stage `frame`'s subview copies in attachment row order, or `None` without subviews.
    /// Example: `SubviewUpload::new(&runtime, &frame)?`.
    pub fn new(
        runtime: &Arc<VulkanRuntime>,
        frame: &RenderFrame,
    ) -> Result<Option<Self>, RenderError> {
        if frame.subview_blits.is_empty() {
            return Ok(None);
        }
        let extent = vk::Extent2D {
            width: frame.width,
            height: frame.height,
        };
        let (bytes, regions) = staged_rows(runtime, extent, &frame.subview_blits)?;
        let usage = vk::BufferUsageFlags::TRANSFER_SRC;
        let buffer =
            DeviceBuffer::new(runtime, bytes.len() as u64, usage, HostBufferAccess::Upload)?;
        buffer.write(&bytes)?;
        Ok(Some(Self { buffer, regions }))
    }

    /// Copy the staged rows into `color`, which the read-back left in `TRANSFER_SRC_OPTIMAL`,
    /// and return it there for the window blit; `upload.record(&runtime, command, color)`.
    pub fn record(&self, runtime: &VulkanRuntime, command: vk::CommandBuffer, color: vk::Image) {
        let device = runtime.device();
        let to_destination = barrier(color, vk::ImageLayout::TRANSFER_SRC_OPTIMAL);
        let to_source = barrier(color, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
        let destination = vk::ImageLayout::TRANSFER_DST_OPTIMAL;
        // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
        unsafe {
            device.cmd_pipeline_barrier2(command, &dependency(&to_destination));
            device.cmd_copy_buffer_to_image(
                command,
                self.buffer.handle,
                color,
                destination,
                &self.regions,
            );
            device.cmd_pipeline_barrier2(command, &dependency(&to_source));
        }
    }
}

/// The subview rows in attachment order and one copy region per subview.
fn staged_rows(
    runtime: &VulkanRuntime,
    extent: vk::Extent2D,
    blits: &[RenderSubviewBlit],
) -> Result<(Vec<u8>, Vec<vk::BufferImageCopy>), RenderError> {
    let coordinates = runtime.attachment_coordinates;
    let mut bytes = Vec::new();
    let mut regions = Vec::with_capacity(blits.len());
    for blit in blits {
        let logical = vk::Rect2D {
            offset: vk::Offset2D {
                x: blit.offset[0] as i32,
                y: blit.offset[1] as i32,
            },
            extent: vk::Extent2D {
                width: blit.extent[0],
                height: blit.extent[1],
            },
        };
        let rect = coordinates.attachment_rect(extent, logical);
        regions.push(copy_region(bytes.len() as u64, rect));
        // Reversing the rows is its own inverse: capture order and attachment order swap alike.
        bytes.extend(coordinates.capture_rows(blit.rgba.clone(), blit.extent[0])?);
    }
    Ok((bytes, regions))
}

/// A tightly packed copy of `rect` from `offset` in the staging buffer.
fn copy_region(offset: u64, rect: vk::Rect2D) -> vk::BufferImageCopy {
    let layers = vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .layer_count(1);
    vk::BufferImageCopy::default()
        .buffer_offset(offset)
        .image_subresource(layers)
        .image_offset(vk::Offset3D {
            x: rect.offset.x,
            y: rect.offset.y,
            z: 0,
        })
        .image_extent(vk::Extent3D {
            width: rect.extent.width,
            height: rect.extent.height,
            depth: 1,
        })
}

/// The transition out of `from` (one transfer layout) into the other transfer layout.
fn barrier(image: vk::Image, from: vk::ImageLayout) -> vk::ImageMemoryBarrier2<'static> {
    let range = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(1)
        .layer_count(1);
    let to_destination = from == vk::ImageLayout::TRANSFER_SRC_OPTIMAL;
    let (to, src_access, dst_access) = if to_destination {
        let read = vk::AccessFlags2::TRANSFER_READ;
        (
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            read,
            vk::AccessFlags2::TRANSFER_WRITE,
        )
    } else {
        let write = vk::AccessFlags2::TRANSFER_WRITE;
        (
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            write,
            vk::AccessFlags2::TRANSFER_READ,
        )
    };
    vk::ImageMemoryBarrier2::default()
        .image(image)
        .subresource_range(range)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .old_layout(from)
        .new_layout(to)
        .src_stage_mask(vk::PipelineStageFlags2::TRANSFER)
        .src_access_mask(src_access)
        .dst_stage_mask(vk::PipelineStageFlags2::TRANSFER)
        .dst_access_mask(dst_access)
}

fn dependency<'a>(barrier: &'a vk::ImageMemoryBarrier2<'a>) -> vk::DependencyInfo<'a> {
    vk::DependencyInfo::default().image_memory_barriers(std::slice::from_ref(barrier))
}

#[cfg(test)]
mod tests {
    use super::super::attachment_coordinates::AttachmentCoordinates;
    use ash::vk;
    fn rect(x: i32, y: i32, width: u32, height: u32) -> vk::Rect2D {
        vk::Rect2D {
            offset: vk::Offset2D { x, y },
            extent: vk::Extent2D { width, height },
        }
    }
    #[test]
    fn legacy_rows_place_the_copy_from_the_bottom() {
        let frame = vk::Extent2D {
            width: 160,
            height: 120,
        };
        let placed =
            AttachmentCoordinates::LegacyBottomUpV1.attachment_rect(frame, rect(80, 2, 78, 58));
        assert_eq!(placed, rect(80, 60, 78, 58));
    }
    #[test]
    fn top_left_rows_shift_by_the_leading_row_of_an_odd_frame() {
        let odd = vk::Extent2D {
            width: 160,
            height: 121,
        };
        let even = vk::Extent2D {
            width: 160,
            height: 120,
        };
        let coordinates = AttachmentCoordinates::TopLeftLogicalV2;
        assert_eq!(
            coordinates.attachment_rect(odd, rect(4, 7, 9, 3)),
            rect(4, 8, 9, 3)
        );
        assert_eq!(
            coordinates.attachment_rect(even, rect(4, 7, 9, 3)),
            rect(4, 7, 9, 3)
        );
    }
}
