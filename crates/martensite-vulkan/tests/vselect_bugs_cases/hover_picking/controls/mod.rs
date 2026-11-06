//! These real null-driver controls are stronger diagnostics, not original image credit.
use super::*;
use martensite_tkservice::null_driver::NullGraphicDriver;
mod circle;
mod cycling;
mod occlusion;
mod pixels;

fn cpu_session() -> ViewerSession {
    let driver = NullGraphicDriver::new_handle();
    ViewerSession::with_graphic_driver(&driver, 409, 409)
        .expect("hover CPU source inputs; expected one genuine409view")
}

fn cycle_session() -> ViewerSession {
    let mut session = cpu_session();
    let inputs = super::cycle::boxes().unwrap();
    super::cycle::scene(&mut session, &inputs).unwrap();
    session.move_cursor(210, 280).unwrap();
    session
}

fn highlighted(session: &ViewerSession) -> Handle<dyn AisInteractiveObject> {
    // The existing accessor snapshots its owner before any selectable callback.
    let actual = session.context().detected_interactive();
    assert!(
        !actual.is_null(),
        "highlighted operand; expected a real detected object"
    );
    actual
}

fn detected_objects(session: &ViewerSession) -> Vec<Handle<dyn AisInteractiveObject>> {
    let context = session.context();
    let mut owners = Vec::new();
    context.init_detected();
    while context.more_detected() {
        owners.push(context.detected_current_owner().unwrap());
        context.next_detected();
    }
    // All cursor operations finish before virtual selectable resolution.
    owners
        .iter()
        .map(|owner| owner.selectable().cast())
        .collect()
}

fn observe(session: &ViewerSession, stage: &str) {
    let owner = session.context().detected_interactive();
    let point = session.detected_point().unwrap().map(f64::to_bits);
    eprintln!(
        "{{\"hover_control\":\"{stage}\",\"context\":{},\"object\":{},\"point_bits\":{:?}}}",
        session.context().object_address() as usize,
        owner.object_address() as usize,
        point
    );
}
