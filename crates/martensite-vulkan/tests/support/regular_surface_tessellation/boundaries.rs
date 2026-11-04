use super::*;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Edge, Shape},
};
use martensite_tkernel::ncollection::Array1;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::toploc::Location;
use martensite_tkmath::{
    gp::Point2,
    poly::{PolygonOnTriangulation, Triangulation},
};
type Polygon = Handle<PolygonOnTriangulation>;

pub(super) fn attach(
    face: &Face,
    mesh: &Handle<Triangulation>,
    bounds: Bounds,
    nbu: i32,
    nbv: i32,
) -> Result<(), Failure> {
    let polygons = polygons(nbu, nbv)?;
    for shape in Explorer::new(
        &Shape::from(face.clone()),
        ShapeEnum::Edge,
        ShapeEnum::Shape,
    )? {
        let edge = topods::edge(&shape)?;
        attach_edge(&edge, face, mesh, bounds, &polygons)?;
    }
    Ok(())
}

fn polygon(indices: impl Iterator<Item = i32>, count: i32) -> Result<Polygon, Failure> {
    let mut nodes = Array1::with_bounds(1, count).map_err(|cause| {
        program_error(&format!(
            "boundary count={count}; expected valid array bounds: {cause}"
        ))
    })?;
    for (offset, index) in indices.enumerate() {
        nodes.set_value(offset as i32 + 1, index).map_err(|cause| {
            program_error(&format!(
                "boundary offset={offset}, count={count}; expected an in-range node: {cause}"
            ))
        })?;
    }
    Ok(Handle::new(PolygonOnTriangulation::from_nodes(&nodes)))
}

fn polygons(nbu: i32, nbv: i32) -> Result<[Polygon; 4], Failure> {
    Ok([
        polygon((0..=nbv).map(|v| 1 + v), nbv + 1)?,
        polygon((0..=nbv).map(|v| 1 + v + nbu * (nbv + 1)), nbv + 1)?,
        polygon((0..=nbu).map(|u| 1 + u * (nbv + 1)), nbu + 1)?,
        polygon((0..=nbu).map(|u| (1 + u) * (nbv + 1)), nbu + 1)?,
    ])
}

fn ends(edge: &Edge, face: &Face) -> Result<(Point2, Point2), Failure> {
    let found = BRepTool::curve_on_face(edge, face)?;
    let curve = found.pcurve.get().ok_or_else(|| {
        program_error("tessellate pcurve=null; expected a natural-boundary pcurve")
    })?;
    Ok((curve.value(found.first)?, curve.value(found.last)?))
}

fn attach_edge(
    edge: &Edge,
    face: &Face,
    mesh: &Handle<Triangulation>,
    (umin, umax, vmin, vmax): Bounds,
    polys: &[Polygon; 4],
) -> Result<(), Failure> {
    let (pair, at_min) = boundary_pair(ends(edge, face)?, (umin, umax, vmin, vmax), polys);
    let builder = BRepBuilder::default();
    if BRepTool::is_closed_on_face(edge, face)? {
        return builder.update_edge_polygons_on_triangulation(
            edge,
            pair,
            mesh,
            &Location::default(),
        );
    }
    builder.update_edge_polygon_on_triangulation(
        edge,
        if at_min { pair.0 } else { pair.1 },
        mesh,
        &Location::default(),
    )
}

fn boundary_pair(
    (first, last): (Point2, Point2),
    (umin, umax, vmin, vmax): Bounds,
    polys: &[Polygon; 4],
) -> ((&Polygon, &Polygon), bool) {
    let u_constant = (first.x() - last.x()).abs() < 0.1 * (umax - umin);
    let pair = if u_constant {
        (&polys[0], &polys[1])
    } else {
        (&polys[2], &polys[3])
    };
    let at_min = if u_constant {
        first.x() < 0.5 * (umin + umax)
    } else {
        first.y() < 0.5 * (vmin + vmax)
    };
    (pair, at_min)
}
