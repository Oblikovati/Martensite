//! A failed sufficient coefficient bound remains incomplete, not a curve witness.
use super::*;
pub(super) struct BoundClaim {
    index: usize,
    vertex: Option<usize>,
    role: &'static str,
    limit: f64,
    coefficient: bool,
}
pub(super) fn append(
    index: usize,
    vertex: Option<usize>,
    role: &'static str,
    limit: f64,
    decision: ResidualDecision,
    coefficient: bool,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    BoundClaim {
        index,
        vertex,
        role,
        limit,
        coefficient,
    }
    .record(decision, errors)
}
impl BoundClaim {
    pub(super) fn new(
        index: usize,
        vertex: Option<usize>,
        role: &'static str,
        limit: f64,
        coefficient: bool,
    ) -> Self {
        Self {
            index,
            vertex,
            role,
            limit,
            coefficient,
        }
    }
    pub(super) fn record(
        &self,
        decision: ResidualDecision,
        errors: &mut Vec<ClosedFaceErrorBound>,
    ) -> CheckResult<()> {
        match decision {
            ResidualDecision::Within {
                proven_squared_upper,
            } => {
                errors.push(self.bound(proven_squared_upper));
                Ok(())
            }
            ResidualDecision::Exceeds { witness } => self.exceeded(witness),
        }
    }
    fn bound(
        &self,
        squared_upper: martensite_tkmath::predicates::roots::DyadicBound,
    ) -> ClosedFaceErrorBound {
        ClosedFaceErrorBound {
            edge: self.index,
            vertex: self.vertex,
            role: self.role,
            spatial_limit: self.limit,
            squared_upper,
        }
    }
    fn exceeded(&self, witness: impl std::fmt::Debug) -> CheckResult<()> {
        let Self {
            index,
            role,
            limit,
            coefficient,
            ..
        } = self;
        if !coefficient {
            return Err(invalid(
                *index,
                "spatial consistency",
                format!("role={role}, limit={limit}, witness={witness:?}"),
            ));
        }
        Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::CoefficientBound {
                edge: *index,
                limit: *limit,
            },
        ))
    }
}
