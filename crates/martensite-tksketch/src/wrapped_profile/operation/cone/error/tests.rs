//! Typed public projection keeps admission, work, cancellation and actual offenders distinct.
use super::*;
use martensite_tkbo::profile_extrusion::{
    DeclineCause, OperationDecline, OperationKind, OperationStage, ProfileCurveId,
};
use std::error::Error;
fn projected(cause: ConeCause) -> WrappedNormalFailure {
    ConeFailure::new(None, None, WrappedFailureClass::Incomplete, cause).public()
}
#[test]
fn native_and_unsupported_failures_override_generic_incomplete_context() {
    let native = projected(ConeCause::Native(Box::new(Failure::new(
        "owned native control",
    ))));
    assert_eq!(native.class(), WrappedFailureClass::Native);
    assert!(find_source::<Failure>(&native).is_some());
    let unsupported = projected(ConeCause::Candidate(cone_mapping::fit::FitFailure::Count(
        7,
    )));
    assert_eq!(unsupported.class(), WrappedFailureClass::Unsupported);
}
#[test]
fn global_planar_decline_reports_only_its_actual_curve_and_public_source() {
    let decline = OperationDecline {
        operation: OperationKind::BuildPlanarRegions,
        stage: OperationStage::Admission,
        cause: DeclineCause::UnsupportedCurve {
            curve: ProfileCurveId(73),
            kind: "control",
        },
    };
    let failure = projected(ConeCause::Planar(Box::new(planar::PlanarFailure::Capture(
        Box::new(decline),
    ))));
    assert_eq!(failure.class(), WrappedFailureClass::Unsupported);
    assert_eq!(
        failure.entities(),
        &[HistorySource::Curve(SurfaceCurveId(73))]
    );
    assert!(find_source::<OperationDecline>(&failure).is_some());
}
#[test]
fn public_math_sources_distinguish_invalid_input_exhaustion_and_cancellation() {
    let invalid = projected(ConeCause::Radius(Box::new(radius_failure(
        f64::NAN,
        100,
        false,
    ))));
    assert_eq!(invalid.class(), WrappedFailureClass::InvalidInput);
    assert!(find_source::<ConeRequestError>(&invalid).is_some());
    let exhausted = projected(ConeCause::Radius(Box::new(radius_failure(0.7, 0, false))));
    assert_eq!(exhausted.class(), WrappedFailureClass::Incomplete);
    let cancelled = projected(ConeCause::Radius(Box::new(radius_failure(0.7, 100, true))));
    assert_eq!(cancelled.class(), WrappedFailureClass::Cancelled);
}
fn radius_failure(angle: f64, maximum: u32, cancelled: bool) -> ConeRequestError {
    let query = ConeRequest {
        poles: &[[4., 0.], [5., 0.]],
        semi_angle: angle,
        depths: [-0.1, 0.2],
        outward: true,
        clearance: 0.,
        max_depth: 0,
        options: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: maximum,
        },
    };
    certify_cone_request(query, || cancelled).unwrap_err()
}
fn find_source<'a, T: Error + 'static>(failure: &'a (dyn Error + 'static)) -> Option<&'a T> {
    let mut current = Some(failure);
    while let Some(error) = current {
        if let Some(found) = error.downcast_ref::<T>() {
            return Some(found);
        }
        current = error.source();
    }
    None
}

#[test]
fn cone_getter_and_error_chain_retain_the_real_planar_work_failure() {
    let request = super::super::tests::request();
    let inputs = input::capture(&request, &mut || false).unwrap();
    let cause = planar::capture_with_work_and_cancel(
        &inputs.originals,
        request.options.spatial_error,
        request.options.planar_work,
        || true,
    )
    .err()
    .unwrap();
    let failure = projected(ConeCause::Planar(Box::new(cause)));
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
    assert!(failure.planar_work_failure().is_some());
    assert!(
        find_source::<martensite_tkbo::profile_extrusion::PlanarRegionWorkFailure>(&failure)
            .is_some()
    );
}

#[test]
fn reference_gate_refusal_keeps_its_public_source_and_incomplete_class() {
    let cause = ConicalReferenceError {
        offender: Some([0, 3]),
        operations: 17,
        cause: ConicalReferenceFailure::Bound {
            field: "reference control",
            actual: 2.,
            limit: 1.,
        },
    };
    let failure = projected(ConeCause::Reference(Box::new(cause)));
    assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
    assert!(find_source::<ConicalReferenceError>(&failure).is_some());
}
#[test]
fn reference_volume_projection_distinguishes_input_shape_and_cancelled_work() {
    let invalid = projected(volume_failure(ConicalReferenceVolumeFailure::Input {
        role: "control",
        index: 0,
        value: f64::NAN,
    }));
    assert_eq!(invalid.class(), WrappedFailureClass::InvalidInput);
    assert!(find_source::<ConicalReferenceVolumeError>(&invalid).is_some());
    let unsupported = projected(volume_failure(ConicalReferenceVolumeFailure::Shape {
        role: "control",
        count: 0,
        expected: "one span",
    }));
    assert_eq!(unsupported.class(), WrappedFailureClass::Unsupported);
    let cancelled = projected(volume_failure(ConicalReferenceVolumeFailure::Arithmetic(
        Box::new(BoundedArithmeticFailure::Cancelled),
    )));
    assert_eq!(cancelled.class(), WrappedFailureClass::Cancelled);
}
fn volume_failure(cause: ConicalReferenceVolumeFailure) -> ConeCause {
    ConeCause::ReferenceVolume(Box::new(ConicalReferenceVolumeError {
        span: Some(3),
        depth: None,
        cause: Box::new(cause),
        operations: 0,
        integration_subintervals: 0,
        requested_options: Default::default(),
        requested_arithmetic: Default::default(),
    }))
}

#[test]
fn private_cone_failure_keeps_a_bounded_stack_representation() {
    let failure_bytes = std::mem::size_of::<ConeFailure>();
    assert!(
        failure_bytes < 128,
        "private cone failure bytes={failure_bytes}"
    );
    eprintln!(
        r#"{{"cone_failure_bytes":{failure_bytes},"cone_cause_bytes":{},"cone_support_bytes":{},"cone_proof_options_bytes":{},"cone_sector_failure_bytes":{}}}"#,
        std::mem::size_of::<ConeCause>(),
        std::mem::size_of::<ConeSupport>(),
        std::mem::size_of::<WrappedConeProofOptions>(),
        std::mem::size_of::<ConicalSectorFailure>(),
    );
}

#[test]
fn boxed_sector_keeps_public_leaf_identity_and_cancellation() {
    let invalid = projected(ConeCause::Sector(Box::new(ConicalSectorFailure::Angle(0.))));
    assert_eq!(invalid.class(), WrappedFailureClass::InvalidInput);
    let leaf = find_source::<ConicalSectorFailure>(&invalid).unwrap();
    assert!(matches!(leaf, ConicalSectorFailure::Angle(0.)));
    let cancelled = ConicalSectorFailure::Arithmetic(BoundedArithmeticFailure::Cancelled);
    let failure = projected(ConeCause::Sector(Box::new(cancelled)));
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
    assert!(find_source::<ConicalSectorFailure>(&failure).is_some());
}

mod boundary;
