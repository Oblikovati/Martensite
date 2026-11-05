use super::*;
use crate::shape_mesh::TriangulationSummary;

#[test]
fn source_dump_predicate_keeps_all_three_distinct_geometry_sections() {
    let empty =
        dump_counts::geometry_dump_counts("Dump of 0 Curve2ds Dump of 0 Curves Dump of 0 surfaces")
            .unwrap();
    assert!(empty.all_removed());
    for text in [
        "Dump of 1 Curve2ds Dump of 0 Curves Dump of 0 surfaces",
        "Dump of 0 Curve2ds Dump of 2 Curves Dump of 0 surfaces",
        "Dump of 0 Curve2ds Dump of 0 Curves Dump of 3 surfaces",
    ] {
        assert!(
            !dump_counts::geometry_dump_counts(text)
                .unwrap()
                .all_removed()
        );
    }
}

#[test]
fn entire_actual_fuse_mesh_cleanup_removes_geometry_but_keeps_mesh_nodes() {
    let shape = fusion::actual_prepared_multi_tool_fuse().unwrap();
    assert!(crate::shape_checks::check_shape_valid(&shape).unwrap());
    assert!(!actual_geometry_counts(&shape).unwrap().all_removed());
    let report = crate::mesh_execution::mesh_with_strategy(
        &shape,
        MeshRequest::new(0.1),
        MeshAlgoType::Default,
    )
    .unwrap();
    assert_eq!(report.status & 4, 0);
    let before = TriangulationSummary::of(&shape).unwrap();
    assert!(before.nodes > 0 && before.triangles > 0);
    remove_geometry_only(&shape).unwrap();
    assert!(actual_geometry_counts(&shape).unwrap().all_removed());
    assert_eq!(TriangulationSummary::of(&shape).unwrap(), before);
}

#[test]
fn absent_or_duplicate_geometry_sections_are_incomplete_not_zero() {
    assert!(dump_counts::geometry_dump_counts("Dump of 0 Curves").is_err());
    assert!(
        dump_counts::geometry_dump_counts(
            "Dump of 0 Curve2ds Dump of 0 Curve2ds Dump of 0 Curves Dump of 0 surfaces"
        )
        .is_err()
    );
}

#[test]
fn unperformed_null_boolean_result_cannot_be_published_as_success() {
    let mut operation = martensite_tkbo::bop_algo::BOPAlgoBOP::new();
    operation.set_operation(martensite_tkbo::bop_algo::Operation::Fuse);
    let error = fusion::completed_fuse_shape(operation)
        .unwrap_err()
        .to_string();
    assert!(error.contains("null=true"));
    assert!(error.contains("expected completed non-null"));
}
