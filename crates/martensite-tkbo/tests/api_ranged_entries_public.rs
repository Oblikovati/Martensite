//! The public caller-range entries of the Boolean API against frozen native rows: for each cancellation
//! query sampled, the final state and a hash of every indicator event, serially, with no tolerance
//! (the recorded answers). Parallel cancellation is not compared.
// The shared recorder resolves its report types through this crate-root name.
#[allow(unused_imports)]
use martensite_tkbo::bop_algo;
use martensite_tkbo::bop_algo::{
    AlertUserBreak, BOPAlgoArgumentAnalyzer, BOPAlgoBuilder, BOPAlgoPaveFiller, CheckResult,
    Operation, Options, ProgressCompletion,
};
use martensite_tkbo::brep_algo_api::{
    BRepAlgoAPIBuilderAlgo, BRepAlgoAPICheck, BRepAlgoAPICommon, BRepAlgoAPICut, BRepAlgoAPIFuse,
    BRepAlgoAPISection, BRepAlgoAPISplitter,
};
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::RuntimeTyped;
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::sync::atomic::Ordering;
#[allow(dead_code)]
#[path = "../src/bop_algo/progress_test_support/recording.rs"]
mod recording;

const EXPECTED: &str = include_str!("api_ranged_entries_public/expected.txt");

fn cube(offset: f64) -> Shape {
    BRepPrimAPIMakeBox::at_point(Point3::new(offset, offset, offset), 1., 1., 1.)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn pair() -> (Shape, Shape) {
    (cube(0.), cube(0.5))
}

fn single(shape: &Shape) -> List<Shape> {
    [shape.clone()].into_iter().collect()
}

fn both(first: &Shape, second: &Shape) -> List<Shape> {
    [first.clone(), second.clone()].into_iter().collect()
}

fn performed_filler(first: &Shape, second: &Shape) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&both(first, second));
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    filler
}

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    if shape.is_null() {
        return 0;
    }
    let mut found = IndexedMapOfShape::with_hasher(Default::default(), 1);
    topexp::map_shapes_of_type(shape, kind, &mut found).unwrap();
    found.extent()
}

fn shape_text(shape: &Shape) -> String {
    let kinds = [
        ShapeEnum::Vertex,
        ShapeEnum::Edge,
        ShapeEnum::Face,
        ShapeEnum::Solid,
    ];
    let counts: Vec<String> = kinds.map(|kind| count(shape, kind).to_string()).to_vec();
    counts.join(",")
}

fn report_text(options: &Options) -> String {
    let errors = u8::from(options.has_errors());
    let user_break = u8::from(options.has_error(AlertUserBreak::static_type()));
    format!("{errors},{user_break}")
}

fn faults_text(faults: &List<CheckResult>) -> String {
    let statuses: Vec<String> = faults
        .iter()
        .map(|fault| (fault.check_status() as i32).to_string())
        .collect();
    statuses.join(",")
}

/// Runs one entry in `range`; returns its state and its completion.
type Entry = fn(&ProgressRange<'_>) -> (String, ProgressCompletion);

fn check_text(check: &BRepAlgoAPICheck) -> String {
    let report = report_text(check.options());
    let valid = u8::from(check.is_valid());
    format!("{report};{valid};{}", faults_text(check.result()))
}

fn check_one(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (check, completion) =
        BRepAlgoAPICheck::from_shape_with_progress(&cube(0.), true, true, range);
    (check_text(&check), completion)
}

fn check_two(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let (check, completion) = BRepAlgoAPICheck::from_shapes_with_progress(
        (&first, &second),
        Operation::Fuse,
        true,
        true,
        range,
    );
    (check_text(&check), completion)
}

fn all_modes(analyzer: &mut BOPAlgoArgumentAnalyzer) {
    *analyzer.argument_type_mode() = true;
    *analyzer.self_inter_mode() = true;
    *analyzer.small_edge_mode() = true;
    *analyzer.rebuild_face_mode() = true;
    *analyzer.tangent_mode() = true;
    *analyzer.merge_vertex_mode() = true;
    *analyzer.merge_edge_mode() = true;
    *analyzer.continuity_mode() = true;
    *analyzer.curve_on_surface_mode() = true;
}

fn analyzer_text(analyzer: &BOPAlgoArgumentAnalyzer) -> String {
    let report = report_text(analyzer.options());
    let faulty = u8::from(analyzer.has_faulty());
    format!("{report};{faulty};{}", faults_text(analyzer.check_result()))
}

fn analyzer_all(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let mut analyzer = BOPAlgoArgumentAnalyzer::new();
    analyzer.set_shape1(&first);
    analyzer.set_shape2(&second);
    *analyzer.operation_type() = Operation::Fuse;
    all_modes(&mut analyzer);
    let completion = analyzer.perform_with_progress(range);
    (analyzer_text(&analyzer), completion)
}

/// A single argument meant for an operation is a bad type, so stop-on-first skips the later tests.
fn analyzer_stop(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let mut analyzer = BOPAlgoArgumentAnalyzer::new();
    analyzer.set_shape1(&cube(0.));
    *analyzer.operation_type() = Operation::Fuse;
    *analyzer.stop_on_first_faulty() = true;
    all_modes(&mut analyzer);
    let completion = analyzer.perform_with_progress(range);
    (analyzer_text(&analyzer), completion)
}

/// The state every Boolean facade reports, the same fields whatever its type.
macro_rules! api_text {
    ($api:expr) => {
        format!(
            "{},{};{};{}",
            u8::from($api.has_errors()),
            u8::from($api.has_error(AlertUserBreak::static_type())),
            u8::from($api.is_done()),
            shape_text($api.shape())
        )
    };
}

fn common_shapes(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let (api, completion) = BRepAlgoAPICommon::from_shapes_with_progress(&first, &second, range);
    (api_text!(api), completion)
}

fn fuse_shapes(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let (api, completion) = BRepAlgoAPIFuse::from_shapes_with_progress(&first, &second, range);
    (api_text!(api), completion)
}

fn cut_shapes(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let (api, completion) = BRepAlgoAPICut::from_shapes_with_progress(&first, &second, range);
    (api_text!(api), completion)
}

fn common_filler(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let filler = performed_filler(&first, &second);
    let (api, completion) =
        BRepAlgoAPICommon::from_shapes_with_filler_progress(&first, &second, filler, range);
    (api_text!(api), completion)
}

fn fuse_filler(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let filler = performed_filler(&first, &second);
    let (api, completion) =
        BRepAlgoAPIFuse::from_shapes_with_filler_progress(&first, &second, filler, range);
    (api_text!(api), completion)
}

fn cut_filler_reverse(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let filler = performed_filler(&first, &second);
    let (api, completion) =
        BRepAlgoAPICut::from_shapes_with_filler_progress(&first, &second, filler, false, range);
    (api_text!(api), completion)
}

fn section_build(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let mut api = BRepAlgoAPISection::from_shapes(&first, &second, None, false).unwrap();
    let completion = api.build_with_progress(range);
    (api_text!(api), completion)
}

fn splitter_build(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let mut api = BRepAlgoAPISplitter::new();
    api.set_arguments(&single(&first));
    api.set_tools(&single(&second));
    let completion = api.build_with_progress(range);
    (api_text!(api), completion)
}

fn builder_build(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let mut api = BRepAlgoAPIBuilderAlgo::new();
    api.set_arguments(&both(&first, &second));
    let completion = api.build_with_progress(range);
    (api_text!(api), completion)
}

fn performed_builder(first: &Shape, second: &Shape) -> BOPAlgoBuilder {
    let mut builder = BOPAlgoBuilder::new();
    builder.set_arguments(&both(first, second));
    builder.perform();
    builder
}

fn build_bop_states(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let mut builder = performed_builder(&first, &second);
    let completion = builder.build_bop_with_progress(
        &single(&first),
        State::Out,
        &single(&second),
        State::In,
        range,
        None,
    );
    let report = report_text(builder.options());
    (
        format!("{report};{}", shape_text(builder.shape())),
        completion,
    )
}

fn build_bop_operation(range: &ProgressRange<'_>) -> (String, ProgressCompletion) {
    let (first, second) = pair();
    let mut builder = performed_builder(&first, &second);
    let completion = builder.build_bop_operation_with_progress(
        &single(&first),
        &single(&second),
        Operation::Common,
        range,
        None,
    );
    let report = report_text(builder.options());
    (
        format!("{report};{}", shape_text(builder.shape())),
        completion,
    )
}

const ENTRIES: [(&str, Entry); 15] = [
    ("check_one", check_one),
    ("check_two", check_two),
    ("analyzer_all", analyzer_all),
    ("analyzer_stop", analyzer_stop),
    ("common_shapes", common_shapes),
    ("fuse_shapes", fuse_shapes),
    ("cut_shapes", cut_shapes),
    ("common_filler", common_filler),
    ("fuse_filler", fuse_filler),
    ("cut_filler_reverse", cut_filler_reverse),
    ("section_build", section_build),
    ("splitter_build", splitter_build),
    ("builder_build", builder_build),
    ("build_bop_states", build_bop_states),
    ("build_bop_operation", build_bop_operation),
];

/// The native FNV-1a hash of the event lines, each ending in a newline.
fn events_hash(events: &[String]) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for event in events {
        for byte in event.bytes().chain(*b"\n") {
            hash = (hash ^ u64::from(byte)).wrapping_mul(1099511628211);
        }
    }
    hash
}

fn run_row(kind: &str, entry: Entry, target: usize) -> (String, usize) {
    let (indicator, trace) = recording::pulse_indicator(target);
    let (state, completion) = entry(&indicator.start());
    let queries = trace.queries.load(Ordering::SeqCst);
    let observed = trace.break_observed.load(Ordering::SeqCst);
    assert!(
        !(target == 0 && !matches!(completion, ProgressCompletion::Completed)),
        "{kind}: an uncancelled run returned {completion:?}"
    );
    assert!(
        observed || target == 0 || queries < target,
        "{kind}: target {target} was reached but not recorded"
    );
    let events = recording::native_events(&trace);
    let hash = events_hash(&events);
    let row = format!("{kind}|{target}|{queries}|{state}|{}|{hash}", events.len());
    (row, queries)
}

/// All queries when there are few; else about sixty evenly spaced ones and the last.
fn sampled_targets(total: usize) -> Vec<usize> {
    let stride = if total <= 60 { 1 } else { total / 60 };
    let mut targets: Vec<usize> = (1..=total).step_by(stride).collect();
    if !(total - 1).is_multiple_of(stride) {
        targets.push(total);
    }
    targets
}

fn kind_rows(kind: &str, entry: Entry) -> Vec<String> {
    let (row, total) = run_row(kind, entry, 0);
    let mut rows = vec![row];
    rows.extend(
        sampled_targets(total)
            .into_iter()
            .map(|target| run_row(kind, entry, target).0),
    );
    rows
}

#[test]
fn public_range_entries_match_every_native_row() {
    let expected: Vec<&str> = EXPECTED.lines().collect();
    let mut actual = Vec::new();
    for (kind, entry) in ENTRIES {
        actual.extend(kind_rows(kind, entry));
    }
    let differing: Vec<String> = actual
        .iter()
        .zip(&expected)
        .filter(|(left, right)| left != right)
        .map(|(left, right)| format!("rust {left}\nnative {right}"))
        .collect();
    assert!(
        differing.is_empty() && actual.len() == expected.len(),
        "{} of {} rows differ ({} native), first:\n{}",
        differing.len(),
        actual.len(),
        expected.len(),
        differing[..differing.len().min(12)].join("\n")
    );
}

#[test]
fn unranged_and_ranged_builds_agree_on_the_result() {
    let (first, second) = pair();
    let unranged = BRepAlgoAPIFuse::from_shapes(&first, &second).unwrap();
    let (first, second) = pair();
    let (ranged, completion) =
        BRepAlgoAPIFuse::from_shapes_with_progress(&first, &second, &ProgressRange::default());
    assert_eq!(completion, ProgressCompletion::Completed);
    assert_eq!(shape_text(ranged.shape()), shape_text(unranged.shape()));
    assert!(ranged.is_done() && unranged.is_done());
}

#[test]
fn a_cancelled_check_reports_cancellation_not_validity() {
    let (indicator, _trace) = recording::before_indicator();
    let (check, completion) =
        BRepAlgoAPICheck::from_shape_with_progress(&cube(0.), true, true, &indicator.start());
    assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
    assert!(check.is_valid(), "a cancelled native check keeps no fault");
    assert!(check.options().has_error(AlertUserBreak::static_type()));
}

#[test]
fn refused_operations_fail_before_any_work() {
    let mut api = BRepAlgoAPICommon::new();
    let completion = api.build_with_progress(&ProgressRange::default());
    assert!(matches!(completion, ProgressCompletion::Failed { .. }));
    assert!(!api.is_done());
}

/// Native keeps a pointer to the supplied filler's data structure; Rust is given the filler again.
#[test]
fn build_bop_on_a_supplied_filler_matches_the_owned_filler_result() {
    let (first, second) = pair();
    let mut filler = performed_filler(&first, &second);
    let mut builder = BOPAlgoBuilder::new();
    builder.set_arguments(&both(&first, &second));
    builder.perform_with_filler(&mut filler);
    let completion = builder.build_bop_operation_with_filler_progress(
        &filler,
        (&single(&first), &single(&second), Operation::Common),
        &ProgressRange::default(),
        None,
    );
    assert_eq!(completion, ProgressCompletion::Completed);
    assert_eq!(shape_text(builder.shape()), "8,12,6,1");
}

#[test]
fn build_bop_without_a_run_records_the_failure_in_the_report() {
    let (first, second) = pair();
    let mut builder = BOPAlgoBuilder::new();
    let completion = builder.build_bop_with_progress(
        &single(&first),
        State::Out,
        &single(&second),
        State::In,
        &ProgressRange::default(),
        None,
    );
    assert!(matches!(completion, ProgressCompletion::Failed { .. }));
    assert!(builder.options().has_errors());
}
