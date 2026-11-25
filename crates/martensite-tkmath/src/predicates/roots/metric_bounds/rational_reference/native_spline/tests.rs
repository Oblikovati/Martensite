// SPDX-License-Identifier: GPL-2.0-only
use super::*;

const SOURCE: NativeSplineSource<'_> = NativeSplineSource {
    degree: 2,
    poles: &[[5., 0.], [5., 1.], [4.5, 2.], [4., 2.]],
    weights: Some(&[1., 1., 1.5, 1.75]),
    knots: &[-2., -2., -2., 1., 7., 7., 7.],
};

#[test]
fn inserts_non_dyadic_homogeneous_knot_with_one_shared_enclosure() {
    let spans = lower_native_bspline(SOURCE, Default::default(), || false).unwrap();
    assert_eq!(spans.len(), 2);
    assert_eq!(spans[0].interval, [-2., 1.]);
    assert_eq!(spans[1].interval, [1., 7.]);
    assert_eq!(spans[0].poles[0], SOURCE.poles[0]);
    assert_eq!(spans[1].poles[2], SOURCE.poles[3]);
    assert_eq!(
        spans[0].poles[2].map(f64::to_bits),
        spans[1].poles[0].map(f64::to_bits)
    );
    assert_eq!(
        spans[0].control_enclosures[2],
        spans[1].control_enclosures[0]
    );
    for ([lo, hi], exact) in spans[0].control_enclosures[2]
        .into_iter()
        .zip([67. / 14., 10. / 7.])
    {
        assert!(lo <= exact && exact <= hi);
    }
    let weight = |span: usize, pole: usize| spans[span].weights.as_ref().unwrap()[pole];
    assert_eq!(weight(0, 2).to_bits(), weight(1, 0).to_bits());
    assert!((weight(0, 2) - 7. / 6.).abs() < 3e-16);
    for span in spans {
        assert!(span.transport.position > 0. && span.transport.position < 1e-13);
        assert!(span.transport.derivative > 0. && span.transport.derivative < 1e-12);
        assert!(span.area_transfer > 0. && span.area_transfer < 1e-12);
        assert!(span.transport.operations > 0);
    }
}

#[test]
fn preserves_polynomial_splines_and_finite_typed_refusals() {
    let polynomial = NativeSplineSource {
        weights: None,
        ..SOURCE
    };
    let spans = lower_native_bspline(polynomial, Default::default(), || false).unwrap();
    assert!(spans.iter().all(|span| span.weights.is_none()));
    assert!((spans[0].poles[2][0] - 29. / 6.).abs() < 2e-15);
    let options = CylindricalProfileOptions {
        max_operations: 10,
        ..Default::default()
    };
    assert!(matches!(
        lower_native_bspline(SOURCE, options, || false),
        Err(NativeSplineFailure::Reference(
            RationalReferenceFailure::Arithmetic(BoundedArithmeticFailure::WorkLimit {
                limit: 10,
                ..
            })
        ))
    ));
    assert!(matches!(
        lower_native_bspline(SOURCE, Default::default(), || true),
        Err(NativeSplineFailure::Reference(
            RationalReferenceFailure::Arithmetic(BoundedArithmeticFailure::Cancelled)
        ))
    ));
    let discontinuous = NativeSplineSource {
        knots: &[-2., -2., -2., -2., 7., 7., 7.],
        ..SOURCE
    };
    assert!(matches!(
        lower_native_bspline(discontinuous, Default::default(), || false),
        Err(NativeSplineFailure::Knots { .. })
    ));
}

#[test]
fn oversized_pole_and_flat_knot_counts_refuse_before_coefficient_reads() {
    let poles = [[f64::NAN; 2]; 66];
    let knots = [f64::NAN; 68];
    let source = NativeSplineSource {
        degree: 1,
        poles: &poles,
        weights: None,
        knots: &knots,
    };
    assert!(matches!(
        source.check_extent(),
        Err(NativeSplineFailure::SpanCount {
            spans: 65,
            limit: 64
        })
    ));
    assert!(matches!(
        lower_native_bspline(source, Default::default(), || false),
        Err(NativeSplineFailure::SpanCount {
            spans: 65,
            limit: 64
        })
    ));
    assert!(matches!(
        lower_native_bspline(source, Default::default(), || true),
        Err(NativeSplineFailure::Reference(
            RationalReferenceFailure::Arithmetic(BoundedArithmeticFailure::Cancelled)
        ))
    ));
}
