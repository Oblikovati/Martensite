//! The planar commands of the `fillet2d` grid and the `bugs` scripts.
use crate::script::{FilletScript, command_stopped};
use crate::steps::Planar;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::{self, Explorer},
    topods::{self, Compound, Edge, Shape},
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
};
use martensite_tkfillet::{
    brep_fillet_api::BRepFilletAPIMakeFillet2d,
    chfi2d::{ChFi2dChamferAPI, ChFi2dFilletAPI, ConstructionError},
};
use martensite_tkg3d::{geom::GeomPlane, top_abs::ShapeEnum};
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIFindPlane, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};

/// Run one planar command; `run_planar(&mut script, &Planar::FilletWire("r", "w", 1.0))?`.
pub fn run_planar(script: &mut FilletScript<'_>, planar: &Planar) -> Result<(), Failure> {
    match planar {
        Planar::FilletEdges(result, e1, e2, radius) => {
            fillet_edges(script, result, e1, e2, *radius)
        }
        Planar::FilletWire(result, wire, radius) => fillet_wire(script, result, wire, *radius),
        Planar::FaceFillet(result, face, e1, e2, radius) => {
            face_fillet(script, result, face, (e1, e2), *radius)
        }
        Planar::ChamferEdges(result, e1, e2, lengths) => {
            chamfer_edges(script, result, e1, e2, *lengths)
        }
    }
}

/// `chfi2d result face e1 e2 F radius`, of one corner.
fn face_fillet(
    script: &mut FilletScript<'_>,
    result: &'static str,
    face: &str,
    (e1, e2): (&str, &str),
    radius: f64,
) -> Result<(), Failure> {
    let (e1, e2) = (script.required(e1)?, script.required(e2)?);
    let (e1, e2) = (topods::edge(&e1)?, topods::edge(&e2)?);
    let mut maker = BRepFilletAPIMakeFillet2d::with_face(&topods::face(&script.required(face)?)?)?;
    let vertex = topexp::common_vertex(&e1, &e2)
        .ok_or_else(|| command_stopped("chfi2d: the edges do not share a vertex"))?;
    maker.add_fillet(&vertex, radius)?;
    if maker.status() != ConstructionError::IsDone {
        return Err(command_stopped("chfi2d: operation failed"));
    }
    maker.build(&ProgressRange::default())?;
    script.set_shape(result, maker.shape()?.clone());
    Ok(())
}

fn plane_of(shape: &Shape) -> Result<Plane3, Failure> {
    let finder = BRepBuilderAPIFindPlane::from_shape(shape, -1.0)?;
    if !finder.found() {
        return Err(command_stopped("the edges are located not in a plane"));
    }
    let plane: Handle<GeomPlane> = finder.plane();
    let live = plane.get().ok_or_else(|| command_stopped("null plane"))?;
    Ok(live.pln())
}

fn fillet_edges(
    script: &mut FilletScript<'_>,
    result: &'static str,
    e1: &'static str,
    e2: &'static str,
    radius: f64,
) -> Result<(), Failure> {
    let (s1, s2) = (script.required(e1)?, script.required(e2)?);
    let pln = plane_of_two(&s1, &s2)?;
    let mut algo = ChFi2dFilletAPI::with_edges(&topods::edge(&s1)?, &topods::edge(&s2)?, &pln)?;
    let common = common_point_of_edges(&s1, &s2)?;
    let (fillet, m1, m2) = planar_fillet(&mut algo, radius, common)?;
    script.set_shape(result, fillet.as_ref().clone());
    script.set_shape(e1, m1.as_ref().clone());
    script.set_shape(e2, m2.as_ref().clone());
    Ok(())
}

/// The plane of two edges, as `fillet2d` finds it from a compound of them.
fn plane_of_two(s1: &Shape, s2: &Shape) -> Result<Plane3, Failure> {
    let builder = BRepBuilder::default();
    let mut both = Compound::default();
    builder.make_compound(&mut both);
    builder.add(&mut both, s1)?;
    builder.add(&mut both, s2)?;
    plane_of(&both.into_shape())
}

/// The fillet of `fillet2d` and the two edges it trims.
fn planar_fillet(
    algo: &mut ChFi2dFilletAPI,
    radius: f64,
    common: Point3,
) -> Result<(Edge, Edge, Edge), Failure> {
    if !algo.perform(radius)? {
        return Err(command_stopped("fillet2d: the algorithm failed"));
    }
    let (mut m1, mut m2) = (Edge::default(), Edge::default());
    if algo.nb_results(common)? == 0 {
        return Err(command_stopped("fillet2d: no solutions"));
    }
    let fillet = algo.result(common, &mut m1, &mut m2, -1)?;
    if fillet.is_null() {
        return Err(command_stopped("fillet2d: no result"));
    }
    Ok((fillet, m1, m2))
}

fn fillet_wire(
    script: &mut FilletScript<'_>,
    result: &'static str,
    wire: &str,
    radius: f64,
) -> Result<(), Failure> {
    let shape = script.required(wire)?;
    let mut algo = ChFi2dFilletAPI::with_wire(&topods::wire(&shape)?, &plane_of(&shape)?)?;
    let common = common_point_of_wire(&shape)?;
    let (fillet, m1, m2) = planar_fillet(&mut algo, radius, common)?;
    let maker = BRepBuilderAPIMakeWire::from_three_edges(&m1, &fillet, &m2)?;
    let built = if maker.is_done() {
        maker.wire()?.into_shape()
    } else {
        fillet.into_shape()
    };
    script.set_shape(result, built);
    Ok(())
}

fn chamfer_edges(
    script: &mut FilletScript<'_>,
    result: &'static str,
    e1: &'static str,
    e2: &'static str,
    [first, second]: [f64; 2],
) -> Result<(), Failure> {
    let (s1, s2) = (script.required(e1)?, script.required(e2)?);
    let mut algo = ChFi2dChamferAPI::with_edges(&topods::edge(&s1)?, &topods::edge(&s2)?);
    algo.perform()?;
    let (mut m1, mut m2) = (Edge::default(), Edge::default());
    let chamfer = algo.result(&mut m1, &mut m2, (first, second))?;
    if chamfer.is_null() {
        return Err(command_stopped("chamfer2d: no result"));
    }
    script.set_shape(result, chamfer.as_ref().clone());
    script.set_shape(e1, m1.as_ref().clone());
    script.set_shape(e2, m2.as_ref().clone());
    Ok(())
}

/// `findCommonPoint(E1, E2)`: the end of the first edge nearest an end of the second.
fn common_point_of_edges(e1: &Shape, e2: &Shape) -> Result<Point3, Failure> {
    let (v11, v12) = topexp::vertices(&topods::edge(e1)?, false);
    let (v21, v22) = topexp::vertices(&topods::edge(e2)?, false);
    let (p11, p12) = (BRepTool::pnt(&v11)?, BRepTool::pnt(&v12)?);
    let (p21, p22) = (BRepTool::pnt(&v21)?, BRepTool::pnt(&v22)?);
    let (a, b, c, d) = (
        p11.square_distance(&p21),
        p11.square_distance(&p22),
        p12.square_distance(&p21),
        p12.square_distance(&p22),
    );
    if (a < b && a < c && a < d) || (b < a && b < c && b < d) {
        return Ok(p11);
    }
    if (c < a && c < b && c < d) || (d < a && d < b && d < c) {
        return Ok(p12);
    }
    Ok(Point3::default())
}

/// `findCommonPoint(W)`: the first vertex met twice in the wire.
fn common_point_of_wire(wire: &Shape) -> Result<Point3, Failure> {
    let mut seen: Vec<Shape> = Vec::new();
    let mut explorer = Explorer::new(wire, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while explorer.more() {
        let current = explorer.current().clone();
        if seen.iter().any(|known| known.is_same(&current)) {
            return BRepTool::pnt(&topods::vertex(&current)?);
        }
        seen.push(current);
        explorer.next()?;
    }
    Ok(Point3::default())
}
