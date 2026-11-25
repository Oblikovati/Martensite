//! Compare a caller-owned chart with the fresh support-decode path.
use super::*;
use crate::surface_sketch::attachment::tests::{body, capture, resolution};
use martensite_tkbrep::{brep::BRepBuilder, topods::Compound};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomCylindricalSurface;
use martensite_tkmath::{
    gp::{Axis3, Point3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::{f64::consts::TAU, hint::black_box, sync::Barrier, time::Instant};

const REVISION: SupportRevision = SupportRevision(3);

fn request(uv: [f64; 2], seam_angle: f64) -> CylinderParameterRequest {
    CylinderParameterRequest {
        uv,
        seam_angle,
        outside: OutOfSupportPolicy::Refuse,
    }
}

fn equal_success_bits(actual: DevelopedCylinderHit, expected: DevelopedCylinderHit) {
    assert_eq!(actual, expected);
    assert_eq!(actual.uv.map(f64::to_bits), expected.uv.map(f64::to_bits));
    for (actual, expected) in actual.coordinates.iter().zip(expected.coordinates) {
        assert_eq!(actual.value.to_bits(), expected.value.to_bits());
        assert_eq!(actual.error_bound.to_bits(), expected.error_bound.to_bits());
    }
}

fn check_chart_parity(
    attachment: &CylinderFaceAttachment,
    chart: &CylinderAuthoringChart,
    revision: SupportRevision,
    sample: CylinderParameterRequest,
) {
    let before = attachment.develop_parameters(revision, sample);
    let after = chart.develop_parameters(revision, sample.uv, sample.outside);
    match (before, after) {
        (Ok(expected), Ok(actual)) => equal_success_bits(actual, expected),
        (Err(expected), Err(actual)) => {
            assert_eq!(actual.class(), expected.class());
            assert_eq!(actual.to_string(), expected.to_string());
        }
        mismatch => {
            panic!("the owned and fresh chart routes disagreed for {sample:?}: {mismatch:?}")
        }
    }
}

#[test]
fn owned_chart_preserves_unwrapped_coordinates_and_trim_results() {
    let attachment = capture();
    for (uv, seam) in [
        ([-0., 2.], -TAU),
        ([TAU, 2.], 0.),
        ([0.25, -0.], -TAU),
        ([0.75, 2.], TAU),
        ([TAU - 0.25, 2.], -TAU),
        ([TAU + 0.25, 2.], TAU),
        ([-0.25, 2.], -TAU),
    ] {
        let chart = attachment.authoring_chart(REVISION, seam).unwrap();
        check_chart_parity(&attachment, &chart, REVISION, request(uv, seam));
    }
}

#[test]
fn owned_chart_preserves_revision_parameter_and_outside_checks() {
    let attachment = capture();
    let chart = attachment.authoring_chart(REVISION, 0.25).unwrap();
    let invalid = request([f64::NAN, 2.], 0.25);
    check_chart_parity(&attachment, &chart, SupportRevision(4), invalid);
    check_chart_parity(&attachment, &chart, REVISION, invalid);
    check_chart_parity(
        &attachment,
        &chart,
        REVISION,
        request([0.75, f64::INFINITY], 0.25),
    );
    let mut outside = request([0.75, 6.], 0.25);
    check_chart_parity(&attachment, &chart, REVISION, outside);
    outside.outside = OutOfSupportPolicy::Report;
    check_chart_parity(&attachment, &chart, REVISION, outside);
}

#[test]
fn opening_checks_revision_before_seam_and_keeps_legacy_input_order() {
    let attachment = capture();
    let stale = attachment
        .authoring_chart(SupportRevision(4), f64::NAN)
        .err()
        .unwrap();
    assert_eq!(stale.class(), AttachmentFailureClass::StaleRevision);
    let invalid = attachment
        .authoring_chart(REVISION, f64::INFINITY)
        .err()
        .unwrap();
    assert!(matches!(
        invalid.cause,
        AttachmentFailureCause::ChartValue {
            field: "seam angle",
            ..
        }
    ));
    let invalid = request([f64::NAN, f64::INFINITY], f64::INFINITY);
    let legacy = attachment
        .develop_parameters(REVISION, invalid)
        .unwrap_err();
    assert!(matches!(
        legacy.cause,
        AttachmentFailureCause::ChartValue {
            field: "native u",
            ..
        }
    ));
}

#[test]
fn owned_chart_has_independent_owners_and_concurrent_readers() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CylinderAuthoringChart>();
    let (body, face) = body(10.);
    let attachment =
        CylinderFaceAttachment::capture(FaceAttachmentId(7), REVISION, &body, &face, resolution())
            .unwrap();
    let restored = CylinderFaceAttachment::restore(attachment.record().clone()).unwrap();
    let sample = request([0.75, 2.], -TAU);
    let chart = restored
        .authoring_chart(REVISION, sample.seam_angle)
        .unwrap();
    let expected = restored.develop_parameters(REVISION, sample).unwrap();
    let first_export = restored.export_native(REVISION).unwrap();
    let second_export = restored.export_native(REVISION).unwrap();
    assert_eq!(restored.record(), attachment.record());
    drop(restored);
    assert!(!chart.face.is_partner(&face));
    assert!(!chart.face.is_partner(&first_export.face));
    assert!(!first_export.face.is_partner(&second_export.face));
    mutate_radius(&face, 12.);
    check_concurrent_mapping(&chart, &first_export.face, sample, expected);
    assert_eq!(encoded_face(&chart.face), encoded_face(&second_export.face));
    assert_eq!(radius_for_test(&second_export.face), 10.);
}

fn check_concurrent_mapping(
    chart: &CylinderAuthoringChart,
    detached: &Face,
    sample: CylinderParameterRequest,
    expected: DevelopedCylinderHit,
) {
    let start = Barrier::new(5);
    std::thread::scope(|scope| {
        scope.spawn(|| {
            start.wait();
            for index in 0..32 {
                mutate_radius(detached, 17. + f64::from(index));
            }
        });
        for _ in 0..4 {
            scope.spawn(|| {
                start.wait();
                for _ in 0..32 {
                    let actual = chart
                        .develop_parameters(REVISION, sample.uv, sample.outside)
                        .unwrap();
                    equal_success_bits(actual, expected);
                }
            });
        }
    });
}

fn mutate_radius(face: &Face, new_radius: f64) {
    let surface = Handle::new(GeomCylindricalSurface::new(Axis3::default(), new_radius).unwrap())
        .into_upcast();
    BRepBuilder::default()
        .update_face(face, &surface, &Location::default(), 1e-7)
        .unwrap();
    assert_eq!(radius_for_test(face), new_radius);
}

fn radius_for_test(face: &Face) -> f64 {
    let attempt = Attempt::new(AttachmentOperation::Develop, FaceAttachmentId(7), REVISION);
    radius(&attempt, face).unwrap()
}

fn encoded_face(face: &Face) -> Vec<u8> {
    let attempt = Attempt::new(AttachmentOperation::Develop, FaceAttachmentId(7), REVISION);
    codec::encode(&attempt, &face.clone().into_shape(), face).unwrap()
}

#[test]
#[ignore = "comparison benchmark for repeated selected-face hit mapping"]
fn benchmark_owned_cylinder_hit_mapping() {
    benchmark_face("cylinder", &capture());
    benchmark_face("cylinder_with_16_disjoint_solids", &compound_support());
}

fn compound_support() -> CylinderFaceAttachment {
    let (body, face) = body(10.);
    let builder = BRepBuilder::default();
    let mut support = Compound::default();
    builder.make_compound(&mut support);
    builder.add(&mut support, &body).unwrap();
    for index in 0..16 {
        let corner = Point3::new(30. + f64::from(index) * 5., 0., 0.);
        let mut maker = BRepPrimAPIMakeBox::at_point(corner, 2., 3., 4.).unwrap();
        builder.add(&mut support, maker.shape().unwrap()).unwrap();
    }
    CylinderFaceAttachment::capture(FaceAttachmentId(7), REVISION, &support, &face, resolution())
        .unwrap()
}

fn benchmark_face(label: &str, attachment: &CylinderFaceAttachment) {
    let chart = attachment.authoring_chart(REVISION, TAU).unwrap();
    let (mut fresh, mut owned) = (Vec::with_capacity(256), Vec::with_capacity(256));
    for index in 0..272 {
        let sample = request([0.125 + f64::from(index % 119) * 0.05, 2.], TAU);
        let (before, after) = benchmark_pair(attachment, &chart, sample, index % 2 == 0);
        if index >= 16 {
            fresh.push(before);
            owned.push(after);
        }
    }
    log_samples(label, "fresh", &mut fresh, attachment.record().brep.len());
    log_samples(
        label,
        "owned_chart",
        &mut owned,
        attachment.record().brep.len(),
    );
}

fn benchmark_pair(
    attachment: &CylinderFaceAttachment,
    chart: &CylinderAuthoringChart,
    sample: CylinderParameterRequest,
    fresh_first: bool,
) -> (u128, u128) {
    let former = || timed(|| black_box(attachment).develop_parameters(REVISION, black_box(sample)));
    let current = || {
        timed(|| {
            black_box(chart).develop_parameters(REVISION, black_box(sample.uv), sample.outside)
        })
    };
    let (before, after) = if fresh_first {
        (former(), current())
    } else {
        let after = current();
        (former(), after)
    };
    equal_success_bits(after.0, before.0);
    (before.1, after.1)
}

fn timed(run: impl FnOnce() -> Checked<DevelopedCylinderHit>) -> (DevelopedCylinderHit, u128) {
    let start = Instant::now();
    let result = black_box(run().unwrap());
    (result, start.elapsed().as_nanos())
}

fn log_samples(label: &str, route: &str, times: &mut [u128], source_bytes: usize) {
    times.sort_unstable();
    let count = times.len();
    let p50 = times[count.div_ceil(2) - 1];
    let p95 = times[(count * 95).div_ceil(100) - 1];
    let max = times[count - 1];
    println!(
        r#"{{"event":"cylinder_hit_mapping","support":"{label}","route":"{route}","n":{count},"source_bytes":{source_bytes},"p50_ns":{p50},"p95_ns":{p95},"max_ns":{max}}}"#
    );
}
