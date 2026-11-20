//! Periodic qualification works over bound decoded caps without building a second proxy.
use super::*;
use crate::surface_sketch::wrapped_profile::operation::cone::material::{
    capture_periodic_program, periodic_requests,
};

pub(super) fn project(members: &[(&Prepared, &[storage::SpanProof])]) -> Vec<Request> {
    periodic_requests(members).unwrap()
}

pub(super) fn planar(
    profiles: &[Request],
    request: &WrappedConeNormalRequest,
    work: &mut Work,
) -> periodic_selection::PlanarSelectionEvidence {
    let version = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    let set = capture_periodic_program(
        profiles,
        request.options.planar_work,
        version,
        work,
        request.proof.integration,
        &mut || false,
    )
    .unwrap();
    periodic_selection::PlanarSelectionEvidence {
        members: planar::select_members(profiles, &set).unwrap(),
        certificate: set.certificate(),
        proof_work: set.proof_work(),
    }
}

pub(super) fn clearance(
    profiles: &[Request],
    work: &mut Work,
) -> planar_clearance::OriginalClearance {
    let allowance = work.remaining();
    let proof =
        planar_clearance::certify_many_with_limit(profiles, profiles, allowance, || false).unwrap();
    let within = proof.certificate.operations();
    let periodic = proof.periodic.operations();
    assert_eq!(
        proof.periodic.requested_options().max_operations,
        allowance.max_operations - within
    );
    work.charge(profiles[0].region, None, within + periodic)
        .unwrap();
    proof
}

pub(super) fn classify(
    profiles: &[Request],
    planar: &periodic_selection::PlanarSelectionEvidence,
    boundary: &planar_clearance::OriginalClearance,
    work: &mut Work,
) -> Result<periodic_selection::PeriodicMaterialEvidence, periodic_selection::MaterialFailure> {
    let remaining = work.remaining();
    let mut options = periodic_selection::MaterialOptions::default();
    options.probes.fractional_bits = remaining.fractional_bits;
    options.probes.max_operations = remaining.max_operations;
    let result = periodic_selection::material::certify_projected_with_cancel(
        profiles,
        planar,
        boundary,
        options,
        || false,
    );
    let spent = result.as_ref().map_or_else(
        |error| error.operations,
        |issued| issued.probes.operations(),
    );
    work.charge(profiles[0].region, None, spent).unwrap();
    result.map_err(|error| error.cause)
}
