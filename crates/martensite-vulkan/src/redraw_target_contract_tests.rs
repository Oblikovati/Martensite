//! Public FBO bindings must be part of fresh-completion identity.
use super::super::view_binding_tests::binding_fixture_for_policy;
use super::*;

#[test]
fn equal_extent_framebuffer_swap_cannot_certify_the_old_cursor() {
    let (view, _) = binding_fixture_for_policy(crate::RenderGuiSurfacePolicy::LegacyV1);
    let api = public_view(&view);
    let first = api.fbo_create(1, 1);
    let second = api.fbo_create(1, 1);
    api.set_fbo(&first);
    let before = view.redraw_cursor().unwrap();
    api.set_fbo(&second);
    view.try_redraw().unwrap();
    assert!(matches!(
        view.check_redraw_since(&before),
        Err(RenderRedrawFailure::TargetChanged {
            binding_changed: true,
            ..
        })
    ));
}

fn public_view(view: &Handle<Graphic3dRenderView>) -> Handle<dyn Graphic3dCView> {
    let interface = view.cast::<dyn Graphic3dCView>();
    assert!(!interface.is_null());
    interface
}

#[test]
fn unchanged_public_framebuffer_binding_completes_without_copying_pixels() {
    let (view, _) = binding_fixture_for_policy(crate::RenderGuiSurfacePolicy::LegacyV1);
    let api = public_view(&view);
    let target = api.fbo_create(1, 1);
    api.set_fbo(&target);
    let before = view.redraw_cursor().unwrap();
    view.try_redraw().unwrap();
    let pixels = capture_pixel_address(&view);
    let after = view.check_redraw_since(&before).unwrap();
    assert_eq!(after.publication(), before.publication() + 1);
    assert_eq!(capture_pixel_address(&view), pixels);
}

fn capture_pixel_address(view: &Graphic3dRenderView) -> *const u8 {
    let capture = view.outcome.read();
    let pixels = &capture.as_ref().unwrap().as_ref().unwrap().rgba;
    pixels.as_ptr()
}

#[test]
fn public_window_framebuffer_transitions_and_framebuffer_aba_are_refused() {
    for selection in [0, 1, 2] {
        assert_public_round_trip_refused(selection);
    }
}

fn assert_public_round_trip_refused(selection: u8) {
    let (view, _) = binding_fixture_for_policy(crate::RenderGuiSurfacePolicy::LegacyV1);
    let api = public_view(&view);
    let (first, second) = selected_public_targets(&*api, selection);
    api.set_fbo(&first);
    let before = view.redraw_cursor().unwrap();
    api.set_fbo(&second);
    if selection == 2 {
        api.set_fbo(&first);
    }
    view.try_redraw().unwrap();
    assert!(matches!(
        view.check_redraw_since(&before),
        Err(RenderRedrawFailure::TargetChanged {
            binding_changed: true,
            ..
        })
    ));
}

fn selected_public_targets(
    api: &dyn Graphic3dCView,
    selection: u8,
) -> (Handle<StandardTransient>, Handle<StandardTransient>) {
    let target = |window| {
        if window {
            Handle::default()
        } else {
            api.fbo_create(1, 1)
        }
    };
    (target(selection == 0), target(selection == 1))
}

#[test]
fn public_viewport_round_trip_retains_a_new_revision_at_the_same_extent() {
    let (view, _) = binding_fixture_for_policy(crate::RenderGuiSurfacePolicy::LegacyV1);
    let api = public_view(&view);
    let target = api.fbo_create(2, 2);
    api.fbo_change_viewport(&target, 1, 1);
    api.set_fbo(&target);
    let before = view.redraw_cursor().unwrap();
    api.fbo_change_viewport(&target, 2, 2);
    api.fbo_change_viewport(&target, 1, 1);
    view.try_redraw().unwrap();
    assert!(matches!(
        view.check_redraw_since(&before),
        Err(RenderRedrawFailure::TargetChanged {
            binding_changed: true,
            ..
        })
    ));
}

#[test]
fn cursor_retains_the_replaced_actual_framebuffer_owner() {
    let (view, _) = binding_fixture_for_policy(crate::RenderGuiSurfacePolicy::LegacyV1);
    let api = public_view(&view);
    let first = api.fbo_create(1, 1);
    let weak = first.downgrade();
    api.set_fbo(&first);
    let before = view.redraw_cursor().unwrap();
    api.set_fbo(&api.fbo_create(1, 1));
    drop(first);
    assert!(!weak.upgrade().is_null());
    drop(before);
    assert!(weak.upgrade().is_null());
}

struct RebindBeforeSubmission {
    replacement: Handle<StandardTransient>,
    cursor_during_redraw: SharedState<Option<RenderRedrawCursor>>,
}
impl crate::RenderViewFrameHooks for RebindBeforeSubmission {
    fn redraw(
        &self,
        view: &Graphic3dRenderView,
        step: &mut crate::RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        view.set_fbo(&self.replacement);
        *self.cursor_during_redraw.write() = Some(view.redraw_cursor().unwrap());
        view.redraw_base(step)
    }
}

#[test]
fn publication_retains_the_attempt_target_instead_of_only_the_current_target() {
    let (view, _) = binding_fixture_for_policy(crate::RenderGuiSurfacePolicy::LegacyV1);
    let api = public_view(&view);
    api.set_fbo(&api.fbo_create(1, 1));
    let hook = Arc::new(RebindBeforeSubmission {
        replacement: api.fbo_create(1, 1),
        cursor_during_redraw: SharedState::default(),
    });
    view.set_frame_hooks(Some(hook.clone()));
    view.try_redraw().unwrap();
    view.set_frame_hooks(None);
    let cursor = hook.cursor_during_redraw.write().take().unwrap();
    assert!(matches!(
        view.check_redraw_since(&cursor),
        Err(RenderRedrawFailure::TargetChanged {
            binding_changed: true,
            ..
        })
    ));
}

#[test]
fn missing_publication_target_is_a_typed_refusal() {
    let (view, _) = binding_fixture_for_policy(crate::RenderGuiSurfacePolicy::LegacyV1);
    let before = view.redraw_cursor().unwrap();
    view.try_redraw().unwrap();
    let retired = view.redraw_target.write().take();
    drop(retired);
    assert!(matches!(
        view.check_redraw_since(&before),
        Err(RenderRedrawFailure::MissingTargetStamp { .. })
    ));
}

#[test]
fn actual_published_output_extent_must_match_the_cursor_target() {
    let (view, window) = binding_fixture_for_policy(crate::RenderGuiSurfacePolicy::LegacyV1);
    window.set_size(2, 1);
    let before = view.redraw_cursor().unwrap();
    view.try_redraw().unwrap();
    assert!(matches!(
        view.check_redraw_since(&before),
        Err(RenderRedrawFailure::TargetChanged {
            accepted: [2, 1],
            observed: [1, 1],
            binding_changed: false
        })
    ));
}
