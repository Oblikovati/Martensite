use crate::{
    FrameCapture, PrimitiveDraw, RenderBackend, RenderFrame, RenderLayer, RenderVertex,
    identity_clip,
    immediate_main::{ImmediateMainCache, ImmediateMainPlan},
};
use std::sync::Arc;

fn triangle(depth: f32, color: [f32; 4], xmax: f32) -> PrimitiveDraw {
    let points = [[-0.9, -0.9, depth], [xmax, -0.9, depth], [-0.9, 0.9, depth]];
    PrimitiveDraw::new(
        points.map(|point| RenderVertex::new(point, color)).to_vec(),
        vec![0, 1, 2],
        identity_clip(),
    )
    .unwrap()
}

fn frame(main: PrimitiveDraw, overlay: Vec<PrimitiveDraw>, clear_depth: bool) -> RenderFrame {
    let layers = vec![
        RenderLayer::new(vec![main], true, true, true),
        RenderLayer::new(overlay, true, true, clear_depth),
    ];
    let mut frame = RenderFrame::with_layers(48, 32, [0., 0., 0., 1.], layers).unwrap();
    frame.immediate_start = Some(1);
    frame
}

fn assert_capture(actual: FrameCapture, reference: FrameCapture) {
    assert_eq!(
        [actual.width, actual.height],
        [reference.width, reference.height]
    );
    assert_eq!(actual.rgba, reference.rgba);
    assert_eq!(actual.depth, reference.depth);
}

#[test]
fn completed_main_images_preserve_color_and_occlusion_for_a_changing_overlay() {
    let mut renderer = crate::backend::resource_lifecycle_test_support::create_resource_renderer();
    assert!(renderer.allows_main_copy());
    let cache = Arc::new(ImmediateMainCache::default());
    let main = triangle(0.2, [1., 0., 0., 1.], -0.1);
    let mut base = frame(main.clone(), vec![], false);
    base.immediate_main = cache.plan(&base);
    assert!(matches!(
        base.immediate_main,
        Some(ImmediateMainPlan::Remember(_))
    ));
    renderer.render(&base).unwrap();

    let overlay = triangle(0.8, [0., 1., 0., 1.], 0.9);
    for depth_clear in [false, true, false] {
        let mut updated = frame(main.clone(), vec![overlay.clone()], depth_clear);
        let full = renderer.render(&updated).unwrap();
        assert!(full.rgba.chunks_exact(4).any(|p| p == [0, 255, 0, 255]));
        assert_eq!(
            full.rgba.chunks_exact(4).any(|p| p == [255, 0, 0, 255]),
            !depth_clear
        );
        updated.immediate_main = cache.plan(&updated);
        assert!(matches!(
            updated.immediate_main,
            Some(ImmediateMainPlan::Restore { start: 1, .. })
        ));
        let prepared = renderer.frame_resources(&updated).unwrap();
        assert_eq!(prepared.first_draw, 1);
        assert_eq!(prepared.draws.len(), 1, "only the overlay is uploaded");
        assert!(prepared.immediate_source.is_some());
        drop(prepared);
        assert_capture(renderer.render(&updated).unwrap(), full);
    }
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn main_camera_color_geometry_and_extent_changes_cannot_reuse_the_base() {
    let mut renderer = crate::backend::resource_lifecycle_test_support::create_resource_renderer();
    let cache = Arc::new(ImmediateMainCache::default());
    let main = triangle(0.2, [1., 0., 0., 1.], -0.1);
    let mut base = frame(main, vec![], false);
    base.immediate_main = cache.plan(&base);
    renderer.render(&base).unwrap();
    assert!(matches!(
        cache.plan(&base),
        Some(ImmediateMainPlan::Restore { .. })
    ));
    let mutations: [fn(&mut RenderFrame); 4] = [
        |frame| frame.draws[0].clip_transform[12] = 0.1,
        |frame| frame.clear_color[2] = 0.8,
        |frame| frame.draws[0].vertices.make_mut()[0].position[0] = -0.5,
        |frame| frame.width += 1,
    ];
    for mutate in mutations {
        let mut changed = base.clone();
        mutate(&mut changed);
        assert!(!matches!(
            cache.plan(&changed),
            Some(ImmediateMainPlan::Restore { .. })
        ));
    }
}

#[test]
fn recycled_main_frame_ticket_falls_back_to_a_complete_redraw() {
    let mut renderer = crate::backend::resource_lifecycle_test_support::create_resource_renderer();
    let cache = Arc::new(ImmediateMainCache::default());
    let main = triangle(0.2, [1., 0., 0., 1.], -0.1);
    let mut seed = frame(main.clone(), vec![], false);
    seed.immediate_main = cache.plan(&seed);
    renderer.render(&seed).unwrap();
    let other_views: Vec<_> = (0..4)
        .map(|_| Arc::new(ImmediateMainCache::default()))
        .collect();
    let mut other = frame(triangle(0.5, [0., 0., 1., 1.], 0.9), vec![], false);
    for view in &other_views {
        other.immediate_main = view.plan(&other);
        renderer.render(&other).unwrap();
    }
    let overlay = triangle(0.8, [0., 1., 0., 1.], 0.9);
    let mut changed = frame(main, vec![overlay], false);
    let full = renderer.render(&changed).unwrap();
    changed.immediate_main = cache.plan(&changed);
    assert!(matches!(
        changed.immediate_main,
        Some(ImmediateMainPlan::Restore { .. })
    ));
    assert!(
        renderer
            .frame_resources(&changed)
            .unwrap()
            .immediate_source
            .is_none()
    );
    assert_capture(renderer.render(&changed).unwrap(), full);
    changed.immediate_main = cache.plan(&changed);
    assert!(
        renderer
            .frame_resources(&changed)
            .unwrap()
            .immediate_source
            .is_some()
    );
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn first_immediate_frame_saves_the_main_prefix_before_drawing_a_visible_overlay() {
    let mut renderer = crate::backend::resource_lifecycle_test_support::create_resource_renderer();
    let cache = Arc::new(ImmediateMainCache::default());
    let main = triangle(0.4, [1., 0., 0., 1.], -0.1);
    let green = triangle(0.1, [0., 1., 0., 1.], 0.9);
    let mut first = frame(main.clone(), vec![green], true);
    let full = renderer.render(&first).unwrap();
    first.immediate_main = cache.plan(&first);
    assert!(
        matches!(&first.immediate_main, Some(ImmediateMainPlan::Remember(seed)) if seed.snapshot_start == Some(1))
    );
    assert_capture(renderer.render(&first).unwrap(), full);
    let blue = triangle(0.7, [0., 0., 1., 1.], 0.9);
    let mut changed = frame(main, vec![blue], false);
    let reference = renderer.render(&changed).unwrap();
    changed.immediate_main = cache.plan(&changed);
    assert!(
        renderer
            .frame_resources(&changed)
            .unwrap()
            .immediate_source
            .is_some()
    );
    assert_capture(renderer.render(&changed).unwrap(), reference);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn an_unobserved_submission_owns_its_source_after_view_invalidation() {
    use super::super::{record, resource_lifecycle_test_support, submission::CommandSubmission};

    let mut renderer = resource_lifecycle_test_support::create_resource_renderer();
    let cache = Arc::new(ImmediateMainCache::default());
    let mut frame = frame(triangle(0.2, [1., 0., 0., 1.], -0.1), vec![], false);
    frame.immediate_main = cache.plan(&frame);
    renderer.render(&frame).unwrap();
    // The owner count below concerns this submission, not the earlier frame in flight.
    renderer.finish_frames().unwrap();
    frame.immediate_main = cache.plan(&frame);
    let resources = renderer.frame_resources(&frame).unwrap();
    let (_, source) = resources.immediate_source.as_ref().unwrap();
    let color = Arc::downgrade(&source.color);
    let depth = Arc::downgrade(&source.depth);
    let commands = CommandSubmission::new(&renderer.runtime).unwrap();
    record::record_frame(&renderer.runtime, commands.command, &resources, &frame);
    cache.discard();
    drop(frame);
    let outcome = resource_lifecycle_test_support::inject_completion_failure(&commands, &[], &[]);
    assert!(
        renderer
            .settle_owned_frame(commands, resources, outcome)
            .is_err()
    );
    assert_eq!(color.strong_count(), 2, "pool and submitted frame");
    assert_eq!(depth.strong_count(), 2, "pool and submitted frame");
    assert!(matches!(
        renderer.close(),
        crate::RenderDeviceCloseOutcome::Closed { .. }
    ));
    assert_eq!(color.strong_count(), 0);
    assert_eq!(depth.strong_count(), 0);
    assert!(renderer.validation_errors().unwrap().is_empty());
}
