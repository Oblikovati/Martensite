// SPDX-License-Identifier: GPL-2.0-only
//! Stage callbacks can refuse; they cannot manufacture public certificate authority.
use super::*;
use crate::body_check::StageVerdict;
use crate::profile_extrusion::{DisplayTolerance, ToleranceCap};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::cell::Cell;
fn member() -> ConicalNativeMember {
    let shape = BRepPrimAPIMakeBox::from_sizes(1., 2., 3.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    ConicalNativeMember {
        id: ConicalActualMemberId(19),
        shape,
        sides: Vec::new(),
        caps: Vec::new(),
    }
}
fn resolution() -> Resolution {
    Resolution::new(
        1e-6,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(1e-5),
    )
}
fn incomplete() -> ConicalBindingError {
    ConicalBindingError {
        member: Some(ConicalActualMemberId(19)),
        side: None,
        cap: None,
        cause: native::rejected("independent ordered-stage refusal control"),
    }
}
#[test]
fn unavailable_certificate_runs_after_geometry_and_blocks_tolerance() {
    let member = member();
    let calls = Cell::new(0);
    let report = validate_member(&member, &resolution(), || {
        calls.set(calls.get() + 1);
        Err(incomplete())
    })
    .unwrap();
    assert_eq!(calls.get(), 1);
    assert_earlier_passed(&report);
    assert_si_refused(&report);
    assert_eq!(
        report.stage(ValidationStage::Tolerance).verdict,
        StageVerdict::NotRun
    );
    assert!(!report.is_valid());
    assert!(!report.is_exact());
}
#[test]
fn invalid_resolution_never_invokes_required_certificate() {
    let member = member();
    let mut invalid = resolution();
    invalid.weld = f64::NAN;
    let calls = Cell::new(0);
    let result = validate_member(&member, &invalid, || {
        calls.set(calls.get() + 1);
        Err(incomplete())
    });
    assert!(result.is_err());
    assert_eq!(calls.get(), 0);
}
#[test]
fn missing_native_faces_cannot_produce_a_bound_family_snapshot() {
    let member = member();
    let error = capture_conical_actual_family(&[member]).err().unwrap();
    assert_eq!(error.member, Some(ConicalActualMemberId(19)));
    assert!(error.cause.to_string().contains("face count differs"));
}

fn assert_earlier_passed(report: &ValidationReport) {
    assert_eq!(
        report.stage(ValidationStage::Topology).verdict,
        StageVerdict::Passed
    );
    assert_eq!(
        report.stage(ValidationStage::Geometry).verdict,
        StageVerdict::Passed
    );
    assert_eq!(
        report.stage(ValidationStage::Geometry).mode,
        CheckMode::Sampled
    );
}
fn assert_si_refused(report: &ValidationReport) {
    let stage = report.stage(ValidationStage::SelfIntersection);
    assert_eq!(stage.verdict, StageVerdict::Inconclusive);
    assert_eq!(stage.mode, CheckMode::Exact);
    assert_eq!(stage.findings.len(), 1);
}
#[test]
fn published_numeric_authority_is_send_and_sync() {
    fn traits<T: Send + Sync>() {}
    traits::<CertifiedConicalBody>();
    traits::<CertifiedPlanarMaterial>();
}

#[test]
fn conical_public_inputs_and_integral_receipts_are_send_and_sync() {
    fn traits<T: Send + Sync>() {}
    traits::<crate::body_check::ConicalNativeCensus>();
    traits::<ConicalNativeMember>();
    traits::<ConicalNativeCap>();
    traits::<ConicalBindingError>();
    traits::<ConicalNativeGeometry>();
    traits::<crate::profile_extrusion::PlanarIntegralEvidence>();
    traits::<crate::profile_extrusion::PlanarIntegralFailure>();
    traits::<crate::profile_extrusion::PlanarIntegralPolicyFailure>();
}
