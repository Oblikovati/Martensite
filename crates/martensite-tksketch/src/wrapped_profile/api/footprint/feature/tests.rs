use super::super::tests::{attachment, freeze, request, resolution, selected_cylinder};
use super::*;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};

pub(super) const REVISION: SupportRevision = SupportRevision(7);

#[test]
fn cylinder_band_boss_publishes_a_single_solid_with_composed_history() {
    check_feature(CylinderFeatureKind::Boss, NormalSense::Outward, 10.25);
}

#[test]
fn cylinder_band_pocket_publishes_a_single_solid_with_composed_history() {
    check_feature(CylinderFeatureKind::Pocket, NormalSense::Inward, 9.75);
}

#[test]
fn cylindrical_attachment_handoff_preserves_the_legacy_fresh_operand_result() {
    let attachment = attachment(None);
    for (kind, normal) in [
        (CylinderFeatureKind::Boss, NormalSense::Outward),
        (CylinderFeatureKind::Pocket, NormalSense::Inward),
    ] {
        let mut request = request(&attachment, [-0.25, 0.25], [1., 2.]);
        request.support.normal = normal;
        let actual = attachment
            .apply_wrapped_profile(REVISION, kind, &request)
            .unwrap();
        let legacy = from_fresh_operand(&attachment, &actual);
        assert_eq!(actual.encoded_brep(), legacy.bytes);
        assert_eq!(actual.history(), &legacy.history);
        assert_eq!(actual.validation(), &legacy.validation);
    }
}

#[test]
fn private_cylindrical_attachment_preparation_preserves_requested_tolerances() {
    use martensite_tkbo::profile_extrusion::PlanarProofVersion;
    let attachment = attachment(None);
    let mut request = request(&attachment, [1., 4.], [1., 2.]);
    request.support.normal = NormalSense::Outward;
    let normal = attachment
        .extrude_within_trim_band(REVISION, &request)
        .unwrap();
    let prepared = selection::Preparation {
        proof: PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: 8192,
        },
        maximum_fit_error: Some(attachment.record().resolution.sew),
    };
    let (_, mut band) = attachment.resolve_band(REVISION, &request).unwrap();
    let (strict, graph) = attachment
        .prepare_trimmed_tool_with_preparation(REVISION, &request, &mut band, prepared, &mut || {
            false
        })
        .unwrap();
    assert!(strict.fitted_spans() > normal.fitted_spans());
    assert_eq!(
        strict.extrusion().options().spatial_error,
        request.options.spatial_error
    );
    assert!(
        strict
            .extrusion()
            .reports()
            .values()
            .all(|report| report.validation.is_valid())
    );
    let tool = strict.extrusion().attachment_operand(graph).unwrap();
    assert_eq!(tool.entities.len(), strict.extrusion().history().len());
}

fn from_fresh_operand(
    attachment: &CylinderFaceAttachment,
    feature: &CylinderFeatureResult,
) -> FeatureArchive {
    let attempt = Attempt {
        attachment: attachment.record().id,
        revision: REVISION,
        kind: feature.kind(),
    };
    let source = attachment.export_native(REVISION).unwrap();
    apply_material(
        attempt,
        &source.face,
        &source.body,
        feature.tool().extrusion().export_native().unwrap(),
        &attachment.record().resolution,
        &mut || false,
    )
    .unwrap()
}

#[test]
fn indirect_cylinder_from_planar_circle_admits_wrapped_boss_and_pocket() {
    let attachment = indirect_extruded_circle();
    let frame = attachment
        .extrusion_support(REVISION, 0., NormalSense::Outward)
        .unwrap();
    assert_eq!([0., 0., 1.], frame.axis);
    let beyond_trim = request(&attachment, [-0.25, 0.25], [-2., -1.]);
    assert!(matches!(
        attachment.certify_footprint(REVISION, &beyond_trim),
        Err(CylinderFootprintFailure::ControlOutside { axis: 1, .. })
    ));
    for (kind, normal, cap_radius) in [
        (CylinderFeatureKind::Boss, NormalSense::Outward, 10.25),
        (CylinderFeatureKind::Pocket, NormalSense::Inward, 9.75),
    ] {
        let mut authored = request(&attachment, [-0.25, 0.25], [1., 2.]);
        authored.support.normal = normal;
        let result = measured_feature(&attachment, kind, &authored);
        assert!(result.validation().is_valid());
        let native = result.export_native().unwrap();
        check_history(&result, &native);
        let expected = 400. * std::f64::consts::PI + (cap_radius * cap_radius - 100.) / 40.;
        assert!((volume(&native.body) - expected).abs() < 1e-4);
    }
}

pub(super) fn indirect_extruded_circle() -> CylinderFaceAttachment {
    use martensite_tkbo::profile_extrusion::{
        ExtrudeOptions, ProfileCurve, ProfileCurveId, ProfilePlane, SketchProfile,
        build_planar_regions, extrude,
    };
    use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
    use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
    let resolution = resolution();
    let profile = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: vec![],
        curves: vec![ProfileCurve::Circle {
            id: ProfileCurveId(1),
            center: [0., 0.],
            radius: 10.,
        }],
    };
    let regions = build_planar_regions(&profile, resolution).unwrap();
    let result = extrude(
        &regions.regions()[0],
        [0., 0., 4.],
        &ExtrudeOptions { resolution },
    )
    .unwrap();
    let body = result.solid().to_shape();
    let side = selected_cylinder(&body);
    let adaptor = BRepAdaptorSurface::with_face(&side, false).unwrap();
    assert!(!adaptor.cylinder().unwrap().direct().unwrap());
    freeze(&body)
}

fn check_feature(kind: CylinderFeatureKind, normal: NormalSense, cap_radius: f64) {
    let attachment = attachment(None);
    let mut request = request(&attachment, [-0.25, 0.25], [1., 2.]);
    request.support.normal = normal;
    let original = request.clone();
    let support = attachment.record().brep.clone();
    let feature = measured_feature(&attachment, kind, &request);
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

fn measured_feature(
    attachment: &CylinderFaceAttachment,
    kind: CylinderFeatureKind,
    request: &WrappedNormalRequest,
) -> CylinderFeatureResult {
    let started = std::time::Instant::now();
    let result = attachment
        .apply_wrapped_profile(REVISION, kind, request)
        .unwrap();
    eprintln!(
        r#"{{"event":"cylinder_feature_api","kind":"{kind:?}","seconds":{}}}"#,
        started.elapsed().as_secs_f64()
    );
    result
}

pub(super) fn volume(shape: &Shape) -> f64 {
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

// The feature Boolean runs in parallel only because it publishes the serial solid exactly.
#[test]
fn parallel_and_serial_feature_booleans_publish_identical_solids() {
    for (kind, normal) in [
        (CylinderFeatureKind::Boss, NormalSense::Outward),
        (CylinderFeatureKind::Pocket, NormalSense::Inward),
    ] {
        let serial = feature_boolean_bytes(kind, normal, false);
        assert_eq!(feature_boolean_bytes(kind, normal, true), serial);
    }
}

fn feature_boolean_bytes(
    kind: CylinderFeatureKind,
    normal: NormalSense,
    parallel: bool,
) -> Vec<u8> {
    let attachment = attachment(None);
    let mut request = request(&attachment, [-0.25, 0.25], [1., 2.]);
    request.support.normal = normal;
    let attempt = Attempt {
        attachment: attachment.record().id,
        revision: REVISION,
        kind,
    };
    let (support, mut band) = attachment.resolve_band(REVISION, &request).unwrap();
    let solid = admit(
        attempt,
        &support.body,
        support.face.orientation(),
        request.depths,
        request.support.normal,
    )
    .unwrap();
    let mut never = || false;
    let tool = attachment
        .build_trimmed_tool(REVISION, &request, &mut band, &mut never)
        .unwrap();
    let native = tool.extrusion().export_native().unwrap();
    let operation = boolean(attempt, &solid, &native.bodies, parallel, &mut never).unwrap();
    encoding::encode::<()>(operation.shape(), &[]).unwrap()
}
