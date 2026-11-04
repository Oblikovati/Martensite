use super::*;
use crate::shape_mesh::{TriangulationSummary, remove_triangulation};

#[test]
fn source_references_keep_exact_negated_counts_and_zero_area() {
    assert_eq!(SHADED.triangles, exact_count(294.));
    assert_eq!(SHADED.nodes, exact_count(300.));
    assert_eq!(CLEARED.triangles, exact_count(26.));
    assert_eq!(CLEARED.nodes, exact_count(28.));
    assert_eq!(CLEARED.area, 0.);
    assert_eq!(INCREMENTAL.triangles, CountExpectation::Not(1315.));
    assert_eq!(INCREMENTAL.nodes, CountExpectation::Not(784.));
    assert_eq!(INCREMENTAL.area, 0.);
}

#[test]
fn each_actual_input_can_be_restored_and_really_cleared() {
    for case in [&SHADED, &CLEARED, &INCREMENTAL] {
        let shape = read_dataset_shape(case.input).unwrap();
        assert!(crate::shape_checks::check_shape_valid(&shape).unwrap());
        remove_triangulation(&shape).unwrap();
        let summary = TriangulationSummary::of(&shape).unwrap();
        assert!(summary.faces > 0);
        assert_eq!((summary.triangles, summary.nodes), (0, 0));
    }
}

#[test]
fn retained_drawing_shares_shape_and_runs_authored_iso_triangle_state_commands() {
    for case in [&CLEARED, &INCREMENTAL] {
        let (shape, drawing) = case.restored_with_drawing().unwrap();
        assert!(shape.is_same(&drawing.rendering().shape()));
        assert_eq!(drawing.rendering().nb_isos(), 2);
        assert!(!drawing.rendering().display_triangulation());
        drawing
            .rendering()
            .change_nb_isos(0, &martensite_raster_core::ProcessReports)
            .unwrap();
        drawing
            .rendering()
            .set_display_triangulation(!drawing.rendering().display_triangulation());
        assert_eq!(drawing.rendering().nb_isos(), 0);
        assert!(drawing.rendering().display_triangulation());
    }
}

#[path = "drawing_controls.rs"]
mod drawing_controls;
