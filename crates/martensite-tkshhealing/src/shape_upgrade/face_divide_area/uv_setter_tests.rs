//! Direct state assertions for both branches and the separate Rust overflow fallback.
use super::*;
use crate::shape_upgrade::uv_setter_defined_cases::{
    AREAS, OVERFLOW, assert_overflow, defined_cases, expected_parts,
};

#[test]
fn face_uv_setter_matches_defined_boundaries_and_all_stored_fields() {
    for (input, expected) in defined_cases() {
        let tool = ShapeUpgradeFaceDivideArea::empty();
        tool.set_splitting_by_number(input.prior_mode);
        tool.set_max_area(12.5);
        tool.set_nb_parts(input.prior_parts);
        tool.set_numbers_uv_splits(input.u, input.v).unwrap();
        let actual = *tool.settings.read();
        assert_eq!((actual.u_splits, actual.v_splits), (input.u, input.v));
        assert_eq!(actual.parts, expected_parts(&input));
        assert_eq!(actual.by_number, input.prior_mode);
        assert_eq!(
            [actual.max_area, f64::from(actual.parts)],
            [expected[0], expected[1]]
        );
    }
}

#[test]
fn face_uv_setter_preserves_selected_max_area_bits_and_repeated_parts() {
    for area in AREAS {
        let tool = ShapeUpgradeFaceDivideArea::empty();
        tool.set_max_area(area);
        tool.set_numbers_uv_splits(i32::MAX, 1).unwrap();
        tool.set_numbers_uv_splits(i32::MIN, i32::MAX).unwrap();
        assert_eq!(tool.nb_parts(), i32::MAX);
        assert_eq!(tool.max_area().to_bits(), area.to_bits());
    }
}

#[test]
fn face_uv_setter_overflow_updates_counts_but_preserves_other_configuration() {
    for (u, v) in OVERFLOW {
        let tool = ShapeUpgradeFaceDivideArea::empty();
        tool.set_max_area(12.5);
        tool.set_nb_parts(7);
        tool.set_splitting_by_number(true);
        let error = tool.set_numbers_uv_splits(u, v).unwrap_err();
        assert_overflow(&error, u, v);
        let actual = *tool.settings.read();
        assert_eq!((actual.u_splits, actual.v_splits, actual.parts), (u, v, 7));
        assert_eq!(actual.max_area, 12.5);
        assert!(actual.by_number);
        tool.set_numbers_uv_splits(2, 3).unwrap();
        assert_eq!(tool.nb_parts(), 6);
    }
}
