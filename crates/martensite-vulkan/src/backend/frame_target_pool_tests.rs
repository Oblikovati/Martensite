//! Exercise retained color, depth and transparency attachments on the actual Vulkan device.
use crate::{RenderBackend, RenderFrame, RenderTransparency};
use ash::vk;
use std::sync::Arc;

fn frame(extent: [u32; 2], clear: [f32; 4]) -> RenderFrame {
    RenderFrame::new(extent[0], extent[1], clear, Vec::new()).unwrap()
}

#[test]
fn target_history_bounds_retained_generations_and_allocation_bytes() {
    use super::{RETAINED_BYTES_PER_FAMILY, RETAINED_GENERATIONS, TargetSlot};
    let mut slot = TargetSlot::<usize, u64>::default();
    let pinned = slot.retain(0, Arc::new(0), 1);
    for key in 1..=RETAINED_GENERATIONS {
        slot.retain(key, Arc::new(0), 1);
    }
    assert_eq!(slot.generations.len(), RETAINED_GENERATIONS);
    assert_eq!(Arc::strong_count(&pinned), 1);
    let large = RETAINED_BYTES_PER_FAMILY + 1;
    slot.retain(10, Arc::new(0), large);
    assert_eq!(slot.generations.len(), 1);
    assert_eq!(slot.bytes, large);
    slot.retain(11, Arc::new(0), 1);
    assert_eq!(slot.generations.len(), 1);
    assert_eq!(slot.bytes, 1);
}

#[test]
fn warmed_attachments_clear_each_frame_and_resize_once() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    let first = frame([10, 8], [1., 0., 0., 1.]);
    let second = frame([10, 8], [0., 1., 0., 1.]);
    let before = renderer.work_ledger().totals();
    let red = renderer.render(&first).unwrap();
    let initial = renderer.work_ledger().totals();
    assert_eq!(initial.since(&before).images_created, 2);
    let green = renderer.render(&second).unwrap();
    let warmed = renderer.work_ledger().totals();
    assert_eq!(warmed.since(&initial).images_created, 0);
    assert_eq!(red.pixel(5, 4), Some([255, 0, 0, 255]));
    assert_eq!(green.pixel(5, 4), Some([0, 255, 0, 255]));
    assert_eq!(green.depth, red.depth);
    assert_resizes_once(&mut renderer, &warmed);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

fn assert_resizes_once(
    renderer: &mut super::super::VulkanRenderer,
    before_resize: &crate::RenderDeviceWork,
) {
    let resized = frame([20, 16], [0., 0., 1., 1.]);
    let capture = renderer.render(&resized).unwrap();
    let first = renderer.work_ledger().totals();
    assert_eq!(first.since(before_resize).images_created, 2);
    assert_eq!([capture.width, capture.height], [20, 16]);
    let repeated = renderer.render(&resized).unwrap();
    assert_eq!(
        renderer.work_ledger().totals().since(&first).images_created,
        0
    );
    assert_eq!(repeated.rgba, capture.rgba);
}

#[test]
fn an_owned_frame_pins_the_exact_attachment_generation() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    let first = frame([12, 10], [1., 0., 0., 1.]);
    let second = frame([12, 10], [0., 1., 0., 1.]);
    let held = renderer.completed_owned_frame(&first).unwrap();
    let replacement = renderer.completed_owned_frame(&second).unwrap();
    assert_ne!(held.color.handle, replacement.color.handle);
    assert_ne!(held.depth.handle, replacement.depth.handle);
    assert_eq!(held.capture().unwrap().pixel(5, 4), Some([255, 0, 0, 255]));
    let expected = replacement.color.handle;
    drop(replacement);
    let warmed = renderer.completed_owned_frame(&second).unwrap();
    assert_eq!(warmed.color.handle, expected);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn supported_multisample_and_weighted_images_follow_effective_attachment_shapes() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    let single = frame([12, 10], [0., 0., 1., 1.]);
    let mut multisampled = single.clone();
    multisampled.raster.requested_samples = 4;
    let selected_samples =
        super::super::frame_raster_plan::frame_raster_plan(&renderer.runtime, &multisampled, false)
            .samples;
    let single_resources = renderer.completed_owned_frame(&single).unwrap();
    let resolve_color = single_resources.color.handle;
    drop(single_resources);
    let first = renderer.completed_owned_frame(&multisampled).unwrap();
    assert_eq!(first.color.handle, resolve_color);
    let attachments = first
        .multisample
        .as_ref()
        .map(|targets| [targets.color.handle, targets.depth.handle]);
    assert_eq!(
        attachments.is_some(),
        selected_samples != vk::SampleCountFlags::TYPE_1
    );
    drop(first);
    let second = renderer.completed_owned_frame(&multisampled).unwrap();
    assert_eq!(
        second
            .multisample
            .as_ref()
            .map(|targets| [targets.color.handle, targets.depth.handle]),
        attachments
    );
    drop(second);
    assert_weighted_scalar_reuse(&mut renderer, single);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

fn assert_weighted_scalar_reuse(renderer: &mut super::super::VulkanRenderer, frame: RenderFrame) {
    let first = frame
        .clone()
        .with_transparency(RenderTransparency::Weighted { depth_factor: 0. })
        .unwrap();
    let second = frame
        .with_transparency(RenderTransparency::Weighted { depth_factor: 1. })
        .unwrap();
    let mode =
        super::super::frame_raster_plan::frame_raster_plan(&renderer.runtime, &second, false)
            .transparency;
    let first = renderer.completed_owned_frame(&first).unwrap();
    let handles = weighted_images(&first);
    assert_eq!(
        handles.is_some(),
        matches!(mode, RenderTransparency::Weighted { .. })
    );
    drop(first);
    let second = renderer.completed_owned_frame(&second).unwrap();
    assert_eq!(weighted_images(&second), handles);
    assert_eq!(
        second.capture().unwrap().pixel(5, 4),
        Some([0, 0, 255, 255])
    );
}

fn weighted_images(frame: &super::super::frame::FrameResources) -> Option<[vk::Image; 2]> {
    let targets = frame.oit.as_ref()?.targets.as_ref();
    match targets {
        super::super::oit_targets::OitTargets::Weighted {
            accumulation,
            coverage,
        } => Some([accumulation.handle, coverage.handle]),
        super::super::oit_targets::OitTargets::Peeling { .. } => None,
    }
}
