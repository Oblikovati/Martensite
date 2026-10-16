use super::super::*;
use std::fmt::Write;
struct ReplacementCase {
    name: &'static str,
    request: i32,
    mode: ReplacementMode,
    live: bool,
}
fn case(name: &'static str, request: i32, mode: ReplacementMode, live: bool) -> ReplacementCase {
    ReplacementCase {
        name,
        request,
        mode,
        live,
    }
}
fn cases() -> Vec<ReplacementCase> {
    use ReplacementMode::{Assign, Copy, Direct, Move};
    vec![
        case("assign_success", 2, Assign, true),
        case("assign_constructor_failure", 99, Assign, true),
        case("assignment_failure", 77, Assign, true),
        case("move_success", 2, Move, true),
        case("move_constructor_failure", 99, Move, true),
        case("copy_success", 2, Copy, true),
        case("copy_constructor_failure", 99, Copy, true),
        case("copy_transfer_failure", 88, Copy, false),
        case("direct_success", 2, Direct, true),
        case("direct_constructor_failure", 99, Direct, false),
    ]
}
fn initialized_slot(trace: &ReplacementTrace) -> MaybeUninit<ReplacementItem> {
    let slot = MaybeUninit::new(ReplacementItem::new(1, trace));
    take_trace(trace);
    slot
}
unsafe fn observe_slot(pointer: *mut ReplacementItem, live: bool) -> String {
    if !live {
        return "dead".to_owned();
    }
    // SAFETY: `live` is true here, so Fixture only reads slots whose construction/assignment left a live object.
    unsafe { (*pointer).value.to_string() }
}
unsafe fn repair_and_destroy(pointer: *mut ReplacementItem, live: bool, trace: &ReplacementTrace) {
    // SAFETY: the caller guarantees `pointer` is the fixture's slot; A failed reconstruct/direct path ended the original lifetime: repair before destroying.
    unsafe {
        if !live {
            ReplacementStorage::construct_at(pointer, || ReplacementItem::new(5, trace));
        }
        ReplacementStorage::destroy_at(pointer);
    }
}
fn scenario_line(spec: &ReplacementCase) -> String {
    let trace = Rc::new(RefCell::new(Vec::new()));
    let mut slot = initialized_slot(&trace);
    let pointer = slot.as_mut_ptr();
    // SAFETY: `pointer` is the fixture's live slot with suspended Drop ownership, which
    // repair_and_destroy repairs afterwards.
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        apply_replacement(pointer, spec.request, spec.mode, &trace);
    }));
    let live = observe_live_item(&trace, spec.live);
    // SAFETY: `live` reports whether the slot still holds a value, and observe_slot reads it
    // only then.
    let value = unsafe { observe_slot(pointer, live) };
    let row = formatted_row(spec, result.is_err(), live, &value, &trace);
    // SAFETY: `spec.live` reports whether the replacement left the slot live, which
    // repair_and_destroy needs.
    unsafe { repair_and_destroy(pointer, spec.live, &trace) };
    assert_cleanup(&trace, spec.live, &value);
    row
}
fn assert_cleanup(trace: &ReplacementTrace, live: bool, value: &str) {
    let expected = if live {
        format!("D{value}")
    } else {
        "C5,D5".to_owned()
    };
    assert_eq!(take_trace(trace), expected);
}
fn observe_live_item(trace: &ReplacementTrace, expected: bool) -> bool {
    // Each live item owns exactly one recorder reference; this local recorder owns the other.
    let owners = Rc::strong_count(trace);
    assert_eq!(owners, if expected { 2 } else { 1 });
    owners == 2
}
fn formatted_row(
    spec: &ReplacementCase,
    failed: bool,
    live: bool,
    value: &str,
    trace: &ReplacementTrace,
) -> String {
    format!(
        "{} {} {} {} {}\n",
        spec.name,
        u8::from(failed),
        u8::from(live),
        value,
        take_trace(trace)
    )
}

fn trivial_line() -> String {
    let mut slot = 1_i32;
    // SAFETY: Primitive assignment keeps the stack slot live and never ends its automatic ownership.
    unsafe {
        DynamicArrayStorageExtension::replace_at_assign(
            &mut slot,
            || 7,
            |left, right| *left = *right,
        );
    }
    format!("trivial_assign 0 1 {slot} none\n")
}
#[test]
fn native_replacement_policy_rows_match_runtime_traces() {
    let mut actual = String::new();
    for spec in cases() {
        write!(actual, "{}", scenario_line(&spec)).unwrap();
    }
    actual.push_str(&trivial_line());
    actual.push_str(&scenario_line(&case(
        "default_assign",
        0,
        ReplacementMode::Assign,
        true,
    )));
    assert_eq!(
        actual,
        include_str!("../fixtures/oracle/tkernel_dynamic_replacement_policy.expected")
    );
}
