use super::super::tests::{attachment, request};
use super::*;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};

const REVISION: SupportRevision = SupportRevision(7);

#[test]
fn cylinder_band_boss_publishes_a_single_solid_with_composed_history() {
    check_feature(CylinderFeatureKind::Boss, NormalSense::Outward, 10.25);
}

#[test]
fn cylinder_band_pocket_publishes_a_single_solid_with_composed_history() {
    check_feature(CylinderFeatureKind::Pocket, NormalSense::Inward, 9.75);
}

fn check_feature(kind: CylinderFeatureKind, normal: NormalSense, cap_radius: f64) {
    let attachment = attachment(None);
    let mut request = request(&attachment, [-0.25, 0.25], [1., 2.]);
    request.support.normal = normal;
    let original = request.clone();
    let support = attachment.record().brep.clone();
    let feature = attachment
        .apply_wrapped_profile(REVISION, kind, &request)
        .unwrap();
    assert_eq!(feature.kind(), kind);
    assert!(feature.validation().is_valid());
    assert_eq!(feature.validation().census.len(), 1);
    assert_eq!(feature.tool().footprint().angular, None);
    let native = feature.export_native().unwrap();
    let expected = 400. * std::f64::consts::PI + (cap_radius * cap_radius - 100.) / 40.;
    assert!((volume(&native.body) - expected).abs() < 1e-4);
    check_history(&feature, &native);
    assert_eq!(attachment.record().brep, support);
    assert_eq!(request, original);
}

fn volume(shape: &Shape) -> f64 {
    let mut props = GProps::default();
    let estimate =
        BRepGProp::volume_properties_gk(shape, &mut props, 1e-8, true, VolumeGKFlags::default())
            .unwrap();
    assert!((0. ..=1e-8).contains(&estimate));
    props.mass()
}

fn check_history(feature: &CylinderFeatureResult, native: &DetachedCylinderFeature) {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<CylinderFeatureResult>();
    assert_eq!(native.body.shape_type().unwrap(), ShapeEnum::Solid);
    let history = feature.history();
    assert_eq!(
        history.images.keys().collect::<Vec<_>>(),
        native.entities.keys().collect::<Vec<_>>()
    );
    assert!(history.images.values().all(|parents| !parents.is_empty()));
    let root = CylinderBodyRef {
        kind: ShapeEnum::Solid,
        ordinal: 1,
    };
    assert_eq!(history.images[&root].len(), 2);
    assert!(
        history.images[&root]
            .iter()
            .all(|parent| parent.relation == CylinderFeatureRelation::Operand)
    );
    check_authored_history(feature);
    let cap = WrappedHistoryKey {
        source: WrappedEntitySource::Region(SurfaceRegionId(3)),
        role: HistoryRole::LastDepthFace,
    };
    assert!(history.images.iter().any(|(image, parents)| {
        image.kind == ShapeEnum::Face
            && parents
                .iter()
                .any(|parent| parent.source == CylinderFeatureSource::Tool(cap))
    }));
    let second = feature.export_native().unwrap();
    assert!(!second.body.is_same(&native.body));
}

fn check_authored_history(feature: &CylinderFeatureResult) {
    let history = feature.history();
    let mut recorded: std::collections::BTreeSet<_> = history.removed.iter().copied().collect();
    recorded.extend(
        history
            .images
            .values()
            .flatten()
            .map(|parent| parent.source),
    );
    for record in feature.tool().extrusion().history() {
        assert!(recorded.contains(&CylinderFeatureSource::Tool(record.result)));
        assert!(!record.parents.is_empty());
    }
    assert!(recorded.contains(&CylinderFeatureSource::SupportFace(FaceAttachmentId(19))));
}

#[test]
fn feature_refuses_a_normal_that_points_into_the_wrong_side() {
    let attachment = attachment(None);
    let proposal = request(&attachment, [0.25, 0.75], [1., 2.]);
    let error = attachment
        .apply_wrapped_profile(REVISION, CylinderFeatureKind::Boss, &proposal)
        .err()
        .unwrap();
    assert_eq!(error.stage, CylinderFeatureStage::Admission);
    assert!(matches!(
        *error.cause,
        CylinderFeatureCause::Normal {
            actual: NormalSense::Inward,
            expected: NormalSense::Outward
        }
    ));
}

#[test]
fn feature_cancellation_returns_no_body() {
    let attachment = attachment(None);
    let proposal = request(&attachment, [0.25, 0.75], [1., 2.]);
    let error = attachment
        .apply_wrapped_profile_with_cancel(REVISION, CylinderFeatureKind::Pocket, &proposal, || {
            true
        })
        .err()
        .unwrap();
    assert_eq!(error.stage, CylinderFeatureStage::Admission);
    assert!(matches!(*error.cause, CylinderFeatureCause::Cancelled));
}
