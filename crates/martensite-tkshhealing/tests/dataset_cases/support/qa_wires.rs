//! The QABugs commands of the moddata cases (QABugs_14.cxx, QABugs_19.cxx at
//! 3d097a0328e71b826377d4814ab05ec3c3d23871): `BUC60868` (the free bounds of a shell
//! chained by BuildBoundWires), `BUC60924` (ShapeAnalysis_Curve::IsPlanar) and
//! `OCC25202` (a ShapeBuild_ReShape replacement of two faces).
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools::ReShape,
    topexp::{self, Explorer},
    topods::{self, Compound, Shape, Wire},
    toptools::{IndexedDataMapOfShapeListOfShape, MapOfShape},
};
use martensite_tkernel::{precision, standard::Handle};
use martensite_tkg3d::{geom::GeomCurve, top_abs::ShapeEnum};
use martensite_tkmath::gp::Xyz;
use martensite_tkshhealing::shape_analysis::{ShapeAnalysisCurve, ShapeAnalysisWireOrder};
use martensite_tkshhealing::shape_build::ShapeBuildReShape;
use martensite_tkshhealing::shape_extend::{ShapeExtendStatus, ShapeExtendWireData};
use martensite_tkshhealing::shape_fix::{FixRoot, ShapeFixWire};

/// `BuildBoundWires`: the non-degenerated edges of one face only, chained into wires by
/// `BuildWires` (FixReorder with the keep-loops order, then split where the chain breaks);
/// `None` when the reorder fails.
fn build_bound_wires(shell: &Shape) -> Option<Vec<Shape>> {
    let mut edge_faces = IndexedDataMapOfShapeListOfShape::default();
    topexp::map_shapes_and_ancestors(shell, ShapeEnum::Edge, ShapeEnum::Face, &mut edge_faces)
        .unwrap();
    let mut boundary = Vec::new();
    for index in 1..=edge_faces.extent() {
        if edge_faces.find_from_index(index).unwrap().extent() != 1 {
            continue;
        }
        let edge = topods::edge(edge_faces.find_key(index).unwrap()).unwrap();
        if !BRepTool::degenerated(&edge).unwrap() {
            boundary.push(edge);
        }
    }
    if boundary.is_empty() {
        return Some(Vec::new());
    }
    let wire_data = Handle::new(ShapeExtendWireData::new());
    let fixer = ShapeFixWire::new();
    fixer.set_context(&Handle::new(ShapeBuildReShape::new()));
    fixer.load_wire_data(&wire_data).unwrap();
    fixer.set_precision(precision::confusion());
    for edge in &boundary {
        wire_data.add_edge(edge, 0).unwrap();
    }
    let mut order = ShapeAnalysisWireOrder::default();
    order.set_keep_loops_mode(true);
    fixer
        .working_analyzer()
        .check_order_with(&mut order, true, true, false)
        .unwrap();
    fixer.fix_reorder_with(&order).unwrap();
    if fixer.status_reorder(ShapeExtendStatus::Fail) {
        return None;
    }
    Some(split_chains(&wire_data))
}

/// The wires of consecutive connected edges of the reordered data, each closed when its
/// ends meet.
fn split_chains(wire_data: &ShapeExtendWireData) -> Vec<Shape> {
    let builder = BRepBuilder::default();
    let mut wires = Vec::new();
    let mut current = Wire::default();
    builder.make_wire(&mut current);
    let mut last = None;
    for index in 1..=wire_data.nb_edges() {
        let edge = wire_data.edge(index).unwrap();
        let (first, end) = topexp::vertices(&edge, true);
        if index > 1
            && !last
                .as_ref()
                .is_some_and(|v: &topods::Vertex| first.is_same(v))
        {
            wires.push(close_if_looped(current));
            current = Wire::default();
            builder.make_wire(&mut current);
        }
        builder
            .add(&mut current.clone().into_shape(), &edge.into_shape())
            .unwrap();
        last = Some(end);
    }
    wires.push(close_if_looped(current));
    wires
}

fn close_if_looped(wire: Wire) -> Shape {
    let (first, last) = topexp::wire_vertices(&wire);
    let shape = wire.into_shape();
    if first.is_same(&last) {
        shape.set_closed(true).unwrap();
    }
    shape
}

/// `BUC60868 result shell`: the single bound wire, a compound of several, or nothing
/// (`no bound`).
pub fn buc60868(shell: &Shape) -> (Option<Shape>, String) {
    let wires = build_bound_wires(shell).unwrap_or_default();
    match wires.len() {
        0 => (None, "no bound\n".into()),
        1 => (Some(wires[0].clone()), String::new()),
        _ => {
            let builder = BRepBuilder::default();
            let mut compound = Compound::default();
            builder.make_compound(&mut compound);
            let mut shape = compound.into_shape();
            for wire in &wires {
                builder.add(&mut shape, wire).unwrap();
            }
            (Some(shape), String::new())
        }
    }
}

/// `BUC60924 curve x y z`: `The curve is planar !` or `Faulty : the curve is not planar!`.
pub fn buc60924(curve: &Handle<dyn GeomCurve>, normal: [f64; 3]) -> String {
    let mut normal = Xyz::new(normal[0], normal[1], normal[2]);
    if ShapeAnalysisCurve::is_planar(curve, &mut normal, 1e-7).unwrap() {
        "The curve is planar !\n".into()
    } else {
        "Faulty : the curve is not planar!\n".into()
    }
}

/// `OCC25202 res shape n1 face1 n2 face2`: the n-th distinct faces of `shape` replaced by
/// the two faces, the shape rebuilt to the wire level (`Apply(shape, TopAbs_WIRE, 2)`).
pub fn occ25202(shape: &Shape, replacements: [(i32, &Shape); 2]) -> (Shape, String) {
    let reshape = ShapeBuildReShape::new();
    let mut seen = MapOfShape::default();
    seen.add(shape.clone());
    let mut count = 0;
    let mut replaced = false;
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while faces.more() {
        let face = faces.current().clone();
        if seen.add(face.clone()) {
            count += 1;
            for (number, by) in replacements {
                if count == number {
                    reshape.replace(&face, by).unwrap();
                    replaced = true;
                }
            }
        }
        faces.next().unwrap();
    }
    let text = if replaced {
        String::new()
    } else {
        "There is not any shape for replacing.\n".into()
    };
    let result = reshape
        .apply_with_build_mode(shape, ShapeEnum::Wire, 2)
        .unwrap_or_else(|failure| panic!("OCC25202: {failure:?}"));
    (result, text)
}
