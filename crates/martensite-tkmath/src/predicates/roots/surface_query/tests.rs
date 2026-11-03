//! Independent affine and annular maps distinguish sufficient proofs from sampled estimates.
use super::*;
fn affine() -> SurfaceQueryInput {
    SurfaceQueryInput {
        periodic_u: false,
        patches: vec![SurfaceQueryPatch {
            degree: [1, 1],
            poles: vec![[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [1., 1., 0.]],
            weights: vec![1.; 4],
        }],
    }
}
fn certificate(input: SurfaceQueryInput) -> SurfaceQueryReport {
    certify_surface_query(input, SurfaceQueryPolicy::bounded(), || false)
}
#[test]
fn affine_tensor_has_exact_global_projection_embedding() {
    let report = certificate(affine());
    assert!(matches!(
        report.certificate(),
        Some(SurfaceQueryTheorem::StrictProjection {
            coordinates: [0, 1],
            signs: [1, 1]
        })
    ));
    assert!(report.work().positive_hulls >= 4);
    assert!(report.incomplete().is_none());
}
#[test]
fn reflected_affine_and_subnormal_positive_scale_preserve_injectivity() {
    for scale in [-1., f64::from_bits(1)] {
        let mut input = affine();
        for point in &mut input.patches[0].poles {
            point[0] *= scale;
        }
        assert!(certificate(input).certificate().is_some());
    }
}
#[test]
fn collapsed_coordinate_and_zero_or_nan_weights_do_not_certify() {
    let mut input = affine();
    for point in &mut input.patches[0].poles {
        point[0] = 0.;
    }
    assert!(certificate(input).certificate().is_none());
    for weight in [0., -1., f64::NAN, f64::INFINITY] {
        let mut input = affine();
        input.patches[0].weights[0] = weight;
        assert!(matches!(
            certificate(input).incomplete(),
            Some(SurfaceQueryCause::Numeric {
                field: "weight",
                ..
            })
        ));
    }
}
fn annulus() -> SurfaceQueryInput {
    let rows = [
        [[1., 0.], [1., 2.], [-0.5, 1.]],
        [[-0.5, 1.], [-2., 0.], [-0.5, -1.]],
        [[-0.5, -1.], [1., -2.], [1., 0.]],
    ];
    let patches = rows.into_iter().map(annular_patch).collect();
    SurfaceQueryInput {
        patches,
        periodic_u: true,
    }
}
fn annular_patch(rows: [[f64; 2]; 3]) -> SurfaceQueryPatch {
    let poles = rows
        .into_iter()
        .flat_map(|p| [[p[0], p[1], 0.], [2. * p[0], 2. * p[1], 1.]])
        .collect();
    SurfaceQueryPatch {
        degree: [2, 1],
        poles,
        weights: vec![1.; 6],
    }
}
#[test]
fn annular_three_chart_cycle_proves_exact_seams_and_degree_one_winding() {
    assert_eq!(
        certificate(annulus()).certificate(),
        Some(&SurfaceQueryTheorem::AxialAngularCycle)
    );
}
#[test]
fn equal_nearby_seam_geometry_is_not_exact_alias_identity() {
    let mut input = annulus();
    input.patches[2].poles[0][0] += 1. / 1024.;
    assert!(certificate(input).certificate().is_none());
}
#[test]
fn reversing_winding_or_removing_axial_separation_refuses() {
    for axis in [1, 2] {
        let mut input = annulus();
        for patch in &mut input.patches {
            for pole in &mut patch.poles {
                pole[axis] *= if axis == 1 { -1. } else { 0. };
            }
        }
        assert!(certificate(input).certificate().is_none());
    }
}
#[test]
fn unsupported_layout_and_zero_declared_budgets_remain_incomplete() {
    let mut input = affine();
    input.patches[0].degree = [3, 1];
    assert!(matches!(
        certificate(input).incomplete(),
        Some(SurfaceQueryCause::Layout {
            field: "degree",
            ..
        })
    ));
    let policy = SurfaceQueryPolicy {
        max_steps: 0,
        ..SurfaceQueryPolicy::bounded()
    };
    assert!(matches!(
        certify_surface_query(affine(), policy, || false).incomplete(),
        Some(SurfaceQueryCause::Limit { field: "steps", .. })
    ));
}
#[test]
fn coefficient_limit_and_cancellation_preserve_typed_noncompletion() {
    let policy = SurfaceQueryPolicy {
        max_coefficient_bits: 0,
        ..SurfaceQueryPolicy::bounded()
    };
    assert!(matches!(
        certify_surface_query(affine(), policy, || false).incomplete(),
        Some(SurfaceQueryCause::Limit {
            field: "coefficient bits",
            ..
        })
    ));
    assert_eq!(
        certify_surface_query(affine(), SurfaceQueryPolicy::bounded(), || true).incomplete(),
        Some(&SurfaceQueryCause::Cancelled)
    );
}
#[test]
fn report_owns_coefficients_and_is_send_sync_without_locks() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<SurfaceQueryReport>();
    let mut input = affine();
    let report = certificate(input.clone());
    input.patches[0].poles[0][0] = 100.;
    assert_eq!(report.input().patches[0].poles[0], [0., 0., 0.]);
    assert_eq!(report, certificate(affine()));
}
#[test]
fn quadratic_fold_with_regular_endpoints_does_not_certify() {
    let poles = [1., -1., 1.]
        .into_iter()
        .flat_map(|x| [[x, 0., 0.], [x, 1., 0.]])
        .collect();
    let input = SurfaceQueryInput {
        periodic_u: false,
        patches: vec![SurfaceQueryPatch {
            degree: [2, 1],
            poles,
            weights: vec![1.; 6],
        }],
    };
    assert!(certificate(input).certificate().is_none());
}
#[test]
fn axial_dependence_cannot_be_hidden_by_an_injective_angular_projection() {
    let mut input = annulus();
    input.patches[0].poles[2][2] += 1. / 1024.;
    assert!(matches!(
        certificate(input).incomplete(),
        Some(SurfaceQueryCause::Unproved {
            relation: "U-independent axial coordinate",
            ..
        })
    ));
}
