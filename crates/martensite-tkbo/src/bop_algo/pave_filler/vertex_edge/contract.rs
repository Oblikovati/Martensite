//! Actual native VE cancellation, scalar outputs, completion causes and worker ownership.
use super::super::{BOPAlgoPaveFiller, pave_block_index_links};
use super::*;
use crate::bop_algo::AlertUserBreak;
use crate::bop_algo::pave_filler::owned_progress_fixture::{
    native_scope_json, progress_scope_ancestry, read_owned_progress_shape,
};
use martensite_tkernel::{
    message::{ProgressIndicator, ProgressIndicatorHooks, ProgressRange},
    ncollection::DataMap,
    standard::RuntimeTyped,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};
type EdgeScopeChain = Vec<(String, f64, f64)>;
type EdgeEvents = Arc<Mutex<Vec<(f64, EdgeScopeChain)>>>;

struct EdgeBoundaryProbe {
    target: usize,
    polls: Arc<AtomicUsize>,
    events: EdgeEvents,
}

impl ProgressIndicatorHooks for EdgeBoundaryProbe {
    fn user_break(&self) -> bool {
        let ordinal = self.polls.fetch_add(1, Ordering::SeqCst) + 1;
        ordinal == self.target
    }

    fn show(&self, scope: &ProgressScope<'_>, position: f64, _: bool) {
        let event = (position, progress_scope_ancestry(scope));
        self.events
            .lock()
            .unwrap_or_else(|cause| {
                panic!("VE events {cause:?}, expected an unpoisoned observation mutex")
            })
            .push(event);
    }
}

#[test]
fn actual_vertex_edge_stage_matches_all_six_native_boundary_states() {
    for target in 0..=5 {
        let (filler, completions, polls, _) = run_edge_stage(target);
        assert_edge_boundary(&filler, target, polls);
        if target == 3 {
            assert!(matches!(
                completions.as_slice(),
                [ProgressCompletion::Cancelled { .. }]
            ));
        } else {
            assert!(
                completions.is_empty(),
                "VE target {target}: unexpected {completions:?}"
            );
        }
    }
}

fn assert_edge_boundary(filler: &BOPAlgoPaveFiller, target: usize, polls: usize) {
    let (state, parameters) = edge_stage_values(filler);
    let expected_polls = if target == 0 || target == 3 {
        5
    } else {
        target
    };
    assert_eq!(polls, expected_polls);
    assert_eq!(state, [6, usize::from(target == 0), 0]);
    assert_eq!(parameters, if target == 0 { vec![0.5] } else { vec![] });
    assert_eq!(filler.iterator().unwrap().more(), target == 1);
    assert_ve_reports(filler, target);
    print_ve_boundary(filler, target, polls, &parameters, state);
}

fn edge_stage_values(filler: &BOPAlgoPaveFiller) -> ([usize; 3], Vec<f64>) {
    let structure = filler.data_structure().unwrap();
    let parameters: Vec<f64> = structure
        .interf_ve_records()
        .iter()
        .map(|r| r.parameter())
        .collect();
    let state = [
        structure.nb_shapes() as usize,
        parameters.len(),
        structure.shape_sd_items().count(),
    ];
    (state, parameters)
}

fn assert_ve_reports(filler: &BOPAlgoPaveFiller, target: usize) {
    let error = target != 0 && target != 3;
    assert_eq!(filler.options().has_errors(), error);
    assert_eq!(filler.options().has_warnings(), target == 3);
    assert_eq!(
        filler.options().has_error(AlertUserBreak::static_type()),
        error
    );
}

fn print_ve_boundary(
    filler: &BOPAlgoPaveFiller,
    target: usize,
    polls: usize,
    parameters: &[f64],
    state: [usize; 3],
) {
    let error = filler.options().has_errors();
    let warning = filler.options().has_warnings();
    let stopped = filler.options().has_error(AlertUserBreak::static_type());
    let more = filler.iterator().unwrap().more();
    println!(
        "{{\"target\":{target},\"state\":{state:?},\"iterator_more\":{more},\"has_errors\":{error},\"has_warnings\":{warning},\"user_break\":{stopped},\"parameters\":{parameters:?},\"poll_count\":{polls}}}"
    );
}

fn run_edge_stage(
    target: usize,
) -> (
    BOPAlgoPaveFiller,
    Vec<ProgressCompletion>,
    usize,
    EdgeEvents,
) {
    let (indicator, polls, events) = edge_indicator(target);
    let range = indicator.start();
    let observed = ProgressCancellation::default();
    let mut filler = edge_fixture();
    let completions = filler.perform_ve_with_progress(&range, &observed).unwrap();
    assert_eq!(observed.is_cancelled(), target != 0);
    let polls = polls.load(Ordering::SeqCst);
    (filler, completions, polls, events)
}

fn edge_indicator(target: usize) -> (ProgressIndicator, Arc<AtomicUsize>, EdgeEvents) {
    let polls = Arc::new(AtomicUsize::new(0));
    let events = Arc::new(Mutex::new(Vec::new()));
    let hooks = EdgeBoundaryProbe {
        target,
        polls: polls.clone(),
        events: events.clone(),
    };
    (ProgressIndicator::new(Box::new(hooks)), polls, events)
}

fn edge_fixture() -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.add_argument(&read_owned_progress_shape(
        include_bytes!("vertex.brep"),
        ShapeEnum::Compound,
    ));
    filler.add_argument(&read_owned_progress_shape(
        include_bytes!("edge.brep"),
        ShapeEnum::Compound,
    ));
    filler.initialize().unwrap();
    filler.perform_vv().unwrap();
    filler
}

#[test]
fn actual_named_vertex_edge_work_advances_before_scope_closure() {
    let (_, _, _, events) = run_edge_stage(0);
    let events = events.lock().unwrap();
    assert!(
        events.iter().any(|(position, names)| *position > 0.0
            && *position < 1.0
            && names
                .iter()
                .any(|(name, _, _)| name == "Performing Vertex-Edge intersection")),
        "VE events {events:?}, expected named actual work before closure"
    );
}

fn work_fixture() -> (VertexEdgeWork, Handle<Context>) {
    let (mut work, context) = prepare_edge_batch(edge_fixture());
    assert_eq!(work.len(), 1);
    (work.pop().unwrap(), context)
}

fn prepare_edge_batch(mut filler: BOPAlgoPaveFiller) -> (Vec<VertexEdgeWork>, Handle<Context>) {
    filler
        .fill_shrunk_data(ShapeEnum::Vertex, ShapeEnum::Edge)
        .unwrap();
    filler
        .initialize_progress_pairs(ShapeEnum::Vertex, ShapeEnum::Edge)
        .unwrap();
    let scope = ProgressScope::default();
    let observed = ProgressCancellation::default();
    let pairs = filler.prepare_ve_pairs(&scope, &observed).unwrap().unwrap();
    let mut same_domain = DataMap::default();
    let work = filler
        .prepare_ve_work(&pairs, &mut same_domain, &scope, &observed)
        .unwrap()
        .unwrap();
    (work, filler.context.clone())
}

fn run_unbound_work(work: &mut VertexEdgeWork, context: &Handle<Context>) {
    let range = ProgressRange::default();
    perform_vertex_edge_work(work, context, &range, 0).unwrap();
}

#[test]
fn actual_ve_scalar_work_is_send_and_keeps_the_independent_line_projection() {
    static_assertions::assert_impl_all!(VertexEdgeWork: Send);
    static_assertions::assert_impl_all!(VertexEdgeInvocation<'static, 'static>: Send);
    static_assertions::assert_not_impl_any!(VertexEdgeInvocation<'static, 'static>: Sync);
    let (mut work, context) = work_fixture();
    run_unbound_work(&mut work, &context);
    assert_eq!(work.solver.flag, 0);
    assert_eq!(
        [work.solver.parameter, work.solver.new_tolerance],
        [0.5, 1e-7]
    );
    assert_eq!(
        work.take_completion().unwrap(),
        ProgressCompletion::Completed
    );
}

#[test]
fn ordinary_negative_projection_flag_is_completed_empty_without_error() {
    let (mut work, context) = work_fixture();
    work.vertex = off_line_vertex();
    run_unbound_work(&mut work, &context);
    assert_eq!(work.solver.flag, -4);
    assert_eq!(
        [work.solver.parameter, work.solver.new_tolerance],
        [0.5, 1.0000001]
    );
    assert!(!work.options.has_errors());
    assert_eq!(
        work.take_completion().unwrap(),
        ProgressCompletion::Completed
    );
}

#[test]
fn actual_ve_kernel_failure_wins_over_a_previously_observed_cancel() {
    let (mut work, context) = work_fixture();
    work.vertex = Vertex::default();
    run_unbound_work(&mut work, &context);
    assert!(
        work.options
            .has_error(AlertIntersectionFailed::static_type())
    );
    let observed = ProgressCancellation::default();
    observed.record_cancelled();
    assert!(matches!(
        work.classify(&observed, 0),
        ProgressCompletion::Failed {
            class: ProgressFailureClass::KernelFailure,
            cause: Some(_),
            ..
        }
    ));
}

#[test]
fn missing_worker_completion_is_an_owned_error_not_success() {
    let (mut work, _) = work_fixture();
    let failure = work.take_completion().unwrap_err();
    assert!(
        failure
            .message()
            .contains("expected one actual worker completion"),
        "VE failure {failure:?}, expected missing callback context"
    );
}

#[test]
fn empty_intersect_ve_has_no_fake_task_or_cancellation() {
    let mut filler = BOPAlgoPaveFiller::new();
    let pairs = pave_block_index_links();
    let observed = ProgressCancellation::default();
    let outcomes = filler
        .intersect_ve_with_progress(&pairs, true, &ProgressRange::default(), &observed)
        .unwrap();
    assert!(outcomes.is_empty());
    assert!(!observed.is_cancelled() && !filler.options().has_errors());
}

fn off_line_vertex() -> Vertex {
    let mut vertex = Vertex::default();
    martensite_tkbrep::brep::BRepBuilder::default()
        .make_vertex_at(
            &mut vertex,
            martensite_tkmath::gp::Point3::new(0.5, 1.0, 0.0),
            1e-7,
        )
        .unwrap();
    vertex
}

#[test]
fn actual_parallel_ve_contexts_preserve_serial_order_and_closed_form_parameters() {
    let expected = edge_batch_rows(false);
    assert_eq!(expected.len(), 4);
    for repetition in 0..32 {
        let actual = edge_batch_rows(true);
        assert_eq!(
            actual, expected,
            "VE repetition {repetition}, expected serial ordered scalars"
        );
    }
}

type EdgeScalarRow = (i32, i32, i32, u64, u64);

fn edge_batch_rows(parallel: bool) -> Vec<EdgeScalarRow> {
    let (mut work, context) = prepare_edge_batch(multiple_edge_fixture(parallel));
    let expected: Vec<f64> = work
        .iter()
        .map(|item| {
            martensite_tkbrep::brep::BRepTool::pnt(&item.vertex)
                .unwrap()
                .x()
        })
        .collect();
    execute_vertex_edge_work(&mut work, &context, &ProgressScope::default(), parallel).unwrap();
    work.iter_mut()
        .zip(expected)
        .map(|(item, parameter)| edge_scalar_row(item, parameter))
        .collect()
}

fn edge_scalar_row(item: &mut VertexEdgeWork, parameter: f64) -> EdgeScalarRow {
    assert_eq!(item.solver.parameter, parameter);
    assert_eq!(item.solver.flag, 0);
    assert_eq!(item.solver.new_tolerance, 1e-7);
    assert_eq!(
        item.take_completion().unwrap(),
        ProgressCompletion::Completed
    );
    (
        item.solver.vertex,
        item.solver.edge,
        item.solver.flag,
        item.solver.parameter.to_bits(),
        item.solver.new_tolerance.to_bits(),
    )
}

fn multiple_edge_fixture(parallel: bool) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.options_mut().set_run_parallel(parallel);
    filler.add_argument(&four_vertex_input());
    filler.add_argument(&read_owned_progress_shape(
        include_bytes!("edge.brep"),
        ShapeEnum::Compound,
    ));
    filler.initialize().unwrap();
    filler.perform_vv().unwrap();
    filler
}

fn four_vertex_input() -> martensite_tkbrep::topods::Shape {
    use martensite_tkbrep::{brep::BRepBuilder, topods::Compound};
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut result = compound.into_shape();
    for x in [0.2, 0.4, 0.6, 0.8] {
        let mut vertex = Vertex::default();
        builder
            .make_vertex_at(
                &mut vertex,
                martensite_tkmath::gp::Point3::new(x, 0.0, 0.0),
                1e-7,
            )
            .unwrap();
        builder.add(&mut result, &vertex.into_shape()).unwrap();
    }
    result
}

#[test]
fn new_cancelled_ve_attempt_does_not_reuse_a_previous_kernel_failure() {
    let (mut work, context) = work_fixture();
    work.vertex = Vertex::default();
    run_unbound_work(&mut work, &context);
    assert!(work.failure.is_some());
    work.options.clear();
    let (indicator, _, _) = edge_indicator(1);
    let range = indicator.start();
    perform_vertex_edge_work(&mut work, &context, &range, 0).unwrap();
    assert!(work.failure.is_none());
    assert!(matches!(
        work.take_completion().unwrap(),
        ProgressCompletion::Cancelled { .. }
    ));
}

#[test]
fn actual_ve_worker_ancestry_matches_native_single_scope_max_and_value() {
    let (_, _, _, events) = run_edge_stage(0);
    let events = events.lock().unwrap();
    let (_, chain) = events
        .iter()
        .find(|(position, _)| *position == 0.9)
        .unwrap_or_else(|| panic!("VE events {events:?}, expected native0.9 worker close"));
    println!("{{\"ve_worker_scope\":{}}}", native_scope_json(chain));
    assert_eq!(*chain, expected_ve_worker_chain());
}

fn expected_ve_worker_chain() -> EdgeScopeChain {
    [
        ("<null>", 1.0, 1.0),
        ("Performing Vertex-Edge intersection", 1.0, 1.0),
        ("<null>", 10.0, 9.0),
        ("<null>", 1.0, 1.0),
        ("<null>", 1.0, 1.0),
    ]
    .into_iter()
    .map(|(name, maximum, value)| (name.to_owned(), maximum, value))
    .collect()
}

#[test]
fn unstarted_ve_range_closes_against_native_parent_without_a_task_scope() {
    let (work, _) = work_fixture();
    let mut work = [work];
    let (indicator, _, events) = edge_indicator(0);
    let range = indicator.start();
    let scope = ProgressScope::with_literal(&range, "Unstarted VE task", 1.0, false);
    let invocations = vertex_edge_invocations(&mut work, &scope);
    drop(invocations);
    assert!(work[0].completion.is_none());
    let events = events.lock().unwrap();
    let (_, chain) = events
        .iter()
        .find(|(position, _)| *position == 1.0)
        .unwrap();
    let expected = native_unstarted_ve_chain();
    println!("{{\"unstarted_ve_scope\":{}}}", native_scope_json(chain));
    assert_eq!(*chain, expected);
}

#[test]
fn owned_ve_range_can_be_consumed_by_only_one_actual_invocation() {
    let (work, context) = work_fixture();
    let mut work = [work];
    let scope = ProgressScope::default();
    let mut invocations = vertex_edge_invocations(&mut work, &scope);
    invocations[0].perform(&context).unwrap();
    let failure = invocations[0].perform(&context).unwrap_err();
    assert!(
        failure
            .message()
            .contains("VE invocation 0, expected one unconsumed worker range"),
        "VE failure {failure:?}, expected the consumed native range diagnostic"
    );
}

fn native_unstarted_ve_chain() -> Vec<(String, f64, f64)> {
    vec![
        ("Unstarted VE task".to_owned(), 1.0, 1.0),
        ("<null>".to_owned(), 1.0, 1.0),
    ]
}
