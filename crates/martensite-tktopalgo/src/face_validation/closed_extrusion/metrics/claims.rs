//! A failed sufficient coefficient bound remains incomplete, not a curve witness.
use super::*;
pub(super) struct BoundClaim {
    index: usize,
    vertex: Option<usize>,
    context: ClosedFaceQueryContext,
    limit: f64,
    coefficient: bool,
}
pub(super) fn append(
    index: usize,
    vertex: Option<usize>,
    context: ClosedFaceQueryContext,
    limit: f64,
    decision: ResidualDecision,
    coefficient: bool,
    errors: &mut Vec<ClosedFaceErrorBound>,
) -> CheckResult<()> {
    BoundClaim {
        index,
        vertex,
        context,
        limit,
        coefficient,
    }
    .record(decision, errors)
}
impl BoundClaim {
    pub(super) fn new(
        index: usize,
        vertex: Option<usize>,
        context: ClosedFaceQueryContext,
        limit: f64,
        coefficient: bool,
    ) -> Self {
        Self {
            index,
            vertex,
            context,
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
            role: self.context.relation.label(),
            spatial_limit: self.limit,
            squared_upper,
        }
    }
    fn exceeded(&self, witness: impl std::fmt::Debug) -> CheckResult<()> {
        let (index, role, limit) = (self.index, self.context.relation.label(), self.limit);
        if !self.coefficient {
            return Err(invalid(
                self.context.owner,
                "spatial consistency",
                format!("role={role}, limit={limit}, witness={witness:?}"),
            ));
        }
        Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::CoefficientBound { edge: index, limit },
        ))
    }
}
