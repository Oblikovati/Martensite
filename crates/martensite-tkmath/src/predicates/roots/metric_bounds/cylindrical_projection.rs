// SPDX-License-Identifier: GPL-2.0-only
//! Exact signed projection roots of a line parallel to a trimmed cylindrical wall.
mod affine;
mod boundary;
mod evidence;
mod native_net;
mod prism;
mod roots;
mod trim;
mod verify;

pub use boundary::{check_projection_line_vertices, check_projection_meridian};
pub use prism::{ProjectionPrismChart, VerifiedProjectionPrism};
pub use verify::*;

use self::affine::{AxialSource, Chart, Vector, bounds};
use self::roots::RadialRoot;
use crate::{
    gp::{Cylinder3, Direction3, Line3},
    predicates::NonFiniteCoordinate,
};
use std::{error::Error, fmt};

/// Literal affine native placement rows.
pub type ProjectionPlacement = [[f64; 4]; 3];

/// An original finite affine line at its captured native placement.
#[derive(Clone, Copy, Debug)]
pub struct AxialProjectionLine {
    /// Actual stored native carrier.
    pub line: Line3,
    /// Captured effective placement.
    pub placement: ProjectionPlacement,
    /// Native increasing trim interval.
    pub range: [f64; 2],
}

/// Coordinates of an independently verified rectangular hole in the stored chart.
#[derive(Clone, Copy, Debug)]
pub struct ProjectionHole {
    /// Increasing angular range, strictly away from the periodic seam.
    pub angle: [f64; 2],
    /// Increasing original axial range.
    pub axial: [f64; 2],
}

/// Actual cylinder coefficients and a validated full-turn trimmed wall.
#[derive(Clone, Copy, Debug)]
pub struct ProjectionCylinder {
    /// Actual native circular support.
    pub cylinder: Cylinder3,
    /// Effective native placement.
    pub placement: ProjectionPlacement,
    /// Increasing original axial trim range.
    pub axial: [f64; 2],
    /// Optional independently proven interior hole.
    pub hole: Option<ProjectionHole>,
}

/// Distinct branch selection policies for one axial line on a cylinder.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AxialProjectionMode {
    /// First admitted root along the positive ray.
    Forward(Direction3),
    /// Nearest admitted root in either signed direction.
    TwoSided(Direction3),
    /// Unique global closest point on the trimmed face.
    Closest,
    /// Every regular normal foot on the trimmed face.
    Normal,
}

/// Source-bound identity of a generator in its original periodic chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CylinderProjectionBranch {
    /// The positive U-axis generator, shared by the two original seam uses.
    PositiveU,
    /// The positive V-axis generator.
    PositiveV,
    /// The negative U-axis generator.
    NegativeU,
    /// The negative V-axis generator.
    NegativeV,
}

/// An analytically classified empty projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CylinderProjectionEmpty {
    /// The infinite support has no root on the requested direction.
    NoSupportRoot,
    /// Every admissible support root misses the retained trim.
    TrimMiss,
}

/// Exact root and coverage decisions before entering the native prism and Section.
#[derive(Clone, Debug)]
pub enum CylinderProjectionPlan {
    /// Independently proven completed empty.
    Empty(CylinderProjectionEmpty),
    /// A bounded native candidate set must supply this exact evidence.
    Regular(Box<AxialProjectionWitness>),
}

/// An exact original source interval, including endpoint admission.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectionSourceInterval {
    /// Outward bounds around each increasing endpoint in the original edge parameter.
    pub endpoints: [[f64; 2]; 2],
    /// Whether each exact endpoint is included in the branch selection.
    pub included: [bool; 2],
}

/// One expected support branch and its certified original material intervals.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectionBranchPlan {
    /// Cardinal identity in the original cylinder chart.
    pub branch: CylinderProjectionBranch,
    /// Outward range of the exact signed travel along the prism direction.
    pub signed_travel: [f64; 2],
    /// Exact retained projection intervals before mode selection.
    pub retained: Vec<ProjectionSourceInterval>,
    /// Intervals chosen by the caller's mode.
    pub selected: Vec<ProjectionSourceInterval>,
}

/// Bounded exact source trimmer for one native projection operation.
#[derive(Clone, Debug)]
pub struct AxialProjectionWitness {
    chart: Chart,
    source: AxialSource,
    direction: Vector,
    signed_direction: Direction3,
    roots: Vec<RadialRoot>,
    selected: Vec<trim::SelectedSpan>,
    forward_only: bool,
}

impl AxialProjectionWitness {
    /// Outward certified native branch summaries with the selected source intervals.
    /// Example: `witness.branch_plans()?`.
    pub fn branch_plans(&self) -> Result<Vec<ProjectionBranchPlan>, CylinderProjectionError> {
        self.roots
            .iter()
            .enumerate()
            .map(|(index, root)| self.branch_plan(index, root))
            .collect()
    }

    fn branch_plan(&self, index: usize, root: &RadialRoot) -> Checked<ProjectionBranchPlan> {
        let selected = self
            .selected
            .iter()
            .filter(|span| span.index == index)
            .map(|span| span.interval.enclose())
            .collect::<Checked<_>>()?;
        Ok(ProjectionBranchPlan {
            branch: root.branch,
            signed_travel: bounds(&root.travel)?,
            retained: root
                .retained
                .iter()
                .map(trim::SourceInterval::enclose)
                .collect::<Checked<_>>()?,
            selected,
        })
    }
}

/// Why a checked native projection cannot supply an admitted branch.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CylinderProjectionError {
    /// Nonfinite captured coordinate.
    Coordinate(NonFiniteCoordinate),
    /// The captured angular chart is outside the exact signed orthonormal domain.
    Frame,
    /// The captured radius is nonfinite or not positive.
    Radius(f64),
    /// The supplied range or rectangular trim is outside the supported domain.
    Trim,
    /// The spatial line does not have regular axial transport.
    Source,
    /// The exact support has a regular root outside the first cardinal radial domain.
    Direction,
    /// A support root has zero discriminant.
    Tangent,
    /// A ray coincides with a cylindrical generator or the normal feet are a continuum.
    Overlap,
    /// Root selection is nonunique at this travel.
    RootTie,
    /// Projection begins at a support root.
    RayStartsOnSupport,
    /// A root angle is too close to an actual hole boundary for this domain.
    HoleAngularBoundary,
    /// A retained interval has only a single point.
    PointOnly,
    /// The nearest stationary foot is absent from the trim.
    ClosestTrim,
    /// An actual prism does not preserve the original face generator or range.
    Prism,
    /// Native geometry or its dual stored chart parameterization is inconsistent.
    Candidate,
    /// The native result does not provide exactly one image for each required interval.
    Coverage,
    /// Candidate span work exceeds the declared finite storage limit.
    WorkLimit,
    /// A limit is invalid.
    FittedLimits,
    /// A required exact value has no finite outward enclosure.
    Unrepresentable,
    /// Operation-local cancellation was observed.
    Cancelled,
}

impl fmt::Display for CylinderProjectionError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Coordinate(cause) => cause.fmt(output),
            Self::Radius(actual) => write!(
                output,
                "cylinder radius {actual} must be finite and positive"
            ),
            other => write!(
                output,
                "axial cylinder projection could not qualify {other:?}"
            ),
        }
    }
}

impl Error for CylinderProjectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Coordinate(cause) => Some(cause),
            _ => None,
        }
    }
}

impl From<NonFiniteCoordinate> for CylinderProjectionError {
    fn from(cause: NonFiniteCoordinate) -> Self {
        Self::Coordinate(cause)
    }
}

type Checked<T> = Result<T, CylinderProjectionError>;

/// Plan signed and trim-restricted branches of one original axial line.
/// Example: `plan_axial_cylinder_projection(cylinder, line, mode, || false)?`.
pub fn plan_axial_cylinder_projection(
    cylinder: ProjectionCylinder,
    line: AxialProjectionLine,
    mode: AxialProjectionMode,
    mut cancelled: impl FnMut() -> bool,
) -> Checked<CylinderProjectionPlan> {
    poll(&mut cancelled)?;
    let chart = Chart::capture(cylinder)?;
    let source = AxialSource::capture(line, &chart)?;
    roots::plan(chart, source, mode, &mut cancelled)
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Checked<()> {
    if cancelled() {
        return Err(CylinderProjectionError::Cancelled);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
