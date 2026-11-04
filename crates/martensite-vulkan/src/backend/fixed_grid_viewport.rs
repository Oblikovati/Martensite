//! Original projection and expanded raster viewports share the attachment coordinate policy.
use super::{attachment_coordinates::AttachmentCoordinates, runtime::VulkanRuntime};
use ash::vk;

pub(super) fn viewport_pair(
    coordinates: AttachmentCoordinates,
    extent: vk::Extent2D,
) -> [vk::Viewport; 2] {
    let full = vk::Rect2D {
        offset: Default::default(),
        extent,
    };
    let original = coordinates.raster_regions(extent, full, full).viewport;
    expanded_viewports(original, coordinates.attachment_extent(extent))
}

pub(super) fn expanded_viewports(
    original: vk::Viewport,
    attachment: vk::Extent2D,
) -> [vk::Viewport; 2] {
    let full = vk::Rect2D {
        offset: Default::default(),
        extent: attachment,
    };
    let mut raster = AttachmentCoordinates::LegacyBottomUpV1
        .raster_regions(attachment, full, full)
        .viewport;
    raster.min_depth = original.min_depth;
    raster.max_depth = original.max_depth;
    [original, raster]
}

pub(super) fn append_viewports(bytes: &mut Vec<u8>, viewports: [vk::Viewport; 2]) {
    bytes.resize(96, 0);
    for viewport in viewports {
        bytes.extend(
            [viewport.x, viewport.y, viewport.width, viewport.height]
                .into_iter()
                .flat_map(f32::to_ne_bytes),
        );
    }
}

pub(super) fn record_viewport(
    runtime: &VulkanRuntime,
    command: vk::CommandBuffer,
    viewport: vk::Viewport,
    front_face: vk::FrontFace,
) {
    // SAFETY: the command buffer is recording on this thread and every handle it names outlives the submission.
    unsafe {
        runtime.device().cmd_set_front_face(command, front_face);
        runtime.device().cmd_set_viewport(command, 0, &[viewport]);
    }
    #[cfg(feature = "command-state-diagnostics")]
    super::command_state_diagnostic::viewport(command, viewport);
    #[cfg(any(
        feature = "vertex-output-diagnostics",
        feature = "command-state-diagnostics"
    ))]
    super::vertex_pipeline_diagnostic::facing(runtime, command, None, front_face);
}

// The shader reflects synthetic Y for a negative original viewport; model mirroring
// never affects these corners. Vulkan's signed polygon area is negative shoelace.
pub(super) fn coverage_front_face(original: vk::Viewport) -> vk::FrontFace {
    if original.height < 0. {
        vk::FrontFace::COUNTER_CLOCKWISE
    } else {
        vk::FrontFace::CLOCKWISE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_original_viewport_and_full_attachment_share_actual_depth_range() {
        let extent = vk::Extent2D {
            width: 23,
            height: 17,
        };
        for policy in [
            AttachmentCoordinates::LegacyBottomUpV1,
            AttachmentCoordinates::TopLeftLogicalV2,
        ] {
            let [original, raster] = viewport_pair(policy, extent);
            assert_eq!(raster.width, 23.);
            assert_eq!(
                raster.height,
                policy.attachment_extent(extent).height as f32
            );
            assert_eq!(
                (raster.min_depth, raster.max_depth),
                (original.min_depth, original.max_depth)
            );
            assert_eq!(
                original.height < 0.,
                policy == AttachmentCoordinates::TopLeftLogicalV2
            );
        }
    }
    #[test]
    fn synthetic_corner_signed_area_keeps_both_triangles_front_facing() {
        for height in [24f32, -24.] {
            let original = vk::Viewport {
                x: 7.,
                y: 31.,
                width: 32.,
                height,
                min_depth: 0.,
                max_depth: 1.,
            };
            let corners = [[11f32, 13.], [14., 13.], [11., 16.], [14., 16.]].map(|mut p| {
                if original.height < 0. {
                    p[1] = 2. * original.y + original.height - p[1];
                }
                p
            });
            for indices in [[0, 1, 2], [2, 1, 3]] {
                let [a, b, c] = indices.map(|i| corners[i]);
                let area = -0.5 * ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]));
                assert_ne!(area, 0.);
                assert_eq!(
                    coverage_front_face(original),
                    if area > 0. {
                        vk::FrontFace::COUNTER_CLOCKWISE
                    } else {
                        vk::FrontFace::CLOCKWISE
                    }
                );
            }
        }
    }
    #[test]
    fn nonfull_viewport_inside_edge_footprint_remains_inside_full_attachment() {
        let attachment = vk::Extent2D {
            width: 64,
            height: 64,
        };
        for (y, height) in [(8., 24.), (32., -24.)] {
            let original = vk::Viewport {
                x: 16.,
                y,
                width: 32.,
                height,
                min_depth: 0.25,
                max_depth: 0.75,
            };
            let [unchanged, expanded] = expanded_viewports(original, attachment);
            assert_eq!(
                (unchanged.x, unchanged.y, unchanged.width, unchanged.height),
                (16., y, 32., height)
            );
            assert_eq!((expanded.min_depth, expanded.max_depth), (0.25, 0.75));
            let rectangle =
                crate::fixed_grid_point::fixed_grid_point_rectangle([47.75, 16.25], 7.).unwrap();
            assert_eq!(rectangle.minimum, [44, 13]);
            let right_edge = (rectangle.minimum[0] + rectangle.extent as i32) as f32;
            assert!(right_edge > original.x + original.width);
            assert!(right_edge < expanded.x + expanded.width);
            assert_eq!(
                (right_edge - expanded.x) / expanded.width * 2. - 1.,
                0.59375
            );
        }
    }
    #[test]
    fn extended_push_preserves_fixed_marker_prefix_and_exact_viewport_offsets() {
        let first = vk::Viewport {
            x: 7.,
            y: 31.,
            width: 23.,
            height: -17.,
            min_depth: 0.25,
            max_depth: 0.75,
        };
        let second = vk::Viewport {
            x: 0.,
            y: 0.,
            width: 64.,
            height: 64.,
            ..first
        };
        let prefix = vec![0x71; 84];
        let mut bytes = prefix.clone();
        append_viewports(&mut bytes, [first, second]);
        assert_eq!(&bytes[..84], prefix);
        assert_eq!(bytes.len(), 128);
        let expected: Vec<u8> = [7f32, 31., 23., -17., 0., 0., 64., 64.]
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect();
        assert_eq!(&bytes[96..], expected);
    }
}
