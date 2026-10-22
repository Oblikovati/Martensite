//! Shape-level mode/max-area effects and atomic overflow fallback stay distinct.
use super::*;
use crate::shape_upgrade::uv_setter_defined_cases::{
    AREAS, OVERFLOW, assert_overflow, defined_cases, expected_parts,
};

#[test]
fn shape_uv_setter_matches_defined_boundaries_and_all_stored_fields() {
    for (input, expected) in defined_cases() {
        let tool = ShapeUpgradeShapeDivideArea::empty();
        tool.set_splitting_by_number(input.prior_mode);
        tool.set_max_area(12.5);
        tool.set_nb_parts(input.prior_parts);
        tool.set_numbers_uv_splits(input.u, input.v).unwrap();
        let actual = *tool.settings.read();
        assert_eq!((actual.u_splits, actual.v_splits), (input.u, input.v));
        assert_eq!(actual.parts, expected_parts(&input));
        assert!(actual.by_number);
        assert_eq!(visible_shape_state(&tool), expected[2..]);
    }
}

fn visible_shape_state(tool: &ShapeUpgradeShapeDivideArea) -> [f64; 4] {
    let child = tool.new_face_tool().cast::<ShapeUpgradeFaceDivideArea>();
    [
        tool.max_area(),
        f64::from(tool.nb_parts()),
        child.max_area(),
        f64::from(child.nb_parts()),
    ]
}

#[test]
fn shape_uv_setter_overwrites_max_area_and_retains_parts_on_nonpositive_counts() {
    for area in AREAS {
        let tool = ShapeUpgradeShapeDivideArea::empty();
        tool.set_max_area(area);
        tool.set_numbers_uv_splits(1, i32::MAX).unwrap();
        tool.set_numbers_uv_splits(i32::MAX, i32::MIN).unwrap();
        assert_eq!(tool.nb_parts(), i32::MAX);
        assert_eq!(tool.max_area().to_bits(), (-1_f64).to_bits());
        assert!(tool.splitting_by_number());
    }
}

#[test]
fn shape_uv_setter_overflow_preserves_all_previous_configuration() {
    for (u, v) in OVERFLOW {
        let tool = ShapeUpgradeShapeDivideArea::empty();
        tool.set_numbers_uv_splits(2, 3).unwrap();
        tool.set_splitting_by_number(false);
        tool.set_max_area(12.5);
        let error = tool.set_numbers_uv_splits(u, v).unwrap_err();
        assert_overflow(&error, u, v);
        let actual = *tool.settings.read();
        assert_eq!((actual.u_splits, actual.v_splits, actual.parts), (2, 3, 6));
        assert_eq!(actual.max_area, 12.5);
        assert!(!actual.by_number);
    }
}
