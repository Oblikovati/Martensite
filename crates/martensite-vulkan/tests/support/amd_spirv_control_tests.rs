//! The AMD SPIR-V control grader accepts only proven, lit pixels of covered controls on RADV.
use super::*;

const RADV: &str = "AMD Radeon Graphics (RADV GFX1151)";
const BACKGROUND: [u8; 4] = [0, 0, 0, 255];
const LIT: [u8; 4] = [187, 136, 27, 255];
const PROVEN: &[(usize, usize)] = &[(1, 1)];

fn proof(coverage_identical: bool) -> SpirvControlProof<'static> {
    SpirvControlProof {
        test: "bug23795",
        image: "bug23795.png",
        coverage_identical,
        coverage_identical_outside_text_line_point: false,
        pixels: PROVEN,
    }
}

#[test]
fn legacy_non_lit_proof_is_not_qualified() {
    let rust = frame(3, [173, 126, 24, 255]);
    let missing = SpirvControlProof {
        coverage_identical_outside_text_line_point: true,
        ..proof(false)
    };
    assert!(
        grade_spirv_control(RADV, &missing, &rust, &frame(3, LIT), 2)
            .unwrap_err()
            .contains("diagnostic only")
    );
}

/// A 2x2 frame: background corner, three lit pixels, with pixel `index` set to `colour`.
fn frame(index: usize, colour: [u8; 4]) -> Vec<u8> {
    let mut rgba = [BACKGROUND, LIT, LIT, LIT].concat();
    rgba[index * 4..index * 4 + 4].copy_from_slice(&colour);
    rgba
}

#[test]
fn legacy_pixel_list_is_not_qualified_without_a_colour_bound() {
    let rust = frame(3, [173, 126, 24, 255]);
    let grade = grade_spirv_control(RADV, &proof(true), &rust, &frame(3, LIT), 2);
    assert!(grade.unwrap_err().contains("diagnostic only"));
}

#[test]
fn other_devices_are_refused() {
    let rust = frame(3, [173, 126, 24, 255]);
    let error = grade_spirv_control("llvmpipe", &proof(true), &rust, &frame(3, LIT), 2);
    assert!(error.unwrap_err().contains("AMD only"));
}

#[test]
fn a_control_with_other_coverage_is_refused() {
    let rust = frame(3, [173, 126, 24, 255]);
    let error = grade_spirv_control(RADV, &proof(false), &rust, &frame(3, LIT), 2);
    assert!(error.unwrap_err().contains("coverage differs"));
}

#[test]
fn an_unproven_pixel_is_refused() {
    let rust = frame(2, [173, 126, 24, 255]);
    let error = grade_spirv_control(RADV, &proof(true), &rust, &frame(2, LIT), 2);
    assert!(error.unwrap_err().contains("diagnostic only"));
}

#[test]
fn a_proven_pixel_that_changes_coverage_is_refused() {
    let rust = frame(3, BACKGROUND);
    let error = grade_spirv_control(RADV, &proof(true), &rust, &frame(3, LIT), 2);
    assert!(error.unwrap_err().contains("diagnostic only"));
}

#[test]
fn malformed_images_are_refused() {
    let native = frame(3, LIT);
    assert!(grade_spirv_control(RADV, &proof(true), &native[..12], &native, 2).is_err());
    assert!(grade_spirv_control(RADV, &proof(true), &native, &native, 3).is_err());
    assert_eq!(
        grade_spirv_control(RADV, &proof(true), &native, &native, 2),
        Ok(0)
    );
}

#[test]
fn historical_proofs_remain_available_as_diagnostics() {
    let receipt = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/recorded_controls/vulkan-amd-shading-controls-2026-10-29/controls.json"
    ))
    .expect("the committed AMD SPIR-V control receipt");
    for (test, image) in [
        ("bug23738", "bug23738.png"),
        ("bug288_2", "bug288_2.png"),
        ("bug23795", "bug23795.png"),
    ] {
        let row = receipt
            .lines()
            .find(|l| l.contains(&format!("\"test\": \"{test}\", \"image\": \"{image}\"")));
        assert!(
            row.is_some_and(|r| r.contains("\"pixels\": [[")),
            "{test} {image}"
        );
    }
}

#[test]
fn every_historical_pixel_row_is_refused_as_nonqualifying() {
    let receipt = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/recorded_controls/vulkan-amd-shading-controls-2026-10-29/controls.json"
    ))
    .unwrap();
    for row in receipt
        .lines()
        .filter(|line| line.contains("\"pixels\": [["))
    {
        let recorded = SpirvControlProof {
            coverage_identical: row.contains("\"control_coverage_identical\": true"),
            coverage_identical_outside_text_line_point: row
                .contains("\"control_coverage_identical_outside_unreplayed\": true"),
            ..proof(false)
        };
        assert!(
            grade_spirv_control(
                RADV,
                &recorded,
                &frame(3, [173, 126, 24, 255]),
                &frame(3, LIT),
                2
            )
            .is_err()
        );
    }
}
