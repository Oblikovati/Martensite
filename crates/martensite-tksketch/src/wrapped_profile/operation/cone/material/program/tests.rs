//! Rational TKBO evidence exercises a positive authored loan in the shared ledger.
use super::*;
use martensite_tkbo::profile_extrusion::{
    PlanarProofWorkLimit, PlanarRegionWorkResult, ProfileCurve, ProfileCurveId, ProfilePlane,
    ProfileVertex, ProfileVertexId, SketchProfile,
    build_planar_regions_with_integration_budget_and_cancel,
};

#[test]
fn actual_authored_cells_are_charged_once_and_reduce_the_next_loan() {
    let options = CylindricalFluxOptions::default();
    let limit = u64::from(options.max_total_subintervals);
    let region = SurfaceRegionId(1);
    let result = issue(limit).map_err(|cause| planar::PlanarFailure::WorkCapture(Box::new(cause)));
    let (pair, cells) = observed(&result);
    let captured = result.unwrap();
    let proof = captured.integral_program_evidence().unwrap();
    assert!(cells > 0);
    assert_eq!(proof.granted_subintervals(), limit);
    assert_eq!(authored_cells(Some(proof), region, limit).unwrap(), cells);
    assert!(authored_cells(Some(proof), region, limit - 1).is_err());

    let mut work = storage::Work::new(ConeClearanceOptions::default());
    charge(&mut work, region, pair, cells, options).unwrap();
    assert_eq!(
        u64::from(work.integration(options).max_total_subintervals),
        limit - cells
    );
    assert_eq!(work.planar_receipt(), Some(captured.proof_work()));
}

#[test]
fn rational_material_cannot_widen_an_empty_authored_loan() {
    let error = issue(0).unwrap_err();
    let proof = error.integral_program_failure().unwrap();
    assert_eq!(proof.entry_allowance(), 0);
    assert_eq!(proof.granted_subintervals(), 0);
    assert_eq!(proof.integration_subintervals(), 0);
    assert!(matches!(
        proof.numerical_cause().unwrap().cause,
        IntegralProgramFailure::AggregateWork { limit: 0, .. }
    ));
}

fn issue(loan: u64) -> PlanarRegionWorkResult {
    build_planar_regions_with_integration_budget_and_cancel(
        &rational_circle(),
        pipeline::resolution(1e-6),
        PlanarProofWorkLimit::CurrentDefault,
        PlanarProofVersion::GeometricCompositeV6 {
            max_components: 64,
            max_total_subintervals: 8192,
        },
        loan,
        || false,
    )
}

fn rational_circle() -> SketchProfile {
    let vertices = [[1., 0.], [0., 1.], [-1., 0.], [0., -1.]];
    let corners = [[1., 1.], [-1., 1.], [-1., -1.], [1., -1.]];
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: vertices
            .iter()
            .enumerate()
            .map(|(i, point)| ProfileVertex {
                id: ProfileVertexId(i as u64),
                point: *point,
            })
            .collect(),
        curves: (0..4)
            .map(|i| {
                let j = (i + 1) % 4;
                ProfileCurve::Spline {
                    id: ProfileCurveId(i as u64),
                    start: ProfileVertexId(i as u64),
                    end: ProfileVertexId(j as u64),
                    degree: 2,
                    poles: vec![vertices[i], corners[i], vertices[j]],
                    weights: Some(vec![1., 1., 2.]),
                    knots: vec![0., 0., 0., 1., 1., 1.],
                }
            })
            .collect(),
    }
}
