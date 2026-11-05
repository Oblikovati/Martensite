use super::*;
#[test]
fn actual_case_cut_copy_scaled_nurbs_mesh_keeps_all_authored_mesh_checks() {
    let shape = geometry::actual_scaled_cut_face().unwrap();
    original_mesh(&shape).unwrap();
    let summary = crate::shape_mesh::TriangulationSummary::of(&shape).unwrap();
    assert_eq!((summary.triangles, summary.nodes), (193, 147));
    assert!((summary.max_deflection - 0.052300780129031083).abs() <= 1e-6);
    assert!(
        crate::mesh_topology_check::check_mesh_topology(&shape)
            .unwrap()
            .is_clean()
    );
}
