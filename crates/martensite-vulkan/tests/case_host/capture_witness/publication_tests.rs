use super::*;
#[test]
fn closed_call_tokens_do_not_depend_on_image_values_or_basename_escaping() {
    let value = scope_json(
        "end",
        19,
        23,
        "a\"b.png",
        json!({"valid":false}),
        Some(false),
    );
    let encoded = value.to_string();
    assert_eq!(serde_json::from_str::<Value>(&encoded).unwrap(), value);
    assert_eq!(value["succeeded"], false);
    assert_eq!(value["cursor"]["valid"], false);
    assert_eq!(value["token"], 19);
}
#[test]
fn changed_publication_and_capture_owner_are_not_replaced_with_a_guess() {
    let capture = FrameCapture {
        width: 1,
        height: 1,
        rgba: vec![0; 4],
        depth: vec![0.0],
    };
    let value = snapshot_json(
        29,
        "a.png",
        &capture,
        json!({"valid":true,"publication":31}),
        json!({"valid":true,"publication":32}),
    );
    assert_eq!(value["cursor_before"]["publication"], 31);
    assert_eq!(value["cursor_after"]["publication"], 32);
    assert_eq!(value["rgba_owner"], capture.rgba.as_ptr() as usize);
}
