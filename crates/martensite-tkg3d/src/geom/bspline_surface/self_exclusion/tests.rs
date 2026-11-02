use super::*;
use crate::geom::{GeomSurface, KnotVector};
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkmath::gp::Point3;
fn array<T: Copy + Default>(values: &[T]) -> Array1<'static, T> {
    let mut result = Array1::with_bounds(1, values.len() as i32).unwrap();
    result.as_mut_slice().copy_from_slice(values);
    result
}
fn poles() -> Array2<'static, Point3> {
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for ((u, v), [x, y, z]) in [(1, 1), (2, 1), (2, 2), (1, 2)].into_iter().zip([
        [0., 0., 0.],
        [2., 0., 1.],
        [3., 3., 2.],
        [0., 2., -1.],
    ]) {
        poles.set_value(u, v, Point3::new(x, y, z)).unwrap();
    }
    poles
}
fn surface(weights: Option<[f64; 4]>) -> GeomBSplineSurface {
    let poles = poles();
    let uk = array(&[2., 6.]);
    let vk = array(&[-3., 5.]);
    let mults = array(&[2, 2]);
    let Some(values) = weights else {
        return GeomBSplineSurface::new(
            &poles,
            (&uk, &mults),
            (&vk, &mults),
            (1, 1),
            (false, false),
        )
        .unwrap();
    };
    weighted_surface(&poles, values, (&uk, &mults), (&vk, &mults))
}
fn weighted_surface(
    poles: &Array2<'_, Point3>,
    values: [f64; 4],
    u: KnotVector<'_, '_>,
    v: KnotVector<'_, '_>,
) -> GeomBSplineSurface {
    let mut weights = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for ((row, col), value) in [(1, 1), (2, 1), (2, 2), (1, 2)].into_iter().zip(values) {
        weights.set_value(row, col, value).unwrap();
    }
    GeomBSplineSurface::with_weights(poles, &weights, u, v, (1, 1), (false, false)).unwrap()
}

fn certificate(surface: &GeomBSplineSurface) -> BilinearSplineCertificate {
    match surface.bilinear_self_exclusion().unwrap() {
        BilinearSplineExclusion::Certified(proof) => proof,
        other => panic!("expected clamped bilinear support certificate, got {other:?}"),
    }
}
#[test]
fn real_spline_adapter_preserves_pole_order_domain_and_evaluation() {
    let surface = surface(None);
    let proof = certificate(&surface);
    assert_eq!(proof.input().domain(), [[2., 6.], [-3., 5.]]);
    assert_eq!(
        proof.input().corners(),
        [[0., 0., 0.], [2., 0., 1.], [3., 3., 2.], [0., 2., -1.]]
    );
    for (s, t) in [(0., 0.), (1., 1.), (0.25, 0.5), (0.5, 0.75)] {
        assert_bilinear_value(&surface, s, t);
    }
}
fn assert_bilinear_value(surface: &GeomBSplineSurface, s: f64, t: f64) {
    let actual = surface.value(2. + 4. * s, -3. + 8. * t).unwrap();
    let expected = Point3::new(2. * s + s * t, 2. * t + s * t, s - t + 2. * s * t);
    assert!(actual.distance(&expected) < 1e-12);
}
#[test]
fn snapshot_remains_owned_after_source_mutation_and_rebinding_refuses_fold() {
    let surface = surface(None);
    let before = certificate(&surface);
    surface.set_pole(2, 2, Point3::new(-2., -2., 0.)).unwrap();
    let current = surface.bilinear_self_exclusion().unwrap();
    let BilinearSplineExclusion::Inconclusive(BilinearSplineInconclusive::Patch { input, .. }) =
        current
    else {
        panic!("expected unsupported strict projection after fold, got {current:?}");
    };
    assert!(!before.input().same_representation(&input));
    assert_eq!(before.input().corners()[2], [3., 3., 2.]);
}
#[test]
fn common_weights_are_supported_and_nonconstant_weights_are_inconclusive() {
    assert_eq!(
        certificate(&surface(Some([7.; 4]))).input().corners(),
        certificate(&surface(None)).input().corners()
    );
    assert!(matches!(surface(Some([1.,2.,1.,1.])).bilinear_self_exclusion().unwrap(),
        BilinearSplineExclusion::Inconclusive(BilinearSplineInconclusive::Patch {
            reason:martensite_tkmath::predicates::bilinear_patch::BilinearInconclusive::NonConstantWeights,.. })));
}
#[test]
fn actual_higher_degree_surface_is_not_treated_as_bilinear() {
    let mut poles = Array2::with_bounds(1, 3, 1, 2).unwrap();
    for u in 1..=3 {
        for v in 1..=2 {
            poles
                .set_value(u, v, Point3::new(f64::from(u), f64::from(v), 0.))
                .unwrap();
        }
    }
    let knots = array(&[0., 1.]);
    let um = array(&[3, 3]);
    let vm = array(&[2, 2]);
    let surface =
        GeomBSplineSurface::new(&poles, (&knots, &um), (&knots, &vm), (2, 1), (false, false))
            .unwrap();
    assert!(matches!(
        surface.bilinear_self_exclusion().unwrap(),
        BilinearSplineExclusion::Inconclusive(BilinearSplineInconclusive::UnsupportedLayout(_))
    ));
}
#[test]
fn invalid_stored_coordinates_are_errors_not_inconclusive_results() {
    let surface = surface(None);
    surface
        .set_pole(1, 1, Point3::new(f64::NAN, 0., 0.))
        .unwrap();
    assert!(matches!(
        surface.bilinear_self_exclusion(),
        Err(BilinearSplineError::StoredValue {
            component: "pole coordinate",
            index: 0,
            ..
        })
    ));
}
#[test]
fn all_adapter_conclusions_are_send_and_sync() {
    fn owned_traits<T: Send + Sync>() {}
    owned_traits::<BilinearSplineCertificate>();
    owned_traits::<BilinearSplineExclusion>();
    owned_traits::<BilinearSplineInconclusive>();
    owned_traits::<BilinearSplineLayout>();
    owned_traits::<BilinearSplineError>();
}
#[test]
fn actual_periodic_support_is_not_admitted() {
    let surface = periodic_surface();
    let actual = surface.bilinear_self_exclusion().unwrap();
    assert!(matches!(
        actual,
        BilinearSplineExclusion::Inconclusive(BilinearSplineInconclusive::UnsupportedLayout(
            BilinearSplineLayout {
                periodic: [true, false],
                ..
            }
        ))
    ));
}
fn periodic_surface() -> GeomBSplineSurface {
    let poles = poles();
    let uknots = array(&[0., 1., 2.]);
    let umults = array(&[1, 1, 1]);
    let vknots = array(&[0., 1.]);
    let vmults = array(&[2, 2]);
    GeomBSplineSurface::new(
        &poles,
        (&uknots, &umults),
        (&vknots, &vmults),
        (1, 1),
        (true, false),
    )
    .unwrap()
}

#[test]
fn stored_input_errors_retain_offending_values_and_expectations() {
    let error = BilinearSplineError::StoredValue {
        component: "weight",
        index: 2,
        value: -1.,
        positive: true,
    };
    let message = error.to_string();
    assert!(message.contains("weight index 2 = -1"));
    assert!(message.contains("positive finite"));
}
#[test]
fn folded_public_evaluator_override_refuses_a_supported_stored_net() {
    use crate::geom_eval::RepSurfaceDescFull;
    use martensite_tkernel::standard::Handle;
    let surface = surface(None);
    let old = certificate(&surface);
    let folded = folded_override();
    surface
        .set_eval_representation(Handle::new(RepSurfaceDescFull::new(folded)).cast())
        .unwrap();
    let first = surface.value(2.5, -2.).unwrap();
    assert_eq!(first.distance(&surface.value(3.5, 0.).unwrap()), 0.);
    assert!(!old.layout().evaluation_override);
    assert_eq!(old.input().corners()[2], [3., 3., 2.]);
    assert_override_refused(&surface);
}
fn assert_override_refused(surface: &GeomBSplineSurface) {
    assert!(matches!(
        surface.bilinear_self_exclusion().unwrap(),
        BilinearSplineExclusion::Inconclusive(BilinearSplineInconclusive::EvaluationOverride(
            BilinearSplineLayout {
                evaluation_override: true,
                ..
            }
        ))
    ));
}
fn folded_override() -> martensite_tkernel::standard::Handle<dyn GeomSurface> {
    use martensite_tkernel::standard::Handle;
    let folded = surface(None);
    folded.set_pole(2, 2, Point3::new(-2., -2., 0.)).unwrap();
    Handle::new(folded).cast()
}
