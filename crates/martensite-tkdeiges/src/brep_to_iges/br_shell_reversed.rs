//! A reversed face as a forward copy on the reversed surface: IGES has no sense for a face.
use super::BRepToIgesOriginMap;
use crate::iges_data::iges_live;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Face, Shape, Wire};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::{Axis2d, Direction2, Point2, Transform2};
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPICopy;
use std::collections::HashSet;

const WHERE: &str = "BRepToIgesBRShell";

/// The surface without its trims.
fn untrimmed(surface: Handle<dyn GeomSurface>) -> Handle<dyn GeomSurface> {
    let mut current = surface;
    loop {
        let trimmed = current.cast::<GeomRectangularTrimmedSurface>();
        match trimmed.get() {
            Some(live) => current = live.basis_surface(),
            None => return current,
        }
    }
}

fn mirrored(
    curve: &Handle<dyn Geom2dCurve>,
    mirror: Transform2,
) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    Ok(iges_live(curve, WHERE)?.transformed(mirror)?.cast())
}

/// Copies the wires to the face: the outer one first, or only the others.
fn add_copied_wires(
    start: &Face,
    copy: &BRepBuilderAPICopy,
    (face, outer): (&mut Face, &Wire),
    origins: &mut BRepToIgesOriginMap,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    for take_outer in [true, false] {
        let mut wires = Explorer::new(start, ShapeEnum::Wire, ShapeEnum::Shape)?;
        while wires.more() {
            let wire = topods::wire(wires.current())?;
            wires.next()?;
            if wire.is_null() || wire.is_same(outer) != take_outer {
                continue;
            }
            let copied = copy.modified_shape(&wire)?.oriented(wire.orientation());
            builder.add(face, &copied)?;
            origins.bind(copied, wire.into_shape());
            if take_outer {
                break;
            }
        }
    }
    Ok(())
}

/// Puts the mirrored pcurve of a copied edge on the new face, with both for a seam.
fn mirror_edge_pcurve(
    edge: &Edge,
    (copied_face, face): (&Face, &Face),
    mirror: Transform2,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    let carried = BRepTool::curve_on_face(edge, copied_face)?;
    let tolerance = BRepTool::edge_tolerance(edge)?;
    if carried.pcurve.is_null() {
        return Ok(());
    }
    let curve_1 = mirrored(&carried.pcurve, mirror)?;
    let mut range = (carried.first, carried.last);
    let mut seam = None;
    if brep_tools::is_really_closed(edge, copied_face)? {
        let reversed = topods::edge(&edge.reversed())?;
        let other = BRepTool::curve_on_face(&reversed, copied_face)?;
        range = (other.first, other.last);
        if other.pcurve.is_some() {
            seam = Some(mirrored(&other.pcurve, mirror)?);
        }
    }
    match seam {
        Some(curve_2) if edge.orientation() == Orientation::Forward => {
            builder.update_edge_pcurves_on_face(edge, (&curve_1, &curve_2), face, tolerance)?;
        }
        Some(curve_2) => {
            builder.update_edge_pcurves_on_face(edge, (&curve_2, &curve_1), face, tolerance)?;
        }
        None => builder.update_edge_pcurve_on_face(edge, &curve_1, face, tolerance)?,
    }
    if BRepTool::degenerated(edge)? {
        builder.range_on_face(edge, face, range.0, range.1)?;
    }
    Ok(())
}

/// A forward face on the U-reversed surface, with mirrored pcurves; `origins` gives what its
/// shapes were in the reversed face.
pub(super) fn forward_copy(
    start: &Face,
    origins: &mut BRepToIgesOriginMap,
) -> Result<Face, Failure> {
    let mut copy = BRepBuilderAPICopy::new();
    copy.perform(start, true, false)?;
    let builder = BRepBuilder::default();
    let (located, location) = BRepTool::surface_with_location(start)?;
    let surface = iges_live(&untrimmed(located), WHERE)?.u_reversed()?;
    let mut face = Face::default();
    builder.make_face_on_located_surface(
        &mut face,
        &surface,
        &location,
        BRepTool::face_tolerance(start)?,
    )?;
    builder.natural_restriction(&face, BRepTool::natural_restriction(start)?)?;
    let container = ShapeAlgo::algo_container();
    let outer = iges_live(&container, WHERE)?.outer_wire(start)?;
    add_copied_wires(start, &copy, (&mut face, &outer), origins)?;
    let bounds = iges_live(&surface, WHERE)?.bounds();
    let center = 0.5 * (bounds.0 + bounds.1);
    let mut mirror = Transform2::default();
    mirror.set_mirror_axis(Axis2d::new(
        Point2::new(center, bounds.2),
        Direction2::new(0.0, 1.0)?,
    ));
    let copied_face = topods::face(&copy.modified_shape(start)?)?;
    let mut seen: HashSet<Shape> = HashSet::new();
    let mut edges = Explorer::new(start, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while edges.more() {
        let original = topods::edge(edges.current())?;
        edges.next()?;
        let copied = copy
            .modified_shape(&original)?
            .oriented(original.orientation());
        if seen.insert(copied.clone()) {
            mirror_edge_pcurve(&topods::edge(&copied)?, (&copied_face, &face), mirror)?;
            origins.bind(copied, original.into_shape());
        }
    }
    origins.bind(start.clone().into_shape(), face.clone().into_shape());
    Ok(face)
}
