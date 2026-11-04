//! Mirrored sample positions on the device: the top-left frame samples where the bottom-up
//! (OpenGL-oriented) frame samples, and the standard pattern in the top-left frame does not.
use super::*;
use crate::{PrimitiveDraw, RenderCoordinateConvention, RenderRasterPolicy, RenderVertex};
use ash::vk;

fn validated() -> VulkanOptions {
    VulkanOptions {
        validation: true,
        device_index: Some(0),
    }
}

/// Two slanted triangles off every dyadic tie, drawn with `samples` samples per pixel.
fn slanted_frame(samples: i32) -> RenderFrame {
    let triangles = [
        [[-0.83, -0.71, 0.3], [0.67, -0.29, 0.3], [-0.11, 0.87, 0.3]],
        [[0.13, -0.91, 0.6], [0.89, 0.37, 0.6], [-0.57, 0.19, 0.6]],
    ];
    let colors = [[1., 0.2, 0., 1.], [0., 0.8, 0.4, 1.]];
    let draws = triangles
        .iter()
        .zip(colors)
        .map(|(points, color)| {
            let vertices = points.map(|p| RenderVertex::new(p, color)).to_vec();
            PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap()
        })
        .collect();
    let mut frame = RenderFrame::new(32, 32, [0., 0., 0.3, 1.], draws).unwrap();
    frame.raster.requested_samples = samples;
    frame
}

fn rendered(mut renderer: VulkanRenderer, samples: i32) -> Vec<u8> {
    let capture = renderer.render(&slanted_frame(samples)).unwrap();
    let errors = renderer.validation_errors().unwrap();
    assert!(errors.is_empty(), "validation: {errors:?}");
    capture.rgba
}

fn differing_pixels(left: &[u8], right: &[u8]) -> usize {
    let left = left.as_chunks::<4>().0;
    let right = right.as_chunks::<4>().0;
    left.iter().zip(right).filter(|(l, r)| l != r).count()
}

#[test]
fn top_left_frames_sample_at_the_bottom_up_positions_with_every_mirrored_count() {
    let probe = VulkanRenderer::new(validated()).unwrap();
    let counts = probe.runtime.mirrored_samples;
    // Both qualified drivers (lavapipe, RADV) expose VK_EXT_sample_locations with 4 samples.
    assert!(counts.contains(vk::SampleCountFlags::TYPE_4), "{counts:?}");
    drop(probe);
    for samples in [2, 4, 8] {
        if !counts.contains(vk::SampleCountFlags::from_raw(samples as u32)) {
            continue;
        }
        let mirrored = rendered(VulkanRenderer::new(validated()).unwrap(), samples);
        let bottom_up = VulkanRenderer::new_with_conventions(
            validated(),
            RenderRasterPolicy::default(),
            RenderCoordinateConvention::LegacyBottomUpV1,
        )
        .unwrap();
        let bottom_up = rendered(bottom_up, samples);
        assert_eq!(
            differing_pixels(&mirrored, &bottom_up),
            0,
            "{samples} samples"
        );
    }
}

/// Rejecting control: the standard pattern in the top-left frame misses the bottom-up samples,
/// so the comparison above detects a frame that does not mirror.
#[test]
fn standard_positions_in_the_top_left_frame_differ_from_the_bottom_up_frame() {
    let standard = VulkanRenderer::new_with_standard_samples(validated()).unwrap();
    assert!(standard.runtime.mirrored_samples.is_empty());
    let standard = rendered(standard, 4);
    let bottom_up = VulkanRenderer::new_with_conventions(
        validated(),
        RenderRasterPolicy::default(),
        RenderCoordinateConvention::LegacyBottomUpV1,
    )
    .unwrap();
    assert!(bottom_up.runtime.mirrored_samples.is_empty());
    let bottom_up = rendered(bottom_up, 4);
    assert!(differing_pixels(&standard, &bottom_up) > 0);
    // Single-sample frames place no locations and agree in both frames.
    let single_top_left = rendered(VulkanRenderer::new(validated()).unwrap(), 0);
    let single_bottom_up = VulkanRenderer::new_with_conventions(
        validated(),
        RenderRasterPolicy::default(),
        RenderCoordinateConvention::LegacyBottomUpV1,
    )
    .unwrap();
    let single_bottom_up = rendered(single_bottom_up, 0);
    assert_eq!(differing_pixels(&single_top_left, &single_bottom_up), 0);
}
