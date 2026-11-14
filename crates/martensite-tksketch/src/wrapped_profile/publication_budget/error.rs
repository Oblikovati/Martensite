use super::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in super::super) enum Quantity {
    Profile,
    CapHomotopy,
    CapDomain,
    Storage,
    Trace,
    TraceTolerance,
    RetainedIdeal,
    RepresentativeIdeal,
    Integration,
    AuthoredIdeal,
}
#[derive(Debug)]
pub(in super::super) enum BudgetFailure {
    Empty,
    Policy {
        field: &'static str,
        value: f64,
    },
    Owners {
        actual: Vec<SurfaceRegionId>,
        expected: Vec<SurfaceRegionId>,
    },
    Inventory {
        field: &'static str,
        actual: usize,
        expected: usize,
    },
    Spans {
        region: SurfaceRegionId,
        actual: Vec<CurveSpan>,
        expected: Vec<CurveSpan>,
    },
    Source(SurfaceRegionId),
    Cones {
        region: SurfaceRegionId,
        cause: Box<planar_cones::ConeFailure>,
    },
    Coverage {
        region: SurfaceRegionId,
        field: &'static str,
        actual: usize,
        expected: usize,
    },
    Native {
        region: SurfaceRegionId,
        stage: martensite_tkbo::body_check::ValidationStage,
    },
    Invalid {
        quantity: Quantity,
        value: f64,
    },
    Limit {
        quantity: Quantity,
        actual: f64,
        limit: f64,
        region: Option<SurfaceRegionId>,
        span: Option<CurveSpan>,
    },
    Overflow {
        quantity: Quantity,
        accumulated: f64,
        contribution: f64,
    },
    Clearance {
        required: f64,
        within: f64,
        periodic: Box<martensite_tkmath::predicates::roots::DyadicBound>,
    },
}
impl std::fmt::Display for BudgetFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "budget regions=0; expected nonempty original selection"),
            Self::Policy { field, value } => write!(
                f,
                "budget {field}={value}; expected admitted finite allowance"
            ),
            Self::Owners { actual, expected } => write!(
                f,
                "budget owners={actual:?}; expected unique original owners={expected:?}"
            ),
            _ => self.identity(f),
        }
    }
}
impl BudgetFailure {
    fn identity(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inventory {
                field,
                actual,
                expected,
            } => write!(f, "budget {field}={actual}; expected {expected}"),
            Self::Source(region) => write!(
                f,
                "budget source={region:?}; expected original source bits and construction context"
            ),
            _ => self.evidence(f),
        }
    }
}
impl BudgetFailure {
    fn evidence(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Coverage {
                region,
                field,
                actual,
                expected,
            } => write!(
                f,
                "budget region={region:?} {field}={actual}; expected {expected}"
            ),
            Self::Native { region, stage } => write!(
                f,
                "budget region={region:?} native stage={stage:?}; expected complete ordered pass with original coverage modes and tolerance cap"
            ),
            _ => self.cone_message(f),
        }
    }
}
impl BudgetFailure {
    fn cone_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cones { region, cause } => write!(f, "budget region={region:?}: {cause}"),
            _ => self.correspondence(f),
        }
    }
}
impl BudgetFailure {
    fn correspondence(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spans {
                region,
                actual,
                expected,
            } => write!(
                f,
                "budget region={region:?} measure spans={actual:?}; expected stored spans={expected:?}"
            ),
            _ => self.clearance_message(f),
        }
    }
}
impl BudgetFailure {
    fn clearance_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Clearance {
                required,
                within,
                periodic,
            } => write!(
                f,
                "budget clearance within={within}, periodic={periodic}; expected both >= {required}"
            ),
            _ => self.bounds(f),
        }
    }
}
impl BudgetFailure {
    fn bounds(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid { quantity, value } => write!(
                f,
                "budget {quantity:?}={value}; expected finite nonnegative certified bound"
            ),
            Self::Overflow {
                quantity,
                accumulated,
                contribution,
            } => write!(
                f,
                "budget {quantity:?} sum={accumulated}+{contribution}; expected representable finite upper bound"
            ),
            _ => self.limit_message(f),
        }
    }
}
impl BudgetFailure {
    fn limit_message(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Limit {
                quantity,
                actual,
                limit,
                region,
                span,
            } => write!(
                f,
                "budget {quantity:?}={actual}, region={region:?}, span={span:?}; expected <= {limit}"
            ),
            _ => write!(f, "{self:?}"),
        }
    }
}
