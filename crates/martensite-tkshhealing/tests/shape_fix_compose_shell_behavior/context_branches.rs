//! Nonempty reshape replacements preserve geometry while respecting source ordering.
use super::*;
use martensite_tkbrep::topods::{Compound, Edge, Face, Shape};
use martensite_tkshhealing::shape_fix::WirePatchIndex;
const PATCH: WirePatchIndex = WirePatchIndex {
    u_min: 1,
    u_max: 2,
    v_min: 1,
    v_max: 2,
};
const EXPECTED: &str = include_str!("../fixtures/behavior/compose_shell_context_branches.expected");

#[test]
fn unchanged_context_keeps_both_source_orientations() {
    check_mode(0);
}

#[test]
fn single_edge_replacement_matches_context_orientation() {
    check_mode(1);
}

#[test]
fn split_container_replacement_retains_order_and_patches() {
    check_mode(2);
}

fn check_mode(mode: usize) {
    assert_eq!(EXPECTED.lines().count(), 6);
    for reverse in [false, true] {
        let expected = EXPECTED
            .lines()
            .nth(mode * 2 + usize::from(reverse))
            .unwrap();
        assert_eq!(inspect_replacement(mode, reverse), expected);
    }
}

fn inspect_replacement(mode: usize, reverse: bool) -> String {
    let face = face_fixture();
    let context = Handle::new(ShapeBuildReShape::new());
    let composer = initialized_composer(&face, &context);
    let mut wires = composer.load_wires().unwrap();
    let source = orient_source(wires.value(1).unwrap(), reverse);
    let replacements = install_replacement(&context, &source, mode);
    let line = Line2::new(Point2::new(3., 0.), Direction2::new(0., 1.).unwrap());
    composer
        .split_segments_by_line(&mut wires, line, true, 1)
        .unwrap();
    let segment = wires.value(1).unwrap();
    assert_replacement_geometry(segment, &face, mode, reverse, &replacements);
    assert_source_unchanged(&source, &face);
    snapshot(&composer, &wires, &face, mode, reverse)
}

fn assert_source_unchanged(source: &Edge, face: &Face) {
    assert_eq!(BRepTool::range(source).unwrap(), (0., 2.));
    let original = initialized_composer(face, &Handle::new(ShapeBuildReShape::new()));
    let wires = original.load_wires().unwrap();
    let wire = wires.value(1).unwrap();
    assert_eq!(wire.nb_edges(), 4);
    assert_eq!(wire.edge(1).unwrap().orientation(), Orientation::Forward);
    assert_planar_replacement(&wire.edge(1).unwrap(), face);
}

fn orient_source(segment: &ShapeFixWireSegment, reverse: bool) -> Edge {
    let mut source = segment.edge(1).unwrap();
    if reverse {
        source.set_orientation(Orientation::Reversed);
    }
    segment.set_edge(1, &source).unwrap();
    segment.set_patch_index(1, PATCH).unwrap();
    source
}

fn install_replacement(
    context: &Handle<ShapeBuildReShape>,
    source: &Edge,
    mode: usize,
) -> Vec<Edge> {
    if mode == 0 {
        return vec![source.clone()];
    }
    let edges = replacement_edges(source, mode);
    let replacement = if mode == 1 {
        edges[0].clone().into_shape()
    } else {
        replacement_container(&edges)
    };
    context
        .replace(&source.clone().into_shape(), &replacement)
        .unwrap();
    edges
}

fn replacement_edges(source: &Edge, mode: usize) -> Vec<Edge> {
    let (curve, first, last) = BRepTool::curve(source).unwrap();
    let ranges = if mode == 2 {
        vec![(first, 1.), (1., last)]
    } else {
        vec![(first, last)]
    };
    ranges
        .into_iter()
        .map(|(a, b)| {
            BRepBuilderAPIMakeEdge::from_curve_params(&curve, a, b)
                .unwrap()
                .edge()
                .unwrap()
        })
        .collect()
}

fn replacement_container(edges: &[Edge]) -> Shape {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    for edge in edges {
        Builder
            .add(&mut compound, &edge.clone().into_shape())
            .unwrap();
    }
    compound.into_shape()
}

fn assert_replacement_geometry(
    segment: &ShapeFixWireSegment,
    face: &Face,
    mode: usize,
    reverse: bool,
    replacements: &[Edge],
) {
    assert_eq!(segment.nb_edges(), if mode == 2 { 5 } else { 4 });
    for offset in 0..replacements.len() {
        let expected = if mode == 2 && reverse {
            1 - offset
        } else {
            offset
        };
        let edge = segment.edge(offset as i32 + 1).unwrap();
        assert!(
            edge.clone()
                .into_shape()
                .is_same(&replacements[expected].clone().into_shape())
        );
        assert_planar_replacement(&edge, face);
        assert_patch_retained(segment, offset as i32 + 1);
    }
}

fn assert_planar_replacement(edge: &Edge, face: &Face) {
    let pcurve = BRepTool::curve_on_face(edge, face).unwrap();
    for parameter in [pcurve.first, (pcurve.first + pcurve.last) / 2., pcurve.last] {
        assert_eq!(
            pcurve.pcurve.value(parameter).unwrap(),
            Point2::new(parameter, 0.)
        );
    }
}

fn assert_patch_retained(segment: &ShapeFixWireSegment, index: i32) {
    let patch = segment.patch_index(index).unwrap();
    assert_eq!(
        (patch.u_min, patch.u_max, patch.v_min, patch.v_max),
        (1, 2, 1, 2)
    );
}

fn snapshot(
    composer: &ShapeFixComposeShell,
    wires: &Sequence<ShapeFixWireSegment>,
    face: &Face,
    mode: usize,
    reverse: bool,
) -> String {
    let segment = wires.value(1).unwrap();
    let mut row = format!(
        "{mode}_{} {} {} {}",
        u8::from(reverse),
        wires.length(),
        segment.nb_edges(),
        u8::from(composer.status(ShapeExtendStatus::Fail4))
    );
    for index in 1..=segment.nb_edges() {
        append_snapshot(&mut row, segment, index, face);
    }
    row
}

fn append_snapshot(row: &mut String, segment: &ShapeFixWireSegment, index: i32, face: &Face) {
    let patch = segment.patch_index(index).unwrap();
    let edge = segment.edge(index).unwrap();
    let curve = BRepTool::curve_on_face(&edge, face).unwrap();
    let start = curve.pcurve.value(curve.first).unwrap();
    let end = curve.pcurve.value(curve.last).unwrap();
    let values = [
        edge.orientation() as u8 as f64,
        f64::from(patch.u_min),
        f64::from(patch.u_max),
        f64::from(patch.v_min),
        f64::from(patch.v_max),
        curve.first,
        curve.last,
        start.x(),
        start.y(),
        end.x(),
        end.y(),
    ];
    for value in values {
        row.push_str(&format!(" {value}"));
    }
}
