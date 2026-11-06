//! Prepared access retains authored geometry and has no whole-Perform preamble.
use super::{fixture, support::*};
use martensite_tkbo::bop_algo::{
    ProgressCompletion, WireOperationContext, WirePreparedExtension, WireSplitter,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::Orientation;
use std::sync::atomic::Ordering;

type IncidenceRows = Vec<Vec<(usize, Orientation)>>;

fn normalized_row(row: &[(usize, Orientation)]) -> Vec<(usize, u8)> {
    let mut result = row
        .iter()
        .map(|(edge, orientation)| (*edge, *orientation as u8))
        .collect::<Vec<_>>();
    result.sort();
    result
}

fn normalized_rows(rows: &IncidenceRows) -> Vec<Vec<(usize, u8)>> {
    let mut result = rows
        .iter()
        .map(|row| normalized_row(row))
        .collect::<Vec<_>>();
    result.sort();
    result
}

fn authored_triangle_rows() -> IncidenceRows {
    use Orientation::{Forward as F, Reversed as R};
    let mut rows = vec![vec![(0, F), (1, F), (4, R)], vec![(2, F), (3, F), (4, F)]];
    let translated = rows.clone().into_iter().map(|row| {
        row.into_iter()
            .map(|(edge, orientation)| (edge + 6, orientation))
            .collect()
    });
    rows.extend(translated);
    rows
}

pub(crate) fn assert_authored_triangles(rows: &IncidenceRows) {
    let expected = authored_triangle_rows();
    assert_eq!(normalized_rows(rows), normalized_rows(&expected));
}

#[test]
fn prepared_irregular_helper_preserves_every_authored_incidence_and_orientation() {
    for parallel in [false, true] {
        let (mut receiver, inputs) = fixture::split_square_splitter(parallel);
        let mut operation = WireOperationContext::new();
        let prepared = receiver.prepare_wires_extension(&mut operation).unwrap();
        assert_eq!(
            prepared.make_wires(&ProgressRange::default()),
            ProgressCompletion::Completed
        );
        assert_authored_triangles(&fixture::input_identity_records(&receiver, &inputs));
    }
}

#[test]
fn prepared_empty_helper_retains_report_without_initial_user_break_poll() {
    let mut receiver = fixture::empty_splitter();
    add_error(&receiver, false);
    let mut operation = WireOperationContext::new();
    let prepared = receiver.prepare_wires_extension(&mut operation).unwrap();
    let (indicator, trace) = counted_indicator(1);
    require_report_failure(prepared.make_wires(&indicator.start()));
    assert_eq!(trace.polls.load(Ordering::Acquire), 0);
    assert!(receiver.options().has_errors());
}

#[test]
fn prepared_task_cancellation_retains_raw_loops_without_adding_alerts() {
    let (mut receiver, _) = fixture::split_square_splitter(false);
    let mut operation = WireOperationContext::new();
    let prepared = receiver.prepare_wires_extension(&mut operation).unwrap();
    let (indicator, _) = counted_indicator(3);
    assert!(matches!(
        prepared.make_wires(&indicator.start()),
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(!receiver.options().has_errors());
    assert!(!receiver.wes().unwrap().shapes().is_empty());
}

#[test]
fn missing_prepared_input_is_typed_failure_and_context_is_not_reusable() {
    let mut receiver = WireSplitter::new();
    let mut operation = WireOperationContext::new();
    let result = receiver
        .prepare_wires_extension(&mut operation)
        .err()
        .unwrap();
    require_kernel_failure(*result, "Standard_NullObject");
    assert!(receiver.context().is_null());
    let second = receiver
        .prepare_wires_extension(&mut operation)
        .err()
        .unwrap();
    require_kernel_failure(*second, "Standard_ProgramError");
}

fn require_send<T: Send>() {
    let drop_owned: fn(T) = std::mem::drop::<T>;
    std::hint::black_box(drop_owned);
}

#[test]
fn operation_and_borrowed_adaptor_have_send_without_unsafe_implementations() {
    require_send::<WireSplitter>();
    require_send::<WireOperationContext>();
    let shared_context: &dyn Sync = &WireOperationContext::new();
    std::hint::black_box(shared_context);
    require_send::<WirePreparedExtension<'static, 'static>>();
    let context = WireOperationContext::default();
    #[allow(clippy::drop_non_drop)] // The move into the thread is what this checks.
    std::thread::spawn(move || drop(context)).join().unwrap();
}

#[test]
fn prepared_callback_failure_retains_actual_owned_kernel_cause() {
    let (mut receiver, _) = fixture::split_square_splitter(false);
    let mut operation = WireOperationContext::new();
    let prepared = receiver.prepare_wires_extension(&mut operation).unwrap();
    let indicator = throwing_prepared_indicator();
    let cause = require_kernel_failure(
        prepared.make_wires(&indicator.start()),
        "Standard_ProgramError",
    );
    // The existing context launcher wraps callback failures, including in serial mode.
    assert_eq!(
        cause.message(),
        "Standard_DomainError: authored wire input check"
    );
    assert_eq!(cause.stack_string(), "wire-owned-stack");
    assert!(receiver.wes().unwrap().shapes().is_empty());
}

fn authored_regular_receiver() -> (WireSplitter, Vec<martensite_tkbrep::topods::Shape>) {
    let (mut receiver, inputs) = fixture::split_square_splitter(false);
    let mut regular = martensite_tkbo::bop_algo::WireEdgeSet::new();
    regular.set_face(receiver.wes().unwrap().face());
    for edge in &inputs[..4] {
        regular.add_start_element(edge);
    }
    receiver.set_wes(regular);
    (receiver, inputs)
}

#[test]
fn prepared_regular_helper_publishes_the_authored_four_forward_boundary_edges() {
    let (mut receiver, inputs) = authored_regular_receiver();
    let mut operation = WireOperationContext::new();
    let prepared = receiver.prepare_wires_extension(&mut operation).unwrap();
    assert_eq!(
        prepared.make_wires(&ProgressRange::default()),
        ProgressCompletion::Completed
    );
    let rows = fixture::input_identity_records(&receiver, &inputs);
    let expected = vec![(0..4).map(|edge| (edge, Orientation::Forward)).collect()];
    assert_eq!(normalized_rows(&rows), normalized_rows(&expected));
}
