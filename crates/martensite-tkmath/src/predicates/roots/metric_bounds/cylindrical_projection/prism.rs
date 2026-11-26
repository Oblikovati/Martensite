// SPDX-License-Identifier: GPL-2.0-only
//! Relate the actual canonical planar sweep to the original source parameters.
use super::{
    AxialProjectionWitness, Checked, CylinderProjectionError as Failure, NativeProjectionEdge,
    ProjectionEdgeCertificate, ProjectionPlacement, ProjectionProofLimits, ProjectionSweep,
    affine::{Ratio, Vector, bounds, sub},
    native_net::finite,
    verify::Limits,
};
use crate::{
    gp::{Axis3, Point3},
    predicates::roots::metric_bounds::placed_plane::{ExactPlacement, dot},
};

/// Literal planar carrier and trimmed UV rectangle of the verified native prism.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectionPrismChart {
    /// Original local carrier origin.
    pub origin: [f64; 3],
    /// Original local U, V and nominal normal axes.
    pub axes: [[f64; 3]; 3],
    /// Actual effective location of the carrier.
    pub placement: ProjectionPlacement,
    /// Increasing ranges in original native U and V, respectively.
    pub trim: [[f64; 2]; 2],
}

/// A native planar prism bound to its original source projection witness.
pub struct VerifiedProjectionPrism<'w> {
    witness: &'w AxialProjectionWitness,
    map: PrismMap,
}

impl AxialProjectionWitness {
    /// Verify exact coincidence of the native trim rectangle and original finite sweep.
    /// Example: `let verified = witness.verify_planar_prism(sweep, actual_chart)?;`.
    pub fn verify_planar_prism(
        &self,
        sweep: ProjectionSweep,
        chart: ProjectionPrismChart,
    ) -> Checked<VerifiedProjectionPrism<'_>> {
        if sweep != self.sweep()? {
            return Err(Failure::Prism);
        }
        let trim = trim_ranges(chart.trim)?;
        let world = WorldPrism::capture(chart)?;
        world.verify_corners(self, sweep, &trim)?;
        Ok(VerifiedProjectionPrism {
            witness: self,
            map: PrismMap::from_world(self, world, trim),
        })
    }
}

impl VerifiedProjectionPrism<'_> {
    /// Enclose the original source parameter and signed travel for a native prism UV point.
    /// Example: `let [source, travel] = prism.source_coordinates([0.0, -1.0])?;`.
    pub fn source_coordinates(&self, uv: [f64; 2]) -> Checked<[[f64; 2]; 2]> {
        let [source, travel] = self.map.coordinates(finite(&uv)?)?;
        Ok([bounds(&source)?, bounds(&travel)?])
    }

    /// Bind actual native Section edges to their unique original source intervals.
    /// Example: `prism.certify_edges(&edges, limits, || false)?`.
    pub fn certify_edges(
        &self,
        candidates: &[NativeProjectionEdge],
        limits: ProjectionProofLimits,
        mut cancelled: impl FnMut() -> bool,
    ) -> Checked<Vec<(usize, ProjectionEdgeCertificate)>> {
        let limits = Limits::capture(limits)?;
        super::evidence::certify(self.witness, &self.map, candidates, &limits, &mut cancelled)
    }
}

struct WorldPrism {
    origin: Vector,
    axes: [Vector; 2],
}

impl WorldPrism {
    fn capture(chart: ProjectionPrismChart) -> Checked<Self> {
        let [u, v, normal] = chart.axes;
        Axis3::from_stored_coefficients(
            Point3::new(chart.origin[0], chart.origin[1], chart.origin[2]),
            normal,
            u,
            v,
        )
        .map_err(|_| Failure::Prism)?;
        let placement = ExactPlacement::capture(chart.placement)?;
        Ok(Self {
            origin: placement.point(finite(&chart.origin)?),
            axes: [placement.vector(finite(&u)?), placement.vector(finite(&v)?)],
        })
    }

    fn point(&self, u: &Ratio, v: &Ratio) -> Vector {
        std::array::from_fn(|i| {
            self.origin[i]
                .add(&self.axes[0][i].multiply(u))
                .add(&self.axes[1][i].multiply(v))
        })
    }

    fn verify_corners(
        &self,
        witness: &AxialProjectionWitness,
        sweep: ProjectionSweep,
        trim: &[[Ratio; 2]; 2],
    ) -> Checked<()> {
        let start = Ratio::from_finite(sweep.start);
        let end = start.add(&Ratio::from_finite(sweep.length));
        let [a, b] = witness.source.range.each_ref();
        let mut expected = corners([a, b], [&start, &end], |t, l| witness.position(t, l));
        let mut actual = corners(trim[0].each_ref(), trim[1].each_ref(), |u, v| {
            self.point(u, v)
        });
        expected.sort_unstable();
        actual.sort_unstable();
        if expected != actual {
            return Err(Failure::Prism);
        }
        Ok(())
    }
}

fn corners(
    [a, b]: [&Ratio; 2],
    [c, d]: [&Ratio; 2],
    point: impl Fn(&Ratio, &Ratio) -> Vector,
) -> [Vector; 4] {
    [point(a, c), point(a, d), point(b, c), point(b, d)]
}

fn trim_ranges(trim: [[f64; 2]; 2]) -> Checked<[[Ratio; 2]; 2]> {
    if trim
        .iter()
        .any(|[lo, hi]| !lo.is_finite() || !hi.is_finite() || lo >= hi)
    {
        return Err(Failure::Prism);
    }
    Ok(trim.map(|range| range.map(Ratio::from_finite)))
}

pub(super) struct PrismMap {
    source: [Ratio; 3],
    travel: [Ratio; 3],
    trim: [[Ratio; 2]; 2],
}

impl PrismMap {
    fn from_world(
        witness: &AxialProjectionWitness,
        world: WorldPrism,
        trim: [[Ratio; 2]; 2],
    ) -> Self {
        let delta = sub(&world.origin, &witness.source.origin);
        let coefficients = [&delta, &world.axes[0], &world.axes[1]];
        let axis = &witness.chart.axes[2];
        let axial_derivative = &witness.source.height[1];
        Self {
            source: coefficients.map(|vector| dot(axis, vector).divide(axial_derivative)),
            travel: coefficients.map(|vector| dot(&witness.direction, vector)),
            trim,
        }
    }

    pub(super) fn coordinates(&self, uv: [Ratio; 2]) -> Checked<[Ratio; 2]> {
        if uv.iter().zip(&self.trim).any(|(x, [a, b])| x < a || x > b) {
            return Err(Failure::Candidate);
        }
        Ok([
            affine_value(&self.source, &uv),
            affine_value(&self.travel, &uv),
        ])
    }
}

fn affine_value([origin, u, v]: &[Ratio; 3], [a, b]: &[Ratio; 2]) -> Ratio {
    origin.add(&u.multiply(a)).add(&v.multiply(b))
}
