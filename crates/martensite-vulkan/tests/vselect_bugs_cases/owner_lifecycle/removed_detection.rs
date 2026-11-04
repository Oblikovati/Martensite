//! OCC28310 drops its last application-owned object before walking detection.
use super::*;
use martensite_tkv3d::ais::AisSelectionScheme;

pub(super) fn detect_and_select(session: &ViewerSession) -> Result<(), Failure> {
    session.view().fit_all(0.01, true)?;
    session.context().move_to(200, 200, session.view(), true)?;
    session
        .context()
        .select_detected(AisSelectionScheme::Replace)?;
    session.context().update_current_viewer();
    Ok(())
}

pub(super) fn walk_legacy_detection(context: &AisInteractiveContext) {
    context.init_detected();
    while context.more_detected() {
        // DetectedInteractive is the legacy last-picked getter, not the cursor's object.
        let object = context.detected_interactive();
        drop(object);
        context.next_detected();
    }
}

pub(super) fn remove_then_iterate(session: &ViewerSession) -> Outcome {
    // Native aBox topology stays alive; only the AIS object's handle is nullified.
    let (_shape, object) = objects::anonymous_box(session.context())?;
    detect_and_select(session)?;
    session.context().remove(&object, true)?;
    drop(object);
    walk_legacy_detection(session.context());
    Ok(())
}

pub(crate) fn removed_object_detection() -> Outcome {
    let mut run = lifecycle_begin(
        "===========",
        "OCC28310: Visualization - crash on iteration through detected interactive objects",
    )?;
    remove_then_iterate(run.session())?;
    run.dump("bug28310.png")?;
    run.finish()
}
