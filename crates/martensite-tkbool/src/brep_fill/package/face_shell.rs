//! `BRepFill::Face` and `BRepFill::Shell`: ruled faces between edges and wires.
use super::ruled::{RuledPatch, clear_same_flags, iso_edge, line2d, ruled_patch, section_pcurve};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Face, Shell, Wire};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Direction2;
use martensite_tktopalgo::brep_lib::same_parameter_shape;

/// The default tolerance of `BRepLib::SameParameter`.
const SAME_PARAMETER_TOLERANCE: f64 = 1.0e-5;

/// `BRepFill::Face(Edge1, Edge2)`: the ruled face between two edges (one seam edge
/// when both edges are closed), made SameParameter.
///
/// ```
/// use martensite_tkmath::gp::Point3;
/// use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeEdge;
/// use martensite_tkbrep::topexp::Explorer;
/// use martensite_tkg3d::top_abs::ShapeEnum;
/// # let edge1 = BRepBuilderAPIMakeEdge::from_points(Point3::new(0.,0.,0.),Point3::new(2.,0.,0.))?.edge()?;
/// # let edge2 = BRepBuilderAPIMakeEdge::from_points(Point3::new(0.,1.,1.),Point3::new(2.,1.,1.))?.edge()?;
/// let face = martensite_tkbool::brep_fill::ruled_face(&edge1, &edge2)?;
/// assert_eq!(Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)?.count(), 4);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub fn ruled_face(edge1: &Edge, edge2: &Edge) -> Result<Face, Failure> {
    let builder = BRepBuilder::default();
    let patch: RuledPatch = ruled_patch(edge1, edge2)?;
    let mut face = Face::default();
    builder.make_face_on_surface(&mut face, &patch.surface, precision::confusion())?;
    let (f1, l1, f2, l2) = patch.bounds;
    // Are they closed edges?
    let closed: bool = patch.v1.0.is_same(&patch.v1.1) && patch.v2.0.is_same(&patch.v2.1);
    let mut edge3: Edge = iso_edge(&patch.surface, f1, (f2, l2), &patch.v1.0, &patch.v2.0)?;
    let edge4: Edge = if closed {
        edge3.clone()
    } else {
        iso_edge(&patch.surface, l1, (f2, l2), &patch.v1.1, &patch.v2.1)?
    };
    edge3.reverse();
    let mut wire = Wire::default();
    builder.make_wire(&mut wire);
    builder.add(&mut wire, edge1)?;
    builder.add(&mut wire, &edge4)?;
    builder.add(&mut wire, &edge2.reversed())?;
    builder.add(&mut wire, &edge3)?;
    wire.set_closed(true)?;
    builder.add(&mut face, &wire)?;
    section_pcurve(edge1, &face, (f1, l1), f2)?;
    section_pcurve(edge2, &face, (f1, l1), l2)?;
    lateral_pcurves(&face, patch.bounds, closed, (&edge3, &edge4))?;
    clear_same_flags([edge1, edge2, &edge3, &edge4])?;
    same_parameter_shape(&face, SAME_PARAMETER_TOLERANCE, false)?;
    if let Some(location) = &patch.same_location {
        face.move_by(location, true)?;
    }
    Ok(face)
}

/// The p-curves of the lateral edges: both lines on the seam of a periodic face, else
/// the iso U lines at the first and last U.
fn lateral_pcurves(
    face: &Face,
    bounds: (f64, f64, f64, f64),
    periodic: bool,
    laterals: (&Edge, &Edge),
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    let t: f64 = precision::confusion();
    let (f1, l1, _, _) = bounds;
    let (edge3, edge4) = laterals;
    if periodic {
        let (c1, c2) = (line2d(l1, 0., Direction2::Y), line2d(f1, 0., Direction2::Y));
        return builder.update_edge_pcurves_on_face(edge3, (&c1, &c2), face, t);
    }
    builder.update_edge_pcurve_on_face(edge3, &line2d(f1, 0., Direction2::Y), face, t)?;
    builder.update_edge_pcurve_on_face(edge4, &line2d(l1, 0., Direction2::Y), face, t)
}

/// The walk state of `BRepFill::Shell`: the first lateral edge (the seam of a closed
/// shell) and the last lateral edge of the previous face.
struct ShellWalk {
    thefirst: bool,
    couture: Edge,
    edge4: Edge,
}

/// `BRepFill::Shell(Wire1, Wire2)`: the ruled shell between two wires of the same
/// number of edges, walked with TopExpExplorer, flagged Closed by `BRepTool::IsClosed`
/// and made SameParameter.
///
/// ```
/// use martensite_tkmath::gp::Point3;
/// use martensite_tktopalgo::brep_lib::BRepLibMakePolygon;
/// use martensite_tkbrep::topexp::Explorer;
/// use martensite_tkg3d::top_abs::ShapeEnum;
/// # let section = |z| BRepLibMakePolygon::from_three_points(Point3::new(0.,0.,z), Point3::new(1.,0.,z), Point3::new(0.,1.,z), true)?.wire();
/// # let wire1 = section(0.)?;
/// # let wire2 = section(1.)?;
/// let shell = martensite_tkbool::brep_fill::ruled_shell(&wire1, &wire2)?;
/// assert_eq!(Explorer::new(&shell, ShapeEnum::Face, ShapeEnum::Shape)?.count(), 3);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub fn ruled_shell(wire1: &Wire, wire2: &Wire) -> Result<Shell, Failure> {
    let builder = BRepBuilder::default();
    let mut shell = Shell::default();
    builder.make_shell(&mut shell);
    let closed: bool = wire1.closed()? && wire2.closed()?;
    let mut ex1 = Explorer::new(wire1, ShapeEnum::Edge, ShapeEnum::Shape)?;
    let mut ex2 = Explorer::new(wire2, ShapeEnum::Edge, ShapeEnum::Shape)?;
    let mut walk = ShellWalk {
        thefirst: true,
        couture: Edge::default(),
        edge4: Edge::default(),
    };
    while ex1.more() && ex2.more() {
        let edge1: Edge = topods::edge(ex1.current())?;
        let edge2: Edge = topods::edge(ex2.current())?;
        let periodic: bool = BRepTool::is_closed(&edge1)? && BRepTool::is_closed(&edge2)?;
        ex1.next()?;
        ex2.next()?;
        let last: bool = closed && !ex1.more() && !ex2.more();
        let face = shell_face(&mut walk, (&edge1, &edge2), periodic, closed, last)?;
        builder.add(&mut shell, &face)?;
    }
    shell.set_closed(BRepTool::is_closed(&shell)?)?;
    same_parameter_shape(&shell, SAME_PARAMETER_TOLERANCE, false)?;
    Ok(shell)
}

/// One face of `BRepFill::Shell`: its lateral edges (shared with the previous face,
/// and the seam for the last face of a closed shell), its wire and its p-curves.
fn shell_face(
    walk: &mut ShellWalk,
    edges: (&Edge, &Edge),
    periodic: bool,
    closed: bool,
    last: bool,
) -> Result<Face, Failure> {
    let builder = BRepBuilder::default();
    let (edge1, edge2) = edges;
    let patch: RuledPatch = ruled_patch(edge1, edge2)?;
    let mut face = Face::default();
    builder.make_face_on_surface(&mut face, &patch.surface, precision::confusion())?;
    let (f1, l1, f2, l2) = patch.bounds;
    let mut edge3: Edge = if walk.thefirst {
        let first = iso_edge(&patch.surface, f1, (f2, l2), &patch.v1.0, &patch.v2.0)?;
        if closed {
            walk.couture = first.clone();
        }
        walk.thefirst = false;
        first
    } else {
        walk.edge4.clone()
    };
    edge3.reverse();
    walk.edge4 = if last {
        walk.couture.clone()
    } else {
        iso_edge(&patch.surface, l1, (f2, l2), &patch.v1.1, &patch.v2.1)?
    };
    let edge4: Edge = walk.edge4.clone();
    let mut wire = Wire::default();
    builder.make_wire(&mut wire);
    builder.add(&mut wire, edge1)?;
    builder.add(&mut wire, &edge4)?;
    builder.add(&mut wire, &edge2.reversed())?;
    builder.add(&mut wire, &edge3)?;
    wire.set_closed(true)?;
    builder.add(&mut face, &wire)?;
    if let Some(location) = &patch.same_location {
        face.move_by(location, true)?;
    }
    section_pcurve(edge1, &face, (f1, l1), f2)?;
    section_pcurve(edge2, &face, (f1, l1), l2)?;
    lateral_pcurves(&face, patch.bounds, periodic, (&edge3, &edge4))?;
    clear_same_flags([edge1, edge2, &edge3, &edge4])?;
    Ok(face)
}
