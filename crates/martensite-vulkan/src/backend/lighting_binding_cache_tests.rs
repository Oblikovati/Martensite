use super::{
    LightingBindingCache, MAX_BINDINGS, StorageDescriptorLayout, optional_allocation_refusal,
};
use crate::backend::{
    host_upload_pages::{FrameStoragePool, FrameUploadPages},
    resource_lifecycle_test_support::create_resource_renderer,
};
use crate::{
    PrimitiveDraw, RenderBackend, RenderError, RenderFrame, RenderLighting, RenderMaterial,
    RenderMaterialCommon, RenderShading, RenderVertex, identity_clip,
};
use ash::vk;
use std::sync::Arc;

fn emission(color: [f32; 4]) -> RenderLighting {
    let material = RenderMaterial {
        common: [RenderMaterialCommon {
            emission: color,
            ..RenderMaterialCommon::default()
        }; 2],
        ..Default::default()
    };
    RenderLighting::new(
        vec![[0., 0., 1.]; 4],
        material,
        identity_clip(),
        identity_clip(),
        false,
        RenderShading::Phong,
    )
    .unwrap()
}

fn lit_rectangle(extent: [f32; 2], color: [f32; 4]) -> PrimitiveDraw {
    let [left, right] = extent;
    let vertices = [
        [left, -1., 0.],
        [right, -1., 0.],
        [right, 1., 0.],
        [left, 1., 0.],
    ]
    .map(|position| RenderVertex::new(position, [1.; 4]))
    .to_vec();
    PrimitiveDraw::new(vertices, vec![0, 1, 2, 0, 2, 3], identity_clip())
        .unwrap()
        .with_lighting(emission(color))
        .unwrap()
}

#[test]
fn identical_stock_lighting_shares_one_set_then_survives_the_frame() {
    let mut renderer = create_resource_renderer();
    let red = [1., 0., 0., 1.];
    let draws = vec![lit_rectangle([-1., 0.], red), lit_rectangle([0., 1.], red)];
    let mut frame = RenderFrame::new(4, 4, [0., 0., 1., 1.], draws).unwrap();
    let counters = renderer.runtime.work.clone();
    let first = renderer.render(&frame).unwrap();
    let before = counters.totals();
    let second = renderer.render(&frame).unwrap();
    assert_eq!(
        counters.totals().since(&before).descriptor_sets_allocated,
        0
    );
    assert_eq!(first.rgba, [255, 0, 0, 255].repeat(16));
    assert_eq!(first.rgba, second.rgba);
    assert_eq!(first.depth, second.depth);
    frame.draws[1].lighting = Some(emission([0., 1., 0., 1.]));
    let before = counters.totals();
    let edited = renderer.render(&frame).unwrap();
    assert_eq!(
        counters.totals().since(&before).descriptor_sets_allocated,
        1
    );
    assert_eq!(edited.pixel(0, 0), Some([255, 0, 0, 255]));
    assert_eq!(edited.pixel(3, 0), Some([0, 255, 0, 255]));
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn bit_differences_and_layout_stages_do_not_alias() {
    let renderer = create_resource_renderer();
    let runtime = &renderer.runtime;
    let pool = FrameStoragePool::default();
    let cache = LightingBindingCache::default();
    let stages = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;
    let both = StorageDescriptorLayout::new(runtime, stages).unwrap();
    let equivalent = StorageDescriptorLayout::new(runtime, stages).unwrap();
    let fragment = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::FRAGMENT).unwrap();
    let mut positive = emission([1., 0., 0., 1.]);
    positive.eye[0] = 0.;
    let mut frame = FrameUploadPages::storage(&pool);
    let first = cache
        .bind_draw(runtime, &equivalent, &positive, &mut frame, true)
        .unwrap();
    let same = cache
        .bind_draw(runtime, &both, &positive, &mut frame, true)
        .unwrap();
    assert!(Arc::ptr_eq(&first, &same));
    assert_different_bindings(&cache, runtime, &pool, &both, &fragment, &positive, &first);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

fn assert_different_bindings(
    cache: &LightingBindingCache,
    runtime: &Arc<super::VulkanRuntime>,
    pool: &FrameStoragePool,
    layout: &StorageDescriptorLayout,
    other_layout: &StorageDescriptorLayout,
    positive: &RenderLighting,
    first: &Arc<super::UploadedStorage>,
) {
    let mut negative = positive.clone();
    negative.eye[0] = -0.;
    let other_bits = cache
        .bind_draw(
            runtime,
            layout,
            &negative,
            &mut FrameUploadPages::storage(pool),
            true,
        )
        .unwrap();
    let other_stage = cache
        .bind_draw(
            runtime,
            other_layout,
            positive,
            &mut FrameUploadPages::storage(pool),
            true,
        )
        .unwrap();
    assert_ne!(first.set, other_bits.set);
    assert_ne!(first.set, other_stage.set);
}

#[test]
fn eviction_preserves_the_owning_draws_binding() {
    let renderer = create_resource_renderer();
    let runtime = &renderer.runtime;
    let pool = FrameStoragePool::default();
    let cache = LightingBindingCache::default();
    let layout = StorageDescriptorLayout::new(runtime, vk::ShaderStageFlags::FRAGMENT).unwrap();
    let binding = |index: usize| {
        let mut lighting = emission([1., 0., 0., 1.]);
        lighting.eye[0] = index as f32;
        cache
            .bind_draw(
                runtime,
                &layout,
                &lighting,
                &mut FrameUploadPages::storage(&pool),
                true,
            )
            .unwrap()
    };
    let pinned = binding(0);
    for index in 1..=MAX_BINDINGS {
        binding(index);
    }
    assert_eq!(cache.bindings.lock().unwrap().len(), MAX_BINDINGS);
    assert_ne!(pinned.set, binding(0).set);
    assert_eq!(Arc::strong_count(&pinned), 1);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn different_lighting_uses_one_frame_page_without_aliasing_draws() {
    let mut renderer = create_resource_renderer();
    let mut draws = vec![
        lit_rectangle([-1., -0.5], [1., 0., 0., 1.]),
        lit_rectangle([-0.5, 0.], [0., 1., 0., 1.]),
        lit_rectangle([0., 0.5], [0., 0., 1., 1.]),
        lit_rectangle([0.5, 1.], [1., 1., 0., 1.]),
    ];
    draws[1].lighting.as_mut().unwrap().world[12] = 2.;
    let frame = RenderFrame::new(4, 4, [0., 0., 0., 1.], draws).unwrap();
    let counters = renderer.runtime.work.clone();
    let cold = renderer.render(&frame).unwrap();
    let before = counters.totals();
    let capture = renderer.render(&frame).unwrap();
    // The first draw's retained set is warm; the other draws share one aligned range binding.
    assert_eq!(
        counters.totals().since(&before).descriptor_sets_allocated,
        1
    );
    assert_eq!(capture.rgba, cold.rgba);
    assert_eq!(capture.depth, cold.depth);
    assert_eq!(capture.pixel(0, 0), Some([255, 0, 0, 255]));
    assert_eq!(capture.pixel(1, 0), Some([0, 255, 0, 255]));
    assert_eq!(capture.pixel(2, 0), Some([0, 0, 255, 255]));
    assert_eq!(capture.pixel(3, 0), Some([255, 255, 0, 255]));
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn optional_storage_refusal_does_not_hide_device_loss() {
    let error = |code: vk::Result| {
        RenderError::device_backend(
            "cache storage",
            code.as_raw(),
            "injected device result".into(),
        )
    };
    assert!(optional_allocation_refusal(&error(
        vk::Result::ERROR_OUT_OF_HOST_MEMORY
    )));
    assert!(optional_allocation_refusal(&error(
        vk::Result::ERROR_TOO_MANY_OBJECTS
    )));
    assert!(!optional_allocation_refusal(&error(
        vk::Result::ERROR_DEVICE_LOST
    )));
}
