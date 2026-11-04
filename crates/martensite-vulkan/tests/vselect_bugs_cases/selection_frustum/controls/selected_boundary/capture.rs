//! Optional post-original control uses the reviewed semantic role and restores before END.
use super::super::frustum_cpu_session;
use super::*;
use crate::source_output::control_capture::control_dump;
use martensite_tkservice::graphic3d::{
    Graphic3dAspectsLineMembers as _, Graphic3dStructureMembers as _,
};

trait SelectedBoundaryCaptureOutput {
    fn write_uncovered(&mut self, session: &ViewerSession) -> Outcome;
    fn write_restored(&mut self, session: &ViewerSession) -> Outcome;
}
struct CaseBoundaryCapture;
impl SelectedBoundaryCaptureOutput for CaseBoundaryCapture {
    fn write_uncovered(&mut self, session: &ViewerSession) -> Outcome {
        let original = "bug26304.png";
        control_dump(session, original, "uncovered")
    }
    fn write_restored(&mut self, session: &ViewerSession) -> Outcome {
        let original = "bug26304.png";
        control_dump(session, original, "restored")
    }
}

/// Capture the authored role only when requested; `capture_selected_boundary(session, true)?`.
pub(crate) fn capture_selected_boundary(session: &ViewerSession, enabled: bool) -> Outcome {
    let mut output = CaseBoundaryCapture;
    with_selected_boundary_capture(session, enabled, &mut output)
}

fn with_selected_boundary_capture(
    session: &ViewerSession,
    enabled: bool,
    output: &mut dyn SelectedBoundaryCaptureOutput,
) -> Outcome {
    if !enabled {
        return Ok(());
    }
    let role = SelectedBoundaryRole::resolve(session, "b")?;
    let before = SceneSnapshot::capture(session);
    let result = capture_scoped_boundary(session, &role, &before, output);
    let after = SceneSnapshot::capture(session);
    let restored = require_restored_scene(&before, &after);
    let restored_image = capture_restored_if_complete(session, output, &restored);
    let captured = combine_control_causes(result, restored, "restoration");
    combine_control_causes(captured, restored_image, "restored image")
}

fn capture_restored_if_complete(
    session: &ViewerSession,
    output: &mut dyn SelectedBoundaryCaptureOutput,
    restored: &Outcome,
) -> Outcome {
    if restored.is_ok() {
        return output.write_restored(session);
    }
    Ok(())
}

fn combine_control_causes(captured: Outcome, restored: Outcome, phase: &str) -> Outcome {
    match (captured, restored) {
        (Err(capture), Err(restore)) => Err(std::io::Error::other(format!(
            "selected control capture failed: {capture}; {phase} failed: {restore}"
        ))
        .into()),
        (Err(error), _) | (_, Err(error)) => Err(error),
        _ => Ok(()),
    }
}

fn require_restored_scene(before: &SceneSnapshot, after: &SceneSnapshot) -> Outcome {
    if before == after {
        return Ok(());
    }
    Err(program_error(&format!(
        "restored selected scene {after:?}; expected {before:?}"
    ))
    .into())
}

fn capture_scoped_boundary(
    session: &ViewerSession,
    role: &SelectedBoundaryRole,
    before: &SceneSnapshot,
    output: &mut dyn SelectedBoundaryCaptureOutput,
) -> Outcome {
    let restore = BoundaryPatternScope::new(&role.object);
    let unchanged = before.check_except_role_aspects(&SceneSnapshot::capture(session), role);
    let result = unchanged
        .map_err(Into::into)
        .and_then(|()| output.write_uncovered(session));
    drop(restore); // The post-state check runs even when capture returns an I/O error.
    result
}

struct NamedBoundaryCapture {
    writes: usize,
    restored_writes: usize,
    refuse: bool,
}
impl SelectedBoundaryCaptureOutput for NamedBoundaryCapture {
    fn write_uncovered(&mut self, session: &ViewerSession) -> Outcome {
        self.writes += 1;
        let object = frustum_operand(session, "b")?;
        assert!(
            object
                .presentations()
                .into_iter()
                .filter(|prs| prs.mode() == 0)
                .flat_map(|prs| prs.groups())
                .any(|group| group.aspects().line_type()
                    == martensite_tkservice::aspect::AspectTypeOfLine::Empty)
        );
        if self.refuse {
            return Err(std::io::Error::other("named capture refusal").into());
        }
        Ok(())
    }
    fn write_restored(&mut self, session: &ViewerSession) -> Outcome {
        self.restored_writes += 1;
        assert_eq!(
            SelectedBoundaryRole::resolve(session, "b")?
                .draws(session)?
                .len(),
            1
        );
        Ok(())
    }
}

#[test]
fn disabled_capture_does_not_require_an_case_scene_or_write() {
    let session = frustum_cpu_session();
    let mut output = NamedBoundaryCapture {
        writes: 0,
        restored_writes: 0,
        refuse: false,
    };
    with_selected_boundary_capture(&session, false, &mut output).unwrap();
    assert_eq!((output.writes, output.restored_writes), (0, 0));
}

#[test]
fn actual_capture_restores_on_success_and_returned_io_error() {
    let probe = actual_persistent_probe();
    let before = SceneSnapshot::capture(&probe.session);
    for refuse in [false, true] {
        let mut output = NamedBoundaryCapture {
            writes: 0,
            restored_writes: 0,
            refuse,
        };
        assert_eq!(
            with_selected_boundary_capture(&probe.session, true, &mut output).is_err(),
            refuse
        );
        assert_eq!((output.writes, output.restored_writes), (1, 1));
        assert_eq!(before, SceneSnapshot::capture(&probe.session));
    }
}

#[test]
fn restored_scene_check_refuses_an_actual_camera_change() {
    let probe = actual_persistent_probe();
    let before = SceneSnapshot::capture(&probe.session);
    let camera = probe.session.view().camera();
    let scale = camera.scale();
    camera.set_scale(scale * 2.);
    let changed = SceneSnapshot::capture(&probe.session);
    assert!(require_restored_scene(&before, &changed).is_err());
    camera.set_scale(scale);
    assert_eq!(before, SceneSnapshot::capture(&probe.session));
}

struct RefusingCameraCapture {
    restored_writes: usize,
}
impl SelectedBoundaryCaptureOutput for RefusingCameraCapture {
    fn write_uncovered(&mut self, session: &ViewerSession) -> Outcome {
        let camera = session.view().camera();
        camera.set_scale(camera.scale() * 2.);
        Err(std::io::Error::other("named camera capture failure").into())
    }
    fn write_restored(&mut self, _session: &ViewerSession) -> Outcome {
        self.restored_writes += 1;
        Err(std::io::Error::other("unexpected restored image after incomplete restoration").into())
    }
}

#[test]
fn capture_and_restoration_failures_retain_both_causes() {
    let probe = actual_persistent_probe();
    let before = SceneSnapshot::capture(&probe.session);
    let object = frustum_operand(&probe.session, "b").unwrap();
    let bindings = boundary_scope::aspect_bindings(&object);
    let camera = probe.session.view().camera();
    let scale = camera.scale();
    let mut output = RefusingCameraCapture { restored_writes: 0 };
    let error = with_selected_boundary_capture(&probe.session, true, &mut output).unwrap_err();
    assert_eq!(output.restored_writes, 0);
    let message = error.to_string();
    assert!(message.contains("named camera capture failure"));
    assert!(message.contains("restored selected scene"));
    assert_eq!(bindings, boundary_scope::aspect_bindings(&object));
    camera.set_scale(scale);
    assert_eq!(before, SceneSnapshot::capture(&probe.session));
}

struct RefusingRestoredCapture {
    uncovered_refusal: bool,
    restored_writes: usize,
}
impl SelectedBoundaryCaptureOutput for RefusingRestoredCapture {
    fn write_uncovered(&mut self, _session: &ViewerSession) -> Outcome {
        if !self.uncovered_refusal {
            return Ok(());
        }
        Err(std::io::Error::other("named uncovered capture failure").into())
    }
    fn write_restored(&mut self, session: &ViewerSession) -> Outcome {
        self.restored_writes += 1;
        SelectedBoundaryRole::resolve(session, "b")?;
        Err(std::io::Error::other("named restored image failure").into())
    }
}

#[test]
fn restored_image_failure_refuses_and_retains_an_uncovered_failure() {
    let probe = actual_persistent_probe();
    let before = SceneSnapshot::capture(&probe.session);
    for uncovered_refusal in [false, true] {
        let mut output = RefusingRestoredCapture {
            uncovered_refusal,
            restored_writes: 0,
        };
        let error = with_selected_boundary_capture(&probe.session, true, &mut output).unwrap_err();
        assert_restored_capture_error(&*error, &output);
        assert_eq!(before, SceneSnapshot::capture(&probe.session));
    }
}

fn assert_restored_capture_error(error: &dyn Error, output: &RefusingRestoredCapture) {
    let message = error.to_string();
    assert!(message.contains("named restored image failure"));
    assert_eq!(
        message.contains("named uncovered capture failure"),
        output.uncovered_refusal
    );
    assert_eq!(output.restored_writes, 1);
}
