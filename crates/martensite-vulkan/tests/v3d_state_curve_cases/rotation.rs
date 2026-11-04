//! Fixed-time playback and a command rotation must produce identical formatted locations.
use super::*;
use crate::primitive_shapes::preview_box_at;
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkv3d::ais::{AisAnimationAxisRotation, AisAnimationMembers, AisAnimationSource};

pub(super) fn rotation_shapes() -> Result<[Shape; 2], Failure> {
    Ok([
        preview_box_at([0.; 3], [2., 100., 100.])?,
        preview_box_at([0.; 3], [2., 100., 100.])?,
    ])
}
fn display_rotation_objects(
    session: &mut ViewerSession,
    shapes: &[Shape; 2],
) -> Result<(), Failure> {
    session.display(
        "b1",
        &shapes[0],
        &DisplayRequest::default().with_display_mode(0),
    )?;
    session.display(
        "b2",
        &shapes[1],
        &DisplayRequest::default().with_display_mode(1),
    )?;
    Ok(())
}
pub(super) fn rotation_scene(
    session: &mut ViewerSession,
    shapes: &[Shape; 2],
) -> Result<Handle<dyn AisAnimationSource>, Failure> {
    display_rotation_objects(session, shapes)?;
    let axis = Axis1::new(Point3::new(2., 100., 0.), Direction3::new(0., 0., 1.)?);
    let object = session.object("b2").cloned().unwrap_or_default();
    let animation = Handle::new(AisAnimationAxisRotation::new(
        "anim",
        session.context(),
        &object,
        axis,
        0.,
        90. * (std::f64::consts::PI / 180.),
    ));
    animation.set_own_duration(2.);
    Ok(animation.cast())
}
pub(super) fn direct_rotation(session: &ViewerSession) -> Result<String, Failure> {
    let object = session.object("b2").cloned().unwrap_or_default();
    let axis = Axis1::new(Point3::new(2., 100., 0.), Direction3::new(0., 0., 1.)?);
    let transform = Transform3::rotation(axis, 45. * (std::f64::consts::PI / 180.))?;
    session
        .context()
        .set_location(&object, &Location::default())?;
    session
        .context()
        .set_location(&object, &Location::new(transform))?;
    session.update_view(|_| Ok(()))?;
    session.transformation_text("b2")
}
const ROTATION_BANNER: [&str; 4] = [
    "============",
    "0032570: Visualization, AisAnimationObject - define rotation around axis",
    "============",
    "",
];
fn check_location_text(first: &str, second: &str) {
    if first != second {
        println!(
            "Error: the location at the middle of animation is different from the location after rotating by 45 degrees"
        );
    }
}
/// Full `bug32570`: the midpoint and the direct rotation have equal command results.
pub(crate) fn axis_rotation_midpoint() -> Outcome {
    banner(&ROTATION_BANNER);
    let shapes = rotation_shapes()?;
    let mut run = SessionRun::new()?;
    let animation = rotation_scene(run.session(), &shapes)?;
    crate::v3d_anim_cases::play_at(run.session(), &animation, 1.)?;
    animation.stop()?;
    run.session().fit()?;
    run.dump("bug32570.png")?;
    let first = run.session().transformation_text("b2")?;
    let second = direct_rotation(run.session())?;
    check_location_text(&first, &second);
    println!("Put the following command to start interactive animation:");
    println!("    vanimation anim -play");
    run.finish()
}
