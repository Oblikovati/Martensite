//! Time legacy selected-face publication for the 239-curve text fixture (#425).
use super::*;
use std::time::{Duration, Instant};

const COLD_DEPTH: f64 = 0.25;
const EDITED_DEPTH: f64 = 0.1875;
// The exact Green area is 42000445/6291456. Apply the radius-10 radial integral.
const BOSS_VOLUME: f64 = 226_802_403. / 134_217_728.;
const POCKET_VOLUME: f64 = 663_607_031. / 402_653_184.;
const EDITED_BOSS_VOLUME: f64 = 2_713_228_747. / 2_147_483_648.;

#[test]
#[ignore = "run one exact release test per process for selected-face publication latency"]
fn legacy_cylinder_multiline_cold_boss() {
    cold("cold_boss", NormalSense::Outward, BOSS_VOLUME);
}

#[test]
#[ignore = "run one exact release test per process for selected-face publication latency"]
fn legacy_cylinder_multiline_cold_pocket() {
    cold("cold_pocket", NormalSense::Inward, POCKET_VOLUME);
}

#[test]
#[ignore = "run one exact release test per process for selected-face publication latency"]
fn legacy_cylinder_multiline_warm_depth_edit() {
    let attachment = original_support();
    let record = attachment.record().clone();
    let request = original_request(&attachment, NormalSense::Outward);
    let (before, _) = time_legacy(&attachment, &request);
    let snapshot = publication(before.export_native().unwrap().body, &before);
    let mut edited = request.clone();
    edited.depths[1] = EDITED_DEPTH;
    let original_edit = edited.clone();
    let (after, elapsed) = time_legacy(&attachment, &edited);

    assert_eq!(attachment.record(), &record);
    assert_eq!(edited, original_edit);
    check_timed_publication(&attachment, &request, &before, BOSS_VOLUME);
    let after_native = check_timed_publication(&attachment, &edited, &after, EDITED_BOSS_VOLUME);
    check_depth_edit(&snapshot, &before, &after, after_native);
    print_sample("warm_depth_edit", &edited, elapsed);
}

fn cold(case: &str, normal: NormalSense, expected: f64) {
    let attachment = original_support();
    let record = attachment.record().clone();
    let request = original_request(&attachment, normal);
    let original = request.clone();
    let (result, elapsed) = time_legacy(&attachment, &request);

    assert_eq!(request, original);
    assert_eq!(attachment.record(), &record);
    check_timed_publication(&attachment, &request, &result, expected);
    print_sample(case, &request, elapsed);
}

fn original_request(
    attachment: &CylinderFaceAttachment,
    normal: NormalSense,
) -> WrappedNormalRequest {
    placed_request(
        attachment,
        FIRST,
        multiline_request(normal),
        [0., 4.],
        COLD_DEPTH,
    )
}

fn time_legacy(
    attachment: &CylinderFaceAttachment,
    request: &WrappedNormalRequest,
) -> (CylinderFeatureResult, Duration) {
    let kind = feature_kind(request.support.normal);
    let start = Instant::now();
    let result = attachment.apply_wrapped_profile(FIRST, kind, request);
    let elapsed = start.elapsed();
    (result.unwrap(), elapsed)
}

fn check_timed_publication(
    attachment: &CylinderFaceAttachment,
    request: &WrappedNormalRequest,
    result: &CylinderFeatureResult,
    expected: f64,
) -> Shape {
    check_publication(result, request, expected);
    let native = result.export_native().unwrap();
    check_one_solid(&native.body);
    let support = attachment.export_native(FIRST).unwrap();
    check_attachment_volume_direction(&native.body, &support.body, request.support.normal);
    native.body
}

fn check_depth_edit(
    snapshot: &Publication,
    before: &CylinderFeatureResult,
    after: &CylinderFeatureResult,
    after_native: Shape,
) {
    assert_eq!(snapshot.archive, before.encoded_brep());
    assert_eq!(snapshot.body, native_snapshot(&snapshot.shape));
    assert_eq!(snapshot.tool, before.tool().extrusion().encoded_brep());
    let old_volume = body_volume_estimate(&snapshot.shape);
    assert!(body_volume_estimate(&after_native) < old_volume);
    let edited = publication(after_native, after);
    assert_ne!(edited.archive, snapshot.archive);
    assert_ne!(edited.tool, snapshot.tool);
    assert_eq!(edited.authored_keys, snapshot.authored_keys);
}

fn print_sample(case: &str, request: &WrappedNormalRequest, elapsed: Duration) {
    // Process RSS includes setup and oracles. The sample's timer covers the API call only.
    eprintln!(
        concat!(
            "\n",
            r#"{{"event":"legacy_cylinder_multiline_attachment","case":"{}","curves":{},"regions":{},"depth":{},"seconds":{}}}"#
        ),
        case,
        request.profile.curves.len(),
        request.selections.len(),
        request.depths[1],
        elapsed.as_secs_f64()
    );
}
