use super::{fixed, fixtures};
use martensite_tkbrep::topods::Shape;
use martensite_tktopalgo::brep_gprop::{
    BRepGProp, SurfaceErrorEvidence, SurfacePropertyOptions, SurfaceRepresentation,
};
use std::f64::consts::PI;

fn area(
    shape: &Shape,
    options: SurfacePropertyOptions,
    expected: f64,
    geometry: usize,
    triangles: usize,
) {
    let report = BRepGProp::surface_properties_with_options(shape, options).unwrap();
    assert!((report.properties().mass() - expected).abs() < 1e-12);
    assert_eq!(
        (report.geometric_faces(), report.triangulated_faces()),
        (geometry, triangles)
    );
    assert_eq!(report.effective_relative_target(), None);
    assert_eq!(report.error_evidence(), SurfaceErrorEvidence::Unknown);
}

#[test]
fn fixed_policy_distinguishes_geometric_mesh_only_and_mixed_sources() {
    let plain = fixtures::cylinder_face();
    let (meshed, _) = fixtures::meshed_cylinder();
    let (triangle, _) = fixtures::triangle_face();
    let prefer = fixed(SurfaceRepresentation::PreferExistingMesh);
    area(&plain, prefer, PI, 1, 0);
    area(&meshed, prefer, 2.0_f64.sqrt() * 2.0, 0, 1);
    area(&triangle, prefer, 1.0, 0, 1);
    let mixed = fixtures::compound(&[plain.into(), meshed.into(), triangle.into()]);
    area(&mixed, prefer, PI + 2.0_f64.sqrt() * 2.0 + 1.0, 1, 2);
    area(
        &mixed,
        fixed(SurfaceRepresentation::PreferGeometry),
        2.0 * PI + 1.0,
        2,
        1,
    );
}

#[test]
fn preference_falls_back_and_adaptive_mesh_measurements_do_not_claim_an_error_estimate() {
    let (triangle, _) = fixtures::triangle_face();
    let report =
        BRepGProp::surface_properties_with_options(&triangle, SurfacePropertyOptions::default())
            .unwrap();
    assert!((report.properties().mass() - 1.0).abs() < 1e-12);
    assert_eq!(report.error_evidence(), SurfaceErrorEvidence::Unknown);
    assert_eq!(
        (report.geometric_faces(), report.triangulated_faces()),
        (0, 1)
    );
}

#[test]
fn shared_and_reversed_face_occurrences_preserve_area_and_skip_policy() {
    let (face, _) = fixtures::meshed_cylinder();
    let forward: Shape = face.into();
    let reversed = forward.reversed();
    let repeated = fixtures::compound(&[forward.clone(), reversed, forward]);
    let mut options = fixed(SurfaceRepresentation::PreferExistingMesh);
    area(&repeated, options, 6.0 * 2.0_f64.sqrt(), 0, 3);
    options.skip_shared = true;
    area(&repeated, options, 2.0 * 2.0_f64.sqrt(), 0, 1);
}
