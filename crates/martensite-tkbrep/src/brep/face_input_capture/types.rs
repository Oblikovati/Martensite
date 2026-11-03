//! Capture reports describe owned input and explicit incomplete work.
use martensite_tkernel::standard::SharedReadFailure;

/// Explicit input-capture admission, independent of geometric certification (#213).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceCaptureBehavior {
    /// Preserve the original plane/extrusion and clamped quadratic input domain.
    StoredRepresentationsV1,
    /// Include stored tensor surfaces, general spatial splines and regularity bindings.
    StoredSurfaceQueriesV2,
}

/// Bounds finite discovery before any numerical validation; `FaceCaptureLimits::bounded()`.
#[derive(Clone, Copy, Debug)]
pub struct FaceCaptureLimits {
    /// Maximum distinct typed mutable owners.
    pub owners: usize,
    /// Maximum distinct mutable cells.
    pub cells: usize,
    /// Maximum copied records, collection elements and location-chain entries.
    pub entries: usize,
}
impl FaceCaptureLimits {
    /// Conservative finite defaults; `FaceCaptureLimits::bounded()`.
    pub const fn bounded() -> Self {
        Self {
            owners: 4096,
            cells: 16384,
            entries: 65536,
        }
    }
}

/// Why the requested coherent input is unavailable; no variant certifies invalid geometry.
#[derive(Clone, Debug, PartialEq)]
pub enum FaceCaptureIncomplete {
    /// The actual mutable cell could not be leased.
    Read {
        /// Invocation-local owner label; it is not a process address.
        owner: usize,
        /// Required mutable cell category.
        cell: &'static str,
        /// Nonblocking lease refusal.
        cause: SharedReadFailure,
    },
    /// Copying the actual stored geometry failed under its held lease.
    Geometry {
        /// Invocation-local source owner.
        owner: usize,
        /// Actual stored-input failure, without a substituted domain.
        cause: Box<martensite_tkernel::standard::Failure>,
    },
    /// A discovered reference changed before the simultaneous read view.
    Changed {
        /// Invocation-local owner label; it is not a process address.
        owner: usize,
    },
    /// The actual representation is outside the finite admitted set.
    Unsupported {
        /// Invocation-local owner label; it is not a process address.
        owner: usize,
        /// Observed value that caused this refusal.
        actual: String,
        /// Required representation or value shape.
        expected: &'static str,
    },
    /// The declared finite capture bound was exhausted.
    Limit {
        /// Observed value that caused this refusal.
        actual: usize,
        /// Declared finite upper bound.
        maximum: usize,
        /// Kind of exhausted capture work.
        dimension: &'static str,
    },
    /// Cancellation was requested outside every live lease.
    Cancelled,
    /// An admitted numeric field was nonfinite.
    NonFinite {
        /// Invocation-local owner label; it is not a process address.
        owner: usize,
        /// Nonfinite stored numeric field.
        field: &'static str,
    },
}

/// One captured immutable datum chain; `location.steps()` preserves its exact powers.
#[derive(Clone, Debug)]
pub struct CapturedLocation {
    pub(super) steps: Vec<(usize, i32, martensite_tkmath::gp::Transform3)>,
    pub(super) effective: martensite_tkmath::gp::Transform3,
}
impl CapturedLocation {
    /// Read the actual cached evaluator map, without reconstructing its product; `location.effective_transform()`.
    pub fn effective_transform(&self) -> martensite_tkmath::gp::Transform3 {
        self.effective
    }

    /// Read owned datum identities, powers and transforms; `location.steps()`.
    pub fn steps(&self) -> &[(usize, i32, martensite_tkmath::gp::Transform3)] {
        &self.steps
    }
}
/// An occurrence keeps shared topology identity separate from location and orientation.
#[derive(Clone, Debug)]
pub struct CapturedOccurrence {
    /// Invocation-local identity of the actual referenced topology owner.
    pub owner: usize,
    /// Owned immutable location chain.
    pub location: CapturedLocation,
    /// Authored occurrence orientation.
    pub orientation: martensite_tkg3d::top_abs::Orientation,
}
/// A lease-backed field copy; `FaceInputSnapshot::cells()` exposes these owned values.
#[derive(Clone, Debug)]
pub enum CapturedFaceCell {
    /// Topology children retain occurrence order and shared entity identities.
    Topology {
        /// Stored topology type and flag word.
        state: u16,
        /// Child occurrences in their actual stored order.
        children: Vec<CapturedOccurrence>,
    },
    /// Located face support and its validation fields; meshes are outside this input contract.
    Face {
        /// Invocation-local label of the actual surface owner.
        surface: usize,
        /// Owned actual location chain.
        location: CapturedLocation,
        /// Stored modelling tolerance.
        tolerance: f64,
        /// Stored natural-restriction flag.
        natural_restriction: bool,
    },
    /// Edge validation flags and all actual geometric representations in stored order.
    Edge {
        /// Stored modelling tolerance.
        tolerance: f64,
        /// Stored edge validation flags.
        flags: i32,
        /// Actual representation-owner labels in stored order.
        representations: Vec<usize>,
    },
    /// Vertex geometry; nonempty auxiliary point representations are unsupported.
    Vertex {
        /// Stored Cartesian point.
        point: [f64; 3],
        /// Stored modelling tolerance.
        tolerance: f64,
    },
    /// The actual spatial geometry binding.
    Spatial {
        /// Invocation-local label of the actual curve owner.
        curve: usize,
    },
    /// First pcurve binding and cached endpoint values.
    Parametric {
        /// Invocation-local label of the actual curve owner.
        curve: usize,
        /// Invocation-local label of the actual surface owner.
        surface: usize,
        /// Stored parameter-space endpoint values.
        endpoints: [[f64; 2]; 2],
    },
    /// Second seam pcurve and actual continuity metadata.
    Seam {
        /// Invocation-local label of the actual curve owner.
        curve: usize,
        /// Stored continuity across the seam.
        continuity: martensite_tkmath::geom_abs::Shape,
        /// Stored parameter-space endpoint values.
        endpoints: [[f64; 2]; 2],
    },
    /// Stored representation range, copied without evaluation.
    Range([f64; 2]),
    /// Stored representation location.
    Location(CapturedLocation),
    /// Stored spatial line axis.
    Line(martensite_tkmath::gp::Axis1),
    /// Stored planar line axis.
    PlanarLine(martensite_tkmath::gp::Axis2d),
    /// Stored spline fields, without numerical certification.
    Spline(martensite_tkg3d::geom::StoredSplineInput),
    /// Stored planar spline fields and one-based array layouts, without evaluation.
    PlanarSpline(martensite_tkg2d::geom2d::StoredPlanarSplineInput),
    /// Stored plane frame, copied from the live plane state.
    Plane(martensite_tkmath::gp::Axis3),
    /// Owned raw tensor-spline coefficients; no numerical certification is implied.
    SplineSurface(martensite_tkg3d::geom::StoredSplineSurfaceInput),
    /// Regularity binds two actual located supports without discarding either reference.
    Regularity {
        /// Invocation-local labels of both actual supports.
        surfaces: [usize; 2],
        /// Actual second support location; the first is the representation location cell.
        location2: CapturedLocation,
        /// Stored continuity metadata.
        continuity: martensite_tkmath::geom_abs::Shape,
    },
    /// Actual extrusion-to-basis binding and direction.
    Extrusion {
        /// Invocation-local label of the actual basis owner.
        basis: usize,
        /// Stored extrusion direction components.
        direction: [f64; 3],
    },
}
/// A simultaneous owned input view; `snapshot.cells()` requires no live geometry or locks.
#[derive(Clone, Debug)]
pub struct FaceInputSnapshot {
    pub(super) root: CapturedOccurrence,
    pub(super) cells: Vec<(usize, CapturedFaceCell)>,
}
impl FaceInputSnapshot {
    /// Read the actual root occurrence; `snapshot.root()`.
    pub fn root(&self) -> &CapturedOccurrence {
        &self.root
    }
    /// Read invocation-local owners and their complete admitted field copies; `snapshot.cells()`.
    pub fn cells(&self) -> &[(usize, CapturedFaceCell)] {
        &self.cells
    }
}

impl FaceCaptureIncomplete {
    pub(super) fn at_owner(self, owner: usize) -> Self {
        match self {
            Self::Read { cell, cause, .. } => Self::Read { owner, cell, cause },
            Self::Geometry { cause, .. } => Self::Geometry { owner, cause },
            Self::Changed { .. } => Self::Changed { owner },
            Self::Unsupported {
                actual, expected, ..
            } => Self::Unsupported {
                owner,
                actual,
                expected,
            },
            Self::NonFinite { field, .. } => Self::NonFinite { owner, field },
            other => other,
        }
    }
}
