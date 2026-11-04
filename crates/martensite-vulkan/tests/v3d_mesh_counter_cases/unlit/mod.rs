//! Optional sphere shading diagnostic changes no authored baseline or geometry.
#[cfg(test)]
mod controls;
mod failures;
mod role;
use crate::session_run::presentation_role_scope as role_scope;
use failures::join_outcomes;
mod snapshot;
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
use martensite_tkv3d::ais::AisInteractiveObject;
use role::SphereAspectRole;
use snapshot::SphereIdentity;
use std::io;
const FLAG: &str = "MARTENSITE_SPHERE_UNLIT_OBSERVER";
type Outcome = Result<(), Box<dyn std::error::Error>>;
fn refused(value: impl std::fmt::Display) -> io::Error {
    io::Error::other(format!(
        "sphere Unlit control {value}; expected existing s mode1 exact shading-aspect bindings"
    ))
}
fn requested(flag: Option<&std::ffi::OsStr>, image: &str) -> io::Result<bool> {
    let Some(flag) = flag else { return Ok(false) };
    if flag != "1"
        || ![
            "bug23200_shape.png",
            "bug23200_shape_auto_triangulation.png",
            "bug23200_shape_not_auto_triangulation.png",
        ]
        .contains(&image)
    {
        return Err(refused(format!(
            "flag={flag:?}, image={image:?}; expected1 and an original sphere image"
        )));
    }
    Ok(true)
}
#[path = "configuration.rs"]
mod configuration;
use configuration::require_flags;
trait SphereControlSink {
    fn capture(&mut self, session: &ViewerSession, image: &str, suffix: &str) -> Outcome;
    fn identity(&mut self, image: &str, state: &SphereIdentity) -> Outcome;
}
#[path = "capture.rs"]
mod capture;
use capture::SphereImageSink;
fn run_control(session: &ViewerSession, image: &str, sink: &mut impl SphereControlSink) -> Outcome {
    let role = SphereAspectRole::resolve(session)?;
    let before = SphereIdentity::capture(session, &role)?;
    sink.capture(session, image, "sphere-unlit-before")?;
    let mut scope = role.begin()?;
    let unlit = sink.capture(session, image, "sphere-unlit");
    let cleanup = join_outcomes(
        scope.restore().map_err(Into::into),
        before.require(session, &role).map_err(Into::into),
    );
    let restored = sink.capture(session, image, "sphere-unlit-restored");
    let state = before.require(session, &role).map_err(Into::into);
    join_outcomes(
        join_outcomes(unlit, cleanup),
        join_outcomes(restored, state),
    )?;
    sink.identity(image, &before)
}
/// Keep the original path untouched unless explicitly requested; `observe(session, image)?`.
pub(super) fn observe(session: &ViewerSession, image: &str) -> Outcome {
    if !requested(std::env::var_os(FLAG).as_deref(), image)? {
        return Ok(());
    }
    require_flags()?;
    run_control(session, image, &mut SphereImageSink)
}
