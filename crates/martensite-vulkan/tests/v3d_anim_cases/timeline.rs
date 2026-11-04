//! A named animation timeline: children with start times and durations, played paused at a
//! presentation time, then the view redrawn.
use super::prelude::*;
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkmath::gp::{Transform3, Vector3};
use martensite_tkv3d::ais::{
    AisAnimation, AisAnimationMembers as _, AisAnimationObject, AisAnimationSource,
};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;

/// The root animation `name` (a cleared timeline); `timeline("anim")`.
pub(super) fn timeline(name: &str) -> Handle<dyn AisAnimationSource> {
    Handle::new(AisAnimation::new(name)).cast()
}

/// Add `child` to `root` starting at `start` for `duration` seconds.
pub(super) fn schedule(
    root: &Handle<dyn AisAnimationSource>,
    child: Handle<dyn AisAnimationSource>,
    start: f64,
    duration: f64,
) -> Result<(), Failure> {
    let value = child.get().expect("a new animation is never null");
    value.set_start_pts(start);
    value.set_own_duration(duration);
    root.get().expect("a root animation").add(&child)
}

/// Update every animation at presentation time `pts` and leave it paused, then redraw, as
/// playing with a zero duration does; `play_at(&session, &root, 1.0)?`.
pub(crate) fn play_at(
    session: &ViewerSession,
    root: &Handle<dyn AisAnimationSource>,
    pts: f64,
) -> Result<(), Failure> {
    let value = root.get().expect("a root animation");
    value.start_timer(pts, 1.0, true, true)?;
    session.update_view(|_| Ok(()))
}

/// The translation of the object `name` from its current location by `from` to `to`.
fn object_move(
    session: &ViewerSession,
    child: &str,
    name: &str,
    from: [f64; 3],
    to: [f64; 3],
) -> Result<Handle<dyn AisAnimationSource>, Failure> {
    let object = session.object(name).cloned().unwrap_or_default();
    let local = object
        .get()
        .map(|o| o.local_transformation())
        .unwrap_or_default();
    let end = |[x, y, z]: [f64; 3]| -> Result<Transform3, Failure> {
        let mut trsf = Transform3::default();
        trsf.set_rotation_quaternion(local.rotation_quaternion());
        trsf.set_translation_part(Vector3::new(x, y, z));
        trsf.set_scale_factor(local.scale_factor())?;
        Ok(trsf)
    };
    let (start, end) = (end(from)?, end(to)?);
    let animation = AisAnimationObject::new(child, session.context(), &object, start, end);
    Ok(Handle::new(animation).cast())
}

/// `objects`: a box and a sphere moved by overlapping object animations.
pub(crate) fn moved_objects() -> Outcome {
    banner(&[
        "==================================",
        "Viewer animation - animate object moving",
        "==================================",
    ]);
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    run.session().set_axonometric()?;
    let request = DisplayRequest::default().with_display_mode(1);
    run.session().display(
        "b1",
        &box_at([-50., -50., 0.], [100., 100., 100.])?,
        &request,
    )?;
    run.session().display("s", &sphere(50.)?, &request)?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(2.499),
        proj: Some([0.546611, -0.600024, -0.584114]),
        up: Some([-0.411832, 0.414728, -0.811415]),
        at: Some([-5.3425, -2.983, 49.216]),
        ..Default::default()
    })?;
    let root = timeline("anim");
    let box_move = object_move(run.session(), "obj1", "b1", [0.; 3], [-100., 0., -100.])?;
    schedule(&root, box_move, 0., 2.)?;
    let sphere_move = object_move(run.session(), "obj2", "s", [0.; 3], [100., 0., 100.])?;
    schedule(&root, sphere_move, 1., 2.)?;
    for (pts, image) in [
        (0., "objects_0.png"),
        (1., "objects_1.png"),
        (2., "objects_2.png"),
    ] {
        play_at(run.session(), &root, pts)?;
        run.dump(image)?;
    }
    if run.session().pixel_color_name(270, 20)? != "DARKGOLDENROD" {
        println!("Error: Box moving result is wrong!");
    }
    if run.session().pixel_color_name(120, 255)? != "DARKGOLDENROD4" {
        println!("Error: Sphere moving result is wrong!");
    }
    interactive_hint();
    run.finish()
}
