// SPDX-License-Identifier: GPL-2.0-only
//! Green's theorem on the final plane's actual affine and circular trim spans.
use super::{
    FaceProof, Failure, Point, Span, Stage, Trig, coefficients, native, point_add, poll, scaled,
    trim,
};
use super::{model::PlanarModel, trim::Trim};
use crate::spatial_prism::volume::interval::{determinant, l1_difference};
use martensite_tkg2d::geom2d::Geom2dCircle;
use martensite_tkmath::gp::Axis3;
use std::f64::consts::FRAC_PI_2;

type Uv = [Span; 2];

pub(super) fn flux(
    frame: Axis3,
    trims: &[Trim],
    proof: FaceProof,
    trig: &mut Trig,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Span, Failure> {
    let map = Placement::new(frame, proof.reference);
    let edges = decode(trims, map, proof, trig, cancelled)?;
    let integral = twice_area(&edges, map, proof)?;
    Ok(Span::at(0.5) * integral * determinant(map.origin, map.u, map.v))
}

pub(super) fn verify_model(
    frame: Axis3,
    trims: &[Trim],
    model: PlanarModel,
    proof: FaceProof,
    trig: &mut Trig,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Failure> {
    let map = Placement::new(frame, proof.reference);
    let edges = decode(trims, map, proof, trig, cancelled)?;
    twice_area(&edges, map, proof)?;
    match model {
        PlanarModel::Band(points) => verify_band(&edges, map, &points, proof),
        PlanarModel::Corner {
            center,
            before,
            after,
        } => verify_corner(&edges, map, center, before, after, proof),
    }
}

fn decode(
    trims: &[Trim],
    map: Placement,
    proof: FaceProof,
    trig: &mut Trig,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Vec<Boundary>, Failure> {
    trims
        .iter()
        .map(|trim| {
            poll(cancelled, Stage::Integration)?;
            boundary(trim, map, proof, trig)
        })
        .collect()
}

struct Boundary {
    ends: [Uv; 2],
    integral: Span,
    arc: Option<ArcTrace>,
}

struct ArcTrace {
    coefficients: [Point; 3],
    angle: Span,
}

fn boundary(
    trim: &Trim,
    map: Placement,
    proof: FaceProof,
    trig: &mut Trig,
) -> Result<Boundary, Failure> {
    if let Some(uv) = trim.linear_uv() {
        let points = uv.map(|uv| map.position(uv));
        let spatial = trim.linear_xyz(proof.reference).ok_or_else(|| {
            proof.refuse("a literal affine spatial trim paired with its planar affine pcurve")
        })?;
        trim::require(
            endpoint_error(points, spatial),
            proof,
            "coincident affine planar and spatial active trims",
        )?;
        trim::vertex_trace(trim, points, proof)?;
        return Ok(Boundary {
            ends: uv,
            integral: cross(uv[0], uv[1]),
            arc: None,
        });
    }
    circular_boundary(trim, map, proof, trig)
}

fn circular_boundary(
    trim: &Trim,
    map: Placement,
    proof: FaceProof,
    trig: &mut Trig,
) -> Result<Boundary, Failure> {
    let circle = trim
        .pcurve
        .pcurve
        .downcast_ref::<Geom2dCircle>()
        .ok_or_else(|| proof.refuse("a literal circular or affine trim on the planar face"))?;
    let circle = native(circle.circ2d(), Stage::Integration)?;
    let c = [circle.location().x(), circle.location().y()].map(Span::at);
    let radius = Span::at(circle.radius());
    let [a, b] = [circle.x_axis().direction(), circle.y_axis().direction()]
        .map(|axis| [axis.x(), axis.y()].map(|value| Span::at(value) * radius));
    let [first, last] = trim.directed_range();
    let directions = [coefficients(trig, first)?, coefficients(trig, last)?];
    let endpoints = directions
        .map(|[cosine, sine]| std::array::from_fn(|i| c[i] + a[i] * cosine + b[i] * sine));
    let actual = trim::spatial_circle(trim, proof)?;
    let model = [map.position(c), map.vector(a), map.vector(b)];
    trim::require(
        coefficient_error(model, actual),
        proof,
        "same-range analytic circular pcurve and actual spatial trim",
    )?;
    trim::vertex_trace(trim, endpoints.map(|p| map.position(p)), proof)?;
    let delta = Span::at(last) - Span::at(first);
    let integral = cross(c, subtract(endpoints[1], endpoints[0])) + cross(a, b) * delta;
    let [cosine, sine] = directions[0];
    let [center, a, b] = model;
    let radial = point_add(scaled(a, cosine), scaled(b, sine));
    let turn = point_add(scaled(a, -sine), scaled(b, cosine));
    let (turn, angle) = if last > first {
        (turn, delta)
    } else {
        (scaled(turn, -Span::ONE), -delta)
    };
    let arc = ArcTrace {
        coefficients: [center, radial, turn],
        angle,
    };
    Ok(Boundary {
        ends: endpoints,
        integral,
        arc: Some(arc),
    })
}

fn twice_area(edges: &[Boundary], map: Placement, proof: FaceProof) -> Result<Span, Failure> {
    let mut sum = Span::ZERO;
    for (edge, next) in edges.iter().zip(edges.iter().cycle().skip(1)) {
        let jump = subtract(next.ends[0], edge.ends[1]);
        let jump_error = l1_difference(map.vector(jump), [Span::ZERO; 3]);
        trim::require(
            jump_error,
            proof,
            "a continuous retained planar boundary within the spatial trace limit",
        )?;
        // Enclose the numerical tolerance gap by including or omitting its closing chord.
        let bridge = cross(edge.ends[1], next.ends[0]).hull(Span::ZERO);
        sum = sum + edge.integral + bridge;
    }
    if !sum.finite() || sum.lo <= 0. {
        return Err(proof.refuse("one positively oriented nonempty outer planar trim"));
    }
    Ok(sum)
}

fn verify_band(
    edges: &[Boundary],
    map: Placement,
    expected: &[Point],
    proof: FaceProof,
) -> Result<(), Failure> {
    if edges.len() != expected.len() || edges.iter().any(|edge| edge.arc.is_some()) {
        return Err(proof.refuse("the full source-clipped affine band boundary"));
    }
    let count = expected.len();
    let mut matches = 0;
    let mut best = f64::INFINITY;
    for offset in 0..count {
        for step in [1, count - 1] {
            let bounds = edges.iter().enumerate().map(|(i, edge)| {
                let [a, b] = [i, i + 1].map(|j| expected[(offset + j * step) % count]);
                endpoint_error(edge.ends.map(|uv| map.position(uv)), [a, b])
            });
            let max = bounds.fold(0., f64::max);
            best = best.min(max);
            matches += usize::from(max <= proof.limit);
        }
    }
    trim::require(
        best,
        proof,
        "native planar band includes its entire source-clipped boundary",
    )?;
    if matches != 1 {
        return Err(proof.refuse("one unambiguous geometric cycle for the already named band"));
    }
    Ok(())
}

fn verify_corner(
    edges: &[Boundary],
    map: Placement,
    center: Point,
    before: Point,
    after: Point,
    proof: FaceProof,
) -> Result<(), Failure> {
    let arc_index = edges.iter().position(|edge| edge.arc.is_some());
    let Some(arc_index) = arc_index.filter(|_| edges.len() == 3) else {
        return Err(
            proof.refuse("two radial sides and one actual circular outside profile boundary")
        );
    };
    let sector = &edges[arc_index];
    let leading = &edges[(arc_index + 1) % 3];
    let trailing = &edges[(arc_index + 2) % 3];
    let Some(arc) = sector
        .arc
        .as_ref()
        .filter(|_| leading.arc.is_none() && trailing.arc.is_none())
    else {
        return Err(proof.refuse("one actual quarter arc and two affine radial sides"));
    };
    let error =
        |radial, tangent| sector_error(map, arc, [leading, trailing], [center, radial, tangent]);
    let first = error(before, after);
    let second = error(after, before);
    trim::require(
        first.min(second),
        proof,
        "complete planar quarter-sector on the original outer profile",
    )?;
    if first.max(second) <= proof.limit {
        return Err(proof.refuse("one unambiguous decoded direction of the planar corner arc"));
    }
    Ok(())
}

fn sector_error(
    map: Placement,
    arc: &ArcTrace,
    [leading, trailing]: [&Boundary; 2],
    [center, start, end]: [Point; 3],
) -> f64 {
    let first = endpoint_error(
        leading.ends.map(|uv| map.position(uv)),
        [point_add(center, end), center],
    );
    let last = endpoint_error(
        trailing.ends.map(|uv| map.position(uv)),
        [center, point_add(center, start)],
    );
    let coefficients = coefficient_error(arc.coefficients, [center, start, end]);
    let radius = l1_difference(start, [Span::ZERO; 3]).max(l1_difference(end, [Span::ZERO; 3]));
    let quarter = Span {
        lo: FRAC_PI_2.next_down(),
        hi: FRAC_PI_2.next_up(),
    };
    let swept = Span::at((arc.angle - quarter).magnitude()) * Span::at(radius);
    let circular = (Span::at(coefficients) + swept).hi;
    first.max(last).max(circular)
}

#[derive(Clone, Copy)]
struct Placement {
    origin: Point,
    u: Point,
    v: Point,
}

impl Placement {
    fn new(frame: Axis3, reference: [f64; 3]) -> Self {
        Self {
            origin: std::array::from_fn(|i| {
                Span::at(frame.location().coordinates()[i]) - Span::at(reference[i])
            }),
            u: frame.x_direction().coordinates().map(Span::at),
            v: frame.y_direction().coordinates().map(Span::at),
        }
    }

    fn position(self, uv: Uv) -> Point {
        point_add(self.origin, self.vector(uv))
    }

    fn vector(self, [u, v]: Uv) -> Point {
        point_add(scaled(self.u, u), scaled(self.v, v))
    }
}

fn cross([a, b]: Uv, [c, d]: Uv) -> Span {
    a * d - b * c
}

fn subtract(a: Uv, b: Uv) -> Uv {
    [a[0] - b[0], a[1] - b[1]]
}

pub(super) fn endpoint_error(a: [Point; 2], b: [Point; 2]) -> f64 {
    a.into_iter()
        .zip(b)
        .map(|(a, b)| l1_difference(a, b))
        .fold(0., f64::max)
}

pub(super) fn coefficient_error(a: [Point; 3], b: [Point; 3]) -> f64 {
    a.into_iter()
        .zip(b)
        .map(|(a, b)| Span::at(l1_difference(a, b)))
        .fold(Span::ZERO, |sum, error| sum + error)
        .hi
}
