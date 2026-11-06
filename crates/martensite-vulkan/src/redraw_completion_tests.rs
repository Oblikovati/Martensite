use super::*;

#[test]
fn cursor_serial_requires_exactly_one_new_unexhausted_publication() {
    assert!(require_fresh_serial(7, 8).is_ok());
    assert_eq!(
        require_fresh_serial(7, 7),
        Err(RenderRedrawFailure::NoFreshPublication {
            before: 7,
            observed: 7
        })
    );
    assert_eq!(
        require_fresh_serial(7, 9),
        Err(RenderRedrawFailure::UnexpectedPublications {
            before: 7,
            observed: 9
        })
    );
    assert!(matches!(
        require_fresh_serial(7, u64::MAX),
        Err(RenderRedrawFailure::SequenceExhausted { .. })
    ));
}

#[test]
fn outcome_validation_keeps_the_actual_failure_and_refuses_absence() {
    let cause = RenderError::rejected("owned fault", "successful frame");
    let failure = require_completed_outcome(Some(&Err(cause.clone())), 8).unwrap_err();
    assert_eq!(
        failure,
        RenderRedrawFailure::FailedPublication {
            publication: 8,
            cause
        }
    );
    assert_eq!(
        require_completed_outcome(None, 8),
        Err(RenderRedrawFailure::MissingOutcome { publication: 8 })
    );
    assert!(failure.to_string().contains("owned fault"));
    assert!(std::error::Error::source(&failure).is_some());
}

#[test]
fn checked_gui_publication_is_fresh_without_copying_or_replacing_capture() {
    let (view, _) = super::super::view_binding_tests::binding_fixture_for_policy(
        crate::RenderGuiSurfacePolicy::CheckedV2,
    );
    super::super::view_binding_tests::attach_binding_gui(&view);
    let before = view.redraw_cursor().unwrap();
    view.try_redraw().unwrap();
    let after = view.check_redraw_since(&before).unwrap();
    assert_eq!(after.publication(), before.publication() + 1);
    assert_eq!(view.capture().unwrap().rgba, vec![0, 255, 0, 255]);
}

#[test]
fn identical_window_rebinding_invalidates_a_previous_cursor() {
    let (view, window) = super::super::view_binding_tests::binding_fixture_for_policy(
        crate::RenderGuiSurfacePolicy::LegacyV1,
    );
    let before = view.redraw_cursor().unwrap();
    view.try_redraw().unwrap();
    view.set_window(&Handle::default(), &window.cast(), 0);
    assert!(matches!(
        view.check_redraw_since(&before),
        Err(RenderRedrawFailure::TargetChanged {
            binding_changed: true,
            ..
        })
    ));
}

#[test]
fn removed_view_cannot_issue_or_complete_a_cursor() {
    let (view, _) = super::super::view_binding_tests::binding_fixture_for_policy(
        crate::RenderGuiSurfacePolicy::LegacyV1,
    );
    let before = view.redraw_cursor().unwrap();
    view.remove();
    assert!(matches!(
        view.redraw_cursor(),
        Err(RenderRedrawFailure::InvalidTarget { .. })
    ));
    assert!(matches!(
        view.check_redraw_since(&before),
        Err(RenderRedrawFailure::InvalidTarget { .. })
    ));
}

#[test]
fn publication_exhaustion_refuses_measurement_without_destroying_pixels() {
    let (view, _) = super::super::view_binding_tests::binding_fixture_for_policy(
        crate::RenderGuiSurfacePolicy::LegacyV1,
    );
    let before = view.redraw_cursor().unwrap();
    view.redraw_serial.store(u64::MAX - 1, Ordering::Relaxed);
    view.try_redraw().unwrap();
    assert!(matches!(
        view.check_redraw_since(&before),
        Err(RenderRedrawFailure::SequenceExhausted { .. })
    ));
    assert_eq!(view.capture().unwrap().rgba, vec![0, 255, 0, 255]);
}

#[test]
fn gui_validation_refuses_stale_extent_without_cloning_the_packet() {
    let (view, _) = super::super::view_binding_tests::binding_fixture_for_policy(
        crate::RenderGuiSurfacePolicy::LegacyV1,
    );
    super::super::view_binding_tests::attach_binding_gui(&view);
    assert!(view.gui_layers.read().validate_extent([1, 1]).is_ok());
    assert!(view.gui_layers.read().validate_extent([2, 1]).is_err());
    let before = view.redraw_cursor().unwrap();
    view.try_redraw().unwrap();
    assert!(view.check_redraw_since(&before).is_ok());
}
