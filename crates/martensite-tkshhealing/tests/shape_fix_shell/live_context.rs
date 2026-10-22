//! Progress callbacks may replace the context used by later shell repair phases.
use super::*;
use martensite_tkbrep::brep_tools::ReShape;
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope},
    standard::Handle,
};
use martensite_tkmath::gp::Plane3;
use martensite_tkshhealing::{shape_build::ShapeBuildReShape, shape_fix::FixRoot};
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeFace, brep_check::BRepCheckAnalyzer,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

struct ContextSwitchProgress {
    shell_tool: Arc<ShapeFixShell>,
    replacement: Handle<ShapeBuildReShape>,
    fired: Arc<AtomicBool>,
    enabled: bool,
}

impl ProgressIndicatorHooks for ContextSwitchProgress {
    fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {}
    fn user_break(&self) -> bool {
        if self.enabled && !self.fired.swap(true, Ordering::SeqCst) {
            self.shell_tool.set_context(&self.replacement);
        }
        false
    }
}

pub(super) fn square_shell() -> Shell {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 2., 0., 2.))
        .unwrap()
        .face()
        .unwrap();
    shell(&[face])
}

pub(super) fn context_switch_tool(
    source: &Shell,
    replacement: &Shell,
    face_mode: i32,
    orientation_mode: i32,
) -> (Arc<ShapeFixShell>, Handle<ShapeBuildReShape>) {
    let tool = Arc::new(ShapeFixShell::from_shell(source));
    tool.set_fix_face_mode(face_mode);
    tool.set_fix_orientation_mode(orientation_mode);
    tool.set_context(&Handle::new(ShapeBuildReShape::new()));
    let next = Handle::new(ShapeBuildReShape::new());
    next.replace(
        &source.clone().into_shape(),
        &replacement.clone().into_shape(),
    )
    .unwrap();
    (tool, next)
}

fn perform_switch(enabled: bool) {
    let source = square_shell();
    let replacement = square_shell();
    replacement.set_closed(true).unwrap();
    let (tool, next) = context_switch_tool(&source, &replacement, -1, 0);
    let original_context = tool.context();
    let fired = Arc::new(AtomicBool::new(false));
    let indicator = switching_indicator(&tool, &next, &fired, enabled);
    assert!(!tool.perform_with_progress(&indicator.start()).unwrap());
    assert_eq!(fired.load(Ordering::SeqCst), enabled);
    assert_eq!(tool.context() == next, enabled);
    assert!(!source.closed().unwrap());
    assert_eq!(replacement.closed().unwrap(), !enabled);
    assert!(!tool.status_flag(ShapeExtendStatus::Done));
    assert_progress_inputs(&tool, &source, &original_context);
}

fn switching_indicator(
    tool: &Arc<ShapeFixShell>,
    next: &Handle<ShapeBuildReShape>,
    fired: &Arc<AtomicBool>,
    enabled: bool,
) -> ProgressIndicator {
    ProgressIndicator::new(Box::new(ContextSwitchProgress {
        shell_tool: tool.clone(),
        replacement: next.clone(),
        fired: fired.clone(),
        enabled,
    }))
}

fn assert_progress_inputs(
    tool: &ShapeFixShell,
    source: &Shell,
    original_context: &Handle<ShapeBuildReShape>,
) {
    assert!(tool.shape().is_same(&source.clone().into_shape()));
    assert!(tool.fix_face_tool().context() == *original_context);
    assert!(
        original_context
            .value(&source.clone().into_shape())
            .unwrap()
            .is_same(&source.clone().into_shape())
    );
    assert_source_square(source);
}

fn assert_source_square(source: &Shell) {
    assert_eq!(faces(&source.clone().into_shape()).len(), 1);
    assert_eq!(
        Explorer::new(source, ShapeEnum::Edge, ShapeEnum::Shape)
            .unwrap()
            .count(),
        4
    );
    assert!(
        BRepCheckAnalyzer::new(&source.clone().into_shape(), true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}

#[test]
fn progress_context_switch_controls_the_later_closed_flag_check() {
    perform_switch(true);
}

#[test]
fn non_switching_progress_keeps_the_original_context_and_other_shell_flags() {
    perform_switch(false);
}
