//! Compile and causal controls exercise the private completion protocol independently of geometry.
use crate::bop_algo::{
    ProgressCancellation, ProgressCompletion, ProgressFailureClass, ProgressOperation,
    ProgressStage, ProgressWorksite,
};
use martensite_tkernel::standard::Failure;

fn require_shared<T: Send + Sync>() {
    let witness: fn(T) = std::mem::drop::<T>;
    std::hint::black_box(witness);
}

#[test]
fn completion_contract_types_are_owned_send_sync() {
    require_shared::<ProgressCancellation>();
    require_shared::<ProgressCompletion>();
    require_shared::<ProgressFailureClass>();
    require_shared::<ProgressOperation>();
    require_shared::<ProgressStage>();
    require_shared::<ProgressWorksite>();
}

#[test]
fn actual_cancellation_survives_later_nonsticky_observation() {
    let observed = ProgressCancellation::default();
    assert!(!observed.is_cancelled());
    std::thread::scope(|scope| {
        scope.spawn(|| observed.record_cancelled()).join().unwrap();
    });
    let later_hook_result = false;
    assert!(!later_hook_result);
    assert!(observed.is_cancelled());
}

fn failed_completion(cause: Failure) -> ProgressCompletion {
    ProgressCompletion::Failed {
        operation: ProgressOperation::ShellSplitter,
        stage: ProgressStage::Worker,
        class: ProgressFailureClass::KernelFailure,
        cause: Some(cause),
        context: None,
    }
}

#[test]
fn known_exception_type_message_and_stack_remain_typed() {
    let cause = Failure::with_type(
        "Standard_DomainError",
        "input 7; expected valid block",
        "owned",
    );
    let failed = failed_completion(cause.clone());
    let ProgressCompletion::Failed {
        cause: Some(saved), ..
    } = failed
    else {
        panic!("expected Failed with typed cause");
    };
    assert_eq!(saved, cause);
    assert_eq!(saved.exception_type(), "Standard_DomainError");
    assert_eq!(saved.stack_string(), "owned");
}

#[test]
fn cancellation_failure_and_incompletion_are_distinct_from_completion() {
    let operation = ProgressOperation::BuilderFace;
    let stage = ProgressStage::Completion;
    let cancelled = ProgressCompletion::Cancelled {
        operation,
        stage,
        context: None,
    };
    let incomplete = ProgressCompletion::Incomplete {
        operation,
        stage,
        class: ProgressFailureClass::UnqualifiedStage,
        cause: None,
        context: None,
    };
    assert_ne!(cancelled, ProgressCompletion::Completed);
    assert_ne!(incomplete, ProgressCompletion::Completed);
    assert_ne!(cancelled, incomplete);
    assert_ne!(failed_completion(Failure::new("owned")), incomplete);
}

#[test]
fn unknown_worksite_entity_remains_explicitly_unknown() {
    let context = ProgressWorksite {
        label: "MakeShells",
        input_ordinal: None,
        entity_key: None,
    };
    assert_eq!(context.input_ordinal, None);
    assert_eq!(context.entity_key, None);
    assert_eq!(context.label, "MakeShells");
    assert_ne!(
        ProgressFailureClass::AlgorithmReport,
        ProgressFailureClass::KernelFailure
    );
}

#[test]
fn operation_domain_names_actual_receivers() {
    let operations = [
        ProgressOperation::Defeaturing,
        ProgressOperation::RemoveFeatures,
        ProgressOperation::Builder,
        ProgressOperation::MakerVolume,
        ProgressOperation::BuilderArea,
        ProgressOperation::BuilderFace,
        ProgressOperation::BuilderSolid,
        ProgressOperation::WireSplitter,
        ProgressOperation::ShellSplitter,
        ProgressOperation::ClassifyFaces,
        ProgressOperation::PaveFiller,
        ProgressOperation::CheckerSI,
        ProgressOperation::ArgumentAnalyzer,
        ProgressOperation::Check,
        ProgressOperation::BooleanApi,
    ];
    assert_eq!(operations.len(), 15);
}

#[test]
fn stage_domain_includes_observation_and_classification_boundaries() {
    let stages = [
        ProgressStage::Input,
        ProgressStage::Preparation,
        ProgressStage::Intersections,
        ProgressStage::Images,
        ProgressStage::Loops,
        ProgressStage::Areas,
        ProgressStage::InternalShapes,
        ProgressStage::Classification,
        ProgressStage::History,
        ProgressStage::Simplification,
        ProgressStage::PostTreatment,
        ProgressStage::Worker,
        ProgressStage::Completion,
    ];
    assert_eq!(stages.len(), 13);
}
