//! Collection refusals name authored owners; capture-local region indices never escape.
use super::*;
#[derive(Debug)]
pub(in super::super) enum SelectionFailure {
    Empty,
    Material(Box<MaterialFailure>),
    Limit {
        role: &'static str,
        actual: usize,
        maximum: usize,
    },
    Admission {
        region: SurfaceRegionId,
        cause: Failure,
    },
    Region {
        region: SurfaceRegionId,
        cause: Box<planar::PlanarFailure>,
    },
    Capture(Box<planar::PlanarFailure>),
    Planning {
        region: SurfaceRegionId,
        cause: Box<subdivision::PlanningFailure>,
    },
    Clearance(Box<planar_clearance::ClearanceFailure>),
    DuplicateRegion(SurfaceRegionId),
    DuplicateCurve {
        curve: SurfaceCurveId,
        owners: [SurfaceRegionId; 2],
    },
    DuplicateVertex {
        vertex: CornerSource,
        owners: [SurfaceRegionId; 2],
    },
    Context {
        region: SurfaceRegionId,
        actual: [u64; 4],
        expected: [u64; 4],
    },
}
impl std::fmt::Display for SelectionFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Material(cause) => write!(f, "periodic material: {cause}"),
            Self::Empty => write!(f, "regions=0; expected at least one selected region"),
            Self::Limit {
                role,
                actual,
                maximum,
            } => write!(f, "{role}={actual}; expected at most {maximum}"),
            Self::DuplicateRegion(region) => write!(
                f,
                "duplicate region={region:?}; expected unique authored region IDs"
            ),
            _ => self.identity_message(f),
        }
    }
}
impl SelectionFailure {
    fn identity_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateCurve { curve, owners } => write!(
                f,
                "curve={curve:?}, regions={owners:?}; expected globally unique authored curve IDs"
            ),
            Self::DuplicateVertex { vertex, owners } => write!(
                f,
                "vertex={vertex:?}, regions={owners:?}; expected disjoint authored vertex IDs"
            ),
            _ => self.context_message(f),
        }
    }
    fn context_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Context {
                region,
                actual,
                expected,
            } => write!(
                f,
                "region={region:?}, radius/depth/tolerance bits={actual:?}; expected common chart and extrusion settings={expected:?}"
            ),
            _ => self.geometry_message(f),
        }
    }
    fn geometry_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admission { region, cause } => write!(f, "region={region:?}, admission: {cause}"),
            Self::Region { region, cause } => {
                write!(f, "region={region:?}, global planar selection: {cause}")
            }
            Self::Planning { region, cause } => {
                write!(f, "region={region:?}, fit planning: {cause}")
            }
            Self::Clearance(cause) => write!(f, "selection boundary clearance: {cause}"),
            Self::Capture(cause) => write!(f, "global planar capture: {cause}"),
            _ => write!(f, "{self:?}"),
        }
    }
}
