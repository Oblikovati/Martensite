//! The Phong compile-path grader accepts only one-level lit-interior residues of recorded cases.
use super::*;

const BACKGROUND: [u8; 4] = [0, 0, 0, 255];
const LIT: [u8; 4] = [220, 166, 76, 255];
const RECORDED: &[&str] = &["bug264_0"];

/// A background pixel followed by three lit pixels, with pixel `index` set to `colour`.
fn frame(index: usize, colour: [u8; 4]) -> Vec<u8> {
    let mut rgba = [BACKGROUND, LIT, LIT, LIT].concat();
    rgba[index * 4..index * 4 + 4].copy_from_slice(&colour);
    rgba
}

fn native() -> Vec<u8> {
    frame(1, LIT)
}

#[test]
fn a_one_level_lit_residue_of_a_recorded_case_passes() {
    let rust = frame(2, [220, 167, 76, 255]);
    let grade = grade_phong_rounding("bug264_0", &rust, &native(), RECORDED);
    assert_eq!(grade, Ok(PhongGrade { differing: 1 }));
}

#[test]
fn a_case_without_a_recorded_control_is_rejected() {
    let rust = frame(2, [220, 167, 76, 255]);
    let error = grade_phong_rounding("bug264_5", &rust, &native(), RECORDED).unwrap_err();
    assert!(error.contains("no recorded control"), "{error}");
}

#[test]
fn a_two_level_difference_is_rejected() {
    let rust = frame(2, [220, 168, 76, 255]);
    let error = grade_phong_rounding("bug264_0", &rust, &native(), RECORDED).unwrap_err();
    assert!(error.contains("within one level"), "{error}");
}

#[test]
fn an_alpha_change_is_rejected() {
    let rust = frame(2, [220, 166, 76, 254]);
    assert!(grade_phong_rounding("bug264_0", &rust, &native(), RECORDED).is_err());
}

#[test]
fn coverage_changes_are_rejected_both_ways() {
    let uncovered = frame(2, BACKGROUND);
    let covered = frame(0, [1, 0, 0, 255]);
    for rust in [uncovered, covered] {
        let error = grade_phong_rounding("bug264_0", &rust, &native(), RECORDED);
        assert!(error.unwrap_err().contains("coverage"));
    }
}

#[test]
fn unequal_buffers_are_rejected() {
    let error = grade_phong_rounding("bug264_0", &[0; 8], &native(), RECORDED);
    assert!(
        error
            .unwrap_err()
            .contains("equal non-empty multiples of 4")
    );
}

/// The quoted names of `key`'s list in the committed control receipt.
fn receipt_list(key: &str) -> Vec<String> {
    let receipt = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/recorded_controls/vulkan-phong-compile-path-2026-10-28/controls.json"
    ))
    .expect("the committed Phong control receipt");
    let start = receipt
        .find(&format!("\"{key}\""))
        .unwrap_or_else(|| panic!("{key} key"));
    let list = &receipt[start..receipt[start..].find(']').map(|end| start + end).unwrap()];
    list.split('"')
        .skip(3)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

#[test]
fn the_recorded_list_matches_the_control_receipt() {
    assert_eq!(receipt_list("recorded_tests"), RECORDED_PHONG_CONTROLS);
}

#[test]
fn the_attribute_rounding_list_matches_the_control_receipt() {
    let names = receipt_list("attribute_rounding_tests");
    assert_eq!(names, RECORDED_ATTRIBUTE_ROUNDING_CONTROLS);
    assert!(
        names
            .iter()
            .all(|n| !RECORDED_PHONG_CONTROLS.contains(&n.as_str()))
    );
}

#[test]
fn attribute_rounding_controls_grant_only_their_own_cases() {
    let rust = frame(2, [220, 167, 76, 255]);
    let recorded = RECORDED_ATTRIBUTE_ROUNDING_CONTROLS;
    assert!(grade_phong_rounding("bug264_5", &rust, &native(), recorded).is_ok());
    assert!(grade_phong_rounding("bug287", &rust, &native(), recorded).is_err());
    let two_levels = frame(2, [220, 168, 76, 255]);
    assert!(grade_phong_rounding("bug264_5", &two_levels, &native(), recorded).is_err());
}
