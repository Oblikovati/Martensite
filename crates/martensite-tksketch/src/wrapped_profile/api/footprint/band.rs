//! Recognize a trim rectangle from every directed native occurrence, including its seam.
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

pub(super) struct Band {
    pub axial: [f64; 2],
    pub angular: Option<[f64; 2]>,
}

struct Use {
    edge: Edge,
    points: [[f64; 2]; 2],
}

pub(super) fn capture(face: &Face) -> FootprintChecked<Band> {
    let forward = native(topods::face(&face.oriented(Orientation::Forward)))?;
    let uses = occurrences(&forward)?;
    let [[u0, u1], [v0, v1]] = envelope(&uses)?;
    let sides = rectangle(&uses, [u0, v0], [u1, v1])?;
    let angular = angular_band(&forward, sides, [u0, u1])?;
    Ok(Band {
        axial: [v0, v1],
        angular,
    })
}

fn occurrences(face: &Face) -> FootprintChecked<[Use; 4]> {
    let wire = only_wire(face)?;
    let mut edges = native(Explorer::new(&wire, ShapeEnum::Edge, ShapeEnum::Shape))?;
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

fn only_wire(face: &Face) -> FootprintChecked<Shape> {
    let mut wires = native(Explorer::new(face, ShapeEnum::Wire, ShapeEnum::Shape))?;
    if !wires.more() {
        return Err(unsupported("one native outer trim wire"));
    }
    let wire = wires.current().clone();
    native(wires.next())?;
    if wires.more() {
        return Err(unsupported(
            "one native outer trim wire without additional loops",
        ));
    }
    Ok(wire)
}

fn edge_use(shape: &Shape, face: &Face) -> FootprintChecked<Use> {
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

fn endpoints(
    origin: [f64; 2],
    direction: [f64; 2],
    interval: [f64; 2],
    orientation: Orientation,
) -> FootprintChecked<[[f64; 2]; 2]> {
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

fn envelope(uses: &[Use; 4]) -> FootprintChecked<[[f64; 2]; 2]> {
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

fn rectangle(
    uses: &[Use; 4],
    [u0, v0]: [f64; 2],
    [u1, v1]: [f64; 2],
) -> FootprintChecked<[&Edge; 2]> {
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

fn angular_band(
    face: &Face,
    sides: [&Edge; 2],
    bounds: [f64; 2],
) -> FootprintChecked<Option<[f64; 2]>> {
    let adaptor = native(BRepAdaptorSurface::with_face(face, false))?;
    let period = native(adaptor.u_period())?;
    let width =
        exact_add(bounds[1], -bounds[0]).ok_or_else(|| unsupported("exact angular trim width"))?;
    if sides[0].is_same(sides[1])
        && width == period
        && native(BRepTool::is_closed_on_face(sides[0], face))?
    {
        return Ok(None);
    }
    if width < period && !sides[0].is_same(sides[1]) {
        return Ok(Some(bounds));
    }
    Err(unsupported(
        "distinct partial angular boundaries or one paired native seam over a period",
    ))
}
