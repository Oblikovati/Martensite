//! Deterministic complete obligation traversal; unproved hull pairs stay incomplete.
use super::super::{
    bezier_pair::hull_separation::{HullSeparation, certify_hulls},
    sketch_incidence::spans::closed_bridge::{
        ProjectedQuadraticSpan, SpanBridgeFailure, projected_spans,
    },
};
use super::types::{
    ClosedExtrusionCause as Cause, ClosedExtrusionCompletion as Completion, ClosedExtrusionPolicy,
    ClosedExtrusionStage,
};
use super::{
    budget::ProofBudget,
    input::ClosedQuadraticExtrusionInput as Input,
    report::{ClosedExtrusionReport, ExclusionWitness},
    self_span,
};

pub(super) fn execute(
    input: Input,
    policy: ClosedExtrusionPolicy,
    cancelled: &mut dyn FnMut() -> bool,
) -> ClosedExtrusionReport {
    let mut budget = ProofBudget::new(policy, cancelled);
    let mut spans = Vec::new();
    let mut witnesses = Vec::new();
    let completion = run(&input, &mut budget, &mut spans, &mut witnesses)
        .map_or_else(|completion| completion, |()| Completion::Certified);
    ClosedExtrusionReport {
        input,
        policy,
        completion,
        work: budget.work,
        spans,
        witnesses,
    }
}

fn run(
    input: &Input,
    budget: &mut ProofBudget<'_>,
    spans: &mut Vec<ProjectedQuadraticSpan>,
    witnesses: &mut Vec<ExclusionWitness>,
) -> Result<(), Completion> {
    admission(input, budget).map_err(Completion::Incomplete)?;
    budget.work.stage = ClosedExtrusionStage::Spans;
    *spans = prepare_spans(input, budget).map_err(Completion::Incomplete)?;
    self_obligations(spans, budget, witnesses)?;
    all_pairs(spans, budget, witnesses).map_err(Completion::Incomplete)?;
    budget.work.stage = ClosedExtrusionStage::Publication;
    budget.checkpoint().map_err(Completion::Incomplete)
}

fn self_obligations(
    spans: &[ProjectedQuadraticSpan],
    budget: &mut ProofBudget<'_>,
    witnesses: &mut Vec<ExclusionWitness>,
) -> Result<(), Completion> {
    budget.work.stage = ClosedExtrusionStage::SelfSpan;
    for (index, span) in spans.iter().enumerate() {
        budget.start_obligation().map_err(Completion::Incomplete)?;
        let witness = self_span::certify_span(span, index).map_err(Completion::NotExcluded)?;
        witnesses.push(ExclusionWitness::SelfSpan {
            span: index,
            witness,
        });
        budget.finish_obligation();
    }
    Ok(())
}

fn admission(input: &Input, budget: &mut ProofBudget<'_>) -> Result<(), Cause> {
    budget.checkpoint()?;
    if budget.policy.version != 1 {
        return Err(Cause::PolicyVersion(budget.policy.version));
    }
    if input.weights.iter().any(|w| *w != input.weights[0]) {
        return Err(Cause::NonConstantWeights);
    }
    declare_obligations(input.knots.len() - 1, budget)
}

fn declare_obligations(n: usize, budget: &mut ProofBudget<'_>) -> Result<(), Cause> {
    budget.work.spans = n;
    let required =
        n.checked_mul(n + 1)
            .and_then(|v| v.checked_div(2))
            .ok_or(Cause::SpanInvariant {
                kind: "obligation count overflow",
                actual: n,
            })?;
    budget.work.scheduled_obligations = required;
    limit("spans", budget.policy.max_spans as u64, n as u64)?;
    limit(
        "obligations",
        budget.policy.max_obligations as u64,
        required as u64,
    )
}

fn prepare_spans(
    input: &Input,
    budget: &mut ProofBudget<'_>,
) -> Result<Vec<ProjectedQuadraticSpan>, Cause> {
    let spans = build_projected(input, budget)?;
    if spans.len() != budget.work.spans {
        return Err(Cause::SpanInvariant {
            kind: "nonempty span coverage",
            actual: spans.len(),
        });
    }
    domain_coverage(input, &spans, budget)?;
    coefficient_limits(&spans, budget.policy.max_coefficient_bits)?;
    Ok(spans)
}

fn build_projected(
    input: &Input,
    budget: &mut ProofBudget<'_>,
) -> Result<Vec<ProjectedQuadraticSpan>, Cause> {
    let mut stop = None;
    let result = projected_spans(
        &input.poles,
        &input.flat_knots(),
        input.direction,
        input.curve_interval(),
        &mut || record_stop(budget, &mut stop),
    );
    if let Some(cause) = stop {
        return Err(cause);
    }
    result.map_err(bridge_cause)
}

fn record_stop(budget: &mut ProofBudget<'_>, stop: &mut Option<Cause>) -> bool {
    match budget.checkpoint() {
        Ok(()) => false,
        Err(cause) => {
            *stop = Some(cause);
            true
        }
    }
}

fn domain_coverage(
    input: &Input,
    spans: &[ProjectedQuadraticSpan],
    budget: &mut ProofBudget<'_>,
) -> Result<(), Cause> {
    for (index, span) in spans.iter().enumerate() {
        budget.checkpoint()?;
        let expected = [input.knots[index], input.knots[index + 1]];
        let next = &spans[(index + 1) % spans.len()];
        if span.range != expected || span.controls[2] != next.controls[0] {
            return Err(Cause::SpanInvariant {
                kind: "full domain and exact shared endpoints",
                actual: index,
            });
        }
    }
    Ok(())
}

fn bridge_cause(cause: SpanBridgeFailure) -> Cause {
    match cause {
        SpanBridgeFailure::Admission(cause) => Cause::SpanAdmission(cause),
        SpanBridgeFailure::Cancelled => Cause::Cancelled,
        SpanBridgeFailure::CoefficientShape(actual) => Cause::SpanInvariant {
            kind: "quadratic coefficients",
            actual,
        },
        SpanBridgeFailure::NonUnitDenominator => Cause::SpanInvariant {
            kind: "partition of unity",
            actual: 0,
        },
        SpanBridgeFailure::Direction(_) => Cause::SpanInvariant {
            kind: "admitted direction",
            actual: 0,
        },
    }
}

fn coefficient_limits(spans: &[ProjectedQuadraticSpan], requested: u64) -> Result<(), Cause> {
    for span in spans {
        for value in span
            .powers
            .iter()
            .flatten()
            .chain(span.controls.iter().flatten())
        {
            let actual = value.numerator().bits().max(value.denominator().bits());
            limit("exact coefficient bits", requested, actual)?;
        }
    }
    Ok(())
}

fn all_pairs(
    spans: &[ProjectedQuadraticSpan],
    budget: &mut ProofBudget<'_>,
    witnesses: &mut Vec<ExclusionWitness>,
) -> Result<(), Cause> {
    budget.work.stage = ClosedExtrusionStage::SpanPair;
    for second in 0..spans.len() {
        for first in 0..second {
            witnesses.push(prove_pair(spans, first, second, budget)?);
            budget.finish_obligation();
        }
    }
    Ok(())
}

fn prove_pair(
    spans: &[ProjectedQuadraticSpan],
    first: usize,
    second: usize,
    budget: &mut ProofBudget<'_>,
) -> Result<ExclusionWitness, Cause> {
    budget.start_obligation()?;
    let endpoint = allowed_endpoint(spans, first, second);
    let witness = pair_hulls(spans, first, second, endpoint.as_ref(), budget)?;
    if witness == HullSeparation::Unresolved {
        return Err(Cause::UnseparatedPair { first, second });
    }
    witness_limits(&witness, budget.policy.max_coefficient_bits)?;
    Ok(ExclusionWitness::Pair {
        first,
        second,
        witness: Box::new(witness),
    })
}

fn witness_limits(witness: &HullSeparation, requested: u64) -> Result<(), Cause> {
    match witness {
        HullSeparation::Disjoint { normal } => ratio_limits(normal.iter(), requested),
        HullSeparation::EndpointOnly { point, normals } => {
            ratio_limits(point.iter().chain(normals.iter().flatten()), requested)
        }
        HullSeparation::Unresolved => Ok(()),
    }
}

fn ratio_limits<'a>(
    values: impl Iterator<Item = &'a super::super::certificate_arithmetic::ExactRatio>,
    requested: u64,
) -> Result<(), Cause> {
    for value in values {
        let actual = value.numerator().bits().max(value.denominator().bits());
        limit("retained witness coefficient bits", requested, actual)?;
    }
    Ok(())
}

fn pair_hulls(
    spans: &[ProjectedQuadraticSpan],
    first: usize,
    second: usize,
    endpoint: Option<&[super::super::certificate_arithmetic::ExactRatio; 2]>,
    budget: &mut ProofBudget<'_>,
) -> Result<HullSeparation, Cause> {
    certify_hulls(
        &spans[first].controls,
        &spans[second].controls,
        endpoint,
        &mut || budget.checkpoint(),
    )
}

fn allowed_endpoint(
    spans: &[ProjectedQuadraticSpan],
    first: usize,
    second: usize,
) -> Option<[super::super::certificate_arithmetic::ExactRatio; 2]> {
    let (left, right) = if first + 1 == second {
        (&spans[first].controls[2], &spans[second].controls[0])
    } else if first == 0 && second + 1 == spans.len() {
        (&spans[first].controls[0], &spans[second].controls[2])
    } else {
        return None;
    };
    (left == right).then(|| left.clone())
}

fn limit(kind: &'static str, requested: u64, actual: u64) -> Result<(), Cause> {
    if actual > requested {
        return Err(Cause::Limit {
            kind,
            requested,
            actual,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::predicates::roots::certificate_arithmetic::ExactRatio;
    #[test]
    fn retained_witness_height_limit_refuses_and_preserves_the_witness() {
        let witness = HullSeparation::Disjoint {
            normal: [ExactRatio::from_finite(1024.), ExactRatio::from_finite(0.)],
        };
        let before = witness.clone();
        assert_eq!(
            witness_limits(&witness, 8),
            Err(Cause::Limit {
                kind: "retained witness coefficient bits",
                requested: 8,
                actual: 11
            })
        );
        assert_eq!(witness, before);
        assert!(witness_limits(&witness, 16).is_ok());
    }
}
