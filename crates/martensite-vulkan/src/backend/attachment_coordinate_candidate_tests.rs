//! Independent rectangle and payload controls for the candidate convention.
use super::tests::rectangle;
use super::*;

#[test]
fn top_left_partial_regions_do_not_reuse_bottom_up_offsets() {
    let frame = rectangle(0, 0, 23, 18).extent;
    let policy = AttachmentCoordinates::TopLeftLogicalV2;
    let regions = policy.raster_regions(frame, rectangle(3, 2, 12, 10), rectangle(5, 4, 7, 5));
    let viewport = regions.viewport;
    assert_eq!((viewport.x, viewport.y), (3., 12.));
    assert_eq!((viewport.width, viewport.height), (12., -10.));
    assert_eq!((viewport.min_depth, viewport.max_depth), (0., 1.));
    assert_eq!(regions.scissor, rectangle(5, 4, 7, 5));
}

#[test]
fn odd_top_left_frames_start_one_attachment_row_down() {
    // OpenGL pairs window rows (0,1),(2,3).. from the bottom; for 17 rows the top row is alone.
    // One leading attachment row gives Vulkan's top-down 2x2 quads the same row pairs.
    let policy = AttachmentCoordinates::TopLeftLogicalV2;
    let frame = rectangle(0, 0, 23, 17).extent;
    assert_eq!(policy.leading_rows(17), 1);
    assert_eq!(policy.leading_rows(18), 0);
    assert_eq!(AttachmentCoordinates::LegacyBottomUpV1.leading_rows(17), 0);
    assert_eq!(
        policy.attachment_extent(frame),
        rectangle(0, 0, 23, 18).extent
    );
    let regions = policy.raster_regions(frame, rectangle(3, 2, 12, 10), rectangle(5, 4, 7, 5));
    assert_eq!((regions.viewport.x, regions.viewport.y), (3., 13.));
    assert_eq!(regions.viewport.height, -10.);
    assert_eq!(regions.scissor, rectangle(5, 5, 7, 5));
    let offsets = policy.presentation_offsets(frame, frame);
    assert_eq!((offsets.source[0].y, offsets.source[1].y), (1, 18));
    let rows: Vec<u8> = (0..3u8).flat_map(|row| [row; 4]).collect();
    let odd = rectangle(0, 0, 1, 3).extent;
    assert_eq!(policy.logical_rows(rows.clone(), odd), rows[4..].to_vec());
    let even = AttachmentCoordinates::LegacyBottomUpV1;
    assert_eq!(even.logical_rows(rows.clone(), odd), rows);
}

#[test]
fn top_left_capture_keeps_color_and_depth_payload_bits() {
    let policy = AttachmentCoordinates::TopLeftLogicalV2;
    let words = [0x8000_0000_u32, 0x3f80_0000, 0x3e80_0000];
    let bytes: Vec<u8> = words.into_iter().flat_map(u32::to_ne_bytes).collect();
    let expected = bytes.clone();
    assert_eq!(policy.capture_rows(bytes, 1).unwrap(), expected);
}

#[test]
fn candidate_rows_preserve_legacy_malformed_payload_errors() {
    for (bytes, width) in [(vec![], 1), (vec![1; 4], 0), (vec![1; 7], 2)] {
        let legacy = AttachmentCoordinates::LegacyBottomUpV1;
        let candidate = AttachmentCoordinates::TopLeftLogicalV2;
        let old = legacy.capture_rows(bytes.clone(), width).unwrap_err();
        let new = candidate.capture_rows(bytes, width).unwrap_err();
        assert_eq!(old.to_string(), new.to_string());
    }
}

#[test]
fn candidate_presentation_and_facing_agree_with_raw_captures() {
    let policy = AttachmentCoordinates::TopLeftLogicalV2;
    let source = rectangle(0, 0, 23, 18).extent;
    let destination = rectangle(0, 0, 31, 19).extent;
    let offsets = policy.presentation_offsets(source, destination);
    assert_eq!((offsets.source[0].y, offsets.source[1].y), (0, 18));
    assert_eq!(
        (offsets.destination[0].y, offsets.destination[1].y),
        (0, 19)
    );
    assert_eq!(policy.front_face(false), vk::FrontFace::COUNTER_CLOCKWISE);
    assert_eq!(policy.front_face(true), vk::FrontFace::CLOCKWISE);
}

#[test]
fn initial_pipeline_facing_changes_only_the_standard_convention() {
    let legacy = AttachmentCoordinates::LegacyBottomUpV1;
    let candidate = AttachmentCoordinates::TopLeftLogicalV2;
    for face in [vk::FrontFace::CLOCKWISE, vk::FrontFace::COUNTER_CLOCKWISE] {
        assert_eq!(legacy.pipeline_front_face(face), face);
        assert_ne!(candidate.pipeline_front_face(face), face);
    }
    let invalid = vk::FrontFace::from_raw(17);
    assert_eq!(candidate.pipeline_front_face(invalid), invalid);
}
