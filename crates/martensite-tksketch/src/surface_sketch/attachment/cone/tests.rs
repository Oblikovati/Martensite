//! Independent native support and parameter controls.
use super::*;
use crate::surface_sketch::NormalSense;
use martensite_tkbrep::{
    brep::BRepBuilder, brep_adaptor::BRepAdaptorSurface, topexp::Explorer, topods,
};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dSurface,
    top_abs::{ShapeEnum, State},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCone;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
mod workflow;

const ID: FaceAttachmentId = FaceAttachmentId(91);
const REVISION: SupportRevision = SupportRevision(12);

#[test]
fn immutable_cone_attachment_record_and_chart_can_be_shared() {
    fn shared<T: Send + Sync>() {}
    shared::<ConeAttachmentRecord>();
    shared::<ConeFaceAttachment>();
    shared::<ConeAuthoringChart>();
    shared::<DevelopedConeHit>();
    shared::<ConeAttachmentFailure>();
}

fn frustum(radii: [f64; 2]) -> (Shape, Face) {
    let mut maker = BRepPrimAPIMakeCone::with_angle(radii[0], radii[1], 10., 2.).unwrap();
    let body = maker.shape().unwrap().clone();
    let face = select_kind(&body, SurfaceType::Cone);
    (body, face)
}
fn select_kind(body: &Shape, kind: SurfaceType) -> Face {
    let attempt = Attempt::new(AttachmentOperation::Capture, ID, REVISION);
    let mut faces = Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while faces.more() {
        let face = topods::face(faces.current()).unwrap();
        if admission::surface_type(&attempt, &face).unwrap() == kind {
            return face;
        }
        faces.next().unwrap();
    }
    panic!("fixture must have a {kind:?} face");
}
fn attach(radii: [f64; 2]) -> ConeFaceAttachment {
    let (body, face) = frustum(radii);
    ConeFaceAttachment::capture(
        ID,
        REVISION,
        &body,
        &face,
        super::super::tests::resolution(),
    )
    .unwrap()
}

#[test]
fn frozen_cone_replays_with_fresh_owners_and_follows_recorded_history() {
    let attachment = attach([2., 4.]);
    let restored = ConeFaceAttachment::restore(attachment.record().clone()).unwrap();
    assert_eq!(attachment.record(), restored.record());
    assert!(restored.validation().is_valid());
    assert!(Arc::ptr_eq(&attachment.frozen, &attachment.clone().frozen));
    let detached = restored.export_native(REVISION).unwrap();
    assert!(
        !detached
            .body
            .is_partner(&attachment.export_native(REVISION).unwrap().body)
    );
    check_history(&restored, &detached);
    BRepBuilder::default()
        .update_face_tolerance(&detached.face, 0.1)
        .unwrap();
    assert_eq!(restored.record(), attachment.record());
}
fn check_history(attachment: &ConeFaceAttachment, source: &DetachedConeSupport) {
    let (target, face) = frustum([3., 5.]);
    let history = History::new();
    history.add_modified(&source.face, &face).unwrap();
    let revision = SupportRevision(13);
    let rebound = attachment
        .reattach(source, &target, revision, &history)
        .unwrap();
    assert_eq!(rebound.record().revision, revision);
    assert_eq!(rebound.record().id, ID);
    let chart = rebound.authoring_chart(revision, 0.).unwrap();
    assert!(
        (chart
            .extrusion_support(revision, NormalSense::Outward)
            .unwrap()
            .apex[2]
            + 15.)
            .abs()
            < 1e-12
    );
}

#[test]
fn wrong_kind_and_mismatched_native_face_are_separate_refusals() {
    let (body, _) = frustum([2., 4.]);
    let (_, foreign) = frustum([2., 4.]);
    let capture = |face| {
        ConeFaceAttachment::capture(ID, REVISION, &body, face, super::super::tests::resolution())
    };
    let plane = select_kind(&body, SurfaceType::Plane);
    assert!(matches!(
        capture(&plane),
        Err(ConeAttachmentFailure::Surface {
            actual: SurfaceType::Plane,
            ..
        })
    ));
    let error = capture(&foreign).err().unwrap();
    assert_eq!(
        shared_cause(error).class(),
        AttachmentFailureClass::InvalidInput
    );
}
fn shared_cause(error: ConeAttachmentFailure) -> Box<AttachmentFailure> {
    let ConeAttachmentFailure::Attachment(cause) = error else {
        panic!("{error}")
    };
    cause
}

#[test]
fn growing_and_narrowing_native_cones_develop_to_the_same_geometric_point() {
    check_world_hit([2., 4.]);
    check_world_hit([4., 2.]);
}
fn check_world_hit(radii: [f64; 2]) {
    let attachment = attach(radii);
    let chart = attachment.authoring_chart(REVISION, 0.4).unwrap();
    let uv = [0.75, 3.];
    let hit = chart
        .develop_parameters(REVISION, request(uv, OutOfSupportPolicy::Refuse))
        .unwrap();
    assert_eq!(hit.trim_state, State::In);
    assert_eq!(
        hit.trim_check,
        martensite_tkbo::body_check::CheckMode::NativeSearch
    );
    let support = chart
        .extrusion_support(REVISION, NormalSense::Outward)
        .unwrap();
    let native = attachment.export_native(REVISION).unwrap();
    let adaptor = BRepAdaptorSurface::with_face(&native.face, false).unwrap();
    let native_point = adaptor.eval_d0(uv[0], uv[1]).unwrap();
    let developed = calculate_world_point(support, hit);
    assert!((native_point.x() - developed[0]).abs() < 1e-12);
    assert!((native_point.y() - developed[1]).abs() < 1e-12);
    assert!((native_point.z() - developed[2]).abs() < 1e-12);
}
fn calculate_world_point(
    support: crate::surface_sketch::ConeSupport,
    hit: DevelopedConeHit,
) -> [f64; 3] {
    let [x, y] = hit.coordinates.map(|coordinate| coordinate.value);
    let radius = martensite_cmath::hypot(x, y);
    let (sine, cosine) = martensite_cmath::sincos(support.semi_angle);
    let u = martensite_cmath::atan2(y, x) / sine + support.chart_origin;
    let (tangential, radial) = martensite_cmath::sincos(u);
    let z = support.axis[2];
    [
        radius * sine * radial,
        z * radius * sine * tangential,
        support.apex[2] + z * radius * cosine,
    ]
}

fn request(uv: [f64; 2], outside: OutOfSupportPolicy) -> ConeParameterRequest {
    ConeParameterRequest { uv, outside }
}

#[test]
fn cached_chart_distinguishes_stale_revision_outside_trim_and_apex_refusals() {
    let chart = attach([2., 4.]).authoring_chart(REVISION, 0.4).unwrap();
    let outside = [0.75, 11.];
    let report = chart
        .develop_parameters(REVISION, request(outside, OutOfSupportPolicy::Report))
        .unwrap();
    assert_eq!(report.trim_state, State::Out);
    let error = chart
        .develop_parameters(REVISION, request(outside, OutOfSupportPolicy::Refuse))
        .unwrap_err();
    assert!(matches!(
        shared_cause(error).cause,
        AttachmentFailureCause::ChartTrim { state: State::Out }
    ));
    let error = chart
        .develop_parameters(
            SupportRevision(1),
            request(outside, OutOfSupportPolicy::Report),
        )
        .unwrap_err();
    assert_eq!(
        shared_cause(error).class(),
        AttachmentFailureClass::StaleRevision
    );
    let error = chart
        .develop_parameters(REVISION, request([0.75, -20.], OutOfSupportPolicy::Report))
        .unwrap_err();
    assert!(
        matches!(error, ConeAttachmentFailure::Development { cause, .. } if matches!(*cause, martensite_tkmath::predicates::roots::metric_bounds::ConicalDevelopmentFailure::Slant { .. }))
    );
}
