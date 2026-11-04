//! Ordinary CPU controls refuse partial driver metadata without requiring a Vulkan loader.
use super::{
    admission, creation_flags,
    fake_api::{FakeExecutableApi, Fault},
    query,
    records::ExecutableGap,
};
use ash::{vk, vk::Handle};

#[test]
fn complete_reader_owns_exact_ir_bytes_after_fake_api_drops() {
    let (witness, calls) = owned_witness_after_fake_scope();
    assert_eq!(calls, 2);
    assert_eq!(witness.pipeline, 41);
    assert_eq!(witness.selected, 0);
    assert_eq!(witness.representations[0].bytes, b"ps code\0");
}
#[test]
fn descriptor_and_byte_faults_never_become_complete_witnesses() {
    for fault in property_faults().into_iter().chain(ir_faults()) {
        let mut fake = FakeExecutableApi::new(fault);
        let actual = query::capture(
            &mut fake,
            vk::Pipeline::from_raw(42),
            &mut super::CaptureByteBudget::default(),
        );
        require_typed_gap(fault, actual);
    }
}
fn property_faults() -> [Fault; 8] {
    [
        Fault::PropertiesError,
        Fault::PropertiesIncomplete,
        Fault::ZeroProperties,
        Fault::TooManyProperties,
        Fault::NoFragment,
        Fault::MixedStages,
        Fault::DuplicateFragments,
        Fault::UnterminatedName,
    ]
}
fn ir_faults() -> [Fault; 13] {
    [
        Fault::IrIncomplete,
        Fault::ZeroIr,
        Fault::TooManyIr,
        Fault::ZeroBytes,
        Fault::OversizedBytes,
        Fault::OverflowBytes,
        Fault::ChangedCount,
        Fault::ChangedMetadata,
        Fault::ChangedSize,
        Fault::ByteReadError,
        Fault::InvalidText,
        Fault::UnterminatedText,
        Fault::InvalidTextFlag,
    ]
}
#[test]
fn absent_assembly_retains_actual_ir_in_typed_gap() {
    let mut fake = FakeExecutableApi::new(Fault::NoAssembly);
    let actual = query::capture(
        &mut fake,
        vk::Pipeline::from_raw(43),
        &mut super::CaptureByteBudget::default(),
    );
    let gap = actual.unwrap_err();
    let ExecutableGap::MissingAssembly(witness) = query_cause(&gap) else {
        panic!("actual={gap:?}; expected MissingAssembly");
    };
    assert_eq!(witness.representations[0].bytes, b"ps code\0");
    assert_eq!(witness.representations[0].metadata.name, "NIR");
}
#[test]
fn binary_output_is_preserved_without_utf8_conversion() {
    let mut fake = FakeExecutableApi::new(Fault::BinaryExtra);
    let witness = query::capture(
        &mut fake,
        vk::Pipeline::from_raw(44),
        &mut super::CaptureByteBudget::default(),
    )
    .unwrap();
    assert!(!witness.representations[1].metadata.text);
    assert_eq!(witness.representations[1].bytes, [255, 1, 2, 3, 4, 5, 6, 0]);
}
#[test]
fn extension_and_feature_admission_refuse_without_device_substitution() {
    let physical = vk::PhysicalDevice::from_raw(51);
    for (extension, feature) in [(false, vk::FALSE), (true, vk::FALSE)] {
        let actual = admission::require(physical, extension, feature);
        assert!(
            matches!(actual, Err(ExecutableGap::Capability { physical: 51, .. })),
            "actual={actual:?}; expected typed unsupported selected physical51"
        );
    }
    assert_eq!(
        admission::require(physical, true, vk::TRUE)
            .unwrap()
            .physical,
        physical
    );
}
#[test]
fn disabled_capture_preserves_every_original_pipeline_flag() {
    let original =
        vk::PipelineCreateFlags::ALLOW_DERIVATIVES | vk::PipelineCreateFlags::DISABLE_OPTIMIZATION;
    assert_eq!(creation_flags(false, original), original);
    assert_eq!(
        creation_flags(true, original),
        original | vk::PipelineCreateFlags::CAPTURE_INTERNAL_REPRESENTATIONS_KHR
    );
    assert_eq!(
        creation_flags(false, vk::PipelineCreateFlags::empty()),
        vk::PipelineCreateFlags::empty()
    );
}

#[test]
fn aggregate_ir_budget_refuses_instead_of_discarding_later_bytes() {
    let mut budget = super::CaptureByteBudget::default();
    budget.charge(8 * 1024 * 1024).unwrap();
    budget.charge(8 * 1024 * 1024).unwrap();
    assert!(budget.charge(1).is_err());
    assert_eq!(budget.used, 16 * 1024 * 1024);
}

fn require_typed_gap(fault: Fault, actual: Result<super::ExecutableWitness, ExecutableGap>) {
    let actual = observed_gap_kind(actual);
    let expected = expected_gap_kind(fault);
    assert_eq!(
        actual, expected,
        "fault={fault:?}; expected original typed refusal class/cause"
    );
}
#[derive(Debug, PartialEq, Eq)]
enum GapKind {
    Api(&'static str, vk::Result),
    Stage,
    Contract(&'static str),
}
fn observed_gap_kind(actual: Result<super::ExecutableWitness, ExecutableGap>) -> GapKind {
    let gap = actual.unwrap_err();
    match query_cause(&gap) {
        ExecutableGap::Api {
            operation,
            pipeline,
            result,
        } => api_gap_kind(operation, *pipeline, *result),
        ExecutableGap::AmbiguousStage(rows) => {
            assert!(!rows.is_empty());
            GapKind::Stage
        }
        ExecutableGap::Contract {
            field,
            actual,
            expected,
        } => contract_gap_kind(field, actual, expected),
        other => panic!("actual={other:?}; expected typed API/stage/contract gap"),
    }
}
fn query_cause(gap: &ExecutableGap) -> &ExecutableGap {
    match gap {
        ExecutableGap::PartialQuery { cause, .. } | ExecutableGap::BoundPipeline { cause, .. } => {
            query_cause(cause)
        }
        _ => gap,
    }
}

fn api_gap_kind(operation: &'static str, pipeline: u64, result: vk::Result) -> GapKind {
    assert_eq!(pipeline, 42);
    // This checks the original status/phase rather than accepting any error.
    GapKind::Api(operation, result)
}
fn contract_gap_kind(field: &'static str, actual: &str, expected: &str) -> GapKind {
    assert!(!actual.is_empty() && !expected.is_empty());
    // Distinct malformed-input controls require the actual rejecting field.
    GapKind::Contract(field)
}
fn expected_gap_kind(fault: Fault) -> GapKind {
    match fault {
        Fault::PropertiesError => {
            GapKind::Api("executable properties", vk::Result::ERROR_DEVICE_LOST)
        }
        Fault::PropertiesIncomplete => {
            GapKind::Api("executable properties", vk::Result::INCOMPLETE)
        }
        Fault::IrIncomplete => GapKind::Api("IR descriptors", vk::Result::INCOMPLETE),
        Fault::ByteReadError => GapKind::Api("IR bytes", vk::Result::ERROR_DEVICE_LOST),
        Fault::NoFragment | Fault::MixedStages | Fault::DuplicateFragments => GapKind::Stage,
        _ => GapKind::Contract(expected_contract_field(fault)),
    }
}
fn expected_contract_field(fault: Fault) -> &'static str {
    match fault {
        Fault::ZeroProperties | Fault::TooManyProperties | Fault::ZeroIr | Fault::TooManyIr => {
            "written count"
        }
        Fault::UnterminatedName => "C string",
        Fault::ZeroBytes | Fault::OversizedBytes => "IR byte sizes",
        Fault::OverflowBytes => "IR sizes",
        Fault::ChangedCount => "IR count",
        Fault::ChangedMetadata | Fault::ChangedSize => "IR metadata",
        Fault::InvalidText => "text UTF-8",
        Fault::UnterminatedText => "text bytes",
        Fault::InvalidTextFlag => "isText",
        other => panic!("fault={other:?}; expected contract-refusal fixture"),
    }
}
#[test]
fn backend_failure_preserves_the_original_renderer_error() {
    let original = crate::RenderError::rejected(17, "admitted device option");
    let wrapped = ExecutableGap::from(original.clone());
    let ExecutableGap::Backend(actual) = wrapped else {
        panic!("actual={wrapped:?}; expected Backend cause");
    };
    assert_eq!(actual, original);
}

#[test]
fn bound_pipeline_gap_keeps_the_original_missing_ir_owner() {
    let mut fake = FakeExecutableApi::new(Fault::NoAssembly);
    let cause = query::capture(
        &mut fake,
        vk::Pipeline::from_raw(63),
        &mut super::CaptureByteBudget::default(),
    )
    .unwrap_err();
    let gap = ExecutableGap::BoundPipeline {
        pipeline: 63,
        cause: Box::new(cause),
    };
    let actual = super::missing_witness(&gap).unwrap();
    assert_eq!(actual.pipeline, 63);
    assert_eq!(actual.representations[0].bytes, b"ps code\0");
}
#[test]
fn owned_executable_results_are_send_and_sync_without_extra_unsafe_impls() {
    let _: &(dyn Send + Sync) = &std::marker::PhantomData::<super::ExecutableWitness>;
    let _: &(dyn Send + Sync) = &std::marker::PhantomData::<super::CapturedExecutableFrame>;
    let _: &(dyn Send + Sync) = &std::marker::PhantomData::<super::ExecutableGap>;
}

fn owned_witness_after_fake_scope() -> (super::ExecutableWitness, usize) {
    let mut fake = FakeExecutableApi::new(Fault::None);
    let witness = query::capture(
        &mut fake,
        vk::Pipeline::from_raw(41),
        &mut super::CaptureByteBudget::default(),
    )
    .unwrap();
    (witness, fake.ir_calls)
}
