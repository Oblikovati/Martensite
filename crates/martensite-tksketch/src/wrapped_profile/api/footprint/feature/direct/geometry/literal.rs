//! Exact local geometry admission for resolved, freshly decoded direct-attachment faces.
use martensite_tkbrep::brep::{
    BRepGCurve, BRepTool,
    access::{edge_tshape, face_tshape, vertex_tshape},
};
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Edge, Face, Shape, Vertex};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dBezierCurve;
use martensite_tkg3d::geom::{GeomBezierCurve, GeomBezierSurface, GeomSurface};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tkmath::toploc::Location;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GeometryRole {
    FirstRow,
    LastRow,
    StartRuling,
    EndRuling,
    StockRim,
    StockGenerator,
}
use GeometryRole::{EndRuling, FirstRow, LastRow, StartRuling, StockGenerator, StockRim};

#[derive(Debug)]
pub(crate) enum GeometryRefusal {
    Native(Failure),
    Face,
    BoundaryInventory,
    WitnessFrame,
    Flags(GeometryRole),
    Representation(GeometryRole),
    ParametricCurve(GeometryRole),
    Endpoint(GeometryRole),
    PointRepresentations(GeometryRole),
    Triangulations,
}
impl std::fmt::Display for GeometryRefusal {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(cause) => write!(out, "{cause}"),
            Self::Flags(role) => write!(out, "{role:?} storage flags"),
            Self::Representation(role) => write!(out, "{role:?} carrier representations"),
            Self::ParametricCurve(role) => write!(out, "{role:?} actual parameter trace"),
            Self::Endpoint(role) => write!(out, "{role:?} curve endpoint"),
            Self::PointRepresentations(role) => write!(out, "{role:?} vertex representations"),
            Self::Face | Self::BoundaryInventory | Self::WitnessFrame | Self::Triangulations => {
                write!(out, "{self:?}")
            }
        }
    }
}

impl From<Failure> for GeometryRefusal {
    fn from(error: Failure) -> Self {
        Self::Native(error)
    }
}
type Checked<T> = Result<T, GeometryRefusal>;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SideGeometryCensus {
    pub(crate) row_uses: usize,
    pub(crate) ruling_uses: usize,
    pub(crate) endpoint_uses: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct StockGeometryCensus {
    pub(crate) rim_edges: usize,
    pub(crate) generator_edges: usize,
    pub(crate) surface_uses: usize,
    pub(crate) vertex_endpoint_uses: usize,
}

fn occurrences(owner: &Shape, kind: ShapeEnum) -> Checked<Vec<Shape>> {
    let mut walk = Explorer::new(owner, kind, ShapeEnum::Shape)?;
    let mut result = Vec::new();
    while walk.more() {
        result.push(walk.current().clone());
        walk.next()?;
    }
    Ok(result)
}

/// Check all four same-parameter boundaries of one resolved actual ruled face.
pub(crate) fn qualify_side_geometry(face: &Face) -> Checked<SideGeometryCensus> {
    let face = topods::face(&face.oriented(Orientation::Forward))?;
    unmeshed(&face)?;
    let (support, face_place) = BRepTool::surface_with_location(&face)?;
    let side = support.downcast::<GeomBezierSurface>();
    if side.is_null() || side.has_eval_representation() || side.nb_v_poles() != 2 {
        return Err(GeometryRefusal::Face);
    }
    let edges = occurrences(&face, ShapeEnum::Edge)?;
    if edges.len() != 4 || occurrences(&face, ShapeEnum::Wire)?.len() != 1 {
        return Err(GeometryRefusal::BoundaryInventory);
    }
    let mut roles = [false; 4];
    for edge in edges {
        let edge = topods::edge(&edge)?;
        let role = qualify_side_edge(&face, &edge, &support, &side, &face_place)?;
        if std::mem::replace(&mut roles[side_slot(role)], true) {
            return Err(GeometryRefusal::BoundaryInventory);
        }
    }
    Ok(SideGeometryCensus {
        row_uses: 2,
        ruling_uses: 2,
        endpoint_uses: 8,
    })
}

fn qualify_side_edge(
    face: &Face,
    edge: &Edge,
    support: &Handle<dyn GeomSurface>,
    side: &GeomBezierSurface,
    face_place: &Location,
) -> Checked<GeometryRole> {
    let trace = BRepTool::curve_on_face(edge, face)?;
    let curve = trace.pcurve.downcast::<Geom2dBezierCurve>();
    let (role, reverse) = direction(&curve, trace.is_stored, [trace.first, trace.last])?;
    check_flags(edge, role)?;
    let relative = face_place.predivided(edge.location())?;
    let shape = SurfaceUse {
        support,
        relative: &relative,
        closed: false,
    };
    check_representations(edge, role, &[shape], false)?;
    let native = BRepTool::curve_with_location(edge)?;
    common_transform(&native.location, face_place)?;
    let space = native.curve.downcast::<GeomBezierCurve>();
    if [native.first, native.last] != [0., 1.] || space.is_null() {
        return Err(GeometryRefusal::ParametricCurve(role));
    }
    exact_boundary(side, &space, role, reverse)?;
    exact_endpoints(edge, &space, &native.location, role)?;
    Ok(role)
}

fn side_slot(role: GeometryRole) -> usize {
    match role {
        FirstRow => 0,
        LastRow => 1,
        StartRuling => 2,
        EndRuling => 3,
        _ => usize::MAX,
    }
}

fn direction(
    curve: &Handle<Geom2dBezierCurve>,
    stored: bool,
    domain: [f64; 2],
) -> Checked<(GeometryRole, bool)> {
    if !stored
        || domain != [0., 1.]
        || curve.is_null()
        || curve.has_eval_representation()
        || curve.degree() != 1
        || curve.weight(1)? != 1.
        || curve.weight(2)? != 1.
    {
        return Err(GeometryRefusal::BoundaryInventory);
    }
    let a = curve.pole(1)?;
    let b = curve.pole(2)?;
    let endpoints = [[a.x(), a.y()], [b.x(), b.y()]];
    for (role, points) in [
        (FirstRow, [[0., 0.], [1., 0.]]),
        (LastRow, [[0., 1.], [1., 1.]]),
        (StartRuling, [[0., 0.], [0., 1.]]),
        (EndRuling, [[1., 0.], [1., 1.]]),
    ] {
        if points == endpoints {
            return Ok((role, false));
        }
        if points == [endpoints[1], endpoints[0]] {
            return Ok((role, true));
        }
    }
    Err(GeometryRefusal::BoundaryInventory)
}

fn exact_boundary(
    side: &GeomBezierSurface,
    curve: &GeomBezierCurve,
    role: GeometryRole,
    reverse: bool,
) -> Checked<()> {
    let row = matches!(role, FirstRow | LastRow);
    let count = if row { side.nb_u_poles() } else { 2 };
    if curve.has_eval_representation() || curve.nb_poles() != count {
        return Err(GeometryRefusal::ParametricCurve(role));
    }
    for i in 1..=count {
        let j = if reverse { count + 1 - i } else { i };
        let (u, v) = match role {
            FirstRow => (j, 1),
            LastRow => (j, 2),
            StartRuling => (1, j),
            EndRuling => (side.nb_u_poles(), j),
            _ => return Err(GeometryRefusal::BoundaryInventory),
        };
        exact_pole(side, curve, (u, v, i), row, role)?;
    }
    Ok(())
}

fn exact_pole(
    side: &GeomBezierSurface,
    curve: &GeomBezierCurve,
    (u, v, i): (i32, i32, i32),
    row: bool,
    role: GeometryRole,
) -> Checked<()> {
    let first_weight = side.weight(u, 1)?;
    let last_weight = side.weight(u, 2)?;
    let spatial_weight = curve.weight(i)?;
    let required_weight = if row { first_weight } else { 1. };
    let point = curve.pole(i)?;
    if !first_weight.is_finite()
        || first_weight <= 0.
        || first_weight.to_bits() != last_weight.to_bits()
        || spatial_weight.to_bits() != required_weight.to_bits()
        || !finite_point(point)
        || point_bits(point) != point_bits(side.pole(u, v)?)
    {
        return Err(GeometryRefusal::ParametricCurve(role));
    }
    Ok(())
}

fn exact_endpoints(
    edge: &Edge,
    curve: &GeomBezierCurve,
    placement: &Location,
    role: GeometryRole,
) -> Checked<()> {
    let (first, last) = topexp::vertices(edge, false);
    if first.is_null() || last.is_null() || occurrences(edge, ShapeEnum::Vertex)?.len() != 2 {
        return Err(GeometryRefusal::Endpoint(role));
    }
    for (vertex, i) in [(&first, 1), (&last, curve.nb_poles())] {
        common_transform(vertex.location(), placement)?;
        let point = check_vertex(vertex, role)?.pnt();
        let bound = curve.pole(i)?;
        if !finite_point(point) || point_bits(point) != point_bits(bound) {
            return Err(GeometryRefusal::Endpoint(role));
        }
    }
    Ok(())
}

fn finite_point(point: Point3) -> bool {
    point.coordinates().iter().all(|x| x.is_finite())
}

fn point_bits(point: Point3) -> [u64; 3] {
    point.coordinates().map(f64::to_bits)
}

fn check_vertex(
    vertex: &Vertex,
    role: GeometryRole,
) -> Checked<&martensite_tkbrep::brep::BRepTVertex> {
    let vertex = vertex_tshape(vertex, "direct attachment vertex")?;
    if !vertex.points().is_empty() {
        return Err(GeometryRefusal::PointRepresentations(role));
    }
    Ok(vertex)
}

pub(crate) fn check_flags(edge: &Edge, role: GeometryRole) -> Checked<()> {
    let raw = edge_tshape(edge, "direct attachment edge")?;
    let tolerance = raw.tolerance();
    if !raw.same_parameter()
        || !raw.same_range()
        || raw.degenerated()
        || !tolerance.is_finite()
        || tolerance < 0.
    {
        return Err(GeometryRefusal::Flags(role));
    }
    Ok(())
}

fn unmeshed(face: &Face) -> Checked<()> {
    let raw = face_tshape(face, "direct attachment face")?;
    if !raw.triangulations().is_empty() {
        return Err(GeometryRefusal::Triangulations);
    }
    Ok(())
}

struct SurfaceUse<'a> {
    support: &'a Handle<dyn GeomSurface>,
    relative: &'a Location,
    closed: bool,
}

fn check_representations(
    edge: &Edge,
    role: GeometryRole,
    wanted: &[SurfaceUse<'_>],
    exhaustive: bool,
) -> Checked<()> {
    let [range_start, range_end] = {
        let space = BRepTool::curve_with_location(edge)?;
        [space.first, space.last]
    };
    if wanted.is_empty()
        || wanted.len() > 2
        || !range_start.is_finite()
        || !range_end.is_finite()
        || range_start >= range_end
    {
        return Err(GeometryRefusal::Representation(role));
    }
    let raw = edge_tshape(edge, "direct attachment edge representations")?;
    let mut space = 0;
    let mut counts = [0; 2];
    for candidate in raw.curves() {
        let rep = candidate
            .get()
            .ok_or(GeometryRefusal::Representation(role))?;
        if rep.is_curve3d() {
            space += 1;
        } else if let Some(i) = wanted
            .iter()
            .position(|owner| rep.is_curve_on_surface_at(owner.support, owner.relative))
        {
            if rep.is_curve_on_closed_surface() != wanted[i].closed
                || candidate
                    .cast::<dyn BRepGCurve>()
                    .get()
                    .map(BRepGCurve::range)
                    != Some((range_start, range_end))
            {
                return Err(GeometryRefusal::Representation(role));
            }
            counts[i] += 1;
        } else if exhaustive || (!rep.is_curve_on_surface() && !rep.is_regularity()) {
            return Err(GeometryRefusal::Representation(role));
        }
    }
    if space != 1 || counts[..wanted.len()].iter().any(|&n| n != 1) {
        return Err(GeometryRefusal::Representation(role));
    }
    Ok(())
}

fn common_transform(space: &Location, surface: &Location) -> Checked<Transform3> {
    let space = space.transformation();
    let surface = surface.transformation();
    if space != surface
        || !space.scale_factor().is_finite()
        || !space
            .matrix()
            .coefficients()
            .iter()
            .flatten()
            .all(|x| x.is_finite())
        || !space
            .translation_part()
            .coordinates()
            .iter()
            .all(|x| x.is_finite())
    {
        return Err(GeometryRefusal::WitnessFrame);
    }
    Ok(space)
}

/// An outward Euclidean norm bound for the exact common conformal XY placement.
/// The paired operands must have the same whole affine map, including translation.
pub(crate) fn retained_cap_affine_norm(curve: &Location, face: &Location) -> Checked<f64> {
    let transform = common_transform(curve, face)?;
    let [[a, b, x], [c, d, y], [z, t, e]] = *transform.matrix().coefficients();
    if transform.scale_factor() != 1.
        || b != -c
        || a != d
        || [x, y, z, t, e] != [0., 0., 0., 0., 1.]
        || (a == 0. && b == 0.)
    {
        return Err(GeometryRefusal::WitnessFrame);
    }
    let square = ((a * a).next_up() + (b * b).next_up()).next_up();
    let norm = square.sqrt().next_up().max(1.);
    if !norm.is_finite() {
        return Err(GeometryRefusal::WitnessFrame);
    }
    Ok(norm)
}

/// Check raw representation closure of the three original stock edges after receipt rebind.
/// The separately issued StockFrustumReceipt proves their metric correspondence.
pub(crate) fn qualify_stock_storage(
    band: &Face,
    base: &Face,
    lid: &Face,
) -> Checked<StockGeometryCensus> {
    let (band_support, band_place) = BRepTool::surface_with_location(band)?;
    let base_rim = qualify_rim(base, &band_support, &band_place)?;
    let lid_rim = qualify_rim(lid, &band_support, &band_place)?;
    unmeshed(band)?;
    let wires = occurrences(band, ShapeEnum::Wire)?;
    let candidates = wires
        .iter()
        .map(|wire| occurrences(wire, ShapeEnum::Edge))
        .collect::<Checked<Vec<_>>>()?;
    let stock = candidates
        .iter()
        .find(|edges| edges.iter().any(|e| e.is_same(&base_rim)))
        .filter(|edges| edges.len() == 4 && edges.iter().any(|e| e.is_same(&lid_rim)))
        .ok_or(GeometryRefusal::BoundaryInventory)?;
    let generators: Vec<_> = stock
        .iter()
        .filter(|s| !s.is_same(&base_rim) && !s.is_same(&lid_rim))
        .collect();
    let [first, second] = generators.as_slice() else {
        return Err(GeometryRefusal::BoundaryInventory);
    };
    if !first.is_same(second) {
        return Err(GeometryRefusal::BoundaryInventory);
    }
    let generator = topods::edge(first)?;
    let relative = band_place.predivided(generator.location())?;
    let usage = SurfaceUse {
        support: &band_support,
        relative: &relative,
        closed: true,
    };
    check_stock_edge(&generator, StockGenerator, &[usage])?;
    Ok(StockGeometryCensus {
        rim_edges: 2,
        generator_edges: 1,
        surface_uses: 6,
        vertex_endpoint_uses: 6,
    })
}

fn qualify_rim(
    face: &Face,
    band_support: &Handle<dyn GeomSurface>,
    band_place: &Location,
) -> Checked<Edge> {
    unmeshed(face)?;
    let [rim]: [Shape; 1] = occurrences(face, ShapeEnum::Edge)?
        .try_into()
        .map_err(|_| GeometryRefusal::BoundaryInventory)?;
    let edge = topods::edge(&rim)?;
    let (cap_support, cap_place) = BRepTool::surface_with_location(face)?;
    let band_relative = band_place.predivided(edge.location())?;
    let cap_relative = cap_place.predivided(edge.location())?;
    let owners = [
        SurfaceUse {
            support: band_support,
            relative: &band_relative,
            closed: false,
        },
        SurfaceUse {
            support: &cap_support,
            relative: &cap_relative,
            closed: false,
        },
    ];
    check_stock_edge(&edge, StockRim, &owners)?;
    Ok(edge)
}

fn check_stock_edge(edge: &Edge, role: GeometryRole, owners: &[SurfaceUse<'_>]) -> Checked<()> {
    check_flags(edge, role)?;
    check_representations(edge, role, owners, true)?;
    let vertices = occurrences(edge, ShapeEnum::Vertex)?;
    if vertices.len() != 2 {
        return Err(GeometryRefusal::Endpoint(role));
    }
    for vertex in vertices {
        check_vertex(&topods::vertex(&vertex)?, role)?;
    }
    Ok(())
}
