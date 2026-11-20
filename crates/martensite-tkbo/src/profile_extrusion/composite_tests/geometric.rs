// SPDX-License-Identifier: GPL-2.0-only
//! V6 shares integration loans, published regions and old replay selection.
use super::*;
fn geometric(cap: u64) -> PlanarProofVersion {
    PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: cap,
    }
}
#[test]
fn geometric_v6_publishes_the_same_rectangle_with_its_actual_program_loan() {
    let source = profile();
    let result = build(&source, geometric(1024), 17).unwrap();
    assert_eq!(result.requested_proof_version(), geometric(1024));
    assert_eq!(result.proof_version(), geometric(17));
    let evidence = result.integral_program_evidence().unwrap();
    assert!(evidence.matches_profile(&source));
    assert_eq!(evidence.granted_subintervals(), 17);
    assert_eq!(result.regions().len(), 1);
    assert!(result.regions()[0].area().encloses(2.));
}
#[test]
fn geometric_v6_never_widens_a_zero_integral_loan() {
    assert_zero_loan(build(&profile(), geometric(1024), 0));
}

#[test]
fn separately_issued_programs_accept_the_same_profile_but_only_clones_share_issuance() {
    let source = profile();
    let first = build(&source, geometric(1024), 17).unwrap();
    let second = build(&source, geometric(1024), 17).unwrap();
    let first = first.integral_program_evidence().unwrap();
    let second = second.integral_program_evidence().unwrap();
    assert!(first.shares_issuance(&first.clone()));
    assert!(first.matches_profile(&source) && second.matches_profile(&source));
    assert!(!first.shares_issuance(second));
}
