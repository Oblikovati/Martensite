//! Time the conditioned attachment through the public entry point.
use super::*;
use martensite_tkbo::body_check::validate_body;
use martensite_tkbrep::topexp::Explorer;
use martensite_tksketch::surface_sketch::{
    CertifiedConeFeatureResult, CylinderFeatureCause, CylinderFeatureSource, CylinderFeatureStage,
    WrappedVolumePolicy,
};
use std::time::Instant;

#[test]
#[ignore = "qualified 239-curve attachment latency fixture"]
fn multiline_cone_certified_boss() {
    cold(WrappedFeatureKind::Boss, NormalSense::Outward);
}

#[test]
#[ignore = "qualified 239-curve attachment latency fixture"]
fn multiline_cone_certified_pocket() {
    cold(WrappedFeatureKind::Pocket, NormalSense::Inward);
}

fn cold(kind: WrappedFeatureKind, normal: NormalSense) {
    let attachment = text_attachment();
    let record = attachment.record().clone();
    let request = attached_request(&attachment, normal);
    let original = request.clone();
    let start = Instant::now();
    let result = apply(&attachment, kind, &request);
    eprintln!(
        r#"{{"event":"certified_cone_cold","kind":"{kind:?}","seconds":{}}}"#,
        start.elapsed().as_secs_f64()
    );
    assert_eq!(attachment.record(), &record);
    assert_eq!(request, original);
    check_attachment(&attachment, &request, &result);
}

#[test]
#[ignore = "qualified 239-curve attachment depth-edit latency fixture"]
fn multiline_cone_certified_depth_edit() {
    let attachment = text_attachment();
    let mut request = attached_request(&attachment, NormalSense::Outward);
    let before = apply(&attachment, WrappedFeatureKind::Boss, &request);
    let bytes = before.encoded_brep().to_vec();
    request.depths[1] *= 0.98;
    let start = Instant::now();
    let after = apply(&attachment, WrappedFeatureKind::Boss, &request);
    eprintln!(
        r#"{{"event":"certified_cone_warm_depth_edit","seconds":{}}}"#,
        start.elapsed().as_secs_f64()
    );
    assert_eq!(before.encoded_brep(), bytes);
    assert_ne!(after.encoded_brep(), bytes);
    assert_eq!(before.history(), after.history());
    check_attachment(&attachment, &request, &after);
    let old = before.export_native().unwrap();
    let edited = after.export_native().unwrap();
    assert!(body_volume_estimate(&edited.body) < body_volume_estimate(&old.body));
}

fn apply(
    attachment: &ConeFaceAttachment,
    kind: WrappedFeatureKind,
    request: &WrappedConeNormalRequest,
) -> CertifiedConeFeatureResult {
    attachment
        .apply_certified_wrapped_profile(REVISION, SEAM, kind, request, certified_selection())
        .unwrap_or_else(|cause| panic!("certified conical attachment refused: {cause:?}"))
}

fn certified_selection() -> WrappedConeExecutionSelection {
    WrappedConeExecutionSelection::DirectedTransportedAuthoredPlanarAndCapCyclesV9 {
        arithmetic: WrappedConeArithmeticGrantPolicy::InputBoundV1,
        authored_max_components: 64,
        authored_max_subintervals: 1024,
    }
}

#[test]
fn named_cone_support_members_resolve_in_certified_boss_history() {
    let attachment = text_attachment();
    let native = attachment.export_native(REVISION).unwrap();
    let edges = Explorer::new(&native.body, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    assert!(edges.more());
    let labels = [
        ("support", native.face.clone().into_shape()),
        ("edge", edges.current().clone()),
    ];
    let kind = WrappedFeatureKind::Boss;
    let bind = |revision, labels: &[_]| {
        attachment.bind_feature_sources(revision, kind, &native.body, &native.face, labels)
    };
    let names = bind(REVISION, &labels).unwrap();
    assert_eq!(
        names["support"],
        CylinderFeatureSource::SupportFace(attachment.record().id)
    );
    assert!(matches!(
        names["edge"],
        CylinderFeatureSource::SupportMember(_)
    ));
    assert_eq!(
        names,
        bind(REVISION, &[labels[1].clone(), labels[0].clone()]).unwrap()
    );
    assert!(bind(SupportRevision(REVISION.0 + 1), &labels).is_err());

    let result = apply(
        &attachment,
        kind,
        &attached_request(&attachment, NormalSense::Outward),
    );
    let history = result.history();
    for name in names.values() {
        assert!(
            history.removed.contains(name)
                || history
                    .images
                    .values()
                    .flatten()
                    .any(|parent| parent.source == *name)
        );
    }
}

#[test]
fn certified_attachment_admission_refuses_volume_gate_and_observes_cancellation() {
    let attachment = text_attachment();
    let record = attachment.record().clone();
    let request = attached_request(&attachment, NormalSense::Outward);
    let mut volume_gated = request.clone();
    volume_gated.options.volume = WrappedVolumePolicy::Refuse(1e-5);
    let outcome = attachment.apply_certified_wrapped_profile(
        REVISION,
        SEAM,
        WrappedFeatureKind::Boss,
        &volume_gated,
        certified_selection(),
    );
    let Err(refusal) = outcome else {
        panic!("expected direct volume policy refusal");
    };
    assert_eq!(refusal.stage, CylinderFeatureStage::Admission);
    assert!(matches!(
        *refusal.cause,
        CylinderFeatureCause::DirectAttachment { .. }
    ));

    let stopped = attachment.apply_certified_wrapped_profile_with_cancel(
        REVISION,
        SEAM,
        WrappedFeatureKind::Boss,
        &request,
        certified_selection(),
        || true,
    );
    let Err(refusal) = stopped else {
        panic!("expected direct attachment cancellation");
    };
    assert_eq!(refusal.stage, CylinderFeatureStage::Admission);
    assert!(matches!(*refusal.cause, CylinderFeatureCause::Cancelled));
    assert_eq!(attachment.record(), &record);
}

fn check_attachment(
    attachment: &ConeFaceAttachment,
    request: &WrappedConeNormalRequest,
    result: &CertifiedConeFeatureResult,
) {
    let report = result.validation();
    assert_eq!(report.check_mode(), CheckMode::Exact);
    assert_eq!(
        [report.faces, report.edges, report.vertices, report.loops],
        [362, 1020, 680, 382]
    );
    assert_eq!(report.genus, 0);
    assert_eq!(report.incident_paths, 678);
    assert!(report.seam_residual_upper < report.requested_sew_limit);
    assert!(report.maximum_tolerance <= 1e-5);
    if let Some(directory) = std::env::var_os("MARTENSITE_CERTIFIED_ATTACHMENT_RECEIPT_DIR") {
        write_receipt(result, std::path::Path::new(&directory));
    }
    let native = result.export_native().unwrap();
    assert_eq!(native.body.shape_type().unwrap(), ShapeEnum::Solid);
    assert_eq!(
        exposed_cap_regions(result.history(), &native),
        request
            .selections
            .iter()
            .map(|selected| selected.region)
            .collect(),
    );
    if std::env::var_os("MARTENSITE_CERTIFIED_ATTACHMENT_P95").is_none() {
        let ordinary = validate_body(&native.body, &attachment.record().resolution).unwrap();
        assert!(ordinary.is_valid(), "{ordinary:?}");
        let source = attachment.export_native(REVISION).unwrap();
        let volume_delta = body_volume_estimate(&native.body) - body_volume_estimate(&source.body);
        assert_expected_volume_change(request.support.normal, volume_delta);
    }
}

fn write_receipt(result: &CertifiedConeFeatureResult, directory: &std::path::Path) {
    std::fs::write(directory.join("body.brep"), result.encoded_brep()).unwrap();
    std::fs::write(
        directory.join("history.txt"),
        format!("{:?}\n{:?}", result.history(), result.authored_history()),
    )
    .unwrap();
    std::fs::write(
        directory.join("validation.txt"),
        format!("{:?}", result.validation()),
    )
    .unwrap();
}

#[test]
#[ignore = "shared-input replay of the full certified attachment fixture"]
fn certified_attachment_replays_exactly_on_independent_threads() {
    let attachment = text_attachment();
    let request = attached_request(&attachment, NormalSense::Outward);
    let record = attachment.record().clone();
    let serial = apply(&attachment, WrappedFeatureKind::Boss, &request);
    std::thread::scope(|scope| {
        let replay = || apply(&attachment, WrappedFeatureKind::Boss, &request);
        let a = scope.spawn(replay);
        let b = scope.spawn(replay);
        for result in [a.join().unwrap(), b.join().unwrap()] {
            assert_eq!(result.encoded_brep(), serial.encoded_brep());
            assert_eq!(result.history(), serial.history());
            assert_eq!(result.authored_history(), serial.authored_history());
            assert_eq!(result.validation(), serial.validation());
            let one = result.export_native().unwrap();
            let two = result.export_native().unwrap();
            assert!(!one.body.is_partner(&two.body));
        }
    });
    assert_eq!(attachment.record(), &record);
}
