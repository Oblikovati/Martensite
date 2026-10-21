//! Raw oriented results remain allocation-sensitive; these checks do not claim determinism.
use super::*;
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{Edge, Vertex},
};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeVertex,
        BRepBuilderAPIMakeWire,
    },
    brep_check::BRepCheckAnalyzer,
};

fn vertex(point: Point3) -> Vertex {
    BRepBuilderAPIMakeVertex::new(point)
        .unwrap()
        .vertex()
        .unwrap()
}
fn edge(first: &Vertex, last: &Vertex) -> Edge {
    BRepBuilderAPIMakeEdge::from_vertices(first, last)
        .unwrap()
        .edge()
        .unwrap()
}

fn cap_faces(noise: usize) -> Vec<Face> {
    let base: Vec<Vertex> = [(0., 0.), (2., 0.), (2., 2.), (0., 2.)]
        .into_iter()
        .map(|(x, y)| vertex(Point3::new(x, y, 0.)))
        .collect();
    let boundary: Vec<Edge> = (0..4).map(|i| edge(&base[i], &base[(i + 1) % 4])).collect();
    let mut junk = Vec::new();
    let mut result = Vec::new();
    for index in 1..=3 {
        for count in 0..noise {
            junk.push(vertex(Point3::new(count as f64, index as f64, 19.)));
        }
        result.extend(one_cap(&base, &boundary, index));
    }
    std::hint::black_box(junk);
    result
}

fn one_cap(base: &[Vertex], boundary: &[Edge], index: usize) -> Vec<Face> {
    let apex = vertex(Point3::new(0.5 * index as f64, 1., 1.));
    let spokes: Vec<Edge> = base.iter().map(|p| edge(p, &apex)).collect();
    (0..4)
        .map(|side| triangle(&boundary[side], &spokes[(side + 1) % 4], &spokes[side]))
        .collect()
}

fn triangle(boundary: &Edge, next: &Edge, previous: &Edge) -> Face {
    let mut wire = BRepBuilderAPIMakeWire::new();
    for edge in [
        boundary.clone(),
        next.clone(),
        topods::edge(&previous.clone().into_shape().reversed()).unwrap(),
    ] {
        wire.add_edge(&edge).unwrap();
    }
    BRepBuilderAPIMakeFace::from_wire(&wire.wire().unwrap(), true)
        .unwrap()
        .face()
        .unwrap()
}

#[test]
fn shared_caps_preserve_source_identity_and_opposed_pairs() {
    for nonmanifold in [false, true] {
        for noise in [0, 1, 2, 5, 11, 23] {
            let snapshot = inspect_caps(noise, nonmanifold);
            println!("CAP_SNAPSHOT {noise}_{} {snapshot}", u8::from(nonmanifold));
        }
    }
}

fn inspect_caps(noise: usize, nonmanifold: bool) -> String {
    let source = cap_faces(noise);
    assert_valid_source_faces(&source);
    let tool = ShapeFixShell::new();
    let changed = tool
        .fix_face_orientation(&shell(&source), true, nonmanifold)
        .unwrap();
    let mut row = cap_status(&tool, changed);
    assert_eq!(tool.nb_shells(), if nonmanifold { 1 } else { 3 });
    let mut labels = Vec::new();
    for output in Explorer::new(&tool.shape(), ShapeEnum::Shell, ShapeEnum::Shape).unwrap() {
        assert!(BRepTool::is_closed(&output).unwrap());
        row.push_str(" shell:1");
        append_labels(&mut row, &mut labels, &output, &source);
    }
    assert_cap_pairs(&labels);
    row
}

fn assert_valid_source_faces(source: &[Face]) {
    assert_eq!(source.len(), 12);
    for face in source {
        assert!(
            BRepCheckAnalyzer::new(&face.clone().into_shape(), true, false, false)
                .unwrap()
                .is_valid()
                .unwrap()
        );
    }
}

fn cap_status(tool: &ShapeFixShell, changed: bool) -> String {
    format!(
        "valid=12 {} {} {} {}",
        u8::from(changed),
        tool.nb_shells(),
        u8::from(tool.status_flag(ShapeExtendStatus::Done2)),
        u8::from(tool.status_flag(ShapeExtendStatus::Fail))
    )
}

fn append_labels(row: &mut String, labels: &mut Vec<(usize, u8)>, output: &Shape, source: &[Face]) {
    for face in faces(output) {
        let index = source
            .iter()
            .position(|candidate| candidate.is_same(&face))
            .unwrap();
        let orientation = face.orientation() as u8;
        labels.push((index, orientation));
        row.push_str(&format!(",{index}/{orientation}"));
    }
}

fn assert_cap_pairs(labels: &[(usize, u8)]) {
    assert_eq!(labels.len(), 24);
    for index in 0..12 {
        assert_eq!(labels.iter().filter(|(id, _)| *id == index).count(), 2);
    }
    for pair in labels.as_chunks::<8>().0 {
        assert_ne!(pair[0].0 / 4, pair[4].0 / 4);
        assert_ne!(pair[0].1, pair[4].1);
        for cap in pair.as_chunks::<4>().0 {
            assert!(
                cap.iter()
                    .all(|(id, orientation)| id / 4 == cap[0].0 / 4 && *orientation == cap[0].1)
            );
        }
    }
}
