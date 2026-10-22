//! An orientation warning may replace the context used by the final shell check.
use super::{
    live_context::{context_switch_tool, square_shell},
    *,
};
use martensite_tkbrep::topods::ShapeIterator;
use martensite_tkernel::{
    impl_transient,
    message::{Gravity, Msg},
    standard::{Handle, StandardTransient},
};
use martensite_tkshhealing::{
    shape_build::ShapeBuildReShape, shape_extend::BasicMsgRegistrator, shape_fix::FixRoot,
};
use std::sync::{
    Arc, Weak,
    atomic::{AtomicBool, Ordering},
};

struct OrientationContextMessages {
    parent: StandardTransient,
    shell_tool: Weak<ShapeFixShell>,
    replacement: Handle<ShapeBuildReShape>,
    fired: Arc<AtomicBool>,
    enabled: bool,
}
impl_transient!(OrientationContextMessages, "OrientationContextMessages", StandardTransient, parent, interfaces:[dyn BasicMsgRegistrator]);

impl BasicMsgRegistrator for OrientationContextMessages {
    fn send_shape(&self, _: &Shape, _: &Msg, _: Gravity) {
        if self.enabled && !self.fired.swap(true, Ordering::SeqCst) {
            self.shell_tool
                .upgrade()
                .unwrap()
                .set_context(&self.replacement);
        }
    }
}

fn duplicate_square_shell() -> Shell {
    let mut source = square_shell();
    let first_face = ShapeIterator::new(&source.clone().into_shape(), true, true)
        .value()
        .unwrap()
        .clone();
    BRepBuilder::default()
        .add(&mut source, &first_face)
        .unwrap();
    source
}

fn warning_registrator(
    tool: &Arc<ShapeFixShell>,
    next: &Handle<ShapeBuildReShape>,
    fired: &Arc<AtomicBool>,
    enabled: bool,
) -> Handle<dyn BasicMsgRegistrator> {
    Handle::new(OrientationContextMessages {
        parent: StandardTransient::default(),
        shell_tool: Arc::downgrade(tool),
        replacement: next.clone(),
        fired: fired.clone(),
        enabled,
    })
    .cast()
}

fn perform_warning_switch(enabled: bool) {
    let source = duplicate_square_shell();
    let replacement = square_shell();
    replacement.set_closed(true).unwrap();
    let (tool, next) = context_switch_tool(&source, &replacement, 0, -1);
    let fired = Arc::new(AtomicBool::new(false));
    tool.set_msg_registrator(&warning_registrator(&tool, &next, &fired, enabled));
    assert!(tool.perform().unwrap());
    assert!(tool.status_flag(ShapeExtendStatus::Done2));
    assert!(!tool.status_flag(ShapeExtendStatus::Done1));
    assert_eq!(fired.load(Ordering::SeqCst), enabled);
    assert_eq!(tool.context() == next, enabled);
    assert_eq!(replacement.closed().unwrap(), !enabled);
    assert!(!source.closed().unwrap());
    assert_eq!(source.nb_children(), 2);
    assert_eq!(tool.nb_shells(), 1);
    assert_eq!(tool.shape().nb_children(), 1);
}

#[test]
fn orientation_warning_context_switch_controls_the_final_closed_flag_check() {
    perform_warning_switch(true);
}

#[test]
fn orientation_warning_without_switch_keeps_other_shell_flags() {
    perform_warning_switch(false);
}
