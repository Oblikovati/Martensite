//! One immutable convention owns raster rectangles and attachment row interpretation.
use crate::RenderError;
use ash::vk;

pub(super) type AttachmentCoordinates = crate::RenderCoordinateConvention;

pub(super) struct RasterRegionDescriptions {
    pub(super) viewport: vk::Viewport,
    pub(super) scissor: vk::Rect2D,
}

pub(super) struct PresentationOffsets {
    pub(super) source: [vk::Offset3D; 2],
    pub(super) destination: [vk::Offset3D; 2],
}

impl AttachmentCoordinates {
    /// Attachment rows above the logical frame; `policy.leading_rows(409)` is 1 under V2.
    ///
    /// OpenGL forms its 2x2 derivative quads from the bottom window row up, Vulkan from the
    /// top attachment row down. A top-left frame of odd height therefore starts one attachment
    /// row down, so both drivers pair the same rows for `dFdx`/`dFdy`/`fwidth` and helper
    /// invocations. Legacy bottom-up frames already pair like OpenGL.
    pub(super) fn leading_rows(self, height: u32) -> u32 {
        u32::from(self == Self::TopLeftLogicalV2 && height % 2 == 1)
    }

    /// The attachment extent holding a logical frame; `policy.attachment_extent(frame)`.
    pub(super) fn attachment_extent(self, frame: vk::Extent2D) -> vk::Extent2D {
        vk::Extent2D {
            width: frame.width,
            height: frame.height + self.leading_rows(frame.height),
        }
    }

    /// Drop the leading attachment rows of a read-back frame; `policy.logical_rows(bytes, frame)`.
    pub(super) fn logical_rows(self, mut bytes: Vec<u8>, frame: vk::Extent2D) -> Vec<u8> {
        let leading = self.leading_rows(frame.height) as usize * frame.width as usize * 4;
        bytes.drain(..leading.min(bytes.len()));
        bytes
    }

    /// Keep caller-validated top-left regions in one convention; `policy.raster_regions(frame, view, clip)`.
    pub(super) fn raster_regions(
        self,
        frame: vk::Extent2D,
        viewport: vk::Rect2D,
        scissor: vk::Rect2D,
    ) -> RasterRegionDescriptions {
        match self {
            Self::LegacyBottomUpV1 => legacy_regions(frame, viewport, scissor),
            Self::TopLeftLogicalV2 => {
                let leading = self.leading_rows(frame.height) as i32;
                RasterRegionDescriptions {
                    viewport: logical_viewport(shifted(viewport, leading)),
                    scissor: shifted(scissor, leading),
                }
            }
        }
    }

    /// Normalize both color and depth identically; `policy.capture_rows(bytes, width)?`.
    pub(super) fn capture_rows(self, bytes: Vec<u8>, width: u32) -> Result<Vec<u8>, RenderError> {
        match self {
            Self::LegacyBottomUpV1 => {
                super::attachment_orientation::top_left_attachment_rows(bytes, width)
            }
            Self::TopLeftLogicalV2 => {
                super::attachment_orientation::validated_attachment_rows(&bytes, width)?;
                Ok(bytes)
            }
        }
    }

    /// Keep legacy row reversal or candidate forward copies; `policy.presentation_offsets(src, dst)`.
    pub(super) fn presentation_offsets(
        self,
        source: vk::Extent2D,
        destination: vk::Extent2D,
    ) -> PresentationOffsets {
        let source = match self {
            Self::LegacyBottomUpV1 => bottom_up_offsets(source),
            Self::TopLeftLogicalV2 => {
                let leading = self.leading_rows(source.height) as i32;
                let mut rows = offsets(source);
                rows[0].y += leading;
                rows[1].y += leading;
                rows
            }
        };
        let destination = offsets(destination);
        PresentationOffsets {
            source,
            destination,
        }
    }

    /// The attachment rectangle of a logical top-left rectangle; `policy.attachment_rect(frame, rect)`.
    pub(super) fn attachment_rect(self, frame: vk::Extent2D, rect: vk::Rect2D) -> vk::Rect2D {
        match self {
            Self::LegacyBottomUpV1 => bottom_up_region(frame, rect),
            Self::TopLeftLogicalV2 => shifted(rect, self.leading_rows(frame.height) as i32),
        }
    }

    /// Keep facing coupled to the attachment convention; `policy.front_face(mirrored)`.
    pub(super) fn front_face(self, mirrored: bool) -> vk::FrontFace {
        // Reversing attachment rows after rasterization also reverses the screen winding.
        let mirrored = mirrored ^ (self == Self::TopLeftLogicalV2);
        if mirrored {
            vk::FrontFace::COUNTER_CLOCKWISE
        } else {
            vk::FrontFace::CLOCKWISE
        }
    }

    /// Couple static pipeline facing to dynamic facing; `policy.pipeline_front_face(original)`.
    pub(super) fn pipeline_front_face(self, original: vk::FrontFace) -> vk::FrontFace {
        if self == Self::LegacyBottomUpV1 {
            return original;
        }
        // Preserve invalid enumerants so Vulkan validation can reject them, rather than repairing them.
        match original {
            vk::FrontFace::CLOCKWISE => vk::FrontFace::COUNTER_CLOCKWISE,
            vk::FrontFace::COUNTER_CLOCKWISE => vk::FrontFace::CLOCKWISE,
            unknown => unknown,
        }
    }
}

fn legacy_regions(
    frame: vk::Extent2D,
    viewport: vk::Rect2D,
    scissor: vk::Rect2D,
) -> RasterRegionDescriptions {
    let scissor = bottom_up_region(frame, scissor);
    let viewport = native_viewport(bottom_up_region(frame, viewport));
    RasterRegionDescriptions { viewport, scissor }
}

fn logical_viewport(region: vk::Rect2D) -> vk::Viewport {
    let mut viewport = native_viewport(region);
    // Geometry and window/readback mapping must select the same orientation.
    viewport.y += viewport.height;
    viewport.height = -viewport.height;
    viewport
}

fn shifted(mut region: vk::Rect2D, rows: i32) -> vk::Rect2D {
    region.offset.y += rows;
    region
}

fn bottom_up_region(extent: vk::Extent2D, mut region: vk::Rect2D) -> vk::Rect2D {
    // Inputs are internal rectangles within the device-validated frame extent.
    region.offset.y = extent.height as i32 - region.offset.y - region.extent.height as i32;
    region
}

fn native_viewport(region: vk::Rect2D) -> vk::Viewport {
    // Native positive-Y raster arithmetic is observable; capture/presentation flip rows afterward.
    vk::Viewport {
        x: region.offset.x as f32,
        y: region.offset.y as f32,
        width: region.extent.width as f32,
        height: region.extent.height as f32,
        min_depth: 0.,
        max_depth: 1.,
    }
}

fn bottom_up_offsets(extent: vk::Extent2D) -> [vk::Offset3D; 2] {
    let mut result = offsets(extent);
    result[0].y = extent.height as i32;
    result[1].y = 0;
    result
}

fn offsets(extent: vk::Extent2D) -> [vk::Offset3D; 2] {
    [
        vk::Offset3D::default(),
        vk::Offset3D {
            x: extent.width as i32,
            y: extent.height as i32,
            z: 1,
        },
    ]
}

#[cfg(test)]
#[path = "attachment_coordinates_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "attachment_coordinate_candidate_tests.rs"]
mod candidate_tests;
