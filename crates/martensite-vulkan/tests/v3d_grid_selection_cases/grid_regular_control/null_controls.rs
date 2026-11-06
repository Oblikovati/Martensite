//! Mutable missing style links return typed refusal before and during the actual Null scope.
use super::controls::fixture;
use super::*;
use martensite_tkv3d::prs3d::{Prs3dDrawer, Prs3dLineAspect};
#[derive(Clone, Copy, Debug)]
enum MissingStyle {
    Drawer,
    Line,
    Aspect,
}
struct SavedStyle {
    drawer: Handle<Prs3dDrawer>,
    line: Handle<Prs3dLineAspect>,
    aspect: Handle<Graphic3dAspectLine3d>,
}
impl SavedStyle {
    fn capture(session: &ViewerSession) -> Self {
        let drawer = session.context().default_drawer().clone();
        let line = drawer.line_aspect();
        let aspect = line.aspect();
        Self {
            drawer,
            line,
            aspect,
        }
    }
    fn clear(&self, session: &ViewerSession, missing: MissingStyle) {
        match missing {
            MissingStyle::Drawer => session.context().set_default_drawer(&Handle::default()),
            MissingStyle::Line => self.drawer.set_line_aspect(Handle::default()),
            MissingStyle::Aspect => self.line.set_aspect(Handle::default()),
        }
    }
    fn restore(&self, session: &ViewerSession) {
        self.line.set_aspect(self.aspect.clone());
        self.drawer.set_line_aspect(self.line.clone());
        session.context().set_default_drawer(&self.drawer);
    }
}
fn refuse_missing(missing: MissingStyle, active: bool) {
    let probe = fixture();
    let role = role::GridRole::resolve(&probe.session).unwrap();
    let saved = SavedStyle::capture(&probe.session);
    let result = attempt_missing(&probe, &saved, missing, active);
    saved.restore(&probe.session);
    assert_eq!(
        role.group.aspects(),
        role.aspect,
        "{missing:?}, active={active}"
    );
    let typed = result.expect("missing style must return a typed refusal, never panic");
    assert!(typed.unwrap_err().to_string().contains("null"));
}
fn attempt_missing(
    probe: &super::super::cpu::final_line_probe::SelectionFrameProbe,
    saved: &SavedStyle,
    missing: MissingStyle,
    active: bool,
) -> std::thread::Result<Result<String, Box<dyn std::error::Error>>> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if active {
            with_regular(&probe.session, || {
                saved.clear(&probe.session, missing);
                Ok(())
            })
        } else {
            saved.clear(&probe.session, missing);
            with_regular(&probe.session, || Ok(()))
        }
    }))
}

#[test]
fn prebegin_null_drawer_line_and_underlying_aspect_are_typed_refusals() {
    for missing in [
        MissingStyle::Drawer,
        MissingStyle::Line,
        MissingStyle::Aspect,
    ] {
        refuse_missing(missing, false);
    }
}
#[test]
fn active_null_drawer_line_and_underlying_aspect_refuse_after_role_restore() {
    for missing in [
        MissingStyle::Drawer,
        MissingStyle::Line,
        MissingStyle::Aspect,
    ] {
        refuse_missing(missing, true);
    }
}
