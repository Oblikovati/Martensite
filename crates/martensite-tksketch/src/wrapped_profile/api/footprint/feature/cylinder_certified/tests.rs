use super::super::tests::{REVISION, indirect_extruded_circle, volume};
use super::*;
use crate::surface_sketch::wrapped_profile::api::footprint::tests::request;
use martensite_tkbo::body_check::{CheckMode, StageVerdict, ValidationStage, validate_body};

mod text;

fn planar() -> PlanarProofVersion {
    PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 65_536,
    }
}

#[test]
fn shared_cylinder_revision_replays_complete_certified_results_concurrently() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CertifiedCylinderFeatureResult>();
    let attachment = indirect_extruded_circle();
    let source = attachment.record().brep.clone();
    let mut sketch = request(&attachment, [-0.25, 0.25], [1., 2.]);
    sketch.support.normal = NormalSense::Outward;
    let issue = || {
        attachment
            .apply_certified_wrapped_profile(REVISION, CylinderFeatureKind::Boss, &sketch, planar())
            .unwrap()
    };
    let serial = issue();
    let [one, two] = std::thread::scope(|scope| {
        let one = scope.spawn(issue);
        let two = scope.spawn(issue);
        [one.join().unwrap(), two.join().unwrap()]
    });
    assert_eq!(source, attachment.record().brep);
    let a = one.export_native().unwrap();
    let b = two.export_native().unwrap();
    assert_replay(&serial, &one, &a);
    assert_replay(&serial, &two, &b);
    assert!(
        a.entities
            .iter()
            .all(|(key, shape)| !shape.is_partner(&b.entities[key]))
    );
}

fn assert_replay(
    serial: &CertifiedCylinderFeatureResult,
    concurrent: &CertifiedCylinderFeatureResult,
    detached: &DetachedCylinderFeature,
) {
    assert_eq!(serial.encoded_brep(), concurrent.encoded_brep());
    assert_eq!(serial.history(), concurrent.history());
    assert_eq!(serial.authored_history(), concurrent.authored_history());
    assert_eq!(serial.validation(), concurrent.validation());
    assert!(
        concurrent
            .history()
            .images
            .keys()
            .eq(detached.entities.keys())
    );
    assert_ordered(concurrent.validation());
}

fn assert_ordered(report: &ValidationReport) {
    assert!(report.is_valid());
    assert_eq!(
        report.stage(ValidationStage::Geometry).mode,
        CheckMode::Sampled
    );
    assert_eq!(
        report.stage(ValidationStage::SelfIntersection).mode,
        CheckMode::Exact
    );
    assert_eq!(
        report.stage(ValidationStage::Tolerance).verdict,
        StageVerdict::Passed
    );
}

#[test]
fn certified_cylinder_checks_both_normals_and_inherited_stock_bindings() {
    let stock = indirect_extruded_circle();
    let source = stock.record().brep.clone();
    for (kind, normal, radius) in [
        (CylinderFeatureKind::Boss, NormalSense::Outward, 10.25_f64),
        (CylinderFeatureKind::Pocket, NormalSense::Inward, 9.75_f64),
    ] {
        let mut sketch = request(&stock, [-0.25, 0.25], [1., 2.]);
        sketch.support.normal = normal;
        let feature = stock
            .apply_certified_wrapped_profile(REVISION, kind, &sketch, planar())
            .unwrap();
        assert_ordered(feature.validation());
        let coverage = feature.attachment_report();
        assert!(coverage.authored_to_stored_upper <= sketch.options.spatial_error);
        assert!(coverage.seam_residual_upper <= stock.record().resolution.sew);
        let native = feature.export_native().unwrap();
        let exact = 400. * std::f64::consts::PI + (radius * radius - 100.) / 40.;
        assert!((volume(&native.body) - exact).abs() < 1e-4);
        assert!(
            validate_body(&native.body, &stock.record().resolution)
                .unwrap()
                .is_valid()
        );
        assert_eq!(feature.kind(), kind);
        assert_eq!(stock.record().brep, source);
        assert!(feature.history().removed.iter().any(|source| matches!(source, CylinderFeatureSource::SupportMember(member) if member.kind == ShapeEnum::Edge)));
    }
}

#[test]
fn a_certified_cylinder_refusal_preserves_the_original_archive() {
    let stock = indirect_extruded_circle();
    let source = stock.record().brep.clone();
    let mut sketch = request(&stock, [-0.25, 0.25], [1., 2.]);
    sketch.support.normal = NormalSense::Outward;
    let cancelled = stock
        .apply_certified_wrapped_profile_with_cancel(
            REVISION,
            CylinderFeatureKind::Boss,
            &sketch,
            planar(),
            || true,
        )
        .err()
        .unwrap();
    assert_eq!(cancelled.stage, CylinderFeatureStage::Admission);
    assert!(matches!(*cancelled.cause, CylinderFeatureCause::Cancelled));
    assert_eq!(stock.record().brep, source);
}
