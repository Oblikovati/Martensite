// SPDX-License-Identifier: GPL-2.0-only
//! Full active-interval readback for native prism and Section representations.
use super::{
    AxialProjectionLine, AxialProjectionWitness, Checked, CylinderProjectionBranch as Branch,
    CylinderProjectionError as Failure, ProjectionPlacement, ProjectionSourceInterval,
    affine::{Ratio, Vector, bounds, sub, zero},
    native_net::{NativeSample, Net, finite},
    poll,
};
use crate::{
    gp::Direction3,
    predicates::roots::{
        certificate_arithmetic::rounding::outward_sqrt,
        metric_bounds::placed_plane::{ExactPlacement, dot, exact_point, exact_vector},
    },
};
use std::cmp::Ordering;

/// Maximum original pole count consumed from a single native representation.
pub const MAX_PROJECTION_CURVE_POINTS: usize = 128;

/// Actual native Section curve data. The active range belongs to its owning edge.
#[derive(Clone, Debug, PartialEq)]
pub enum NativeProjectionCurve<const N: usize> {
    /// An affine line in its original parameter.
    Line {
        /// Original origin.
        origin: [f64; N],
        /// Original direction.
        direction: [f64; N],
    },
    /// A checked degree-one nonrational spline with no evaluation override.
    DegreeOne {
        /// Original increasing knots, one per pole.
        knots: Vec<f64>,
        /// Original 3D or chart-space poles.
        poles: Vec<[f64; N]>,
    },
}

/// Native positive work limits kept in their original units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectionProofLimits {
    /// Maximum Euclidean transport or restriction residual.
    pub spatial: f64,
    /// Maximum original source parameter endpoint difference.
    pub source_parameter: f64,
    /// Maximum angular residual in radians on the cylinder.
    pub angle: f64,
}

/// Finite sweep endpoints, outward from all considered signed roots.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectionSweep {
    /// Request's chosen unit direction.
    pub direction: Direction3,
    /// Original-source travel of the native sweep start.
    pub start: f64,
    /// Nonnegative extent applied in the chosen direction.
    pub length: f64,
}

/// A stored vertex before applying its actual native placement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectionVertex {
    /// Original local point.
    pub point: [f64; 3],
    /// Effective placement of the occurrence.
    pub placement: ProjectionPlacement,
}

/// Literal representations of one returned native Section edge.
#[derive(Clone, Debug, PartialEq)]
pub struct NativeProjectionEdge {
    /// Active range shared by the original representations.
    pub range: [f64; 2],
    /// Actual original 3D Section edge.
    pub spatial: NativeProjectionCurve<3>,
    /// The effective location of the spatial representation.
    pub placement: ProjectionPlacement,
    /// Actual retained pcurve on the ruled prism (U is the original source parameter).
    pub prism: NativeProjectionCurve<2>,
    /// Actual first retained pcurve on the target cylinder.
    pub target: NativeProjectionCurve<2>,
    /// Actual second retained pcurve where the result is a native seam.
    pub second_target: Option<NativeProjectionCurve<2>>,
    /// Actual original topological endpoints in increasing native-parameter order.
    pub vertices: [ProjectionVertex; 2],
}

/// One selected source interval paired with the actual native Section range.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectedProjectionInterval {
    /// Exact source coverage and endpoint inclusion.
    pub source: ProjectionSourceInterval,
    /// Outward Section edge parameter endpoints, directed as the source interval.
    pub section_parameters: [[f64; 2]; 2],
}

/// Original remaining interval endpoint authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectionTrimEndpoint {
    /// Lower native source edge parameter.
    SourceFirst,
    /// Upper native source edge parameter.
    SourceLast,
    /// Lower axial side of the retained target opening.
    HoleLower,
    /// Upper axial side of the retained target opening.
    HoleUpper,
}

/// Whole-interval evidence for an actual retained Section edge.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectionEdgeCertificate {
    /// Stable original cylinder chart branch.
    pub branch: Branch,
    /// Outward exact travel in the requested signed direction.
    pub signed_travel: [f64; 2],
    /// Complete exact source interval for this native edge.
    pub retained: ProjectionSourceInterval,
    /// Proven original endpoint relation in increasing source parameter order.
    pub origins: [ProjectionTrimEndpoint; 2],
    /// Mode-selected subsets of the retained source interval.
    pub selected: Vec<SelectedProjectionInterval>,
    /// Position error bound over the actual active native range.
    pub spatial_error: f64,
    /// Source-parameter endpoint mismatch upper bound.
    pub source_parameter_error: f64,
    /// Actual pcurve angular residual upper bound in radians.
    pub angle_error: f64,
}

impl AxialProjectionWitness {
    /// Select a finite prism extent with strict room beyond every admitted signed root.
    /// Example: `witness.sweep()?`.
    pub fn sweep(&self) -> Checked<ProjectionSweep> {
        let zero = Ratio::from_finite(0.);
        let (lo, hi) = self
            .roots
            .iter()
            .fold((zero.clone(), zero), |(lo, hi), root| {
                (lo.min(root.travel.clone()), hi.max(root.travel.clone()))
            });
        let start = if self.forward_only {
            0.
        } else {
            bounds(&lo.subtract(&self.chart.radius))?[0]
        };
        let length = bounds(
            &hi.add(&self.chart.radius)
                .subtract(&Ratio::from_finite(start)),
        )?[1];
        if length <= 0. {
            return Err(Failure::Prism);
        }
        Ok(ProjectionSweep {
            direction: self.signed_direction,
            start,
            length,
        })
    }

    /// Verify the actual retained extrusion basis and its two native parameter domains.
    /// Example: `witness.verify_prism(sweep, basis, ruled_direction, native_v)?`.
    pub fn verify_prism(
        &self,
        sweep: ProjectionSweep,
        basis: AxialProjectionLine,
        ruled_direction: Direction3,
        native_v: [f64; 2],
    ) -> Checked<()> {
        let placement = ExactPlacement::capture(basis.placement)?;
        let origin = placement.point(exact_point(basis.line.location(), 0)?);
        let slope = placement.vector(exact_vector(*basis.line.direction().coordinates(), 1)?);
        let direction = placement.vector(exact_vector(*ruled_direction.coordinates(), 2)?);
        if sweep != self.sweep()?
            || !basis.range.into_iter().all(f64::is_finite)
            || basis.range.map(Ratio::from_finite) != self.source.range
            || slope != self.source.slope
            || origin != self.position(&Ratio::from_finite(0.), &Ratio::from_finite(sweep.start))
            || direction != self.direction.each_ref().map(Ratio::negated)
            || native_v != [-sweep.length, 0.]
        {
            return Err(Failure::Prism);
        }
        Ok(())
    }

    /// Bind native Section edges to unique source intervals and return each actual input index.
    /// Example: `witness.certify_native_edges(sweep, &edges, limits, || false)?`.
    pub fn certify_native_edges(
        &self,
        sweep: ProjectionSweep,
        candidates: &[NativeProjectionEdge],
        limits: ProjectionProofLimits,
        mut cancelled: impl FnMut() -> bool,
    ) -> Checked<Vec<(usize, ProjectionEdgeCertificate)>> {
        let limits = Limits::capture(limits)?;
        if sweep != self.sweep()? {
            return Err(Failure::Prism);
        }
        super::evidence::certify(self, sweep, candidates, &limits, &mut cancelled)
    }

    pub(super) fn position(&self, t: &Ratio, travel: &Ratio) -> Vector {
        std::array::from_fn(|axis| {
            self.source.origin[axis]
                .add(&self.source.slope[axis].multiply(t))
                .add(&self.direction[axis].multiply(travel))
        })
    }
}

pub(super) struct Limits {
    pub spatial: Ratio,
    pub source: Ratio,
    pub angle: Ratio,
}

impl Limits {
    fn capture(limits: ProjectionProofLimits) -> Checked<Self> {
        let values = [limits.spatial, limits.source_parameter, limits.angle];
        if values
            .into_iter()
            .any(|value| !value.is_finite() || value < 0.)
        {
            return Err(Failure::FittedLimits);
        }
        let [spatial, source, angle] = values.map(Ratio::from_finite);
        Ok(Self {
            spatial,
            source,
            angle,
        })
    }
}

pub(super) struct Readback {
    pub samples: Vec<NativeSample>,
    pub vertex_error: Ratio,
    pub forward: bool,
}

impl Readback {
    pub fn capture(
        candidate: &NativeProjectionEdge,
        limits: &Limits,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Checked<Self> {
        let range = finite(&candidate.range)?;
        if range[0] >= range[1] {
            return Err(Failure::Candidate);
        }
        let nets = ReadbackNets::capture(candidate)?;
        let cuts = nets.cuts(&range)?;
        let mut samples = Vec::with_capacity(cuts.len());
        for parameter in cuts {
            poll(cancelled)?;
            samples.push(nets.sample(parameter)?);
        }
        let [first, last] = [
            samples.first().ok_or(Failure::Candidate)?,
            samples.last().ok_or(Failure::Candidate)?,
        ];
        let direction = first.prism[0].cmp(&last.prism[0]);
        if direction == Ordering::Equal
            || samples
                .windows(2)
                .any(|pair| pair[0].prism[0].cmp(&pair[1].prism[0]) != direction)
        {
            return Err(Failure::Candidate);
        }
        let vertex_error = vertex_error(first, last, &candidate.vertices)?;
        if vertex_error > limits.spatial.multiply(&limits.spatial) {
            return Err(Failure::Candidate);
        }
        Ok(Self {
            samples,
            vertex_error,
            forward: direction == Ordering::Less,
        })
    }
}

struct ReadbackNets {
    spatial: Net<3>,
    prism: Net<2>,
    target: Net<2>,
    second_target: Option<Net<2>>,
}

impl ReadbackNets {
    fn capture(candidate: &NativeProjectionEdge) -> Checked<Self> {
        let mut spatial = Net::capture(&candidate.spatial)?;
        spatial.place(&ExactPlacement::capture(candidate.placement)?);
        Ok(Self {
            spatial,
            prism: Net::capture(&candidate.prism)?,
            target: Net::capture(&candidate.target)?,
            second_target: candidate
                .second_target
                .as_ref()
                .map(Net::capture)
                .transpose()?,
        })
    }

    fn cuts(&self, range: &[Ratio; 2]) -> Checked<Vec<Ratio>> {
        let mut cuts = range.to_vec();
        self.spatial.add_cuts(&mut cuts, range)?;
        self.prism.add_cuts(&mut cuts, range)?;
        self.target.add_cuts(&mut cuts, range)?;
        if let Some(second) = &self.second_target {
            second.add_cuts(&mut cuts, range)?;
        }
        cuts.sort_unstable();
        cuts.dedup();
        Ok(cuts)
    }

    fn sample(&self, parameter: Ratio) -> Checked<NativeSample> {
        Ok(NativeSample {
            position: self.spatial.at(&parameter)?,
            prism: self.prism.at(&parameter)?,
            target: self.target.at(&parameter)?,
            second_target: self
                .second_target
                .as_ref()
                .map(|curve| curve.at(&parameter))
                .transpose()?,
            parameter,
        })
    }
}

fn vertex_error(
    first: &NativeSample,
    last: &NativeSample,
    vertices: &[ProjectionVertex; 2],
) -> Checked<Ratio> {
    let metric = |sample: &NativeSample, vertex: ProjectionVertex| -> Checked<Ratio> {
        let p = ExactPlacement::capture(vertex.placement)?.point(finite(&vertex.point)?);
        let delta = sub(&sample.position, &p);
        Ok(dot(&delta, &delta))
    };
    Ok(metric(first, vertices[0])?.max(metric(last, vertices[1])?))
}

pub(super) fn abs(value: Ratio) -> Ratio {
    if value.sign() == Ordering::Less {
        value.negated()
    } else {
        value
    }
}

pub(super) fn sqrt_bound(value: &Ratio) -> Checked<f64> {
    if zero(value) {
        return Ok(0.);
    }
    outward_sqrt(value).ok_or(Failure::Unrepresentable)
}
