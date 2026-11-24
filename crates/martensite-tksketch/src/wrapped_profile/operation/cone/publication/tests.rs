use super::*;

pub(in super::super) fn assert_publication(
    result: &WrappedConeNormalResult,
    request: &WrappedConeNormalRequest,
) {
    assert_eq!(result.depths(), request.depths);
    assert_eq!(result.options(), request.options);
    assert_eq!(result.proof_options(), request.proof);
    assert_eq!(result.boundary_flux_origin(), [0.; 3]);
    assert_report_sources(result);
    assert_material_engine(result, request);
    assert_storage_sources(result);
    assert_measure_sources(result);
    assert_proxy_volume(result);
    assert_reference_measures(result);
    assert_reference_bounds(result);
    assert_work_sources(result);
    assert_budget_sources(result);
    assert_detached_history(result);
}

fn assert_material_engine(result: &WrappedConeNormalResult, request: &WrappedConeNormalRequest) {
    let region = SurfaceRegionId(1);
    let geometry = &result.family.family.members[&region].geometry;
    let material = &result.reports()[&region].material;
    assert_eq!(material.planar_proof, geometry.material.proof_version());
    assert_eq!(material.planar_proof, request.proof.planar_proof);
}

fn assert_proxy_volume(result: &WrappedConeNormalResult) {
    let region = SurfaceRegionId(1);
    let geometry = &result.family.family.members[&region].geometry;
    let report = &result.reports()[&region];
    assert_eq!(
        report.volume.discrepancy.upper,
        measures::upper(geometry.authored.difference()).unwrap()
    );
    assert_eq!(
        report.volume.integration_error.upper,
        geometry.flux.absolute_error_bound()
    );
}

fn assert_storage_sources(result: &WrappedConeNormalResult) {
    let region = SurfaceRegionId(1);
    let geometry = &result.family.family.members[&region].geometry;
    let report = &result.reports()[&region];
    let (depths, ruling, allowance) = reports::storage_bounds(geometry).unwrap();
    assert_eq!(report.depth_storage_error.map(|bound| bound.upper), depths);
    assert_eq!(report.ruled_storage_error.upper, ruling);
    assert_eq!(report.storage_allowance, allowance);
    assert!(depths.into_iter().all(|error| error <= allowance));
    assert!(ruling <= allowance);
}

fn assert_report_sources(result: &WrappedConeNormalResult) {
    let region = SurfaceRegionId(1);
    let member = &result.family.family.members[&region];
    let report = &result.reports()[&region];
    let geometry = &member.geometry;
    assert_eq!(report.coverage.cap_pcurves, geometry.spans.len() * 2);
    assert_eq!(report.coverage.side_pcurves, geometry.spans.len() * 4);
}

fn assert_measure_sources(result: &WrappedConeNormalResult) {
    let region = SurfaceRegionId(1);
    let geometry = &result.family.family.members[&region].geometry;
    let report = &result.reports()[&region];
    assert_eq!(
        report.boundary_flux,
        api::reports::range(geometry.flux.enclosure())
    );
    assert_eq!(
        report.authored_volume,
        measures::range(geometry.authored.volume()).unwrap()
    );
}

fn assert_work_sources(result: &WrappedConeNormalResult) {
    let region = SurfaceRegionId(1);
    let geometry = &result.family.family.members[&region].geometry;
    assert_eq!(
        result.flux_receipt(region).unwrap().span_count(),
        geometry.spans.len()
    );
    assert_eq!(
        result.authored_receipt(region).unwrap().operations(),
        geometry.authored.operations()
    );
}

fn assert_budget_sources(result: &WrappedConeNormalResult) {
    let budget = result.budget();
    assert_eq!(
        budget.requested_spatial_error,
        result.options().spatial_error
    );
    assert_eq!(
        budget.achieved_spatial_error.upper,
        result.family.budget.spatial_error
    );
    assert!(budget.achieved_spatial_error.upper <= budget.requested_spatial_error);
    assert_eq!(budget.volume, result.family.budget.volume);
    assert_family_work(result);
}

fn assert_family_work(result: &WrappedConeNormalResult) {
    let budget = result.budget();
    assert_eq!(
        budget.source_planar_work,
        result.family.family.selection.work
    );
    assert_eq!(budget.planar_work, result.family.budget.planar_work);
    assert_eq!(
        budget.arithmetic_operations,
        result.family.budget.arithmetic_operations
    );
    assert_eq!(
        budget.integration_subintervals,
        result.family.budget.integration_subintervals
    );
}

fn assert_reference_measures(result: &WrappedConeNormalResult) {
    let region = SurfaceRegionId(1);
    let geometry = &result.family.family.members[&region].geometry;
    let report = &result.reports()[&region];
    let comparison = measures::comparison(geometry).unwrap();
    let reference = result.reference_volume_receipt(region).unwrap();
    assert_eq!(
        report.reference_volume,
        api::reports::range(reference.enclosure())
    );
    assert_comparison_bounds(report, comparison);
    assert_eq!(
        report.volume.reference_integration_error.upper,
        reference.absolute_error_bound()
    );
}

fn assert_comparison_bounds(
    report: &WrappedConeMemberReport,
    comparison: measures::ComparisonBounds,
) {
    assert_eq!(
        report.volume.authored_reference_discrepancy.upper,
        comparison.authored_reference_discrepancy
    );
    assert_eq!(
        report.volume.proxy_reference_discrepancy.upper,
        comparison.proxy_reference_discrepancy
    );
}

fn assert_reference_bounds(result: &WrappedConeNormalResult) {
    let budget = result.budget();
    let reference = result.reference_body_receipt();
    assert_eq!(
        budget.reference_developed_error.upper,
        reference.developed_error()
    );
    assert_eq!(
        budget.reference_filled_error.upper,
        reference.filled_error()
    );
    assert_eq!(
        budget.reference_storage_error.upper,
        reference.storage_error()
    );
    assert_eq!(
        budget.cap_reference_proxy_error.upper,
        result.family.budget.cap_reference_proxy_error
    );
}

fn assert_detached_history(result: &WrappedConeNormalResult) {
    let first = result.export_native().unwrap();
    let second = result.export_native().unwrap();
    assert_eq!(first.entities.len(), result.history().len());
    assert_eq!(
        first.entities.keys().collect::<Vec<_>>(),
        second.entities.keys().collect::<Vec<_>>()
    );
    assert!(!first.bodies.is_partner(&second.bodies));
}
