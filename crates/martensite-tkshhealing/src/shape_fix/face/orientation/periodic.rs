use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Face, ShapeIterator, Wire};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Vector2;

pub(super) fn shift_wire_pcurves(face: &Face, wire: &Wire, shift: Vector2) -> Result<(), Failure> {
    let mut edges = ShapeIterator::new(&wire.clone().into_shape(), false, true);
    while edges.more() {
        translate_edge_pcurve(face, &topods::edge(edges.value()?)?, shift)?;
        edges.next();
    }
    Ok(())
}

fn translate_edge_pcurve(
    face: &Face,
    edge: &martensite_tkbrep::topods::Edge,
    shift: Vector2,
) -> Result<(), Failure> {
    let curve = BRepTool::curve_on_face(edge, face)?;
    if !curve.pcurve.is_null() {
        curve.pcurve.translate(shift)?;
    }
    Ok(())
}
