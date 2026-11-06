//! Independent support inputs consume one invocation's remaining exact proof limits.
use super::budget::Budget;
use martensite_tkmath::predicates::roots::closed_extrusion::*;
pub(super) fn prove<F: FnMut() -> bool>(
    input: ClosedQuadraticExtrusionInput,
    budget: &mut Budget<'_, F>,
) -> ClosedExtrusionReport {
    let policy = remaining(budget);
    let proof = certify_closed_quadratic_extrusion(input, policy, &mut *budget.cancel);
    record(proof.work(), budget);
    proof
}
pub(super) fn remaining<F: FnMut() -> bool>(budget: &Budget<'_, F>) -> ClosedExtrusionPolicy {
    let mut policy = budget.policy.support;
    policy.max_spans = policy.max_spans.saturating_sub(budget.work.support_spans);
    policy.max_obligations = policy
        .max_obligations
        .saturating_sub(budget.work.support_obligations);
    policy.max_proof_steps = policy
        .max_proof_steps
        .saturating_sub(budget.work.support_steps);
    policy
}
pub(super) fn record<F: FnMut() -> bool>(work: ClosedExtrusionWork, budget: &mut Budget<'_, F>) {
    budget.work.support_spans = budget.work.support_spans.saturating_add(work.spans);
    budget.work.support_obligations = budget
        .work
        .support_obligations
        .saturating_add(work.scheduled_obligations);
    budget.work.support_finished = budget
        .work
        .support_finished
        .saturating_add(work.finished_obligations);
    budget.work.support_steps = budget
        .work
        .support_steps
        .saturating_add(work.started_proof_steps);
}
