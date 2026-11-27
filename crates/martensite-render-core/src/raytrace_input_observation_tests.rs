//! Owned fixtures make joins fail independently of Vulkan availability.
use super::*;
fn capture() -> FrameCapture {
    FrameCapture {
        width: 1,
        height: 1,
        rgba: vec![11, 22, 33, 255],
        depth: vec![-0.0],
    }
}
fn shader() -> Vec<u8> {
    [0x07230203u32, 0x00010000, 0, 1, 0]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect()
}
fn pipeline() -> RayPipelineInputs {
    RayPipelineInputs {
        pipeline: 13,
        layout: 17,
        modules: vec![19, 23],
        scene_layout: 43,
        specialization_entries: (0..10).map(|id| [id, 4 * id, 4]).collect(),
        program: "Render".into(),
        specialization: (0..40).collect(),
        vertex: shader(),
        fragment: shader(),
    }
}
fn upload() -> RayUploadInputs {
    RayUploadInputs {
        buffer: 5,
        set: 7,
        layout: 43,
        bytes: (0..416).map(|i| (i % 251) as u8).collect(),
    }
}
fn dispatch() -> RayDispatchInputs {
    RayDispatchInputs {
        command: 29,
        sets: vec![7],
        pipeline: pipeline(),
    }
}
fn command() -> RayCommandInputs {
    RayCommandInputs {
        command: 29,
        logical: [409, 409],
        attachment: [409, 410],
    }
}
fn complete(token: &RayInputObservation) {
    token.uploaded(upload());
    token.dispatched(dispatch());
    token.command(command());
}
fn accepted() -> publication::ViewRayInputs {
    let mut state = ViewRayInputs::default();
    let token = state.begin(101, 3).unwrap();
    complete(&token);
    state.finished(&token, &capture());
    state.publish(&Ok(capture()), 41);
    state.dumped(41, &capture(), true);
    state
}
#[test]
fn exact_upload_pipeline_constants_and_frame_identity_survive_publication() {
    let json: serde_json::Value =
        serde_json::from_str(&accepted().witness(101, 41, &capture()).unwrap()).unwrap();
    assert_eq!(json["uniform_bytes"], serde_json::json!(upload().bytes));
    assert_eq!(
        json["specialization_bytes"],
        serde_json::json!(pipeline().specialization)
    );
    assert_eq!(json["attachment_extent"], serde_json::json!([409, 410]));
    assert_eq!(json["view_id"], 3);
    assert_eq!(json["accepted_publication"], 41);
    assert_eq!(json["post_vertex_observed"], false);
}
#[test]
fn wrong_view_publication_or_capture_never_reuses_accepted_inputs() {
    let state = accepted();
    assert!(state.witness(102, 41, &capture()).is_err());
    assert!(state.witness(101, 42, &capture()).is_err());
    let mut changed = capture();
    changed.rgba[0] += 1;
    assert!(state.witness(101, 41, &changed).is_err());
    changed = capture();
    changed.depth[0] = 0.0;
    assert!(state.witness(101, 41, &changed).is_err());
}
#[test]
fn duplicate_upload_dispatch_or_frame_is_ambiguous() {
    for repeat in [0, 1, 2] {
        let token = RayInputObservation::new(101, 3, 1);
        complete(&token);
        match repeat {
            0 => token.uploaded(upload()),
            1 => token.dispatched(dispatch()),
            _ => token.command(command()),
        }
        assert!(token.encode(41).is_err());
    }
}
#[test]
fn different_descriptor_or_command_never_joins_by_order() {
    for changed_set in [false, true] {
        let token = RayInputObservation::new(101, 3, 1);
        token.uploaded(upload());
        token.command(command());
        let mut actual = dispatch();
        if changed_set {
            actual.sets = vec![31];
        } else {
            actual.command = 31;
        }
        token.dispatched(actual);
        assert!(token.encode(41).is_err());
    }
}
#[test]
fn missing_and_explicit_unsupported_observations_refuse() {
    let token = RayInputObservation::new(101, 3, 1);
    assert!(token.encode(41).is_err());
    complete(&token);
    token.refuse("unsupported FSAA");
    assert!(token.encode(41).unwrap_err().to_string().contains("FSAA"));
}
#[test]
fn overlapping_redraws_are_sticky_ambiguous_even_for_identical_images() {
    let mut state = ViewRayInputs::default();
    let first = state.begin(101, 3).unwrap();
    let second = state.begin(101, 3).unwrap();
    complete(&first);
    complete(&second);
    state.finished(&second, &capture());
    state.publish(&Ok(capture()), 41);
    state.finished(&first, &capture());
    state.publish(&Ok(capture()), 42);
    assert!(state.witness(101, 42, &capture()).is_err());
}
#[test]
fn failed_publication_and_new_unfinished_frame_invalidate_old_evidence() {
    let mut state = accepted();
    state.begin(101, 3).unwrap();
    assert!(state.witness(101, 41, &capture()).is_err());
    state.publish(&Err(rejected("owned failure", "completed frame")), 42);
    assert!(state.witness(101, 42, &capture()).is_err());
}
#[test]
fn malformed_uniform_and_pipeline_payloads_refuse() {
    let token = RayInputObservation::new(101, 3, 1);
    let mut short = upload();
    short.bytes.pop();
    token.uploaded(short);
    token.command(command());
    token.dispatched(dispatch());
    assert!(token.encode(41).is_err());
    let token = RayInputObservation::new(101, 3, 2);
    token.uploaded(upload());
    token.command(command());
    let mut wrong = dispatch();
    wrong.pipeline.specialization.pop();
    token.dispatched(wrong);
    assert!(token.encode(42).is_err());
}
#[test]
fn incomplete_payload_or_changed_completed_image_never_publishes() {
    let mut state = ViewRayInputs::default();
    let token = state.begin(101, 3).unwrap();
    complete(&token);
    state.finished(&token, &capture());
    let mut other = capture();
    other.rgba[0] = 77;
    state.publish(&Ok(other.clone()), 41);
    assert!(state.witness(101, 41, &other).is_err());
}
#[test]
fn selector_is_explicit_and_never_accepts_truthy_strings() {
    use std::ffi::OsStr;
    assert!(!selected(None).unwrap());
    assert!(selected(Some(OsStr::new("1"))).unwrap());
    for value in ["", "0", "true", "1 "] {
        assert!(selected(Some(OsStr::new(value))).is_err());
    }
}
#[test]
fn every_composed_or_multiple_ray_path_requires_a_future_observer_contract() {
    assert!(supported_scope(Some((false, false)), false, false, false));
    assert!(!supported_scope(None, false, false, false));
    for flags in 1u8..32 {
        let ray = Some((flags & 1 != 0, flags & 2 != 0));
        assert!(!supported_scope(
            ray,
            flags & 4 != 0,
            flags & 8 != 0,
            flags & 16 != 0
        ));
    }
}
#[test]
fn malformed_capture_is_not_retained_and_source_modules_are_bounded() {
    let mut state = ViewRayInputs::default();
    let token = state.begin(101, 3).unwrap();
    complete(&token);
    let mut malformed = capture();
    malformed.depth.clear();
    state.finished(&token, &malformed);
    state.publish(&Ok(malformed.clone()), 41);
    assert!(state.witness(101, 41, &malformed).is_err());
    assert!(!encoding::valid_spirv(&[]));
}

#[test]
fn failed_or_different_buffer_dump_cannot_reuse_a_prior_success() {
    let mut state = accepted();
    state.dumped(41, &capture(), false);
    assert!(state.witness(101, 41, &capture()).is_err());
    state.dumped(42, &capture(), true);
    assert!(state.witness(101, 41, &capture()).is_err());
    state.dumped(41, &capture(), true);
    let json: serde_json::Value =
        serde_json::from_str(&state.witness(101, 41, &capture()).unwrap()).unwrap();
    assert_eq!(json["accepted_buffer_dump_ordinal"], 2);
    assert_eq!(json["buffer_dump_succeeded"], true);
}
#[test]
fn wrong_descriptor_layout_and_specialization_entry_offsets_refuse() {
    for changed_layout in [false, true] {
        let token = RayInputObservation::new(101, 3, 1);
        token.uploaded(upload());
        token.command(command());
        let mut actual = dispatch();
        if changed_layout {
            actual.pipeline.scene_layout = 47;
        } else {
            actual.pipeline.specialization_entries[4][1] = 0;
        }
        token.dispatched(actual);
        assert!(token.encode(41).is_err());
    }
}

#[test]
fn external_stream_markers_bind_actual_command_to_immutable_view_frame_token() {
    let token = RayInputObservation::new(101, 3, 7);
    for (phase, expected) in [
        (RayInputPhase::Frame, "raytrace-input-frame"),
        (RayInputPhase::Dispatch, "raytrace-input-render-dispatch"),
    ] {
        let json: serde_json::Value = serde_json::from_str(&token.marker(phase, 29)).unwrap();
        assert_eq!(json["kind"], expected);
        assert_eq!(json["view_address"], 101);
        assert_eq!(json["view_id"], 3);
        assert_eq!(json["frame_generation"], 7);
        assert_eq!(json["command"], 29);
        assert_eq!(json["post_vertex_observed"], false);
    }
}
