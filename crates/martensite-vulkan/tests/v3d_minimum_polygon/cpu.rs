use super::*;
use comparison::NodeComparison;

fn fixture_dump_polygon_nodes(first: &str, second: &str) -> Vec<dump_nodes::DumpNode> {
    dump_nodes::parse_polygon_dump_nodes(&format!("prefix 1 : Polygon3D with 2 Nodes with parameters Deflection : 0 Nodes : 1 : {first} 2 : {second}")).unwrap()
}

#[test]
fn actual_dump_word_offsets_keep_every_coordinate_and_refuse_truncation_overflow() {
    let nodes = fixture_dump_polygon_nodes("1 2 3", "4 5 6");
    assert_eq!(nodes[0].coordinates, [1., 2., 3.]);
    assert_eq!(nodes[1].coordinates, [4., 5., 6.]);
    assert_eq!(dump_nodes::case_xyz_list_offset(3, 1).unwrap(), 16);
    assert!(dump_nodes::case_xyz_list_offset(usize::MAX, 1).is_err());
    assert!(dump_nodes::parse_polygon_dump_nodes("no polygon").is_err());
    assert!(dump_nodes::parse_polygon_dump_nodes("Polygon3D with 1 Nodes").is_err());
}

#[test]
fn authored_and_condition_does_not_become_any_coordinate_failure() {
    let zero = fixture_dump_polygon_nodes("0 0 0", "0 0 0");
    assert_eq!(
        comparison::compare_dump_polygon_nodes(
            &zero,
            &fixture_dump_polygon_nodes("0 5 7", "0 0 0")
        ),
        NodeComparison::Equal
    );
    assert_eq!(
        comparison::compare_dump_polygon_nodes(
            &zero,
            &fixture_dump_polygon_nodes("1 2 3", "0 0 0")
        ),
        NodeComparison::DifferentCoordinates(0)
    );
    assert_eq!(
        comparison::compare_dump_polygon_nodes(&zero, &zero[..1]),
        NodeComparison::DifferentCount
    );
}

#[test]
fn both_actual_source_meshes_have_matching_dumped_node_lists() {
    let mut lists = Vec::new();
    for minimum in [Some(0.06), None] {
        let shape = read_dataset_shape("bug26533_aal2.brep").unwrap();
        mesh_polygon_source_edge(&shape, minimum).unwrap();
        let nodes = read_edge_polygon_dump_nodes(&shape).unwrap();
        assert!(!nodes.is_empty());
        lists.push(nodes);
    }
    assert_eq!(
        comparison::compare_dump_polygon_nodes(&lists[0], &lists[1]),
        NodeComparison::Equal
    );
}

#[test]
fn authored_strict_greater_than_keeps_exact_threshold_equal() {
    let zero = fixture_dump_polygon_nodes("0 0 0", "0 0 0");
    let threshold = fixture_dump_polygon_nodes("0.0000001 0.0000001 0.0000001", "0 0 0");
    assert_eq!(
        comparison::compare_dump_polygon_nodes(&zero, &threshold),
        NodeComparison::Equal
    );
}
