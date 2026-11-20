// SPDX-License-Identifier: GPL-2.0-only
//! Native vertices and shared corners equal the exact actual surface boundaries.
use super::*;
use martensite_tkbrep::topexp;
pub(super) fn verify(side: &SideGeometry) -> Result<(), Failure> {
    let rows = [ends(&side.edges[0])?, ends(&side.edges[1])?];
    for (row, vertices) in rows.iter().enumerate() {
        let last = side.rows[row].len() - 1;
        for (vertex, pole) in vertices
            .iter()
            .zip([side.rows[row][0], side.rows[row][last]])
        {
            endpoint(vertex, pole)?;
        }
    }
    ruling(&side.edges[2], [&rows[0][0], &rows[1][0]])?;
    ruling(&side.edges[3], [&rows[0][1], &rows[1][1]])
}
fn ends(shape: &Shape) -> Result<[topods::Vertex; 2], Failure> {
    let edge = topods::edge(shape)?;
    let (first, last) = topexp::vertices(&edge, false);
    if first.is_null() || last.is_null() || children(&edge, ShapeEnum::Vertex)?.len() != 2 {
        return Err(rejected(
            "actual side boundary requires exactly two native endpoint occurrences",
        ));
    }
    Ok([first, last])
}
fn ruling(shape: &Shape, expected: [&topods::Vertex; 2]) -> Result<(), Failure> {
    let actual = ends(shape)?;
    let forward = actual[0].is_same(expected[0]) && actual[1].is_same(expected[1]);
    let reverse = actual[0].is_same(expected[1]) && actual[1].is_same(expected[0]);
    if !forward && !reverse {
        return Err(rejected("ruling/row native corner identities differ"));
    }
    Ok(())
}
pub(in crate::body_check::certified_conical) fn complete(
    root: &Shape,
    sides: &[Side],
) -> Result<(), Failure> {
    let edges: Vec<_> = sides.iter().flat_map(|side| &side.edges).collect();
    for actual in children(root, ShapeEnum::Edge)? {
        if !edges.iter().any(|edge| edge.is_same(&actual)) {
            return Err(rejected("native root has an uncovered edge"));
        }
    }
    let endpoints = edges
        .iter()
        .map(|edge| ends(edge))
        .collect::<Result<Vec<_>, _>>()?;
    for actual in children(root, ShapeEnum::Vertex)? {
        if !endpoints
            .iter()
            .flatten()
            .any(|vertex| vertex.is_same(&actual))
        {
            return Err(rejected("native root has an uncovered vertex"));
        }
    }
    Ok(())
}

fn endpoint(vertex: &topods::Vertex, pole: [f64; 3]) -> Result<(), Failure> {
    let p = BRepTool::pnt(vertex)?;
    if [p.x(), p.y(), p.z()].map(f64::to_bits) != pole.map(f64::to_bits) {
        return Err(rejected(
            "actual row endpoint vertex differs from exact boundary control",
        ));
    }
    Ok(())
}
