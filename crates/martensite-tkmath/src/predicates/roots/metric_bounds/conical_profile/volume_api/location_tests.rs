// SPDX-License-Identifier: GPL-2.0-only
use super::*;
pub(super) fn with_query(body: impl FnOnce(ConicalAuthoredFluxQuery<'_>)) {
    let poles = [
        [[2., 0.], [3., 0.]],
        [[3., 0.], [3., 1.]],
        [[3., 1.], [2., 1.]],
        [[2., 1.], [2., 0.]],
    ];
    let spans: Vec<&[[f64; 2]]> = poles.iter().map(|poles| poles.as_slice()).collect();
    let contours = [ConicalAuthoredContour {
        spans: &spans,
        reverse: false,
    }];
    body(query(&contours));
}
fn query<'a>(contours: &'a [ConicalAuthoredContour<'a>]) -> ConicalAuthoredFluxQuery<'a> {
    ConicalAuthoredFluxQuery {
        contours,
        semi_angle: 0.5,
        depths: [-0.01, 0.02],
        outward: true,
        level: 6,
        clearance_level: 0,
        volume_width: 1.,
        flux: [0., 0.],
        arithmetic: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: 4096,
        },
    }
}
#[test]
fn real_span_work_failure_retains_its_authored_source_position() {
    with_query(|query| {
        let error = certify_conical_authored_flux(query, || false)
            .err()
            .unwrap();
        assert!(matches!(
            &error.cause,
            Failure::Volume(BoundaryFailure::Span { .. })
        ));
        assert!(error.span_location().is_some());
        assert!(error.depth_cause().is_none());
        assert!(matches!(
            error.arithmetic_cause(),
            Some(BoundedArithmeticFailure::WorkLimit {
                completed: 4096,
                limit: 4096
            })
        ));
        assert_eq!(error.operations(), 4096);
    });
}
#[test]
fn real_depth_clearance_failure_shares_the_generic_source_location() {
    with_query(|mut query| {
        query.depths = [-100., -99.];
        query.arithmetic.max_operations = 100_000;
        let error = certify_conical_authored_flux(query, || false)
            .err()
            .unwrap();
        let (contour, span, _) = error.depth_cause().unwrap();
        assert_eq!(error.span_location(), Some((contour, span)));
        assert_eq!(error.span_location(), Some((0, 0)));
    });
}
#[test]
fn real_global_support_failure_does_not_invent_an_authored_entity() {
    with_query(|mut query| {
        query.semi_angle = 0.;
        let error = certify_conical_authored_flux(query, || false)
            .err()
            .unwrap();
        assert!(matches!(
            &error.cause,
            Failure::Volume(BoundaryFailure::InvalidSupport(0.))
        ));
        assert_eq!(error.span_location(), None);
        assert_eq!(error.operations(), 0);
    });
}
