//! A single proven containment refutes disjointness; incomplete queries never certify it.
use super::*;
#[derive(Debug)]
pub(in super::super::super) enum MaterialFailure {
    Policy(MaterialOptions),
    Source(SurfaceRegionId),
    Members {
        requested: usize,
        planar: usize,
        prepared: usize,
    },
    Cancelled,
    Admission(Box<SelectionFailure>),
    Clearance(Box<planar_clearance::ClearanceFailure>),
    Probe(Box<CylindricalProbeError>),
    Limit {
        role: &'static str,
        actual: u64,
        maximum: u64,
    },
    Winding {
        region: SurfaceRegionId,
        contour: SurfaceCurveId,
        probe: usize,
        previous_bisections: u16,
        effective: RootIsolationOptions,
        cause: Box<BezierWireError>,
    },
    NonJordan {
        region: SurfaceRegionId,
        contour: SurfaceCurveId,
        probe: usize,
        actual: BezierWireLocation,
    },
    Overlap {
        source: SurfaceRegionId,
        target: SurfaceRegionId,
        witness: SurfaceCurveId,
        probe: CylindricalPeriodicProbe,
    },
}
impl std::fmt::Display for MaterialFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => write!(
                f,
                "periodic material cancelled before classification; expected a completed operation"
            ),
            Self::Admission(cause) => write!(f, "material admission: {cause}"),
            Self::Clearance(cause) => write!(f, "material source binding: {cause}"),
            Self::Probe(cause) => write!(f, "material translated witness: {cause}"),
            _ => self.source_message(f),
        }
    }
}

impl MaterialFailure {
    fn relation_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit {
                role,
                actual,
                maximum,
            } => write!(f, "{role}={actual}; expected at most {maximum}"),
            Self::Overlap {
                source,
                target,
                witness,
                probe,
            } => write!(
                f,
                "source={source:?}, witness={witness:?} lies in target={target:?} at {probe:?}; expected disjoint periodic material"
            ),
            _ => self.winding_message(f),
        }
    }
    fn winding_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Winding {
                region,
                contour,
                probe,
                previous_bisections,
                effective,
                cause,
            } => write!(
                f,
                "region={region:?}, contour={contour:?}, probe={probe}, earlier bisections={previous_bisections}, effective={effective:?}: {cause}; expected a complete exact winding within the remaining allowance"
            ),
            _ => self.location_message(f),
        }
    }
    fn location_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonJordan {
                region,
                contour,
                probe,
                actual,
            } => write!(
                f,
                "region={region:?}, contour={contour:?}, probe={probe}, location={actual:?}; expected an off-boundary winding of zero or magnitude one"
            ),
            _ => write!(f, "{self:?}"),
        }
    }
    fn source_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Policy(options) => write!(
                f,
                "periodic material policy={options:?}; expected finite root/point budgets"
            ),
            Self::Source(id) => write!(
                f,
                "region={id:?}; expected the original recorded planar material and context"
            ),
            _ => self.member_message(f),
        }
    }
    fn member_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Members {
                requested,
                planar,
                prepared,
            } => write!(
                f,
                "requested regions={requested}, recorded planar members={planar}, prepared members={prepared}; expected equal nonempty memberships"
            ),
            _ => self.relation_message(f),
        }
    }
}
