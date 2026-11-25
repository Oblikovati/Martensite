// SPDX-License-Identifier: GPL-2.0-only
use super::interval::{Point, Span, determinant, l1_difference, mapped_vector, placed};
use super::{
    FluxMode, affine_curve,
    bilinear::{Bilinear, oriented_flux},
    unbound, vertex_point,
};
use crate::spatial_prism::{
    SpatialPrismFailure as Failure, SpatialPrismStage as Stage, failure::native, poll,
};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::{self, Explorer},
    topods::{self, Edge, Face, Vertex},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::{
    geom::{GeomBSplineSurface, GeomPlane},
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::gp::{Axis3, Transform3};

pub(super) fn contribution(
    face: &Face,
    index: usize,
    reference: [f64; 3],
    limit: f64,
    mode: FluxMode,
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
    let carrier = carrier(&forward, reference, mode, index)?;
    let edges = native(
        Explorer::new(&outer, ShapeEnum::Edge, ShapeEnum::Shape),
        Stage::Sweep,
    )?;
    let trims = edges
        .map(|edge| {
            poll(cancelled, Stage::Sweep)?;
            let edge = native(topods::edge(&edge), Stage::Sweep)?;
            decode_trim(
                &edge,
                &forward,
                TrimContext {
                    carrier,
                    reference,
                    limit,
                    mode,
                    index,
                },
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let trace = trims.iter().map(|trim| trim.trace).fold(0., f64::max);
    let cycle = ordered_trims(&trims, index)?;
    let flux = match carrier {
        Carrier::Plane(_) => planar_flux(&cycle),
        Carrier::Bilinear(patch) => bilinear_flux(patch, &cycle, index)?,
    };
    Ok((sign * flux, trace))
}

#[derive(Clone, Copy)]
struct PlaneMap {
    basis: Axis3,
    placement: Transform3,
    reference: [f64; 3],
}

#[derive(Clone, Copy)]
enum Carrier {
    Plane(PlaneMap),
    Bilinear(Bilinear),
}

fn carrier(
    face: &Face,
    reference: [f64; 3],
    mode: FluxMode,
    index: usize,
) -> Result<Carrier, Failure> {
    let (stored, location) = native(BRepTool::surface_with_location(face), Stage::Sweep)?;
    let placement = location.transformation();
    if let Some(plane) = stored.downcast_ref::<GeomPlane>() {
        return Ok(Carrier::Plane(PlaneMap {
            basis: plane.pln().position(),
            placement,
            reference,
        }));
    }
    if mode == FluxMode::Ruled
        && let Some(patch) = stored.downcast_ref::<GeomBSplineSurface>()
        && let Some(parsed) = Bilinear::decode(patch, placement, reference)
    {
        return Ok(Carrier::Bilinear(parsed));
    }
    Err(unbound(
        Some(index),
        "a decoded planar or admitted literal ruled face",
    ))
}

type TrimPoints = ([Point; 2], Option<[usize; 2]>);

impl Carrier {
    fn trim(self, uv: [[Span; 2]; 2], index: usize) -> Result<TrimPoints, Failure> {
        match self {
            Self::Plane(plane) => Ok((uv.map(|point| plane.uv(point)), None)),
            Self::Bilinear(patch) => {
                let [Some(first), Some(last)] = uv.map(|point| patch.corner(point)) else {
                    return Err(unbound(
                        Some(index),
                        "literal UV corner endpoints of the ruled cell",
                    ));
                };
                if (first + 1) % 4 != last && (last + 1) % 4 != first {
                    return Err(unbound(
                        Some(index),
                        "a full directed iso side of the ruled cell",
                    ));
                }
                Ok(([patch.poles[first], patch.poles[last]], Some([first, last])))
            }
        }
    }
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
    corners: Option<[usize; 2]>,
}

struct TrimContext {
    carrier: Carrier,
    reference: [f64; 3],
    limit: f64,
    mode: FluxMode,
    index: usize,
}

fn decode_trim(edge: &Edge, face: &Face, context: TrimContext) -> Result<Trim, Failure> {
    let TrimContext {
        carrier,
        reference,
        limit,
        mode,
        index,
    } = context;
    let incomplete = |expected| unbound(Some(index), expected);
    let reverse = match edge.orientation() {
        Orientation::Forward => false,
        Orientation::Reversed => true,
        _ => return Err(incomplete("an oriented trim occurrence")),
    };
    let pcurve = native(BRepTool::curve_on_face(edge, face), Stage::Sweep)?;
    let degree_one = mode == FluxMode::Ruled;
    let uv = affine_curve::planar(&pcurve, reverse, degree_one)
        .ok_or_else(|| incomplete("a finite affine pcurve trim"))?;
    let (mapped, corners) = carrier.trim(uv, index)?;
    let curve = native(BRepTool::curve_with_location(edge), Stage::Sweep)?;
    let line = affine_curve::spatial(&curve, reverse, degree_one, reference)
        .ok_or_else(|| incomplete("a finite affine 3D trim"))?;
    let (start, end) = topexp::vertices(edge, true);
    let vertices = [start, end];
    let mut joined = mapped;
    let mut trace = 0.0_f64;
    for (i, vertex) in vertices.iter().enumerate() {
        let point = vertex_point(vertex, reference)?;
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
        corners,
    })
}

fn ordered_trims(trims: &[Trim], index: usize) -> Result<Vec<&Trim>, Failure> {
    if trims.len() < 3 {
        return Err(unbound(Some(index), "at least three affine trims per face"));
    }
    let outgoing = unique_outgoing(trims, index)?;
    let incomplete = || {
        unbound(
            Some(index),
            "a single closed directed vertex incidence cycle",
        )
    };
    let mut result = Vec::with_capacity(trims.len());
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
        result.push(trim);
        current = successor;
    }
    Ok(result)
}

fn planar_flux(trims: &[&Trim]) -> Span {
    let first = trims[0].start;
    let mut total = Span::ZERO;
    for (trim, next) in trims.iter().zip(trims.iter().cycle().skip(1)) {
        // Include the potential closing segment at every shared native vertex.
        total = total + determinant(first, trim.start, trim.end);
        total = total + determinant(first, trim.end, next.start).hull(Span::ZERO);
    }
    total
}

fn bilinear_flux(patch: Bilinear, trims: &[&Trim], index: usize) -> Result<Span, Failure> {
    let incomplete = || unbound(Some(index), "one consistently oriented exact UV rectangle");
    if trims.len() != 4 {
        return Err(incomplete());
    }
    let [first, last] = trims[0].corners.ok_or_else(incomplete)?;
    let forward = (first + 1) % 4 == last;
    let mut corner = first;
    let mut poles = patch.poles;
    for trim in trims {
        let next = (corner + if forward { 1 } else { 3 }) % 4;
        if trim.corners != Some([corner, next]) {
            return Err(incomplete());
        }
        for (index, endpoint) in [(corner, trim.start), (next, trim.end)] {
            poles[index] = std::array::from_fn(|axis| poles[index][axis].hull(endpoint[axis]));
        }
        corner = next;
    }
    Ok(oriented_flux(poles, forward))
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
