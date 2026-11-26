use super::*;
#[test]
fn unrequested_metadata_preserves_existing_complete_bytes() {
    let original = "schema=existing\nlimits=no-uploaded-buffer-or-shader-interpolation-witness\n";
    assert_eq!(raytrace_completion(original.into(), None), original);
}
#[test]
fn additive_inputs_stay_inside_the_existing_sealed_complete_piece() {
    let original = "schema=existing\n";
    let complete = raytrace_completion(original.into(), Some("{\"view_id\":7}"));
    assert!(complete.starts_with(original));
    let json = complete
        .lines()
        .find_map(|line| line.strip_prefix("raytrace_inputs_json="))
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(parsed["view_id"], 7);
    assert_eq!(complete.matches("raytrace_inputs_json=").count(), 1);
}
