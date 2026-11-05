//! Native-owned input bytes pin the successful FF store before cancellation repair.
use super::super::owned_progress_fixture::read_owned_progress_shape;
use super::*;
use crate::bop_algo::ParallelAlgo;
use martensite_tkernel::{standard::RuntimeTyped, storage::IoStream};
use martensite_tkgeombase::geom_tools;
use martensite_tkmath::bnd::BoundingBox3;
type StoredCurves = martensite_tkernel::ncollection::DynamicArray<crate::bop_ds::BopdsCurve>;

#[test]
fn completed_ff_store_records_the_whole_curve_and_pcurve_definitions() {
    let mut filler = fixture_filler();
    filler.perform_ff().unwrap();
    assert_complete_record(&filler);
    let snapshot = complete_snapshot(&filler);
    assert_eq!(
        snapshot,
        include_str!("face_face_store_fixture/complete.expected.json").trim_end()
    );
    println!("{snapshot}");
}

fn assert_complete_record(filler: &BOPAlgoPaveFiller) {
    let structure = filler.data_structure().unwrap();
    assert!(!filler.options().has_errors());
    assert_eq!(structure.interf_ff_records().len(), 1);
    let record = structure.interf_ff_records().value(0).unwrap();
    assert_eq!((record.curves().len(), record.points().len()), (1, 0));
    assert!(structure.has_interference_between(record.index1(), record.index2()));
    assert_eq!(
        whole_curve_definitions(record.curves()),
        include_bytes!("face_face_store_fixture/complete.curves")
    );
}

fn fixture_filler() -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.add_argument(&read_owned_progress_shape(
        include_bytes!("face_face_store_fixture/plane.brep"),
        ShapeEnum::Face,
    ));
    filler.add_argument(&read_owned_progress_shape(
        include_bytes!("face_face_store_fixture/cylinder.brep"),
        ShapeEnum::Face,
    ));
    filler.initialize().unwrap();
    filler
}

fn complete_snapshot(filler: &BOPAlgoPaveFiller) -> String {
    let structure = filler.data_structure().unwrap();
    let record = structure.interf_ff_records().value(0).unwrap();
    let geometry = whole_curve_definitions(record.curves());
    let bounds = curve_bounds(record.curves());
    let tolerances = curve_tolerances(record.curves());
    format!(
        "{{\"shapes\":[{},{}],\"indices\":[{},{}],\"tangent\":{},\"points\":{},\"bounds\":{bounds:?},\"tolerances\":{tolerances:?},\"geometry\":{geometry:?}}}",
        structure.nb_source_shapes(),
        structure.nb_shapes(),
        record.index1(),
        record.index2(),
        record.tangent_faces(),
        record.points().len()
    )
}

fn whole_curve_definitions(curves: &StoredCurves) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(17);
    for record in curves.iter() {
        let curve = record.curve();
        geom_tools::write_curve(curve.curve(), &mut stream).unwrap();
        geom_tools::write_curve2d(curve.first_curve2d(), &mut stream).unwrap();
        geom_tools::write_curve2d(curve.second_curve2d(), &mut stream).unwrap();
    }
    stream_bytes(&mut stream)
}

fn stream_bytes(stream: &mut IoStream) -> Vec<u8> {
    let length = stream.tellp();
    let mut bytes = vec![
        0;
        usize::try_from(length).unwrap_or_else(|_| panic!(
            "geometry stream length {length}, expected a nonnegative length"
        ))
    ];
    stream.seekg(0).read(&mut bytes);
    assert_eq!(stream.gcount(), length);
    bytes
}

fn curve_bounds(curves: &StoredCurves) -> Vec<[u64; 6]> {
    curves
        .iter()
        .map(|curve| curve.box_bounds().get().unwrap().map(f64::to_bits))
        .collect()
}

fn curve_tolerances(curves: &StoredCurves) -> Vec<u64> {
    curves
        .iter()
        .map(|curve| curve.tolerance().to_bits())
        .collect()
}

struct CurveStopProbe {
    calls: std::sync::atomic::AtomicUsize,
    shows: std::sync::atomic::AtomicUsize,
}

impl martensite_tkernel::message::ProgressIndicatorHooks for CurveStopProbe {
    fn show(&self, _: &martensite_tkernel::message::ProgressScope<'_>, _: f64, _: bool) {
        // Observe scope closure without inspecting the builder through an aliased borrow.
        self.shows
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn user_break(&self) -> bool {
        let ordinal = self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        ordinal == 0
    }
}

fn stop_indicator() -> martensite_tkernel::message::ProgressIndicator {
    let probe = CurveStopProbe {
        calls: 0.into(),
        shows: 0.into(),
    };
    martensite_tkernel::message::ProgressIndicator::new(Box::new(probe))
}

fn prepared_fixture_task(filler: &mut BOPAlgoPaveFiller) -> BOPAlgoFaceFaceTask<'static> {
    let pairs = filler
        .iterator_pairs(ShapeEnum::Face, ShapeEnum::Face)
        .unwrap();
    assert_eq!(
        pairs.len(),
        1,
        "fixture pairs {pairs:?}, expected one FF pair"
    );
    filler.update_touched_face_info(&pairs).unwrap();
    let points = filler.ee_new_vertices_by_edges().unwrap();
    filler
        .face_face_input(pairs[0].0, pairs[0].1, &points)
        .unwrap()
        .unwrap_or_else(|| panic!("fixture pairs {pairs:?}, expected a real FF task"))
        .into_task()
}

#[test]
fn curve_stop_retains_native_partial_record_and_interference_without_geometry() {
    use crate::bop_algo::ProgressCancellation;
    let mut filler = fixture_filler();
    let mut task = prepared_fixture_task(&mut filler);
    task.perform_parallel().unwrap();
    let indicator = stop_indicator();
    let range = indicator.start();
    let scope = martensite_tkernel::message::ProgressScope::unnamed(&range, 1.0, false);
    let observed = ProgressCancellation::default();
    let continued = filler
        .store_face_task_with_progress(&mut task, &scope, &observed)
        .unwrap();
    assert!(!continued && observed.is_cancelled());
    assert_partial_record(&filler);
}

fn assert_partial_record(filler: &BOPAlgoPaveFiller) {
    let structure = filler.data_structure().unwrap();
    assert_eq!(structure.interf_ff_records().len(), 1);
    let record = structure.interf_ff_records().value(0).unwrap();
    assert_eq!((record.curves().len(), record.points().len()), (0, 0));
    assert!(structure.has_interference_between(record.index1(), record.index2()));
    assert_user_break(filler);
}

#[test]
fn stop_before_native_record_publication_keeps_the_record_sequence_empty() {
    let filler = fixture_filler();
    let indicator = stop_indicator();
    let range = indicator.start();
    let scope = martensite_tkernel::message::ProgressScope::unnamed(&range, 1.0, false);
    let observed = crate::bop_algo::ProgressCancellation::default();
    assert!(super::super::progress_observation::native_user_break(
        filler.options(),
        &scope,
        &observed
    ));
    assert!(
        filler
            .data_structure()
            .unwrap()
            .interf_ff_records()
            .is_empty()
    );
    assert_user_break(&filler);
}

#[test]
fn actual_failed_task_keeps_native_empty_record_warning_and_raw_error_flags() {
    let mut filler = fixture_filler();
    let mut task = prepared_fixture_task(&mut filler);
    task.set_boxes(&BoundingBox3::default(), &BoundingBox3::default());
    task.perform_parallel().unwrap();
    assert!(task.parallel_options().has_errors());
    filler.store_face_face_task(&mut task).unwrap();
    let record = filler
        .data_structure()
        .unwrap()
        .interf_ff_records()
        .value(0)
        .unwrap();
    assert_eq!((record.curves().len(), record.points().len()), (0, 0));
    assert!(filler.options().has_warnings());
    assert!(!filler.options().has_errors());
}

fn assert_user_break(filler: &BOPAlgoPaveFiller) {
    assert!(
        filler
            .options()
            .has_error(crate::bop_algo::AlertUserBreak::static_type())
    );
}

struct NativeFfBoundaryStop {
    shows: std::sync::atomic::AtomicUsize,
    calls: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    target: usize,
}

impl martensite_tkernel::message::ProgressIndicatorHooks for NativeFfBoundaryStop {
    fn show(&self, _: &martensite_tkernel::message::ProgressScope<'_>, _: f64, _: bool) {
        // Retain actual presentation activity independently of non-sticky poll ordinals.
        self.shows
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn user_break(&self) -> bool {
        let ordinal = self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        ordinal == self.target
    }
}

#[test]
fn whole_ff_stage_stops_at_actual_native_fourth_and_fifth_boundaries() {
    for target in [1, 4, 5] {
        let (filler, _, polls) = stop_whole_ff_at(target);
        assert_eq!(polls, target);
        print_ff_stage_state(&filler, target, polls);
        assert_eq!(filler.iterator().unwrap().more(), target == 1);
        assert_user_break(&filler);
        let records = filler.data_structure().unwrap().interf_ff_records();
        assert_eq!(records.len(), usize::from(target == 5));
        if target == 5 {
            assert_partial_record(&filler);
        }
    }
}

fn stop_whole_ff_at(target: usize) -> (BOPAlgoPaveFiller, Vec<ProgressCompletion>, usize) {
    let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let hooks = NativeFfBoundaryStop {
        shows: 0.into(),
        calls: calls.clone(),
        target,
    };
    let indicator = martensite_tkernel::message::ProgressIndicator::new(Box::new(hooks));
    let range = indicator.start();
    let observed = crate::bop_algo::ProgressCancellation::default();
    let mut filler = fixture_filler();
    let completions = filler.perform_ff_with_progress(&range, &observed).unwrap();
    assert_eq!(observed.is_cancelled(), target != 0);
    let polls = calls.load(std::sync::atomic::Ordering::SeqCst);
    (filler, completions, polls)
}

#[test]
fn configured_parallel_whole_ff_preserves_the_frozen_successful_store() {
    let mut filler = fixture_filler();
    filler.options_mut().set_run_parallel(true);
    filler.perform_ff().unwrap();
    assert_complete_record(&filler);
    assert_eq!(
        complete_snapshot(&filler),
        include_str!("face_face_store_fixture/complete.expected.json").trim_end()
    );
}

#[test]
fn nonsticky_actual_task_cancellation_keeps_typed_cause_without_forging_parent_error() {
    let (filler, completions, polls) = stop_whole_ff_at(2);
    assert_eq!(polls, 4);
    print_ff_stage_state(&filler, 2, polls);
    assert_eq!(completions.len(), 1);
    assert!(matches!(
        completions[0],
        ProgressCompletion::Cancelled { .. }
    ));
    assert!(!filler.options().has_errors());
    assert!(filler.options().has_warnings());
    let structure = filler.data_structure().unwrap();
    let record = structure.interf_ff_records().value(0).unwrap();
    assert_eq!((record.curves().len(), record.points().len()), (0, 0));
    assert!(!structure.has_interference_between(record.index1(), record.index2()));
}

#[test]
fn completed_live_ff_follows_five_native_polls_and_whole_curve_definitions() {
    let (filler, completions, polls) = stop_whole_ff_at(0);
    assert_eq!(polls, 5);
    assert!(completions.is_empty());
    assert_complete_record(&filler);
    print_ff_stage_state(&filler, 0, polls);
}

fn print_ff_stage_state(filler: &BOPAlgoPaveFiller, target: usize, polls: usize) {
    let structure = filler.data_structure().unwrap();
    let records = structure.interf_ff_records();
    let (curves, points, interference) = records.iter().last().map_or((0, 0, false), |record| {
        (
            record.curves().len(),
            record.points().len(),
            structure.has_interference_between(record.index1(), record.index2()),
        )
    });
    let stopped = filler
        .options()
        .has_error(crate::bop_algo::AlertUserBreak::static_type());
    println!(
        "{{\"stage\":\"ff\",\"target\":{target},\"ff\":{},\"curves\":{curves},\"points\":{points},\"has_interference\":{interference},\"has_errors\":{},\"has_warnings\":{},\"user_break\":{stopped},\"poll_count\":{polls}}}",
        records.len(),
        filler.options().has_errors(),
        filler.options().has_warnings()
    );
}
