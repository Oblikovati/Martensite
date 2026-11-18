//! Proof routes retain their own labels while producing identical geometry and achieved bounds.
use super::*;
use martensite_tkbo::profile_extrusion::PlanarProofVersion;
pub(super) fn accepted(mode: PlanarProofVersion) -> WrappedConeNormalResult {
    let mut request = request();
    request.proof.planar_proof = mode;
    let result = check_completed(request);
    assert_mode(&result, mode);
    result
}
pub(super) fn assert_mode(result: &WrappedConeNormalResult, expected: PlanarProofVersion) {
    assert_eq!(result.proof_options().planar_proof, expected);
    for (region, report) in result.reports() {
        assert_eq!(report.material.planar_proof, expected);
        let geometry = &result.family.family.members[region].geometry;
        assert_eq!(geometry.material.proof_version(), expected);
    }
}
#[test]
fn proof_modes_preserve_native_bytes_history_and_achieved_bounds() {
    let filtered = accepted(PlanarProofVersion::FilteredBernsteinV4);
    let bernstein = accepted(PlanarProofVersion::BernsteinV3);
    assert_ne!(
        filtered.proof_options().planar_proof,
        bernstein.proof_options().planar_proof
    );
    assert_geometry(&filtered, &bernstein);
}
fn assert_geometry(filtered: &WrappedConeNormalResult, bernstein: &WrappedConeNormalResult) {
    assert_eq!(filtered.encoded_brep(), bernstein.encoded_brep());
    assert_eq!(filtered.history(), bernstein.history());
    assert_eq!(filtered.support(), bernstein.support());
    assert_eq!(filtered.depths(), bernstein.depths());
    assert_eq!(filtered.options(), bernstein.options());
    assert_eq!(
        family_spatial(filtered.budget()),
        family_spatial(bernstein.budget())
    );
    assert_eq!(filtered.budget().volume, bernstein.budget().volume);
    assert_eq!(
        filtered.reports().keys().collect::<Vec<_>>(),
        bernstein.reports().keys().collect::<Vec<_>>()
    );
    for (region, report) in filtered.reports() {
        assert_member_bounds(report, &bernstein.reports()[region]);
    }
}
fn family_spatial(report: &WrappedConeBudgetReport) -> [WrappedCertifiedBound; 5] {
    [
        report.achieved_spatial_error,
        report.reference_developed_error,
        report.reference_filled_error,
        report.reference_storage_error,
        report.cap_reference_proxy_error,
    ]
}
fn assert_member_bounds(filtered: &WrappedConeMemberReport, bernstein: &WrappedConeMemberReport) {
    assert_eq!(member_spatial(filtered), member_spatial(bernstein));
    assert_eq!(member_intervals(filtered), member_intervals(bernstein));
    assert_eq!(filtered.volume, bernstein.volume);
    assert_eq!(filtered.native_tolerance, bernstein.native_tolerance);
    assert_eq!(filtered.storage_allowance, bernstein.storage_allowance);
    assert_eq!(filtered.native_volume, bernstein.native_volume);
}
fn member_spatial(report: &WrappedConeMemberReport) -> [WrappedCertifiedBound; 6] {
    [
        report.boundary_error,
        report.cap_trace_error,
        report.depth_storage_error[0],
        report.depth_storage_error[1],
        report.ruled_storage_error,
        report.material.cap_realization_error,
    ]
}
fn member_intervals(report: &WrappedConeMemberReport) -> [WrappedCertifiedRange; 3] {
    [
        report.authored_volume,
        report.reference_volume,
        report.boundary_flux,
    ]
}
