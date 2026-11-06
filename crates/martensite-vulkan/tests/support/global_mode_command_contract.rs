//! Closed-form command boundaries are independent of either graphics implementation.
use super::mode_command_fixture::*;
use martensite_tkernel::standard::{Failure, catch_native_failure};

#[test]
fn box_scene_executes_actual_bare_global_mode_boundaries() {
    replay_mode_cases(BareModeFacade::Box);
    // This is a command-adapter test, not a second renderer or an image reference.
}
#[test]
fn named_scene_executes_actual_bare_global_mode_boundaries() {
    replay_mode_cases(BareModeFacade::Named);
    // Both public facade routes must reach the same actual-context operation.
}

fn replay_mode_cases(route: BareModeFacade) {
    require_send_sync(&route);
    for (index, case) in mode_cases().into_iter().enumerate() {
        require_send_sync(&case);
        let mut fixture = ModeCommandFixture::new(case);
        let facade = fixture.facade(route);
        let result = catch_native_failure(|| facade.run(case.requested));
        let events = fixture.finish();
        require_mode_result(case, &fixture, &events, &result);
        print_mode_record(index, &fixture, &events, &result);
    }
}

fn print_mode_record(
    index: usize,
    fixture: &ModeCommandFixture,
    events: &[[i32; 3]],
    result: &Result<(), Failure>,
) {
    let failed = i32::from(result.is_err());
    let error_type = result.as_ref().err().map_or("", Failure::exception_type);
    let message = result.as_ref().err().map_or("", Failure::message);
    let mode = fixture.context.display_mode();
    let immediate = observed_immediate(fixture.view.view_base());
    println!(
        "{{\"case\":{index},\"events\":{events:?},\"failed\":{failed},\"mode\":{mode},\"immediate\":{immediate},\"rust_type\":{error_type:?},\"rust_message\":{message:?}}}"
    );
}

fn require_mode_result(
    case: BareModeCase,
    fixture: &ModeCommandFixture,
    events: &[[i32; 3]],
    result: &Result<(), Failure>,
) {
    let (count, failed) = expected_mode_result(case);
    require_mode_events(case, events, count);
    assert_eq!(
        result.is_err(),
        failed,
        "case={case:?}; expected failure stopping boundary"
    );
    assert_eq!(fixture.context.display_mode(), case.requested);
    let immediate = observed_immediate(fixture.view.view_base());
    assert_eq!(immediate, i32::from(case.immediate));
    require_typed_failure(result, case.fail_at);
}
fn expected_mode_result(case: BareModeCase) -> (usize, bool) {
    let total = if case.initial == case.requested { 1 } else { 2 };
    let failed = case.fail_at > 0 && case.fail_at <= total;
    let count = if failed { case.fail_at } else { total };
    (count, failed)
}
fn require_mode_events(case: BareModeCase, events: &[[i32; 3]], count: usize) {
    assert_eq!(
        events.len(),
        count,
        "case={case:?}; expected actual redraw boundaries"
    );
    for (index, row) in events.iter().enumerate() {
        let expected = [
            (index + 1) as i32,
            i32::from(case.immediate),
            case.requested,
        ];
        assert_eq!(
            *row, expected,
            "case={case:?}; expected ordered live-state event"
        );
    }
}

fn require_typed_failure(result: &Result<(), Failure>, ordinal: usize) {
    if let Err(cause) = result {
        assert_eq!(cause.exception_type(), "Standard_Failure");
        assert_eq!(cause.message(), failure_message(ordinal));
    }
}
fn require_send_sync<T: Send + Sync>(value: &T) {
    let _: &dyn Send = value;
    let _: &dyn Sync = value;
}
