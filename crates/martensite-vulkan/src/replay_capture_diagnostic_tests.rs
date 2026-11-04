use super::*;

fn owned_capture_fixture() -> FrameCapture {
    FrameCapture {
        width: 1,
        height: 1,
        rgba: vec![11, 22, 33, 44],
        depth: vec![f32::from_bits(0x7fc00123)],
    }
}
fn readback_arguments_fixture() -> ReadbackArguments {
    ReadbackArguments {
        color: [1, 2, 3],
        depth: [4, 5, 6],
        formats: [37, 129],
        samples: [1, 1],
        attachment_extent: [1, 2],
        leading_rows: 1,
        coordinates: RenderCoordinateConvention::TopLeftLogicalV2,
    }
}
#[test]
fn actual_readback_bytes_depth_bits_and_row_mapping_are_owned() {
    let mut capture = owned_capture_fixture();
    let value = readback_json(&capture, readback_arguments_fixture());
    capture.rgba.fill(0);
    capture.depth.fill(0.0);
    assert_eq!(value["rgba"], json!([11, 22, 33, 44]));
    assert_eq!(value["depth_bits"], json!([0x7fc00123u32]));
    assert_eq!(value["color_image_view_buffer"], json!([1, 2, 3]));
    assert_eq!(value["leading_rows"], 1);
    assert_eq!(value["submission_completion_join_required"], true);
}
#[test]
fn equal_image_values_do_not_collapse_distinct_live_owners() {
    let first = owned_capture_fixture();
    let second = first.clone();
    assert_eq!(first.rgba, second.rgba);
    assert_ne!(
        capture_json(&first)["rgba_owner"],
        capture_json(&second)["rgba_owner"]
    );
    assert_ne!(
        capture_json(&first)["depth_owner"],
        capture_json(&second)["depth_owner"]
    );
}
#[test]
fn failed_publication_cannot_gain_a_capture_owner() {
    let outcome = Err(crate::error::rejected(17, "a successful publication"));
    let value = publication_json(71, 19, &outcome);
    assert!(value["capture"].is_null());
    assert_eq!(
        value["publication_refusal"],
        json!(outcome.as_ref().unwrap_err().to_string())
    );
    assert!(value.get("error").is_none());
    assert_eq!(value["view"], 71);
    assert_eq!(value["publication"], 19);
}
#[test]
fn clone_retains_both_source_and_new_output_tokens() {
    let source = Some(Ok(owned_capture_fixture()));
    let output = source.clone();
    let value = clone_json(73, 23, &source, &output);
    assert_ne!(value["source"]["rgba_owner"], value["output"]["rgba_owner"]);
    assert_eq!(value["source"]["extent"], value["output"]["extent"]);
    assert_eq!(value["publication"], 23);
}
#[test]
fn missing_capture_and_changed_publication_stay_explicit() {
    assert!(clone_json(79, 29, &None, &None)["output"].is_null());
    let value = dump_json(79, [29, 30], "Rgb", false, &owned_capture_fixture());
    assert_eq!(value["publications_before_after"], json!([29, 30]));
    assert_eq!(value["copied"], false);
}

#[test]
fn unsupported_gui_and_transform_routes_are_explicit_observations() {
    let value = rendered_json(83, &owned_capture_fixture(), [1, 2, 1, 1]);
    assert_eq!(value["scaled_subviews_flip_gui"], json!([1, 2, 1, 1]));
}

#[test]
fn successful_publication_has_no_bare_fatal_diagnostic_key() {
    let value = publication_json(71, 19, &Ok(owned_capture_fixture()));
    assert!(value["publication_refusal"].is_null());
    assert!(value.get("error").is_none());
}
