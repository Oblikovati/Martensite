// SPDX-License-Identifier: GPL-2.0-only
//! Exact storage readback for the admitted affine and circular native trims.
use super::{
    Failure, Stage,
    failure::native,
    refusal,
    source::{Uv, Xyz},
};
use martensite_tkbrep::{
    brep::{BRepTool, EdgePCurve, access::vertex_tshape},
    topexp::{self, Explorer},
    topods::{self, Edge, Face, Vertex},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dLine};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomCylindricalSurface, GeomLine, GeomPlane, GeomSurface},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::{
    gp::{Axis3, Point3},
    predicates::roots::metric_bounds::{
        exact_affine_coordinate, exact_placed_line_ends, exact_placed_point,
    },
    toploc::Location,
};
use martensite_tksketch3d::CurveId;

#[derive(Clone, Copy)]
pub(super) struct Subject(pub Option<CurveId>);

impl Subject {
    pub fn refused(self, expected: &'static str) -> Failure {
        refusal(Stage::Sweep, self.0, expected)
    }

    pub fn require(self, condition: bool, expected: &'static str) -> Result<(), Failure> {
        if condition {
            return Ok(());
        }
        Err(self.refused(expected))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Frame {
    pub origin: Xyz,
    pub axes: [Xyz; 3],
}

impl Frame {
    pub fn place(self, uv: Uv, subject: Subject) -> Result<Xyz, Failure> {
        let frame = std::array::from_fn(|i| [self.axes[0][i], self.axes[1][i], 0., self.origin[i]]);
        place_point([uv[0], uv[1], 0.], frame, subject)
    }
}

pub(super) fn plane(face: &Face, subject: Subject) -> Result<Frame, Failure> {
    let (surface, location) = surface(face)?;
    let plane = surface
        .downcast_ref::<GeomPlane>()
        .ok_or_else(|| subject.refused("a literal retained native plane"))?;
    let plane = plane.pln();
    capture_frame(plane.position(), &location, subject)
}

pub(super) fn cylinder(face: &Face, subject: Subject) -> Result<(Frame, f64), Failure> {
    let (surface, location) = surface(face)?;
    let carrier = surface
        .downcast_ref::<GeomCylindricalSurface>()
        .ok_or_else(|| subject.refused("a literal retained native cylinder"))?;
    let cylinder = native(carrier.cylinder(), Stage::Sweep)?;
    Ok((
        capture_frame(cylinder.position(), &location, subject)?,
        cylinder.radius(),
    ))
}

fn surface(face: &Face) -> Result<(Handle<dyn GeomSurface>, Location), Failure> {
    native(BRepTool::surface_with_location(face), Stage::Sweep)
}

fn capture_frame(raw: Axis3, location: &Location, subject: Subject) -> Result<Frame, Failure> {
    let axes = [raw.x_direction(), raw.y_direction(), raw.direction()]
        .map(|direction| *direction.coordinates());
    let (origin, axes) = place_frame(*raw.location().coordinates(), axes, location, subject)?;
    Ok(Frame { origin, axes })
}

fn place_frame(
    origin: Xyz,
    axes: [Xyz; 3],
    location: &Location,
    subject: Subject,
) -> Result<(Xyz, [Xyz; 3]), Failure> {
    let mut rows = checked_rows(location, subject)?;
    let origin = place_point(origin, rows, subject)?;
    rows.iter_mut().for_each(|row| row[3] = 0.);
    let [x, y, z] = axes.map(|axis| place_point(axis, rows, subject));
    Ok((origin, [x?, y?, z?]))
}

fn checked_rows(location: &Location, subject: Subject) -> Result<[[f64; 4]; 3], Failure> {
    location
        .transformation()
        .checked_unit_placement_rows()
        .map_err(|_| subject.refused("a finite unit native placement with coherent evaluator rows"))
}

fn place_point(point: Xyz, rows: [[f64; 4]; 3], subject: Subject) -> Result<Xyz, Failure> {
    let [x, y, z] = point;
    exact_placed_point(Point3::new(x, y, z), rows)
        .ok()
        .flatten()
        .ok_or_else(|| subject.refused("an exactly representable world point or direction"))
}

pub(super) fn vertex(vertex: &Vertex, expected: Xyz, subject: Subject) -> Result<(), Failure> {
    let stored = native(
        vertex_tshape(vertex, "mixed prism result vertex"),
        Stage::Sweep,
    )?
    .pnt();
    let actual = place_point(
        *stored.coordinates(),
        checked_rows(&vertex.location(), subject)?,
        subject,
    )?;
    subject.require(
        actual == expected,
        "the exact authored point or its exact translated image",
    )
}

pub(super) fn edge_vertices(
    edge: &Edge,
    ends: [&Vertex; 2],
    subject: Subject,
) -> Result<(), Failure> {
    let (first, last) = topexp::vertices(edge, false);
    subject.require(
        first.is_same(ends[0]) && last.is_same(ends[1]),
        "the ordered native history endpoint identities",
    )
}

pub(super) fn boundary_edges(
    face: &Face,
    subject: Subject,
    max_edges: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<Edge>, Failure> {
    let forward = native(
        topods::face(&face.oriented(Orientation::Forward)),
        Stage::Sweep,
    )?;
    let mut wires = native(
        Explorer::new(&forward, ShapeEnum::Wire, ShapeEnum::Shape),
        Stage::Sweep,
    )?;
    subject.require(wires.more(), "one actual closed wire on each retained face")?;
    let wire = wires.current().clone();
    native(wires.next(), Stage::Sweep)?;
    subject.require(
        !wires.more() && native(BRepTool::is_closed(&wire), Stage::Sweep)?,
        "exactly one closed native wire",
    )?;
    let mut edges = native(
        Explorer::new(&wire, ShapeEnum::Edge, ShapeEnum::Shape),
        Stage::Sweep,
    )?;
    let mut actual = Vec::with_capacity(max_edges);
    while edges.more() {
        super::poll(cancelled, Stage::Sweep)?;
        subject.require(
            actual.len() < max_edges,
            "only expected native boundary edges",
        )?;
        actual.push(native(topods::edge(edges.current()), Stage::Sweep)?);
        native(edges.next(), Stage::Sweep)?;
    }
    subject.require(
        actual.len() == max_edges,
        "the complete native face boundary",
    )?;
    Ok(actual)
}

pub(super) fn line_xyz(
    edge: &Edge,
    expected: [Xyz; 2],
    subject: Subject,
) -> Result<[f64; 2], Failure> {
    let trim = native(BRepTool::curve_with_location(edge), Stage::Sweep)?;
    let line = trim
        .curve
        .downcast_ref::<GeomLine>()
        .ok_or_else(|| subject.refused("an analytic native affine line trim"))?;
    let range = [trim.first, trim.last];
    let actual = exact_placed_line_ends(line.lin(), checked_rows(&trim.location, subject)?, range)
        .ok()
        .flatten();
    subject.require(
        actual == Some(expected),
        "the exact affine trace in the increasing native trim",
    )?;
    Ok(range)
}

pub(super) fn circle_xyz(
    edge: &Edge,
    expected: Frame,
    radius: f64,
    range: [f64; 2],
    subject: Subject,
) -> Result<(), Failure> {
    let trim = native(BRepTool::curve_with_location(edge), Stage::Sweep)?;
    let carrier = trim
        .curve
        .downcast_ref::<GeomCircle>()
        .ok_or_else(|| subject.refused("an analytic native circular trim"))?;
    let circle = native(carrier.circ(), Stage::Sweep)?;
    let frame = Axis3::from_axis2(circle.position());
    let actual = capture_frame(frame, &trim.location, subject)?;
    subject.require(
        actual == expected && circle.radius() == radius && [trim.first, trim.last] == range,
        "the exact source circle carrier and native parameter interval",
    )
}

pub(super) fn pcurve(
    edge: &Edge,
    face: &Face,
    range: [f64; 2],
    require_stored: bool,
    subject: Subject,
) -> Result<EdgePCurve, Failure> {
    let pcurve = native(BRepTool::curve_on_face(edge, face), Stage::Sweep)?;
    let matches = [pcurve.first, pcurve.last] == range && (!require_stored || pcurve.is_stored);
    subject.require(
        matches,
        "the same native parameter interval and required stored pcurve",
    )?;
    Ok(pcurve)
}

pub(super) fn line_uv(pcurve: &EdgePCurve, subject: Subject) -> Result<[Uv; 2], Failure> {
    let line = pcurve
        .pcurve
        .downcast_ref::<Geom2dLine>()
        .ok_or_else(|| subject.refused("a literal analytic UV line trim"))?;
    let origin = [line.location().x(), line.location().y()];
    let axis = [line.direction().x(), line.direction().y()];
    let [a, b] = [pcurve.first, pcurve.last].map(|t| exact_uv(origin, axis, t));
    a.zip(b)
        .map(|(a, b)| [a, b])
        .ok_or_else(|| subject.refused("exact binary64 affine trim endpoints"))
}

fn exact_uv(origin: Uv, direction: Uv, parameter: f64) -> Option<Uv> {
    let first = exact_affine_coordinate(origin[0], direction[0], parameter).ok()??;
    let second = exact_affine_coordinate(origin[1], direction[1], parameter).ok()??;
    Some([first, second])
}

pub(super) fn circle_uv(pcurve: &EdgePCurve, radius: f64, subject: Subject) -> Result<(), Failure> {
    let circle = pcurve
        .pcurve
        .downcast_ref::<Geom2dCircle>()
        .ok_or_else(|| subject.refused("a literal native circular cap pcurve"))?;
    let circle = native(circle.circ2d(), Stage::Sweep)?;
    let x = circle.x_axis().direction();
    let y = circle.y_axis().direction();
    subject.require(
        circle.radius() == radius
            && circle.location().x() == 0.
            && circle.location().y() == 0.
            && [x.x(), x.y(), y.x(), y.y()] == [1., 0., 0., 1.],
        "the original circle in the exact cap parameters",
    )
}

pub(super) fn planar_trace(
    edge: &Edge,
    face: &Face,
    frame: Frame,
    range: [f64; 2],
    xyz: [Xyz; 2],
    subject: Subject,
) -> Result<(), Failure> {
    let trim = pcurve(edge, face, range, false, subject)?;
    // Native planar sides omit stored pcurves. Their analytic projection must
    // agree over the whole affine range, which is fixed by these exact endpoints.
    let [a, b] = line_uv(&trim, subject)?.map(|uv| frame.place(uv, subject));
    subject.require(
        [a?, b?] == xyz,
        "the complete planar pcurve trace of the retained 3D line",
    )
}
