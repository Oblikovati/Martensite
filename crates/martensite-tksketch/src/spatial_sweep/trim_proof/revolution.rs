// SPDX-License-Identifier: GPL-2.0-only
//! Integrate the actual linear meridian and precisely owned rectangular UV domain.
use super::trim::Trim;
use super::{
    FaceProof, Failure, Point, Span, Stage, Trig, coefficients, dot, inverse, offset,
    plane::{coefficient_error, endpoint_error},
    point_add, poll, scaled, trim,
};
use crate::{
    spatial_prism::volume::affine_curve,
    spatial_sweep::{admission::Axis, geometry::cardinal},
};
use martensite_tkbrep::brep::EdgeCurve3D;
use martensite_tkg3d::geom::{
    GeomRectangularTrimmedSurface, GeomSurface, GeomSurfaceOfRevolution, GeomSweptSurface,
};

pub(super) fn flux(
    trimmed: &GeomRectangularTrimmedSurface,
    trims: &[Trim],
    proof: FaceProof,
    trig: &mut Trig,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Span, Failure> {
    let patch = Patch::decode(trimmed, proof)?;
    let sign = boundary(trimmed.bounds(), &patch, trims, trig, proof, cancelled)?;
    let [c0, s0] = coefficients(trig, patch.angle[0])?;
    let [c1, s1] = coefficients(trig, patch.angle[1])?;
    let moments = [
        s1 - s0,
        c0 - c1,
        Span::at(patch.angle[1]) - Span::at(patch.angle[0]),
    ];
    Ok(sign * patch.integral(moments))
}

pub(super) fn verify_model(
    trimmed: &GeomRectangularTrimmedSurface,
    trims: &[Trim],
    proof: FaceProof,
    trig: &mut Trig,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), Failure> {
    let patch = Patch::decode(trimmed, proof)?;
    boundary(trimmed.bounds(), &patch, trims, trig, proof, cancelled)?;
    Ok(())
}

struct Patch {
    axis: Axis,
    center: Point,
    meridian: [Point; 2],
    angle: [f64; 2],
}

impl Patch {
    fn decode(trimmed: &GeomRectangularTrimmedSurface, proof: FaceProof) -> Result<Self, Failure> {
        let expected = || {
            proof.refuse(
                "one actual cardinal quarter-revolution carrier of a literal affine meridian",
            )
        };
        let (u0, u1, v0, v1) = trimmed.bounds();
        if u0 != 0. || !u1.is_finite() || u1 <= 0. || u1 > std::f64::consts::PI {
            return Err(expected());
        }
        let base = trimmed.basis_surface();
        let carrier = base
            .downcast_ref::<GeomSurfaceOfRevolution>()
            .filter(|surface| !surface.has_eval_representation())
            .ok_or_else(expected)?;
        let axis = carrier.axis();
        let center = offset(
            axis.location().coordinates().map(Span::at),
            proof.reference.map(Span::at),
        );
        let axis = cardinal(axis.direction()).ok_or_else(expected)?;
        let linear = EdgeCurve3D {
            curve: carrier.basis_curve(),
            first: v0,
            last: v1,
            ..Default::default()
        };
        let meridian =
            affine_curve::spatial(&linear, false, true, proof.reference).ok_or_else(expected)?;
        Ok(Self {
            axis,
            center,
            meridian,
            angle: [u0, u1],
        })
    }

    fn section(&self, v: usize) -> [Point; 3] {
        let radial = self.radial(v);
        let mut center = self.center;
        center[self.axis.index] = self.meridian[v][self.axis.index];
        [center, radial, axis_cross(self.axis, radial)]
    }

    fn radial(&self, v: usize) -> Point {
        let mut radial = offset(self.meridian[v], self.center);
        radial[self.axis.index] = Span::ZERO;
        radial
    }

    fn integral(&self, moments: [Span; 3]) -> Span {
        let [first, last] = [self.radial(0), self.radial(1)];
        let middle = scaled(point_add(first, last), Span::at(0.5));
        let radial_rate = offset(last, first);
        let height = self
            .meridian
            .map(|point| Span::at(self.axis.sign) * point[self.axis.index]);
        let rate = height[1] - height[0];
        let middle_height = Span::at(0.5) * (height[0] + height[1]);
        let [a, b, c] = [
            (first, height[0]),
            (middle, middle_height),
            (last, height[1]),
        ]
        .map(|(radial, height)| self.strip(radial, height, radial_rate, rate, moments));
        // The V integrand has degree at most two. Simpson's weights are exact.
        (a + Span::at(4.) * b + c) * inverse(6.)
    }

    fn strip(
        &self,
        radial: Point,
        height: Span,
        step: Point,
        rate: Span,
        [cosine, sine, width]: [Span; 3],
    ) -> Span {
        let rotation = point_add(
            scaled(radial, cosine),
            scaled(axis_cross(self.axis, radial), sine),
        );
        let radial_flux = rate * (dot(self.center, rotation) + width * dot(radial, radial));
        radial_flux - width * height * dot(radial, step)
    }
}

fn axis_cross(axis: Axis, radial: Point) -> Point {
    let [i, j, k] = [axis.index, (axis.index + 1) % 3, (axis.index + 2) % 3];
    let mut cross = [Span::ZERO; 3];
    cross[j] = -Span::at(axis.sign) * radial[k];
    cross[k] = Span::at(axis.sign) * radial[j];
    cross[i] = Span::ZERO;
    cross
}

fn boundary(
    (u0, u1, v0, v1): (f64, f64, f64, f64),
    patch: &Patch,
    trims: &[Trim],
    trig: &mut Trig,
    proof: FaceProof,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Span, Failure> {
    let expected =
        || proof.refuse("the actual four directed, exact UV iso sides of the revolution rectangle");
    let corners = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]].map(|p| p.map(Span::at));
    let mut edges = Vec::with_capacity(4);
    if trims.len() != 4 {
        return Err(expected());
    }
    for trim in trims {
        poll(cancelled, Stage::Integration)?;
        let uv = trim
            .linear_uv()
            .filter(|_| trim.pcurve.is_stored)
            .ok_or_else(expected)?;
        let [Some(first), Some(last)] = uv.map(|p| corners.iter().position(|corner| *corner == p))
        else {
            return Err(expected());
        };
        if ![1, 3].contains(&((last + 4 - first) % 4)) {
            return Err(expected());
        }
        edges.push([first, last]);
    }
    let positive = (edges[0][0] + 1) % 4 == edges[0][1];
    let step = if positive { 1 } else { 3 };
    if edges
        .iter()
        .zip(edges.iter().cycle().skip(1))
        .any(|(a, b)| a[1] != b[0] || (a[0] + step) % 4 != a[1])
    {
        return Err(expected());
    }
    for (trim, edge) in trims.iter().zip(edges) {
        poll(cancelled, Stage::Integration)?;
        compare_active_curve(patch, trim, edge, proof, trig)?;
    }
    Ok(if positive { Span::ONE } else { -Span::ONE })
}

fn compare_active_curve(
    patch: &Patch,
    trim: &Trim,
    corners: [usize; 2],
    proof: FaceProof,
    trig: &mut Trig,
) -> Result<(), Failure> {
    let ports = corners.map(|i| (usize::from(matches!(i, 1 | 2)), usize::from(i >= 2)));
    let a = coefficients(trig, patch.angle[ports[0].0])?;
    let b = coefficients(trig, patch.angle[ports[1].0])?;
    let points = [
        rotate(patch.section(ports[0].1), a),
        rotate(patch.section(ports[1].1), b),
    ];
    trim::vertex_trace(trim, points, proof)?;
    if ports[0].0 == ports[1].0 {
        let line = trim
            .linear_xyz(proof.reference)
            .ok_or_else(|| proof.refuse("an affine native 3D meridian for an active U iso edge"))?;
        return trim::require(
            endpoint_error(points, line),
            proof,
            "coincident native meridian and revolution U iso edge",
        );
    }
    let circle = trim::spatial_circle(trim, proof)?;
    if trim.pcurve.first != patch.angle[0] || trim.pcurve.last != patch.angle[1] {
        return Err(
            proof.refuse("the literal native angular parameter of the retained circular iso edge")
        );
    }
    trim::require(
        coefficient_error(patch.section(ports[0].1), circle),
        proof,
        "same-range circular spatial trim and revolution V iso edge",
    )
}

fn rotate([center, cosine, sine]: [Point; 3], [u, v]: [Span; 2]) -> Point {
    point_add(center, point_add(scaled(cosine, u), scaled(sine, v)))
}
