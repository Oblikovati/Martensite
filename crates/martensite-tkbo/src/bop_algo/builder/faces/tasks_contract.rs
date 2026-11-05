//! Closed-form planar tasks and diagnostic-only cancellation qualify the actual worker wrapper.
use super::*;
use crate::bop_algo::progress_test_support::{indicator, pulse_indicator};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Plane3;
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeFace, brep_gprop::BRepGProp};

fn planar_task(width: f64) -> BuilderFace {
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., width, 0., width))
        .unwrap()
        .face()
        .unwrap()
        .clone();
    let edges: List<_> = Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .collect();
    let mut builder = BuilderFace::new();
    builder.set_face(&face);
    builder.set_shapes(&edges);
    builder
}

fn assert_planar_area(builder: &BuilderFace, expected: f64) {
    assert_eq!(builder.areas().len(), 1);
    let shape = builder.areas().iter().next().unwrap();
    let mut props = GProps::default();
    BRepGProp::surface_properties(shape, &mut props, false, false).unwrap();
    assert!((props.mass() - expected).abs() < 1.0e-10);
}

#[test]
fn serial_and_parallel_tasks_preserve_original_order_and_closed_form_areas() {
    for parallel in [false, true] {
        let observed = ProgressCancellation::default();
        let (progress, _) = indicator(0);
        let (builders, completion) = perform_face_tasks(
            vec![planar_task(2.), planar_task(3.)],
            parallel,
            &progress.start(),
            &observed,
        )
        .unwrap();
        assert_eq!(completion, ProgressCompletion::Completed);
        assert!(!observed.is_cancelled());
        assert_planar_area(&builders[0], 4.);
        assert_planar_area(&builders[1], 9.);
    }
}

#[test]
fn worker_more_cancellation_remains_silent_but_typed_non_success() {
    let observed = ProgressCancellation::default();
    let (progress, _) = pulse_indicator(1);
    let (builders, completion) = perform_face_tasks(
        vec![BuilderFace::new()],
        false,
        &progress.start(),
        &observed,
    )
    .unwrap();
    assert_eq!(completion, worker_cancelled());
    assert!(observed.is_cancelled());
    assert!(!builders[0].options().has_errors());
    assert!(builders[0].areas().is_empty());
}

#[test]
fn actual_face_nonsticky_cancellation_is_carried_without_enum_reconstruction() {
    let observed = ProgressCancellation::default();
    let (progress, _) = pulse_indicator(2);
    let (_, completion) =
        perform_face_tasks(vec![planar_task(2.)], false, &progress.start(), &observed).unwrap();
    assert!(matches!(
        completion,
        ProgressCompletion::Cancelled {
            operation: ProgressOperation::BuilderFace,
            stage: ProgressStage::Completion,
            context: None,
        }
    ));
    assert!(observed.is_cancelled());
}

#[test]
fn failure_cause_survives_both_ordered_collection_and_later_cancel() {
    let failed = confirmed_domain_failure();
    assert_ordered_failure_carry(failed);
}

#[test]
fn unexecuted_task_never_becomes_completed_empty() {
    let task = SplitFaceTask {
        face: BuilderFace::new(),
        progress: None,
        completion: None,
    };
    let (_, completion) = collect_task_outcomes(vec![task]);
    assert_eq!(completion, unexecuted_task());
    assert_eq!(
        prefer_confirmed_failure(worker_cancelled(), completion.clone()),
        completion
    );
}

#[test]
fn consumed_task_scope_refuses_duplicate_dispatch_with_typed_cause() {
    let mut task = SplitFaceTask {
        face: BuilderFace::new(),
        progress: None,
        completion: None,
    };
    let error = task.perform(&ProgressCancellation::default()).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_ProgramError");
    assert!(error.message().contains("progress=None"));
}

fn confirmed_domain_failure() -> ProgressCompletion {
    let cause = Failure::with_message(
        "Standard_DomainError",
        "input=-1; expected valid face domain",
    );
    ProgressCompletion::Failed {
        operation: ProgressOperation::BuilderFace,
        stage: ProgressStage::Preparation,
        class: ProgressFailureClass::KernelFailure,
        cause: Some(cause),
        context: None,
    }
}

fn assert_ordered_failure_carry(failed: ProgressCompletion) {
    let tasks = vec![
        recorded_task(worker_cancelled()),
        recorded_task(failed.clone()),
    ];
    let (_, actual) = collect_task_outcomes(tasks);
    assert_eq!(actual, failed);
    assert_eq!(
        prefer_confirmed_failure(failed.clone(), worker_cancelled()),
        failed
    );
    assert_eq!(
        super::super::progress::cancelled_after_children(failed.clone()),
        failed
    );
}

fn recorded_task(completion: ProgressCompletion) -> SplitFaceTask<'static> {
    SplitFaceTask {
        face: BuilderFace::new(),
        progress: None,
        completion: Some(completion),
    }
}

fn malformed_surface_task() -> BuilderFace {
    let mut task = planar_task(2.);
    let mut face = martensite_tkbrep::topods::Face::default();
    martensite_tkbrep::brep::BRepBuilder::default().make_face(&mut face);
    task.set_face(&face);
    task
}

fn assert_malformed_task_throws(parallel: bool) {
    let original_cause = direct_malformed_cause(parallel);
    let cause = malformed_task_result(parallel)
        .err()
        .expect("task result=Ok; expected the actual missing-surface Failure");
    assert_eq!(cause, original_cause);
    assert_eq!(cause.exception_type(), "Standard_NullObject");
    assert!(cause.message().contains("GeomAdaptor_Surface::Load"));
    record_thrown_failure(parallel, &cause);
}

fn malformed_task_result(
    parallel: bool,
) -> Result<(Vec<BuilderFace>, ProgressCompletion), Failure> {
    let observed = ProgressCancellation::default();
    let mut task = malformed_surface_task();
    task.options_mut().set_run_parallel(parallel);
    perform_face_tasks(
        vec![task],
        parallel,
        &martensite_tkernel::message::ProgressRange::default(),
        &observed,
    )
}

fn record_thrown_failure(parallel: bool, cause: &Failure) {
    let message_hex: String = cause
        .what()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    println!(
        r#"{{"parallel":{parallel},"threw":true,"type":"{}","message_hex":"{message_hex}"}}"#,
        cause.exception_type()
    );
}

#[test]
fn malformed_surface_serial_task_preserves_original_throw_boundary() {
    let parallel = false;
    assert_malformed_task_throws(parallel);
}

#[test]
fn malformed_surface_parallel_task_uses_existing_launcher_error_boundary() {
    let parallel = true;
    assert_malformed_task_throws(parallel);
}

fn direct_malformed_cause(parallel: bool) -> Failure {
    let mut task = malformed_surface_task();
    task.options_mut().set_run_parallel(parallel);
    let range = martensite_tkernel::message::ProgressRange::default();
    match task.perform_with_progress(&range) {
        ProgressCompletion::Failed {
            class: ProgressFailureClass::KernelFailure,
            cause: Some(cause),
            ..
        } => cause,
        actual => {
            panic!("direct malformed Face={actual:?}; expected actual typed KernelFailure cause")
        }
    }
}
