// SPDX-License-Identifier: GPL-2.0-only
use super::interval::{Point, Span, determinant, l1_difference, mapped_vector, placed};
use super::{unbound, vertex_point};
use crate::spatial_prism::{
    SpatialPrismFailure as Failure, SpatialPrismStage as Stage, failure::native, poll,
};
use martensite_tkbrep::{
    brep::{BRepTool, EdgePCurve},
    topexp::{self, Explorer},
    topods::{self, Edge, Face, Vertex},
    toptools::IndexedMapOfShape,
};
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::{
    geom::{GeomLine, GeomPlane},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::gp::{Axis3, Transform3};

pub(super) fn contribution(
    face: &Face,
    index: usize,
    reference: [f64; 3],
    limit: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Span, f64), Failure> {
    let incomplete = |expected| unbound(Some(index), expected);
    let sign = match face.orientation() {
        Orientation::Forward => Span::ONE,
        Orientation::Reversed => Span::at(-1.),
        _ => return Err(incomplete("an oriented face occurrence")),
    };
    let forward = native(
        topods::face(&face.oriented(Orientation::Forward)),
        Stage::Sweep,
    )?;
    let mut wires = native(
        Explorer::new(&forward, ShapeEnum::Wire, ShapeEnum::Shape),
        Stage::Sweep,
    )?;
    if !wires.more() {
        return Err(incomplete("one trimmed outer wire"));
    }
    let outer = wires.current().clone();
    native(Explorer::next(&mut wires), Stage::Sweep)?;
    if wires.more() {
        return Err(incomplete("one trimmed outer wire"));
    }
    let plane = plane_map(&forward, reference, index)?;
    let edges = native(
        Explorer::new(&outer, ShapeEnum::Edge, ShapeEnum::Shape),
        Stage::Sweep,
    )?;
    let trims = edges
        .map(|edge| {
            poll(cancelled, Stage::Sweep)?;
            let edge = native(topods::edge(&edge), Stage::Sweep)?;
            decode_trim(&edge, &forward, plane, limit, index)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let trace = trims.iter().map(|trim| trim.trace).fold(0., f64::max);
    Ok((sign * flux(&trims, index)?, trace))
}

#[derive(Clone, Copy)]
struct PlaneMap {
    basis: Axis3,
    placement: Transform3,
    reference: [f64; 3],
}

fn plane_map(face: &Face, reference: [f64; 3], index: usize) -> Result<PlaneMap, Failure> {
    let (stored, location) = native(BRepTool::surface_with_location(face), Stage::Sweep)?;
    let plane = stored
        .downcast_ref::<GeomPlane>()
        .ok_or_else(|| unbound(Some(index), "a decoded planar face"))?;
    Ok(PlaneMap {
        basis: plane.pln().position(),
        placement: location.transformation(),
        reference,
    })
}

impl PlaneMap {
    fn uv(self, [u, v]: [Span; 2]) -> Point {
        let origin = placed(
            self.basis.location().coordinates().map(Span::at),
            self.placement,
            self.reference,
        );
        let x = mapped_vector(
            self.basis.x_direction().coordinates().map(Span::at),
            self.placement,
        );
        let y = mapped_vector(
            self.basis.y_direction().coordinates().map(Span::at),
            self.placement,
        );
        std::array::from_fn(|axis| origin[axis] + u * x[axis] + v * y[axis])
    }
}

struct Trim {
    start: Point,
    end: Point,
    vertices: [Vertex; 2],
    trace: f64,
}

fn decode_trim(
    edge: &Edge,
    face: &Face,
    plane: PlaneMap,
    limit: f64,
    index: usize,
) -> Result<Trim, Failure> {
    let incomplete = |expected| unbound(Some(index), expected);
    let reverse = match edge.orientation() {
        Orientation::Forward => false,
        Orientation::Reversed => true,
        _ => return Err(incomplete("an oriented trim occurrence")),
    };
    let pcurve = native(BRepTool::curve_on_face(edge, face), Stage::Sweep)?;
    let uv =
        pcurve_points(&pcurve, reverse).ok_or_else(|| incomplete("a finite affine pcurve trim"))?;
    let mapped = uv.map(|point| plane.uv(point));
    let line = line_points(edge, reverse, plane.reference, index)?;
    let (start, end) = topexp::vertices(edge, true);
    let vertices = [start, end];
    let mut joined = mapped;
    let mut trace = 0.0_f64;
    for (i, vertex) in vertices.iter().enumerate() {
        let point = vertex_point(vertex, plane.reference)?;
        let bound = (Span::at(l1_difference(mapped[i], line[i]))
            + Span::at(l1_difference(line[i], point)))
        .hi;
        if !bound.is_finite() || bound > limit {
            return Err(Failure::SweepFaceTrace {
                face: index,
                bound,
                limit,
            });
        }
        trace = trace.max(bound);
        joined[i] =
            std::array::from_fn(|axis| mapped[i][axis].hull(point[axis]).hull(line[i][axis]));
    }
    Ok(Trim {
        start: joined[0],
        end: joined[1],
        vertices,
        trace,
    })
}

fn pcurve_points(pcurve: &EdgePCurve, reverse: bool) -> Option<[[Span; 2]; 2]> {
    let line = pcurve.pcurve.downcast_ref::<Geom2dLine>()?;
    let parameters = ordered_parameters(pcurve.first, pcurve.last, reverse)?;
    let origin = [line.location().x(), line.location().y()];
    let direction = [line.direction().x(), line.direction().y()];
    Some(parameters.map(|t| {
        std::array::from_fn(|axis| Span::at(origin[axis]) + Span::at(t) * Span::at(direction[axis]))
    }))
}

fn line_points(
    edge: &Edge,
    reverse: bool,
    reference: [f64; 3],
    index: usize,
) -> Result<[Point; 2], Failure> {
    let stored = native(BRepTool::curve_with_location(edge), Stage::Sweep)?;
    let parameters = ordered_parameters(stored.first, stored.last, reverse);
    let (Some(line), Some(parameters)) = (stored.curve.downcast_ref::<GeomLine>(), parameters)
    else {
        return Err(unbound(Some(index), "a finite affine 3D trim"));
    };
    let line = line.lin();
    let transform = stored.location.transformation();
    let origin = placed(
        line.location().coordinates().map(Span::at),
        transform,
        reference,
    );
    let direction = mapped_vector(line.direction().coordinates().map(Span::at), transform);
    Ok(
        parameters
            .map(|t| std::array::from_fn(|axis| origin[axis] + Span::at(t) * direction[axis])),
    )
}

fn ordered_parameters(first: f64, last: f64, reverse: bool) -> Option<[f64; 2]> {
    (first.is_finite() && last.is_finite() && first < last).then_some(if reverse {
        [last, first]
    } else {
        [first, last]
    })
}

fn flux(trims: &[Trim], index: usize) -> Result<Span, Failure> {
    let [first, _, _, ..] = trims else {
        return Err(unbound(Some(index), "at least three affine trims per face"));
    };
    let outgoing = unique_outgoing(trims, index)?;
    let incomplete = || {
        unbound(
            Some(index),
            "a single closed directed vertex incidence cycle",
        )
    };
    let mut total = Span::ZERO;
    let mut current = 0;
    for visited in 1..=trims.len() {
        let trim = &trims[current];
        let successor = outgoing
            .find_index(&trim.vertices[1])
            .checked_sub(1)
            .ok_or_else(incomplete)?;
        if (successor == 0) != (visited == trims.len()) {
            return Err(incomplete());
        }
        let next = &trims[successor];
        // The pcurve endpoints and shared topological vertices are inside the trace
        // envelopes. Include the possible closure connector between the two.
        total = total + determinant(first.start, trim.start, trim.end);
        total = total + determinant(first.start, trim.end, next.start).hull(Span::ZERO);
        current = successor;
    }
    Ok(total)
}

fn unique_outgoing(trims: &[Trim], index: usize) -> Result<IndexedMapOfShape, Failure> {
    let mut outgoing = IndexedMapOfShape::default();
    for (ordinal, trim) in trims.iter().enumerate() {
        if outgoing.add(trim.vertices[0].to_shape()) != ordinal + 1 {
            return Err(unbound(
                Some(index),
                "exactly one outgoing trim at each native vertex",
            ));
        }
    }
    Ok(outgoing)
}

#[cfg(test)]
#[path = "face_tests.rs"]
mod tests;
