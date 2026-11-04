//! Whole wire/edge comparison uses the actual fifth exploded wire and ranked state strings.
use super::*;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{Shape, Vertex},
    toptools::MapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};

/// Complete bug25098 and its source diagnostics; `wire_edge_characteristics()?`.
pub(crate) fn wire_edge_characteristics() -> Outcome {
    banner(&[
        "============",
        "0025098: Visualization - Calculation of depth on selection of a wire is not accurate",
        "============",
        "",
    ]);
    let mut run = SessionRun::new()?;
    frustum_wire_scene(run.session())?;
    run.session().fit()?;
    run.session().move_cursor(240, 300)?;
    let bindings = state_rows::frustum_bindings(run.session(), &["b_5", "e"])?;
    let rows = state_rows::frustum_wire_rows(run.session(), &bindings)?;
    for line in frustum_wire_diagnostics(&rows) {
        println!("{line}");
    }
    run.dump("bug25098.png")?;
    run.finish()
}

/// Display then remove the box before its actual fifth wire and edge; `frustum_wire_scene(session)?`.
pub(super) fn frustum_wire_scene(session: &mut ViewerSession) -> Outcome {
    let box_shape = box_sized([10.; 3])?;
    session.display("b", &box_shape, &DisplayRequest::default())?;
    session.remove(&["b"])?;
    let wires = frustum_distinct_wires(&box_shape)?;
    let fifth = wires.get(4).ok_or_else(|| {
        program_error(&format!(
            "box wire count {}; expected at least five unique explored wires",
            wires.len(),
        ))
    })?;
    session.display("b_5", fifth, &DisplayRequest::default())?;
    let edge = frustum_separate_edge()?;
    session.display("e", &edge, &DisplayRequest::default())?;
    Ok(())
}

/// Match explode identity deduplication and exploration order; `frustum_distinct_wires(shape)?`.
pub(super) fn frustum_distinct_wires(shape: &Shape) -> Result<Vec<Shape>, Failure> {
    let mut seen = MapOfShape::default();
    seen.add(shape.clone()); // topology-command::explode seeds the root before visiting descendants.
    let mut explorer = Explorer::new(shape, ShapeEnum::Wire, ShapeEnum::Shape)?;
    let mut wires = Vec::new();
    while explorer.more() {
        if seen.add(explorer.current().clone()) {
            wires.push(explorer.current().clone());
        }
        explorer.next()?;
    }
    Ok(wires)
}

fn frustum_vertex([x, y, z]: [f64; 3]) -> Result<Vertex, Failure> {
    let maker = BRepBuilderAPIMakeVertex::new(Point3::new(x, y, z))?;
    maker.vertex()
}

/// Construct the separate source edge from its two real vertices; `frustum_separate_edge()?`.
pub(super) fn frustum_separate_edge() -> Result<Shape, Failure> {
    let first = frustum_vertex([10., 0., 0.])?;
    let last = frustum_vertex([10., 10., 0.])?;
    Ok(BRepBuilderAPIMakeEdge::from_vertices(&first, &last)?
        .edge()?
        .into_shape())
}

/// Extract exactly the source edge/depth/point/type fields; `frustum_parse_characteristics(row)`.
pub(super) fn frustum_parse_characteristics(row: &str) -> [String; 4] {
    let words: Vec<&str> = row.split_whitespace().collect();
    let mut result = [String::new(), String::new(), String::new(), String::new()];
    result[0] = if words.contains(&"e") { "1" } else { "0" }.to_owned();
    for (index, word) in words.iter().enumerate() {
        match *word {
            "Depth:" => result[1] = words.get(index + 1).unwrap_or(&"").to_string(),
            "Point:" => {
                result[2] = (1..=3)
                    .map(|offset| *words.get(index + offset).unwrap_or(&""))
                    .collect::<Vec<_>>()
                    .join(" ")
            }
            value if value.starts_with('(') => result[3] = value.to_owned(),
            _ => {}
        }
    }
    result
}

/// Preserve both possible source row layouts and ordered errors; `frustum_wire_diagnostics(rows)`.
pub(super) fn frustum_wire_diagnostics(rows: &[String]) -> Vec<String> {
    let row = |index| rows.get(index).map_or("", String::as_str);
    let first = frustum_parse_characteristics(row(1));
    let edge_first = first[0] == "1";
    let second = frustum_parse_characteristics(row(if edge_first { 3 } else { 4 }));
    let mut lines = frustum_characteristic_errors(&first, &second);
    let (edge, wire, edge_row, wire_row) = if edge_first {
        (&first, &second, 2, 4)
    } else {
        (&second, &first, 5, 2)
    };
    lines.extend(frustum_type_errors([
        edge[3].as_str(),
        row(edge_row).trim(),
        wire[3].as_str(),
        row(wire_row).trim(),
        row(wire_row + 1).trim(),
    ]));
    lines
}

fn frustum_characteristic_errors(first: &[String; 4], second: &[String; 4]) -> Vec<String> {
    let mut lines = Vec::new();
    for index in 1..3 {
        if first[index] != second[index] {
            lines.push(format!(
                "Characteristics are not equal at value nb: {}",
                index + 1
            ));
            lines.push(format!(
                "The values are: {} and {}",
                first[index], second[index]
            ));
            lines.extend(["ERROR".to_owned(), String::new()]);
        }
    }
    lines
}

const FRUSTUM_TYPE_CHECKS: [(&str, &str); 5] = [
    (
        "(Select3DSensitiveSegment)",
        "Wrong sensitive for segment! Value is: ",
    ),
    (
        "Detected Shape: BRep_TEdge",
        "Wrong type of edge! Value is: ",
    ),
    (
        "(Select3DSensitiveWire)",
        "Wrong sensitive for wire! Value is: ",
    ),
    (
        "Detected Shape: TopoDS_TWire",
        "Wrong type of wire! Value is: ",
    ),
    (
        "Detected Child: Select3DSensitiveSegment",
        "Wrong type of wire's inner sensitive! Value is: ",
    ),
];

fn frustum_type_errors(values: [&str; 5]) -> Vec<String> {
    let mut lines = Vec::new();
    for (index, (actual, (expected, message))) in
        values.into_iter().zip(FRUSTUM_TYPE_CHECKS).enumerate()
    {
        if actual != expected {
            lines.extend([
                message.to_owned(),
                actual.to_owned(),
                format!("Must be: {expected}"),
                "ERROR".to_owned(),
            ]);
            if index < 4 {
                lines.push(String::new());
            }
        }
    }
    lines
}
