//! Capture the selected native outer trim and the enclosures of all inner wires.
use super::*;
use martensite_tkbrep::{
    brep::BRepTool, brep_adaptor::BRepAdaptorSurface, topexp::Explorer, topods,
};
use martensite_tkg2d::{adaptor2d::Adaptor2dCurve2d, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkg3d::{
    adaptor3d::Adaptor3dSurface,
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::geom_abs::CurveType;

pub(super) enum BandFailure {
    Native(Box<Failure>),
    Unsupported(&'static str),
}
pub(super) type BandChecked<T> = Result<T, BandFailure>;

fn native<T>(result: Result<T, Failure>) -> BandChecked<T> {
    result.map_err(|cause| BandFailure::Native(Box::new(cause)))
}
fn unsupported(expected: &'static str) -> BandFailure {
    BandFailure::Unsupported(expected)
}

pub(super) struct Band {
    pub axial: [f64; 2],
    pub angular: Option<[f64; 2]>,
    pub holes: Vec<holes::TrimBox>,
    pub outer: Option<outer::OuterTrim>,
}

struct Use {
    edge: Edge,
    points: [[f64; 2]; 2],
}

pub(super) fn capture(face: &Face) -> BandChecked<Band> {
    let forward = native(topods::face(&face.oriented(Orientation::Forward)))?;
    let (outer, inner) = face_wires(&forward)?;
    let (bounds, angular, outer) = match rectangle_domain(&forward, &outer) {
        Ok((bounds, angular)) => (bounds, angular, None),
        Err(BandFailure::Unsupported(_)) => {
            let shaped = outer::OuterTrim::capture(&forward, &outer)?;
            let bounds = shaped.bounds();
            (bounds, Some(bounds.0[0]), Some(shaped))
        }
        Err(cause) => return Err(cause),
    };
    Ok(Band {
        axial: bounds.0[1],
        angular,
        holes: holes::capture(&forward, &inner, bounds.0)?,
        outer,
    })
}

fn rectangle_domain(face: &Face, wire: &Shape) -> BandChecked<(holes::TrimBox, Option<[f64; 2]>)> {
    let uses = occurrences(face, wire)?;
    let [[u0, u1], [v0, v1]] = envelope(&uses)?;
    let sides = rectangle(&uses, [u0, v0], [u1, v1])?;
    let angular = angular_band(face, sides, [u0, u1])?;
    Ok((holes::TrimBox([[u0, u1], [v0, v1]]), angular))
}

fn occurrences(face: &Face, wire: &Shape) -> BandChecked<[Use; 4]> {
    let mut edges = native(Explorer::new(wire, ShapeEnum::Edge, ShapeEnum::Shape))?;
    let mut uses = Vec::with_capacity(4);
    while edges.more() {
        if uses.len() == 4 {
            return Err(unsupported("four directed rectangular trim occurrences"));
        }
        uses.push(edge_use(edges.current(), face)?);
        native(edges.next())?;
    }
    uses.try_into()
        .map_err(|_| unsupported("four directed rectangular trim occurrences"))
}

fn face_wires(face: &Face) -> BandChecked<(Shape, Vec<Shape>)> {
    let mut wires = native(Explorer::new(face, ShapeEnum::Wire, ShapeEnum::Shape))?;
    if !wires.more() {
        return Err(unsupported("one native outer trim wire"));
    }
    let first = wires.current().clone();
    native(wires.next())?;
    if !wires.more() {
        return Ok((first, Vec::new()));
    }
    let outer = native(martensite_tkbrep::brep_tools::outer_wire(face))?.into_shape();
    let mut inner = Vec::new();
    let mut occurrences = usize::from(first.is_same(&outer));
    if occurrences == 0 {
        inner.push(first);
    }
    while wires.more() {
        if wires.current().is_same(&outer) {
            occurrences += 1;
        } else {
            inner.push(wires.current().clone());
        }
        native(wires.next())?;
    }
    if occurrences != 1 {
        return Err(unsupported("one uniquely occurring outer wire"));
    }
    Ok((outer, inner))
}

fn edge_use(shape: &Shape, face: &Face) -> BandChecked<Use> {
    let edge = native(topods::edge(shape))?;
    let pcurve = native(BRepTool::curve_on_face(&edge, face))?;
    if !pcurve.is_stored
        || pcurve.pcurve.is_null()
        || !pcurve.first.is_finite()
        || !pcurve.last.is_finite()
        || pcurve.first >= pcurve.last
    {
        return Err(unsupported(
            "stored trim with a finite increasing parameter interval",
        ));
    }
    let curve = native(Geom2dAdaptorCurve::with_curve(&pcurve.pcurve))?;
    if native(curve.get_type())? != CurveType::Line {
        return Err(unsupported("straight native trim pcurve"));
    }
    let line = native(curve.line())?;
    let points = endpoints(
        [line.location().x(), line.location().y()],
        [line.direction().x(), line.direction().y()],
        [pcurve.first, pcurve.last],
        edge.orientation(),
    )?;
    Ok(Use { edge, points })
}

pub(super) fn endpoints(
    origin: [f64; 2],
    direction: [f64; 2],
    interval: [f64; 2],
    orientation: Orientation,
) -> BandChecked<[[f64; 2]; 2]> {
    let moving = match direction.map(f64::abs) {
        [1., 0.] => 0,
        [0., 1.] => 1,
        _ => return Err(unsupported("axis-aligned trim line")),
    };
    let mut points = [origin; 2];
    for (point, t) in points.iter_mut().zip(interval) {
        point[moving] = exact_add(origin[moving], direction[moving] * t)
            .ok_or_else(|| unsupported("exact binary64 trim corner"))?;
    }
    match orientation {
        Orientation::Forward => Ok(points),
        Orientation::Reversed => Ok([points[1], points[0]]),
        _ => Err(unsupported("forward or reversed trim occurrence")),
    }
}

fn exact_add(a: f64, b: f64) -> Option<f64> {
    // Ordered FastTwoSum determines whether the rounded binary64 sum is exact.
    let (large, small) = if a.abs() >= b.abs() { (a, b) } else { (b, a) };
    let sum = large + small;
    (sum.is_finite() && sum - large == small).then_some(sum)
}

fn envelope(uses: &[Use; 4]) -> BandChecked<[[f64; 2]; 2]> {
    let mut bounds = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
    for point in uses.iter().flat_map(|edge| edge.points) {
        if !point.iter().all(|value| value.is_finite()) {
            return Err(unsupported("finite native trim corners"));
        }
        for (bound, value) in bounds.iter_mut().zip(point) {
            *bound = [bound[0].min(value), bound[1].max(value)];
        }
    }
    if bounds.iter().any(|pair| pair[0] >= pair[1]) {
        return Err(unsupported("positive rectangular trim width and height"));
    }
    Ok(bounds)
}

fn rectangle(uses: &[Use; 4], [u0, v0]: [f64; 2], [u1, v1]: [f64; 2]) -> BandChecked<[&Edge; 2]> {
    let corners = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];
    let mut sides = Vec::with_capacity(4);
    for i in 0..4 {
        let expected = [corners[i], corners[(i + 1) % 4]];
        let mut matching = uses.iter().filter(|edge| edge.points == expected);
        let side = matching
            .next()
            .ok_or_else(|| unsupported("positive rectangular wire with exact corner joins"))?;
        if matching.next().is_some() {
            return Err(unsupported(
                "one directed trim occurrence per rectangle side",
            ));
        }
        sides.push(&side.edge);
    }
    Ok([sides[1], sides[3]])
}

fn angular_band(face: &Face, sides: [&Edge; 2], bounds: [f64; 2]) -> BandChecked<Option<[f64; 2]>> {
    let adaptor = native(BRepAdaptorSurface::with_face(face, false))?;
    let period = native(adaptor.u_period())?;
    let width =
        exact_add(bounds[1], -bounds[0]).ok_or_else(|| unsupported("exact angular trim width"))?;
    if sides[0].is_same(sides[1]) && width == period && stored_seam(face, sides[0])? {
        return Ok(None);
    }
    if width < period && !sides[0].is_same(sides[1]) {
        return Ok(Some(bounds));
    }
    Err(unsupported(
        "distinct partial angular boundaries or one paired native seam over a period",
    ))
}

pub(super) fn stored_seam(face: &Face, edge: &Edge) -> BandChecked<bool> {
    let (surface, location) = native(BRepTool::surface_with_location(face))?;
    native(BRepTool::is_closed_on_surface(edge, &surface, &location))
}
