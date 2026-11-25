//! Capture exactly the native side, two disks and their shared Circle→Extrude trims.
use super::*;
use crate::surface_sketch::wrapped_profile::api::footprint::band;
use martensite_tkbrep::brep::EdgePCurve;
use martensite_tkmath::toploc::Location;

#[path = "source_seam.rs"]
mod seam;
use seam::Seam;

pub(super) struct Stock<'a> {
    pub(super) side: &'a Face,
    pub(super) shell: Shape,
    pub(super) side_wire: Shape,
    pub(super) ends: [Rim; 2],
    pub(super) seam: Edge,
    pub(super) seam_tol: f64,
    pub(super) seam_order: [usize; 2],
    pub(super) base: usize,
    pub(super) origin: [f64; 3],
    pub(super) radius: f64,
    pub(super) height: f64,
    pub(super) tolerance: f64,
}

pub(super) struct Rim {
    pub(super) face: Face,
    pub(super) wire: Shape,
    pub(super) edge: Edge,
    pub(super) anchor: Vertex,
    pub(super) curve: EdgeCurve3D,
    pub(super) cap_pcurve: Handle<dyn Geom2dCurve>,
    pub(super) center: [f64; 3],
    pub(super) v: f64,
    pub(super) edge_tol: f64,
    pub(super) vertex_tol: f64,
    pub(super) disk_tol: f64,
}

struct Carrier {
    origin: [f64; 3],
    radius: f64,
    height: f64,
    tolerance: f64,
}

impl<'a> Stock<'a> {
    pub(super) fn capture(
        attempt: RecutCheck,
        solid: &Shape,
        side: &'a Face,
    ) -> FeatureChecked<Self> {
        let (shell, faces) = stock_faces(attempt, solid, side)?;
        let carrier = Carrier::capture(attempt, side)?;
        let side_wire = only_wire(attempt, side)?;
        let edges = wire_edges(attempt, &side_wire)?;
        let ends = [
            Rim::capture(attempt, side, &edges, &carrier, &faces[0])?,
            Rim::capture(attempt, side, &edges, &carrier, &faces[1])?,
        ];
        let seam = Seam::capture(attempt, side, &edges, &carrier, &ends)?;
        let tolerances = ends
            .iter()
            .flat_map(|rim| [rim.edge_tol, rim.vertex_tol, rim.disk_tol])
            .chain([carrier.tolerance, seam.tolerance]);
        let tolerance = strict_stock_limit(attempt, tolerances)?;
        Ok(Self {
            side,
            shell,
            side_wire,
            ends,
            seam: seam.edge,
            seam_tol: seam.tolerance,
            seam_order: seam.order,
            base: seam.base,
            origin: carrier.origin,
            radius: carrier.radius,
            height: carrier.height,
            tolerance,
        })
    }
}

fn strict_stock_limit(
    attempt: RecutCheck,
    mut tolerances: impl Iterator<Item = f64>,
) -> FeatureChecked<f64> {
    tolerances.try_fold(f64::INFINITY, |limit, tolerance| {
        if !tolerance.is_finite() || tolerance <= 0. {
            return Err(
                attempt.refusal("finite positive original band, disk, rim and seam tolerances")
            );
        }
        Ok(limit.min(tolerance))
    })
}

fn stock_faces(
    attempt: RecutCheck,
    solid: &Shape,
    side: &Face,
) -> FeatureChecked<(Shape, [Shape; 2])> {
    let stock = |shape: &Shape, kind| {
        shape.shape_type().is_ok_and(|actual| actual == kind) && shape.location().is_identity()
    };
    let [shell]: [Shape; 1] = children(attempt, solid)?
        .try_into()
        .map_err(|_| attempt.refusal("one native stock shell"))?;
    if solid.orientation() != Orientation::Forward
        || !matches!(
            shell.orientation(),
            Orientation::Forward | Orientation::Reversed
        )
        || !stock(solid, ShapeEnum::Solid)
        || !stock(&shell, ShapeEnum::Shell)
        || !attempt.native("closed source shell", BRepTool::is_closed(&shell))?
    {
        return Err(attempt.refusal("closed outward stock solid with one shell"));
    }
    let mut faces = children(attempt, &shell)?;
    if faces.len() != 3 || faces.iter().filter(|face| face.is_equal(side)).count() != 1 {
        return Err(attempt.refusal("the exact selected band and two stock disks"));
    }
    faces.retain(|face| !face.is_same(side));
    let ends = faces
        .try_into()
        .map_err(|_| attempt.refusal("two distinct native end disks"))?;
    Ok((shell, ends))
}

impl Carrier {
    fn capture(attempt: RecutCheck, side: &Face) -> FeatureChecked<Self> {
        let (surface, location) = attempt.native(
            "selected stock carrier",
            BRepTool::surface_with_location(side),
        )?;
        let raw = surface
            .downcast_ref::<GeomCylindricalSurface>()
            .ok_or_else(|| attempt.refusal("stored analytic stock cylinder"))?;
        let cylinder = attempt.native("selected cylinder geometry", raw.cylinder())?;
        let frame = cylinder.position();
        let origin = world_point(attempt, *frame.location().coordinates(), &location)?;
        let native = band::capture(side).map_err(|cause| attempt.0.footprint(cause.into()))?;
        if side.orientation() != Orientation::Reversed
            || *frame.x_direction().coordinates() != X
            || *frame.y_direction().coordinates() != Y
            || *frame.direction().coordinates() != [0., 0., -1.]
            || native.angular.is_some()
            || native.outer.is_some()
            || !native.holes.is_empty()
            || native.axial[1] != 0.
            || !cylinder.radius().is_finite()
            || cylinder.radius() <= 0.
        {
            return Err(
                attempt.refusal("indirect cardinal full-cylinder stock without extra trims")
            );
        }
        let tolerance = attempt
            .native("stock side tolerance", face_tshape(side, "cylinder recut"))?
            .tolerance();
        Ok(Self {
            origin,
            radius: cylinder.radius(),
            height: -native.axial[0],
            tolerance,
        })
    }

    fn check_anchor(
        &self,
        attempt: RecutCheck,
        anchor: &Vertex,
        [x, y, z]: [f64; 3],
        tolerance: f64,
    ) -> FeatureChecked<()> {
        let (x, error) = sum_with_residual(x, self.radius);
        let point = attempt.native("native original seam anchor", BRepTool::pnt(anchor))?;
        if *point.coordinates() != [x, y, z] || !tolerance.is_finite() || error >= tolerance {
            return Err(attempt.refusal("original seam anchor on the matched circular rim"));
        }
        Ok(())
    }

    fn center_at(&self, attempt: RecutCheck, v: f64) -> FeatureChecked<[f64; 3]> {
        let [x, y, z] = self.origin;
        let (height, residual) = sum_with_residual(z, -v);
        if !height.is_finite() || residual != 0. {
            return Err(attempt.refusal("exact axial position of the stock circle"));
        }
        Ok([x, y, height])
    }
}

fn only_wire(attempt: RecutCheck, face: &Face) -> FeatureChecked<Shape> {
    let [wire]: [Shape; 1] = children(attempt, face)?
        .try_into()
        .map_err(|_| attempt.refusal("one original stock trim wire"))?;
    if !wire.shape_type().is_ok_and(|kind| kind == ShapeEnum::Wire)
        || !attempt.native("closed stock wire", BRepTool::is_closed(&wire))?
    {
        return Err(attempt.refusal("closed original stock trim wire"));
    }
    Ok(wire)
}

fn wire_edges(attempt: RecutCheck, wire: &Shape) -> FeatureChecked<Vec<Edge>> {
    children(attempt, wire)?
        .iter()
        .map(|child| attempt.native("stock trim edge", topods::edge(child)))
        .collect()
}

impl Rim {
    fn capture(
        attempt: RecutCheck,
        side: &Face,
        edges: &[Edge],
        carrier: &Carrier,
        cap: &Shape,
    ) -> FeatureChecked<Self> {
        let face = attempt.native("stock disk", topods::face(cap))?;
        let wire = only_wire(attempt, &face)?;
        let [edge]: [Edge; 1] = wire_edges(attempt, &wire)?
            .try_into()
            .map_err(|_| attempt.refusal("one closed circle per stock end"))?;
        let on_side = side_rim(attempt, &edge, edges)?;
        let lateral = trim(attempt, on_side, side)?;
        let [u, v] = line(attempt, &lateral, [1., 0.])?;
        let center = carrier.center_at(attempt, v)?;
        if [lateral.first, lateral.last] != [0., TAU]
            || u != 0.
            || (v != 0. && v != -carrier.height)
        {
            return Err(attempt.refusal("native rim over one exact stored turn at a band endpoint"));
        }
        let curve = rim_curve(attempt, &edge, center, carrier.radius)?;
        let cap_pcurve = disk_circle(attempt, &face, &edge, center, carrier.radius)?;
        let anchor = topexp::first_vertex(&edge, false);
        if anchor.is_null() || !anchor.is_same(&topexp::last_vertex(&edge, false)) {
            return Err(attempt.refusal("both ends of the rim own one exact native anchor"));
        }
        let edge_tol = attempt
            .native("source rim tolerance", edge_tshape(&edge, "cylinder recut"))?
            .tolerance();
        let vertex_tol = attempt
            .native(
                "source rim anchor tolerance",
                vertex_tshape(&anchor, "cylinder recut"),
            )?
            .tolerance();
        let disk_tol = attempt
            .native(
                "source disk tolerance",
                face_tshape(&face, "cylinder recut"),
            )?
            .tolerance();
        carrier.check_anchor(attempt, &anchor, center, vertex_tol)?;
        Ok(Self {
            face,
            wire,
            edge,
            anchor,
            curve,
            cap_pcurve,
            center,
            v,
            edge_tol,
            vertex_tol,
            disk_tol,
        })
    }
}

fn opposite(a: Orientation, b: Orientation) -> bool {
    use Orientation::{Forward, Reversed};
    matches!((a, b), (Forward, Reversed) | (Reversed, Forward))
}

fn side_rim<'a>(attempt: RecutCheck, disk: &Edge, edges: &'a [Edge]) -> FeatureChecked<&'a Edge> {
    let mut uses = edges.iter().filter(|edge| edge.is_same(disk));
    let edge = uses
        .next()
        .filter(|_| uses.next().is_none())
        .ok_or_else(|| attempt.refusal("one stock side occurrence of the original rim"))?;
    if !opposite(edge.orientation(), disk.orientation()) {
        return Err(attempt.refusal("opposite side and disk uses of the original rim"));
    }
    Ok(edge)
}

fn rim_curve(
    attempt: RecutCheck,
    edge: &Edge,
    center: [f64; 3],
    radius: f64,
) -> FeatureChecked<EdgeCurve3D> {
    let stored = curve(attempt, edge)?;
    let circle = stored
        .curve
        .downcast_ref::<GeomCircle>()
        .ok_or_else(|| attempt.refusal("stored spatial rim circle"))?;
    let circle = attempt.native("rim circle geometry", circle.circ())?;
    let frame = circle.position();
    if [stored.first, stored.last] != [0., TAU]
        || world_point(attempt, *frame.location().coordinates(), &stored.location)? != center
        || *frame.x_direction().coordinates() != X
        || *frame.y_direction().coordinates() != Y
        || circle.radius() != radius
    {
        return Err(
            attempt.refusal("stock circle matches the side's exact radial frame and radius")
        );
    }
    Ok(stored)
}

fn disk_circle(
    attempt: RecutCheck,
    face: &Face,
    edge: &Edge,
    center: [f64; 3],
    radius: f64,
) -> FeatureChecked<Handle<dyn Geom2dCurve>> {
    let (surface, location) =
        attempt.native("stock disk surface", BRepTool::surface_with_location(face))?;
    let plane = surface
        .downcast_ref::<GeomPlane>()
        .ok_or_else(|| attempt.refusal("native planar end disk"))?;
    let frame = plane.pln().position();
    let origin = world_point(attempt, *frame.location().coordinates(), &location)?;
    let pcurve = trim(attempt, edge, face)?;
    let circle = pcurve
        .pcurve
        .downcast_ref::<Geom2dCircle>()
        .ok_or_else(|| attempt.refusal("stored plane trim is one analytic circle"))?;
    let circle = attempt.native("disk circle geometry", circle.circ2d())?;
    let (u, v) = (circle.location().x(), circle.location().y());
    let planar = circle.position();
    let ((x, ex), (y, ey)) = (
        sum_with_residual(origin[0], u),
        sum_with_residual(origin[1], v),
    );
    if [pcurve.first, pcurve.last] != [0., TAU]
        || [ex, ey] != [0., 0.]
        || [x, y, origin[2]] != center
        || circle.radius() != radius
        || *frame.x_direction().coordinates() != X
        || *frame.y_direction().coordinates() != Y
        || [planar.x_direction().x(), planar.x_direction().y()] != [1., 0.]
        || [planar.y_direction().x(), planar.y_direction().y()] != [0., 1.]
    {
        return Err(attempt.refusal("projected disk circle matches the literal 3D rim parameter"));
    }
    Ok(pcurve.pcurve)
}

fn world_point(
    attempt: RecutCheck,
    point: [f64; 3],
    location: &Location,
) -> FeatureChecked<[f64; 3]> {
    let transform = location.transformation();
    if transform.scale_factor() != 1.
        || transform.matrix().coefficients() != &[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]
    {
        return Err(attempt.refusal("native stock frame with translation only"));
    }
    let shift = *transform.translation_part().coordinates();
    let sums = std::array::from_fn(|i| sum_with_residual(point[i], shift[i]));
    if sums
        .iter()
        .any(|(sum, error)| !sum.is_finite() || *error != 0.)
    {
        return Err(attempt.refusal("exact native stock translation"));
    }
    Ok(sums.map(|(sum, _)| sum))
}

fn curve(attempt: RecutCheck, edge: &Edge) -> FeatureChecked<EdgeCurve3D> {
    let stored = attempt.native("original stored curve", BRepTool::curve_with_location(edge))?;
    if stored.curve.is_null()
        || !attempt.native("native same parameter", BRepTool::same_parameter(edge))?
        || !attempt.native("native same range", BRepTool::same_range(edge))?
    {
        return Err(
            attempt.refusal("nonempty original spatial stock curve with common native ranges")
        );
    }
    Ok(stored)
}

fn trim(attempt: RecutCheck, edge: &Edge, face: &Face) -> FeatureChecked<EdgePCurve> {
    let stored = attempt.native(
        "original located stock trim",
        BRepTool::curve_on_face(edge, face),
    )?;
    if !stored.is_stored || stored.pcurve.is_null() {
        return Err(attempt.refusal("stored original trim on its incident stock face"));
    }
    Ok(stored)
}

fn line(attempt: RecutCheck, trim: &EdgePCurve, direction: [f64; 2]) -> FeatureChecked<[f64; 2]> {
    let line = trim
        .pcurve
        .downcast_ref::<Geom2dLine>()
        .filter(|line| [line.direction().x(), line.direction().y()] == direction)
        .ok_or_else(|| attempt.refusal("axis-aligned native trim parameter"))?;
    Ok([line.location().x(), line.location().y()])
}
