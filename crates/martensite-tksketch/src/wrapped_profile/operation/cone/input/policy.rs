//! Admit the new request's explicit numerical policies before any proof work is spent.
use super::*;
pub(super) fn admit(proof: WrappedConeProofOptions) -> ConeChecked<()> {
    admit_with_ceiling(proof, 1_000_000)
}
pub(super) fn admit_with_ceiling(proof: WrappedConeProofOptions, ceiling: u32) -> ConeChecked<()> {
    let arithmetic = proof.arithmetic;
    let authored = proof.authored_volume;
    let valid = (8..=4096).contains(&arithmetic.fractional_bits)
        && arithmetic.max_operations <= ceiling
        && authored.subdivision_level <= 12
        && authored.clearance_level <= 64
        && authored.enclosure_width.is_finite()
        && authored.enclosure_width > 0.;
    if valid && integration(proof.integration) {
        return Ok(());
    }
    let cause = if ceiling == 1_000_000 {
        ConeCause::ProofOptions(Box::new(proof))
    } else {
        ConeCause::InputBoundProofOptions {
            actual: Box::new(proof),
            ceiling,
        }
    };
    Err(
        ConeFailure::new(None, None, WrappedFailureClass::InvalidInput, cause)
            .at(WrappedStage::Admission),
    )
}
fn integration(options: CylindricalFluxOptions) -> bool {
    (1..=2).contains(&options.behavior_version)
        && options.max_degree <= 16
        && (1..=4096).contains(&options.max_spans)
        && (1..=65536).contains(&options.max_subintervals_per_span)
        && options.max_total_subintervals <= 1_000_000
        && options.absolute_error_limit.is_finite()
        && options.absolute_error_limit >= 0.
}
