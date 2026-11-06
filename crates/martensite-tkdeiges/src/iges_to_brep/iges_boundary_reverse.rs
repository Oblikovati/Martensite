//! The reversal of the curves of a boundary.
use crate::iges_data::iges_live;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{Builder, Edge, Face, Wire};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs;
use martensite_tkshhealing::shape_build::ShapeBuildEdge;
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tkshhealing::shape_fix::ShapeFixWire;

const WHERE: &str = "IgesToBRepIgesBoundary";

/// Reverses a wire of edges with 3D curves: each curve is reversed in a new edge, and the wire is
/// connected again; `reverse_curves_3d(&edges)?`.
///
/// # Errors
/// A failure of a curve or of the construction of the wire.
pub fn reverse_curves_3d(edges: &Handle<ShapeExtendWireData>) -> Result<(), Failure> {
    let list = iges_live(edges, WHERE)?;
    list.reverse()?;
    let mut wire = Wire::default();
    let builder = Builder;
    builder.make_wire(&mut wire);
    for rank in 1..=list.nb_edges() {
        let old = list.edge(rank)?;
        let carried = BRepTool::curve_with_location(&old)?;
        let curve = iges_live(&carried.curve, WHERE)?;
        let (p1, p2) = (carried.first, carried.last);
        let (first, last) = if curve.is_periodic() {
            (curve.reversed_parameter(p2)?, curve.reversed_parameter(p1)?)
        } else {
            // The reversed bounds hold the range.
            (
                curve
                    .reversed_parameter(curve.last_parameter())?
                    .max(curve.reversed_parameter(p2)?),
                curve
                    .reversed_parameter(curve.first_parameter())?
                    .min(curve.reversed_parameter(p1)?),
            )
        };
        let mut made = Edge::default();
        ShapeBuildEdge.make_edge_on_curve_range(
            &mut made,
            &curve.reversed()?,
            &carried.location,
            first,
            last,
        )?;
        made.set_orientation(top_abs::reverse(old.orientation()));
        builder.add(&mut wire, &made)?;
    }
    let fix = ShapeFixWire::new();
    fix.load(&wire)?;
    fix.fix_connected_all(-1.0)?;
    list.init_wire(&fix.wire()?, true, true)?;
    Ok(())
}

/// Reverses a wire of edges with pcurves on a face; `reverse_curves_2d(&edges, &face)?`.
///
/// # Errors
/// A failure of a curve or of the construction of an edge.
pub fn reverse_curves_2d(edges: &Handle<ShapeExtendWireData>, face: &Face) -> Result<(), Failure> {
    let list = iges_live(edges, WHERE)?;
    list.reverse_on_face(face)?;
    for rank in 1..=list.nb_edges() {
        let old = list.edge(rank)?;
        let carried = BRepTool::curve_on_face(&old, face)?;
        let curve = iges_live(&carried.pcurve, WHERE)?;
        let (p1, p2) = (carried.first, carried.last);
        // A periodic curve keeps its range when reversed, as when the 2D curves are forced.
        let (first, last) = if curve.is_periodic() {
            (curve.reversed_parameter(p2)?, curve.reversed_parameter(p1)?)
        } else {
            (
                curve.first_parameter().max(curve.reversed_parameter(p2)?),
                curve.last_parameter().min(curve.reversed_parameter(p1)?),
            )
        };
        let mut made = Edge::default();
        ShapeBuildEdge.make_edge_on_face_range(&mut made, &curve.reversed()?, face, first, last)?;
        made.set_orientation(old.orientation());
        list.set(&made, rank)?;
    }
    Ok(())
}
