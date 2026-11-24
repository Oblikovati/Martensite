//! Native image checks for retained placement frames (#594).
use super::Stage;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Handle,
};
use martensite_tkmath::{gp::Transform3, toploc::Location};
use martensite_tkservice::graphic3d::{
    Graphic3dCView, Graphic3dMaterialAspect, Graphic3dNameOfMaterial, Graphic3dViewAffinity,
};
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject};
use martensite_vulkan::FrameCapture;

pub(super) fn verify_placement_edits(
    stage: &Stage,
    context: &AisInteractiveContext,
    part: &Handle<dyn AisInteractiveObject>,
    occurrence: &Handle<dyn AisInteractiveObject>,
) {
    verify_unrelated_visibility_write(stage);
    verify_placement_culling(stage);
    compare_placement_edit(stage, "transform", || {
        let location = Location::new(Transform3::translation([0., 0., 30.]));
        context.set_location(occurrence, &location).unwrap();
    });
    compare_placement_edit(stage, "material", || {
        let gold = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Gold);
        context.set_material(part, &gold, false).unwrap();
    });
    compare_placement_edit(stage, "color", || {
        context
            .set_color(part, Color::named(NamedColor::RED), false)
            .unwrap();
    });
    verify_placement_affinity(stage, context, occurrence);
    compare_placement_edit(stage, "erase", || context.erase(occurrence, false).unwrap());
}

fn verify_unrelated_visibility_write(stage: &Stage) {
    let view = stage.render_view();
    for angle in 15..=17 {
        stage.redraw_orbit(angle);
    }
    let hits = view.placement_replay_hits();
    assert!(hits > 0, "expected a warm batch");
    let unrelated = Graphic3dViewAffinity::default();
    unrelated.set_visible(false);
    stage.redraw_orbit(18);
    assert_eq!(
        view.placement_replay_hits(),
        hits + 1,
        "an unrelated write invalidated the batch"
    );
}

fn verify_placement_affinity(
    stage: &Stage,
    context: &AisInteractiveContext,
    occurrence: &Handle<dyn AisInteractiveObject>,
) {
    compare_placement_edit(stage, "view affinity", || {
        context
            .set_view_affinity(occurrence, &stage.view, false)
            .unwrap();
    });
    context
        .set_view_affinity(occurrence, &stage.view, true)
        .unwrap();
}

fn verify_placement_culling(stage: &Stage) {
    let view = stage.render_view();
    let camera = stage.view.camera();
    let scale = camera.scale();
    view.enable_fused_replay(true);
    stage.redraw_orbit(16);
    stage.redraw_orbit(17);
    let warm_hits = view.placement_replay_hits();
    assert!(warm_hits > 0);
    let warm = view.capture().unwrap();
    let all_visible = visible_structures(&view);
    camera.set_scale(scale / 2.);
    stage.redraw_orbit(17);
    let cropped_hits = if view.supports_placement_compaction() {
        warm_hits + 1
    } else {
        0
    };
    assert_eq!(
        view.placement_replay_hits(),
        cropped_hits,
        "zoom changed culling"
    );
    let cropped = view.capture().unwrap();
    let cropped_visible = visible_structures(&view);
    assert!((0..all_visible).contains(&cropped_visible));
    verify_restored_frustum(stage, scale, cropped, cropped_visible, &warm, all_visible);
}

fn verify_restored_frustum(
    stage: &Stage,
    scale: f64,
    cropped: FrameCapture,
    cropped_visible: i32,
    warm: &FrameCapture,
    all_visible: i32,
) {
    let view = stage.render_view();
    view.enable_fused_replay(false);
    stage.redraw_orbit(17);
    assert_placement_capture_equal(&cropped, &view.capture().unwrap(), "cropped view");
    assert_eq!(visible_structures(&view), cropped_visible);
    let before_restore = view.placement_replay_hits();
    view.enable_fused_replay(true);
    stage.view.camera().set_scale(scale);
    stage.redraw_orbit(17);
    let restored_hits = if view.supports_placement_compaction() {
        before_restore + 1
    } else {
        0
    };
    assert_eq!(
        view.placement_replay_hits(),
        restored_hits,
        "restored frustum"
    );
    assert_placement_capture_equal(warm, &view.capture().unwrap(), "restored frustum");
    assert_eq!(visible_structures(&view), all_visible);
}

fn visible_structures(view: &martensite_vulkan::Graphic3dRenderView) -> i32 {
    view.layers()
        .iter()
        .map(|layer| layer.nb_structures_not_culled())
        .sum()
}

fn compare_placement_edit(stage: &Stage, edit: &str, apply: impl FnOnce()) {
    let view = stage.render_view();
    view.enable_fused_replay(true);
    // A visibility restore may change culling flags on the first redraw and reseed on the second.
    for angle in 15..=17 {
        stage.redraw_orbit(angle);
    }
    let hits = view.placement_replay_hits();
    assert!(
        hits > 0,
        "{edit}: expected a warm placement frame before the edit"
    );
    let before = view.capture().unwrap();
    view.enable_fused_replay(false);
    stage.redraw_orbit(17);
    assert_placement_capture_equal(&before, &view.capture().unwrap(), edit);
    view.enable_fused_replay(true);
    apply();
    stage.redraw_orbit(17);
    assert_eq!(
        view.placement_replay_hits(),
        0,
        "{edit}: expected the batch to be invalidated"
    );
    let fallback = view.capture().unwrap();
    view.enable_fused_replay(false);
    stage.redraw_orbit(17);
    assert_placement_capture_equal(&fallback, &view.capture().unwrap(), edit);
    assert_ne!(
        before.rgba, fallback.rgba,
        "{edit}: the edit must change the image"
    );
    eprintln!("Retained placement {edit} fell back to the exact ordinary result");
}

fn assert_placement_capture_equal(fallback: &FrameCapture, normal: &FrameCapture, edit: &str) {
    assert_eq!(fallback.rgba.as_slice(), normal.rgba.as_slice(), "{edit}");
    let fallback_depth = fallback.depth.iter().map(|depth| depth.to_bits());
    let normal_depth = normal.depth.iter().map(|depth| depth.to_bits());
    assert!(fallback_depth.eq(normal_depth), "{edit}: depth differed");
}
