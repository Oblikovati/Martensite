use super::dump_nodes::DumpNode;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum NodeComparison {
    DifferentCount,
    DifferentCoordinates(usize),
    Equal,
}

pub(super) fn compare_dump_polygon_nodes(
    first: &[DumpNode],
    second: &[DumpNode],
) -> NodeComparison {
    if first.len() != second.len() {
        return NodeComparison::DifferentCount;
    }
    let bad = first.iter().zip(second).position(|(a, b)| {
        a.coordinates
            .iter()
            .zip(b.coordinates)
            .all(|(x, y)| (x - y).abs() > 1e-7)
    });
    bad.map(NodeComparison::DifferentCoordinates)
        .unwrap_or(NodeComparison::Equal)
}

pub(super) fn report_dump_polygon_nodes(first: &[DumpNode], second: &[DumpNode]) {
    let verdict = compare_dump_polygon_nodes(first, second);
    if verdict == NodeComparison::DifferentCount {
        println!("Error: Numbers of nodes are not equal");
        return;
    }
    println!("OK: Numbers of nodes are equal");
    if let NodeComparison::DifferentCoordinates(index) = verdict {
        println!("Following coordinates are not equal:");
        report_dump_coordinates(index + 1, 1, &first[index]);
        report_dump_coordinates(index + 1, 2, &second[index]);
        println!("Error: Coordinates of Polygon3Dare are not equal");
        return;
    }
    println!("OK: Coordinates of Polygon3Dare are equal");
}

fn report_dump_coordinates(index: usize, branch: usize, node: &DumpNode) {
    let [x, y, z] = &node.raw;
    println!("i={index} x{branch}={x} y{branch}={y} z{branch}={z}");
}
