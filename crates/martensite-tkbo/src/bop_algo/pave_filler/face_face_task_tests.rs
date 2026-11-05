use super::face_face_task::BOPAlgoFaceFaceTask;
use crate::bop_algo::ParallelAlgo;
use martensite_tkbrep::topods::Face;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkmath::{
    bnd::BoundingBox3,
    gp::{Axis3, Direction3, Point3},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

#[test]
fn precision_translation_task_matches_native_plane_intersection() {
    let mut task = configured_task(true);
    task.set_indices(12, 21);
    task.set_parameters(true, true, true, 1e-7);
    task.set_tol_ff(0.125);
    task.set_fuzzy_value(1e-7);
    task.set_context(&Handle::new(crate::int_tools::Context::default()));
    task.intersector_mut().set_list(&[]);
    assert_eq!(task.indices(), [12, 21]);
    assert_eq!(task.tol_ff(), 0.125);
    assert!(!task.face1().is_null());
    assert!(!task.face2().is_null());
    task.perform_parallel().unwrap();
    assert!(task.intersector().is_done());
    task.apply_trsf().unwrap();
    assert_native_curve_row(&task, true);
}

#[test]
fn nearby_face_task_keeps_identity_transform() {
    let mut task = configured_task(false);
    task.perform_parallel().unwrap();
    assert!(task.intersector().is_done());
    assert_eq!(task.trsf().translation_part().x(), 0.0);
    assert_native_curve_row(&task, false);
}

fn configured_task<'a>(far: bool) -> BOPAlgoFaceFaceTask<'a> {
    let (faces, boxes) = if far {
        (far_plane_faces(), far_plane_boxes())
    } else {
        (near_plane_faces(), near_plane_boxes())
    };
    let mut task = BOPAlgoFaceFaceTask::default();
    task.set_faces(&faces[0], &faces[1]);
    task.set_boxes(&boxes[0], &boxes[1]);
    task
}

fn far_plane_faces() -> [Face; 2] {
    [
        bounded_plane(Point3::new(1e9, 0., 1e9), Direction3::Z, Direction3::X),
        bounded_plane(
            Point3::new(1e9 + 0.5, 0., 1e9),
            Direction3::X,
            Direction3::Y,
        ),
    ]
}

fn near_plane_faces() -> [Face; 2] {
    [
        bounded_plane(Point3::default(), Direction3::Z, Direction3::X),
        bounded_plane(Point3::new(0.5, 0., 0.), Direction3::X, Direction3::Y),
    ]
}

fn bounded_plane(origin: Point3, normal: Direction3, x_direction: Direction3) -> Face {
    let frame = Axis3::new(origin, normal, x_direction).unwrap();
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
    BRepBuilderAPIMakeFace::from_surface_bounds(&surface, (-2., 2., -2., 2.), 1e-7)
        .unwrap()
        .face()
        .unwrap()
}

fn far_plane_boxes() -> [BoundingBox3; 2] {
    let minimum = Point3::new(1e9 - 2., -2., 1e9 - 2.);
    let maximum = Point3::new(1e9 + 2., 2., 1e9 + 2.);
    [
        BoundingBox3::new(minimum, maximum),
        BoundingBox3::new(minimum, maximum),
    ]
}

fn near_plane_boxes() -> [BoundingBox3; 2] {
    let minimum = Point3::new(-2., -2., -2.);
    let maximum = Point3::new(2., 2., 2.);
    [
        BoundingBox3::new(minimum, maximum),
        BoundingBox3::new(minimum, maximum),
    ]
}

fn assert_native_curve_row(task: &BOPAlgoFaceFaceTask<'_>, far: bool) {
    let curve = task.intersector().lines().unwrap()[0].curve();
    let first = curve
        .get()
        .unwrap()
        .eval_d0(curve.first_parameter())
        .unwrap();
    let last = curve
        .get()
        .unwrap()
        .eval_d0(curve.last_parameter())
        .unwrap();
    let expected = native_values(far);
    assert_task_status(task, far, &expected);
    assert_curve_range(curve.get().unwrap(), &expected);
    assert_point_close(first, [expected[5], expected[6], expected[7]]);
    assert_point_close(last, [expected[8], expected[9], expected[10]]);
}

fn assert_task_status(task: &BOPAlgoFaceFaceTask<'_>, far: bool, expected: &[f64]) {
    assert_eq!(expected[0], f64::from(far));
    assert_eq!(expected[1], 1.0);
    assert_eq!(
        expected[2],
        task.intersector().lines().unwrap().len() as f64
    );
    let translation = task.trsf().translation_part().x();
    assert_eq!(translation == 0.0, !far);
}

fn assert_curve_range(curve: &dyn martensite_tkg3d::geom::GeomCurve, expected: &[f64]) {
    assert_close(curve.first_parameter(), expected[3], 1e-12);
    assert_close(curve.last_parameter(), expected[4], 1e-12);
}

fn native_values(far: bool) -> Vec<f64> {
    include_str!("../../../tests/fixtures/recorded/tkbo_face_face_task.expected")
        .lines()
        .nth(usize::from(!far))
        .unwrap()
        .split_whitespace()
        .map(|value| value.parse().unwrap())
        .collect()
}

fn assert_point_close(point: martensite_tkmath::gp::Point3, expected: [f64; 3]) {
    for (actual, wanted) in point.coordinates().iter().zip(expected) {
        assert_close(*actual, wanted, 1e-7);
    }
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} != {expected}"
    );
}

#[test]
fn void_boxes_follow_native_failure_alert_path() {
    let mut task = BOPAlgoFaceFaceTask::default();
    task.perform_parallel().unwrap();
    assert!(task.parallel_options().has_errors());
}

struct ToggleProgress {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
}
impl martensite_tkernel::message::ProgressIndicatorHooks for ToggleProgress {
    fn show(&self, _: &martensite_tkernel::message::ProgressScope<'_>, _: f64, _: bool) {
        // The test controls cancellation without console or timing side effects.
    }
    fn user_break(&self) -> bool {
        // The same task can resume after explicit caller reset.
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
    }
}

#[test]
fn real_face_pair_cancellation_precedes_intersector_and_can_resume() {
    let (flag, indicator) = toggle_indicator();
    let range = indicator.start();
    let mut task = configured_task(false);
    task.set_progress_range(&range);
    task.perform_parallel().unwrap();
    assert_before_task_cancel(&task);
    flag.store(false, std::sync::atomic::Ordering::Release);
    task.parallel_options().clear();
    let resumed = indicator.start();
    task.set_progress_range(&resumed);
    task.perform_parallel().unwrap();
    assert!(task.intersector().is_done());
    assert!(!task.parallel_options().has_errors());
}

fn toggle_indicator() -> (
    std::sync::Arc<std::sync::atomic::AtomicBool>,
    martensite_tkernel::message::ProgressIndicator,
) {
    let flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let indicator = martensite_tkernel::message::ProgressIndicator::new(Box::new(ToggleProgress {
        cancelled: flag.clone(),
    }));
    (flag, indicator)
}

fn assert_before_task_cancel(task: &BOPAlgoFaceFaceTask<'_>) {
    use martensite_tkernel::standard::RuntimeTyped;
    assert!(!task.intersector().is_done());
    assert!(
        task.parallel_options()
            .has_error(crate::bop_algo::AlertUserBreak::static_type())
    );
}

#[test]
fn worker_local_face_task_range_returns_only_owned_geometry_observations() {
    static_assertions::assert_not_impl_any!(BOPAlgoFaceFaceTask<'static>: Send, Sync);
    let (flag, indicator) = toggle_indicator();
    flag.store(false, std::sync::atomic::Ordering::Release);
    let range = indicator.start();
    let scope = martensite_tkernel::message::ProgressScope::unnamed(&range, 1.0, false);
    let observed = crate::bop_algo::ProgressCancellation::default();
    let done = std::thread::scope(|workers| {
        workers
            .spawn(|| run_owned_face_task(&scope, &observed))
            .join()
            .unwrap()
    });
    assert!(done, "worker-local actual face intersection must complete");
    assert!(!observed.is_cancelled());
}

fn run_owned_face_task(
    scope: &martensite_tkernel::message::ProgressScope<'_>,
    observed: &crate::bop_algo::ProgressCancellation,
) -> bool {
    let range = scope.next(1.0);
    let mut task = configured_task(false);
    task.perform_with_range(&range, observed).unwrap();
    assert!(!task.parallel_base().progress_range().is_active());
    assert_native_curve_row(&task, false);
    task.intersector().is_done()
}

#[test]
fn worker_task_cancel_records_cause_after_the_original_hook_clears() {
    let (flag, indicator) = toggle_indicator();
    let range = indicator.start();
    let observed = crate::bop_algo::ProgressCancellation::default();
    let mut task = configured_task(false);
    task.perform_with_range(&range, &observed).unwrap();
    assert_before_task_cancel(&task);
    flag.store(false, std::sync::atomic::Ordering::Release);
    assert!(
        observed.is_cancelled(),
        "actual stop must outlive a non-sticky hook"
    );
    assert!(!task.parallel_base().progress_range().is_active());
}

#[test]
fn terminal_task_packet_preserves_coordinator_preparation_and_precision_transform() {
    use super::face_face_task::FaceFaceTaskOutput;
    static_assertions::assert_impl_all!(FaceFaceTaskOutput: Send);
    for far in [false, true] {
        let packet = std::thread::spawn(move || completed_task_packet(far))
            .join()
            .unwrap();
        assert_eq!(
            packet.completion,
            crate::bop_algo::ProgressCompletion::Completed
        );
        let mut task = packet.into_task();
        task.intersector_mut().prepare_lines_3d(false).unwrap();
        task.apply_trsf().unwrap();
        assert_native_curve_row(&task, far);
    }
}

fn completed_task_packet(far: bool) -> super::face_face_task::FaceFaceTaskOutput {
    let mut task = configured_task(far);
    task.perform_parallel().unwrap();
    assert!(task.intersector().is_done());
    task.into_output(&crate::bop_algo::ProgressCancellation::default(), 0)
}

#[test]
fn terminal_task_packet_retains_actual_cancel_report_after_worker_drop() {
    let (_, indicator) = toggle_indicator();
    let range = indicator.start();
    let scope = martensite_tkernel::message::ProgressScope::unnamed(&range, 1.0, false);
    let packet = std::thread::scope(|workers| {
        workers
            .spawn(|| cancelled_task_packet(&scope))
            .join()
            .unwrap()
    });
    assert!(matches!(
        packet.completion,
        crate::bop_algo::ProgressCompletion::Cancelled { .. }
    ));
    let task = packet.into_task();
    assert_before_task_cancel(&task);
}

fn cancelled_task_packet(
    scope: &martensite_tkernel::message::ProgressScope<'_>,
) -> super::face_face_task::FaceFaceTaskOutput {
    use crate::bop_algo::ProgressCancellation;
    let observed = ProgressCancellation::default();
    let range = scope.next(1.0);
    let mut task = configured_task(false);
    task.perform_with_range(&range, &observed).unwrap();
    assert!(observed.is_cancelled());
    task.into_output(&observed, 0)
}

#[test]
fn prepared_task_input_preserves_actual_policy_and_ordered_pair() {
    static_assertions::assert_impl_all!(super::face_face_task::FaceFaceTaskInput: Send);
    let mut task = prepared_plane_input().into_task();
    assert_eq!(task.indices(), [12, 21]);
    assert_eq!(task.tol_ff(), 0.125);
    assert_eq!(task.intersector().fuzzy_value(), 0.00025);
    task.perform_parallel().unwrap();
    let curves = task.intersector().lines().unwrap();
    assert_eq!(curves.len(), 1);
    assert!(curves[0].first_curve2d().is_null());
    assert!(!curves[0].second_curve2d().is_null());
}

/// Supplies one owned finite-plane task input for the shared execution controls.
/// Example: `let input = prepared_plane_input();`.
pub(super) fn prepared_plane_input() -> super::face_face_task::FaceFaceTaskInput {
    super::face_face_task::FaceFaceTaskInput {
        indices: [12, 21],
        faces: near_plane_faces(),
        boxes: near_plane_boxes(),
        tolerance: 0.125,
        seeds: Vec::new(),
        parameters: [true, false, true],
        fuzzy: 0.00025,
        parallel: false,
    }
}

#[test]
fn terminal_failure_packet_preserves_typed_kernel_cause_and_native_alert() {
    let packet = std::thread::spawn(failed_task_packet).join().unwrap();
    assert_kernel_failure_completion(&packet.completion);
    let task = packet.into_task();
    assert!(task.parallel_options().has_errors());
    assert!(!task.intersector().is_done());
}

fn failed_task_packet() -> super::face_face_task::FaceFaceTaskOutput {
    let mut task = BOPAlgoFaceFaceTask::default();
    task.perform_parallel().unwrap();
    task.into_output(&crate::bop_algo::ProgressCancellation::default(), 3)
}

fn assert_kernel_failure_completion(completion: &crate::bop_algo::ProgressCompletion) {
    use crate::bop_algo::{ProgressCompletion, ProgressFailureClass};
    let expected = martensite_tkernel::standard::Failure::with_type(
        "Standard_ConstructionError",
        "Bnd_Box is void",
        "",
    );
    assert!(
        matches!(completion, ProgressCompletion::Failed {
        class: ProgressFailureClass::KernelFailure, cause: Some(cause), ..
    } if cause == &expected),
        "typed task completion {:?}, expected original kernel cause {expected:?}",
        completion
    );
}

#[test]
fn owned_kernel_failure_wins_over_an_already_observed_cancellation() {
    let mut task = BOPAlgoFaceFaceTask::default();
    task.perform_parallel().unwrap();
    let observed = crate::bop_algo::ProgressCancellation::default();
    observed.record_cancelled();
    let packet = task.into_output(&observed, 3);
    assert_kernel_failure_completion(&packet.completion);
    assert!(packet.into_task().parallel_options().has_errors());
}

#[test]
fn a_new_cancelled_attempt_does_not_reuse_the_previous_kernel_cause() {
    let mut task = BOPAlgoFaceFaceTask::default();
    task.perform_parallel().unwrap();
    task.parallel_options().clear();
    let (_, indicator) = toggle_indicator();
    let range = indicator.start();
    let observed = crate::bop_algo::ProgressCancellation::default();
    task.perform_with_range(&range, &observed).unwrap();
    let packet = task.into_output(&observed, 0);
    assert!(matches!(
        packet.completion,
        crate::bop_algo::ProgressCompletion::Cancelled { .. }
    ));
    assert_before_task_cancel(&packet.into_task());
}
