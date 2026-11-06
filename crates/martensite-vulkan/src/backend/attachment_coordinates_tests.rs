//! Closed forms pin the extracted convention without selecting a new rendering behavior.
use super::*;

pub(super) fn rectangle(x: i32, y: i32, width: u32, height: u32) -> vk::Rect2D {
    vk::Rect2D {
        offset: vk::Offset2D { x, y },
        extent: vk::Extent2D { width, height },
    }
}

#[test]
fn legacy_regions_preserve_asymmetric_offsets_and_depth_chart() {
    let frame = vk::Extent2D {
        width: 23,
        height: 17,
    };
    let regions = AttachmentCoordinates::LegacyBottomUpV1.raster_regions(
        frame,
        rectangle(3, 2, 12, 10),
        rectangle(5, 4, 7, 5),
    );
    let viewport = regions.viewport;
    assert_eq!(
        (viewport.x, viewport.y, viewport.width, viewport.height),
        (3., 5., 12., 10.)
    );
    assert_eq!((viewport.min_depth, viewport.max_depth), (0., 1.));
    assert_eq!(regions.scissor, rectangle(5, 8, 7, 5));
    let _: &(dyn Send + Sync) = &regions;
}

#[test]
fn odd_and_even_presentation_sizes_keep_one_source_reversal() {
    let policy = AttachmentCoordinates::LegacyBottomUpV1;
    for (width, height) in [(23, 17), (24, 18)] {
        let offsets =
            policy.presentation_offsets(fixture_extent(width, height), fixture_extent(31, 19));
        assert_eq!(
            (offsets.source[0].y, offsets.source[1].y),
            (height as i32, 0)
        );
        assert_eq!(
            (offsets.destination[0].y, offsets.destination[1].y),
            (0, 19)
        );
        assert_eq!(
            (offsets.source[1].x, offsets.destination[1].x),
            (width as i32, 31)
        );
        let _: &(dyn Send + Sync) = &offsets;
    }
}

#[test]
fn legacy_capture_rows_keep_rgba_and_depth_bits_identical() {
    let policy = AttachmentCoordinates::LegacyBottomUpV1;
    let bytes = word_bytes(&[1u32, 2, 3, 4, 5, 6]);
    let expected = word_bytes(&[5u32, 6, 3, 4, 1, 2]);
    assert_eq!(policy.capture_rows(bytes, 2).unwrap(), expected);
    let depth = word_bytes(&[0x3f80_0000u32, 0x8000_0000, 0x3e80_0000]);
    let expected = word_bytes(&[0x3e80_0000u32, 0x8000_0000, 0x3f80_0000]);
    assert_eq!(policy.capture_rows(depth, 1).unwrap(), expected);
}

#[test]
fn malformed_capture_rows_retain_original_typed_error_context() {
    let policy = AttachmentCoordinates::LegacyBottomUpV1;
    for (bytes, width) in [(vec![1u8; 7], 1), (vec![1u8; 4], 0)] {
        let direct =
            super::super::attachment_orientation::top_left_attachment_rows(bytes.clone(), width)
                .unwrap_err();
        let selected = policy.capture_rows(bytes, width).unwrap_err();
        assert_eq!(selected.to_string(), direct.to_string());
        assert!(
            selected
                .to_string()
                .contains("complete positive rows of four-byte pixels")
        );
    }
}

#[test]
fn legacy_selection_keeps_mirrored_facing_and_owned_thread_contract() {
    let policy = AttachmentCoordinates::LegacyBottomUpV1;
    assert_eq!(policy.front_face(false), vk::FrontFace::CLOCKWISE);
    assert_eq!(policy.front_face(true), vk::FrontFace::COUNTER_CLOCKWISE);
    let _: &(dyn Send + Sync) = &policy;
}

#[test]
fn top_left_default_reverses_facing_for_logical_y() {
    // Behavior version 2 (decision D1, 2026-10-27) is the default attachment mapping.
    let policy = AttachmentCoordinates::default();
    assert_eq!(policy, AttachmentCoordinates::TopLeftLogicalV2);
    assert_eq!(policy.front_face(false), vk::FrontFace::COUNTER_CLOCKWISE);
    assert_eq!(policy.front_face(true), vk::FrontFace::CLOCKWISE);
}

fn fixture_extent(width: u32, height: u32) -> vk::Extent2D {
    // Keep the same independent top-left rectangle fixture for both output extents.
    rectangle(0, 0, width, height).extent
}

fn word_bytes(words: &[u32]) -> Vec<u8> {
    // Preserve payload bits without routing the expectation through float decoding.
    words.iter().flat_map(|word| word.to_ne_bytes()).collect()
}
