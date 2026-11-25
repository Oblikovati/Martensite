//! Exercise the normal tool, native Boolean, composed history and frozen result together.
use super::*;
use crate::surface_sketch::{
    ConeFeatureResult, CylinderFeatureCause, CylinderFeatureSource, CylinderFeatureStage,
    WrappedFeatureKind, WrappedHistoryKey,
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{
    brep_class3d::BRepClass3dSolidClassifier,
    brep_gprop::{BRepGProp, VolumeGKFlags},
};
use std::{
    fmt::Write as _,
    io::{Read as _, Write as _},
    process::{Command, Stdio},
};

const SEAM: f64 = 0.7;
const REGION: SurfaceRegionId = SurfaceRegionId(19);
const WORKER: &str = "surface_sketch::attachment::cone::tests::workflow::material::replay_worker";
const PREFIX: &str = "CONE_FEATURE=";

#[test]
fn cone_boss_changes_the_support_solid_with_complete_history_and_process_replay() {
    let source = oblique_attachment();
    let feature = checked_material(&source, WrappedFeatureKind::Boss, NormalSense::Outward);
    let child = replay(source.record());
    assert!(
        child.status.success(),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    let text = String::from_utf8(child.stdout).unwrap();
    let replayed = text
        .lines()
        .find_map(|line| line.split_once(PREFIX).map(|(_, body)| body));
    assert_eq!(replayed, Some(signature(&feature).as_str()));
}

#[test]
fn cone_pocket_changes_the_support_solid_with_complete_history() {
    checked_material(
        &attach([2., 4.]),
        WrappedFeatureKind::Pocket,
        NormalSense::Inward,
    );
}

#[test]
fn cone_pocket_preserves_the_island_inside_a_curved_hole() {
    let source = attach([2., 4.]);
    let chart = source.authoring_chart(REVISION, SEAM).unwrap();
    let mut request = material_request(&source);
    request.support.normal = NormalSense::Inward;
    add_curved_hole(&chart, &mut request);
    let kind = WrappedFeatureKind::Pocket;
    let feature = source
        .apply_wrapped_profile(REVISION, SEAM, kind, &request, directed())
        .unwrap();
    let detached = feature.export_native().unwrap();
    verify_body(&feature, &detached);
    verify_volume(kind, &feature, &detached.body, 2.);
    let support = source.export_native(REVISION).unwrap();
    verify_island(&support.face, &detached.body);
}

fn add_curved_hole(chart: &ConeAuthoringChart, request: &mut WrappedConeNormalRequest) {
    let hole = [[0.6, 3.25], [0.8, 3.25], [0.8, 3.75], [0.6, 3.75]];
    append_loop(
        chart,
        hole,
        OutOfSupportPolicy::Refuse,
        5,
        &mut request.profile,
    );
    request.selections[0].holes.push(contour(5));
    let first = request.profile.vertices[4].point;
    let last = request.profile.vertices[5].point;
    let inner = |t| {
        [
            first[0] + (last[0] - first[0]) * t + 0.0125,
            first[1] + (last[1] - first[1]) * t,
        ]
    };
    request.profile.curves[4] = ProfileCurve::Spline {
        id: ProfileCurveId(5),
        start: ProfileVertexId(5),
        end: ProfileVertexId(6),
        degree: 3,
        poles: vec![first, inner(1. / 3.), inner(2. / 3.), last],
        weights: None,
        knots: vec![0., 0., 0., 0., 1., 1., 1., 1.],
    };
}

fn verify_island(face: &Face, result: &Shape) {
    let native = BRepAdaptorSurface::with_face(face, false).unwrap();
    let mut classifier = BRepClass3dSolidClassifier::with_shape(result).unwrap();
    for (hit, expected) in [([0.7, 3.5], State::In), ([0.55, 3.1], State::Out)] {
        let [x, y, z] = native_normal_offset(&native, hit, -0.1);
        classifier.perform(&Point3::new(x, y, z), 1e-7).unwrap();
        assert_eq!(classifier.state(), expected, "UV {hit:?} at depth -0.1");
    }
}

fn replay(record: &ConeAttachmentRecord) -> std::process::Output {
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", WORKER, "--ignored", "--nocapture"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&record.brep).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
#[ignore = "independent process for cone feature replay"]
fn replay_worker() {
    let mut brep = Vec::new();
    std::io::stdin().read_to_end(&mut brep).unwrap();
    let source = ConeFaceAttachment::restore(ConeAttachmentRecord {
        id: ID,
        revision: REVISION,
        resolution: super::super::super::super::tests::resolution(),
        brep,
    })
    .unwrap();
    let feature = checked_material(&source, WrappedFeatureKind::Boss, NormalSense::Outward);
    println!("{PREFIX}{}", signature(&feature));
}

fn checked_material(
    source: &ConeFaceAttachment,
    kind: WrappedFeatureKind,
    normal: NormalSense,
) -> ConeFeatureResult {
    let mut request = material_request(source);
    request.support.normal = normal;
    let before = source.record().clone();
    let feature = measured_feature(source, kind, &request);
    assert_eq!(source.record(), &before);
    let detached = feature.export_native().unwrap();
    verify_body(&feature, &detached);
    verify_volume(kind, &feature, &detached.body, 2.);
    assert!(
        !detached
            .body
            .is_partner(&feature.export_native().unwrap().body)
    );
    feature
}

fn measured_feature(
    source: &ConeFaceAttachment,
    kind: WrappedFeatureKind,
    request: &WrappedConeNormalRequest,
) -> ConeFeatureResult {
    let started = std::time::Instant::now();
    let feature = source
        .apply_wrapped_profile(REVISION, SEAM, kind, request, directed())
        .unwrap_or_else(|cause| panic!("conical material operation refused: {cause:?}"));
    eprintln!(
        r#"{{"event":"cone_feature_api","kind":"{kind:?}","seconds":{}}}"#,
        started.elapsed().as_secs_f64()
    );
    feature
}

fn material_request(source: &ConeFaceAttachment) -> WrappedConeNormalRequest {
    let chart = source.authoring_chart(REVISION, SEAM).unwrap();
    let mut request = proposal(&chart);
    request.depths = [0., 0.2];
    request.options.spatial_error = 1e-5;
    request
}

fn verify_body(
    feature: &ConeFeatureResult,
    detached: &crate::surface_sketch::DetachedWrappedFeature,
) {
    fn shared<T: Send + Sync>() {}
    shared::<ConeFeatureResult>();
    assert_eq!(detached.body.shape_type().unwrap(), ShapeEnum::Solid);
    assert!(feature.validation().is_valid());
    let images = &feature.history().images;
    assert_eq!(images.len(), detached.entities.len());
    let cap = CylinderFeatureSource::Tool(WrappedHistoryKey {
        source: WrappedEntitySource::Region(REGION),
        role: HistoryRole::LastDepthFace,
    });
    assert!(images.iter().any(|(image, parents)| {
        image.kind == ShapeEnum::Face && parents.iter().any(|parent| parent.source == cap)
    }));
    assert!(images.values().all(|parents| !parents.is_empty()));
}

fn verify_volume(kind: WrappedFeatureKind, feature: &ConeFeatureResult, solid: &Shape, angle: f64) {
    // A frustum sector with radii 2 and 4 and height 10.
    let source = angle * 140. / 3.;
    let actual = volume(solid);
    let added = match kind {
        WrappedFeatureKind::Boss => actual - source,
        WrappedFeatureKind::Pocket => source - actual,
    };
    let tool = &feature.tool().extrusion().placement().members[&REGION];
    assert!(added > 0.);
    assert!(
        added >= tool.boundary_flux.lower - 1e-5 && added <= tool.boundary_flux.upper + 1e-5,
        "{added} vs {:?}",
        tool.boundary_flux,
    );
}

#[test]
fn cone_pocket_crosses_the_supports_periodic_seam() {
    let mut maker = BRepPrimAPIMakeCone::new(2., 4., 10.).unwrap();
    let body = maker.shape().unwrap();
    let face = select_kind(body, SurfaceType::Cone);
    let attachment = ConeFaceAttachment::capture(
        ID,
        REVISION,
        body,
        &face,
        super::super::super::super::tests::resolution(),
    )
    .unwrap();
    let chart = attachment.authoring_chart(REVISION, 0.).unwrap();
    let mut request = proposal(&chart);
    request.depths = [0., 0.2];
    request.options.spatial_error = 1e-5;
    request.support.normal = NormalSense::Inward;
    let cross = [[-0.12, 3.], [-0.12, 4.], [0.12, 4.], [0.12, 3.]];
    request.profile = profile_at(&chart, cross, OutOfSupportPolicy::Report);
    let kind = WrappedFeatureKind::Pocket;
    let feature = attachment
        .apply_wrapped_profile(REVISION, 0., kind, &request, directed())
        .unwrap();
    assert!(feature.tool().footprint().native_angular.is_none());
    let detached = feature.export_native().unwrap();
    verify_body(&feature, &detached);
    verify_volume(kind, &feature, &detached.body, std::f64::consts::TAU);
}

fn volume(shape: &Shape) -> f64 {
    let mut props = GProps::default();
    let error =
        BRepGProp::volume_properties_gk(shape, &mut props, 1e-8, true, VolumeGKFlags::default())
            .unwrap();
    assert!((0. ..=1e-8).contains(&error));
    props.mass()
}

fn signature(feature: &ConeFeatureResult) -> String {
    let mut value = format!("{:?}|{:?}|", feature.history(), feature.tool().footprint());
    for byte in feature.encoded_brep() {
        write!(&mut value, "{byte:02x}").unwrap();
    }
    value
}

#[test]
fn cone_feature_refuses_invalid_depth_and_normal_at_admission() {
    let source = attach([2., 4.]);
    let mut request = material_request(&source);
    let kind = WrappedFeatureKind::Pocket;
    request.depths[0] = 0.01;
    let error = source
        .apply_wrapped_profile(REVISION, SEAM, kind, &request, directed())
        .err()
        .unwrap();
    assert_eq!(error.stage, CylinderFeatureStage::Admission);
    assert!(matches!(*error.cause, CylinderFeatureCause::Depths(_)));
    request.depths[0] = 0.;
    let error = source
        .apply_wrapped_profile(REVISION, SEAM, kind, &request, directed())
        .err()
        .unwrap();
    assert!(matches!(*error.cause, CylinderFeatureCause::Normal { .. }));
}
