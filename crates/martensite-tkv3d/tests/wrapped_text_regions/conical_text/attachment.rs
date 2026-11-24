//! Attach the owned multiline cone text to a selected analytic support face (#425).
use super::super::body_controls::{attachment_resolution, body_volume_estimate};
use super::*;
use martensite_tkbo::body_check::{CheckMode, ValidationStage};
use martensite_tkbrep::{brep_adaptor::BRepAdaptorSurface, topexp::Explorer, topods};
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};
use martensite_tkmath::{
    geom_abs::SurfaceType,
    gp::{Axis2, Point3, StandardDirection3},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCone;
use martensite_tksketch::surface_sketch::{
    HistoryRole,
    attachment::{ConeFaceAttachment, FaceAttachmentId, SupportRevision},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::collections::BTreeSet;

const REVISION: SupportRevision = SupportRevision(3);
const SEAM: f64 = 2.5;

#[test]
fn multiline_cone_boss_retains_all_selected_caps() {
    attached_text(WrappedFeatureKind::Boss, NormalSense::Outward);
}

#[test]
fn multiline_cone_pocket_retains_all_selected_caps() {
    attached_text(WrappedFeatureKind::Pocket, NormalSense::Inward);
}

#[test]
#[ignore = "latency fixture for exact recomputation after an edit"]
fn multiline_cone_boss_depth_edit_retains_all_selected_caps() {
    let attachment = text_attachment();
    let mut request = attached_request(&attachment, NormalSense::Outward);
    let previous = apply_boss(&attachment, &request);
    let previous_bytes = previous.encoded_brep().to_vec();
    request.depths[1] *= 0.98;
    let started = std::time::Instant::now();
    let edited = apply_boss(&attachment, &request);
    eprintln!(
        r#"{{"event":"conical_text_depth_edit_api","seconds":{}}}"#,
        started.elapsed().as_secs_f64()
    );
    check_boss_depth_edit(&previous, &edited, &previous_bytes, &request);
}

fn apply_boss(
    attachment: &ConeFaceAttachment,
    request: &WrappedConeNormalRequest,
) -> ConeFeatureResult {
    attachment
        .apply_wrapped_profile(
            REVISION,
            SEAM,
            WrappedFeatureKind::Boss,
            request,
            selection(),
        )
        .unwrap_or_else(|cause| panic!("attached cone text refused: {cause:?}"))
}

fn check_boss_depth_edit(
    previous: &ConeFeatureResult,
    edited: &ConeFeatureResult,
    previous_bytes: &[u8],
    request: &WrappedConeNormalRequest,
) {
    assert_eq!(previous.encoded_brep(), previous_bytes);
    assert_ne!(edited.encoded_brep(), previous_bytes);
    assert!(edited.validation().is_valid());
    check_intersection_modes(edited, CheckMode::Exact);
    let detached = edited.export_native().unwrap();
    let before = previous.export_native().unwrap();
    assert_eq!(detached.body.shape_type().unwrap(), ShapeEnum::Solid);
    assert_eq!(
        exposed_cap_regions(edited.history(), &detached),
        request.selections.iter().map(|item| item.region).collect()
    );
    assert!(body_volume_estimate(&detached.body) < body_volume_estimate(&before.body));
}

fn selection() -> WrappedConeExecutionSelection {
    WrappedConeExecutionSelection::DirectedTransportedPlacementV5 {
        domain: WrappedConeDomainPolicy::PositiveXFamilyChartV3,
        arithmetic: WrappedConeArithmeticGrantPolicy::InputBoundV1,
    }
}

fn attached_text(kind: WrappedFeatureKind, normal: NormalSense) {
    let attachment = text_attachment();
    let before = attachment.record().clone();
    let request = attached_request(&attachment, normal);
    let original = request.clone();
    let started = std::time::Instant::now();
    let outcome = attachment.apply_wrapped_profile(REVISION, SEAM, kind, &request, selection());
    eprintln!(
        r#"{{"event":"conical_text_attached_api","normal":"{normal:?}","seconds":{}}}"#,
        started.elapsed().as_secs_f64()
    );
    let feature = outcome.unwrap_or_else(|cause| panic!("attached cone text refused: {cause:?}"));
    check_attached_result(&feature, &attachment, &request);
    check_intersection_modes(&feature, CheckMode::Exact);
    assert_eq!(attachment.record(), &before);
    assert_eq!(request, original);
}

fn attached_request(
    attachment: &ConeFaceAttachment,
    normal: NormalSense,
) -> WrappedConeNormalRequest {
    let mut request = request(normal);
    let chart = attachment.authoring_chart(REVISION, SEAM).unwrap();
    request.support = chart.extrusion_support(REVISION, normal).unwrap();
    request.proof.arithmetic.max_operations = 8_810_496;
    request
}

fn text_attachment() -> ConeFaceAttachment {
    let (sine, cosine) = martensite_cmath::sincos(0.125);
    let axes = Axis2::with_standard_frame(
        Point3::new(0., 0., 85. * cosine),
        StandardDirection3::Z,
        StandardDirection3::X,
    );
    let mut maker =
        BRepPrimAPIMakeCone::with_axes(axes, 85. * sine, 115. * sine, 30. * cosine).unwrap();
    let body = maker.shape().unwrap();
    ConeFaceAttachment::capture(
        FaceAttachmentId(44),
        REVISION,
        body,
        &lateral_face(body),
        attachment_resolution(),
    )
    .unwrap()
}

fn lateral_face(body: &topods::Shape) -> topods::Face {
    let mut faces = Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while faces.more() {
        let face = topods::face(faces.current()).unwrap();
        let surface = BRepAdaptorSurface::with_face(&face, false).unwrap();
        if surface.get_type().unwrap() == SurfaceType::Cone {
            return face;
        }
        faces.next().unwrap();
    }
    panic!("expected a conical lateral face on the multiline support");
}

fn check_attached_result(
    feature: &ConeFeatureResult,
    attachment: &ConeFaceAttachment,
    request: &WrappedConeNormalRequest,
) {
    let canonical = feature.tool().extrusion().canonical();
    check_reports(canonical);
    oracles::check_volume(canonical, request.support.normal);
    assert_eq!(feature.tool().footprint().regions.len(), 18);
    assert!(feature.validation().is_valid());
    let detached = feature.export_native().unwrap();
    assert_eq!(detached.body.shape_type().unwrap(), ShapeEnum::Solid);
    let caps = exposed_cap_regions(feature.history(), &detached);
    assert_eq!(
        caps,
        request.selections.iter().map(|item| item.region).collect()
    );
    let support = attachment.export_native(REVISION).unwrap();
    let delta = body_volume_estimate(&detached.body) - body_volume_estimate(&support.body);
    assert_expected_volume_change(request.support.normal, delta);
}

fn exposed_cap_regions(
    history: &WrappedFeatureHistory,
    detached: &DetachedWrappedFeature,
) -> BTreeSet<SurfaceRegionId> {
    let mut covered = BTreeSet::new();
    for (reference, parents) in &history.images {
        assert_eq!(
            detached.entities[reference].shape_type().unwrap(),
            reference.kind
        );
        if reference.kind == ShapeEnum::Face {
            covered.extend(parents.iter().filter_map(|parent| match parent.source {
                CylinderFeatureSource::Tool(WrappedHistoryKey {
                    source: WrappedEntitySource::Region(region),
                    role: HistoryRole::LastDepthFace,
                }) => Some(region),
                _ => None,
            }));
        }
    }
    covered
}

fn assert_expected_volume_change(normal: NormalSense, delta: f64) {
    match normal {
        NormalSense::Outward => assert!(delta > 0., "boss volume delta={delta}"),
        NormalSense::Inward => assert!(delta < 0., "pocket volume delta={delta}"),
    }
}

fn check_intersection_modes(feature: &ConeFeatureResult, placed: CheckMode) {
    for member in feature.tool().extrusion().placement().members.values() {
        assert!(member.validation.is_valid());
        assert_eq!(
            member
                .validation
                .stage(ValidationStage::SelfIntersection)
                .mode,
            placed
        );
    }
    assert_eq!(
        feature
            .validation()
            .stage(ValidationStage::SelfIntersection)
            .mode,
        CheckMode::NativeSearch
    );
}
