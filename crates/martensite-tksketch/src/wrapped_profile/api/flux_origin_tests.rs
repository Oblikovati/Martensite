//! The centered-field origin is bound to the decoded graph on every public path (#528).
use super::*;
use crate::surface_sketch::HistoryRole;
use martensite_tkbo::profile_extrusion::PlanarProofWorkLimit;
use martensite_tkbrep::{brep::BRepTool, topods::vertex};

#[test]
fn canonical_boundary_flux_uses_the_identity_graph_origin() {
    let mut request = request();
    request.support.origin = [-0., 0., -0.];
    let result = normal_extrude_profile(&request).unwrap();
    assert!(result.placement().is_none());
    assert_eq!(result.boundary_flux_origin().map(f64::to_bits), [0; 3]);
    let export = result.export_native().unwrap();
    assert!(export.bodies.location().is_identity());
    assert_ideal_volume_comparison(result.reports());
}
#[test]
fn oblique_boundary_flux_origin_is_the_decoded_large_translation() {
    let result = normal_extrude_profile(&translated_request()).unwrap();
    let placement = result.placement().unwrap();
    let origin = result.boundary_flux_origin();
    assert_same_origin(origin, placement.spatial_budget.input().translation);
    assert_field_reader(
        &result.export_native().unwrap(),
        origin,
        placement.spatial_budget.frame().input().stored,
        result.options().spatial_error,
    );
    assert_ideal_volume_comparison(result.reports());
}
#[test]
fn chart_boundary_flux_origin_comes_from_its_own_decoded_placement() {
    let mut request = translated_request();
    request.support.chart_origin = 0.3;
    let result = normal_extrude_profile_on_chart(&request).unwrap();
    let origin = result.boundary_flux_origin();
    let placement = result.placement();
    assert_same_origin(origin, placement.spatial_budget.input().translation);
    assert_field_reader(
        &result.export_native().unwrap(),
        origin,
        placement.spatial_budget.frame().input().stored,
        result.options().spatial_error,
    );
    assert_ideal_volume_comparison(result.reports());
}
fn request() -> WrappedNormalRequest {
    let mut request = operation_tests::request();
    request.options.planar_work = PlanarProofWorkLimit::PairBisections(8192);
    request
}
fn translated_request() -> WrappedNormalRequest {
    let mut request = request();
    request.support.origin = [1048576., -2097152., 524288.];
    request.support.axis = [1., 2., 3.];
    request.support.radial = [3., -1., 2.];
    request
}
fn assert_field_reader(
    export: &DetachedWrappedBodies,
    origin: [f64; 3],
    matrix: [[f64; 3]; 3],
    epsilon: f64,
) {
    assert_same_origin(origin, decoded_origin(export));
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Vertex(SurfaceVertexId(10)),
        role: HistoryRole::FirstDepthVertex,
    };
    let point = BRepTool::pnt(&vertex(&export.entities[&key]).unwrap()).unwrap();
    for (i, row) in matrix.iter().enumerate() {
        let field = (point.coordinates()[i] - origin[i]) / 3.;
        assert!((field - 3. * row[0]).abs() <= epsilon);
    }
}
fn assert_ideal_volume_comparison(
    reports: &std::collections::BTreeMap<SurfaceRegionId, WrappedMemberReport>,
) {
    for (region, expected) in [(5, 11.10375), (100, 3.15)] {
        let measures = &reports[&SurfaceRegionId(region)].measures;
        let flux = measures.boundary_flux;
        let discrepancy = measures.retained_ideal_discrepancy.upper;
        assert!(flux.lower - discrepancy <= expected && flux.upper + discrepancy >= expected);
        assert!(measures.ideal_volume.lower <= expected && measures.ideal_volume.upper >= expected);
    }
}

fn decoded_origin(export: &DetachedWrappedBodies) -> [f64; 3] {
    *export
        .bodies
        .location()
        .transformation()
        .translation_part()
        .coordinates()
}
fn assert_same_origin(actual: [f64; 3], expected: [f64; 3]) {
    assert_eq!(actual.map(f64::to_bits), expected.map(f64::to_bits));
}
