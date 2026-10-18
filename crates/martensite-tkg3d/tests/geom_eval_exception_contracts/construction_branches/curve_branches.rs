use super::*;
#[test]
fn validation_branches_and_order() {
    check_cases(CASES);
}
const CASES: &[RejectedCase] = &[
    (
        "TBezierCurve",
        "number of poles must be odd and >= 3",
        || GeomEvalTBezierCurve::new(&Array1::with_bounds(1, 1).unwrap(), 0.).map(|_| ()),
    ),
    (
        "TBezierCurve",
        "number of poles must be odd and >= 3",
        || GeomEvalTBezierCurve::new(&Array1::with_bounds(1, 2).unwrap(), 0.).map(|_| ()),
    ),
    (
        "TBezierCurve",
        "number of poles must be odd and >= 3",
        || GeomEvalTBezierCurve::new(&Array1::with_bounds(1, 4).unwrap(), 0.).map(|_| ()),
    ),
    ("TBezierCurve", "alpha must be > 0", || {
        GeomEvalTBezierCurve::new(&curve_poles(), 0.).map(|_| ())
    }),
    ("AHTBezierCurve", "algebraic degree must be >= 0", || {
        GeomEvalAHTBezierCurve::new(&curve_poles(), -1, -1., -1.).map(|_| ())
    }),
    ("AHTBezierCurve", "alpha must be >= 0", || {
        GeomEvalAHTBezierCurve::new(&curve_poles(), 2, -1., -1.).map(|_| ())
    }),
    ("AHTBezierCurve", "beta must be >= 0", || {
        GeomEvalAHTBezierCurve::new(&curve_poles(), 2, 0., -1.).map(|_| ())
    }),
    (
        "AHTBezierCurve",
        "NbPoles must equal basisDimension",
        || GeomEvalAHTBezierCurve::new(&curve_poles(), 1, 0., 0.).map(|_| ()),
    ),
    (
        "TBezierCurve",
        "number of poles must be odd and >= 3",
        || {
            GeomEvalTBezierCurve::with_weights(
                &Array1::with_bounds(1, 1).unwrap(),
                &weights(2, 0.),
                0.,
            )
            .map(|_| ())
        },
    ),
    (
        "TBezierCurve",
        "number of poles must be odd and >= 3",
        || {
            GeomEvalTBezierCurve::with_weights(
                &Array1::with_bounds(1, 2).unwrap(),
                &weights(2, 0.),
                0.,
            )
            .map(|_| ())
        },
    ),
    (
        "TBezierCurve",
        "number of poles must be odd and >= 3",
        || {
            GeomEvalTBezierCurve::with_weights(
                &Array1::with_bounds(1, 4).unwrap(),
                &weights(2, 0.),
                0.,
            )
            .map(|_| ())
        },
    ),
    ("TBezierCurve", "alpha must be > 0", || {
        GeomEvalTBezierCurve::with_weights(&curve_poles(), &weights(2, 0.), 0.).map(|_| ())
    }),
    ("AHTBezierCurve", "algebraic degree must be >= 0", || {
        GeomEvalAHTBezierCurve::with_weights(&curve_poles(), &weights(2, 0.), -1, -1., -1.)
            .map(|_| ())
    }),
    ("AHTBezierCurve", "alpha must be >= 0", || {
        GeomEvalAHTBezierCurve::with_weights(&curve_poles(), &weights(2, 0.), 2, -1., -1.)
            .map(|_| ())
    }),
    ("AHTBezierCurve", "beta must be >= 0", || {
        GeomEvalAHTBezierCurve::with_weights(&curve_poles(), &weights(2, 0.), 2, 0., -1.)
            .map(|_| ())
    }),
    (
        "AHTBezierCurve",
        "NbPoles must equal basisDimension",
        || {
            GeomEvalAHTBezierCurve::with_weights(&curve_poles(), &weights(2, 0.), 1, 0., 0.)
                .map(|_| ())
        },
    ),
    ("TBezierCurve", "weights size must equal poles size", || {
        GeomEvalTBezierCurve::with_weights(&curve_poles(), &weights(2, 0.), 1.).map(|_| ())
    }),
    ("TBezierCurve", "all weights must be > 0", || {
        GeomEvalTBezierCurve::with_weights(&curve_poles(), &weights(3, 0.), 1.).map(|_| ())
    }),
    (
        "AHTBezierCurve",
        "weights array size must match poles array size",
        || {
            GeomEvalAHTBezierCurve::with_weights(&curve_poles(), &weights(2, 0.), 2, 0., 0.)
                .map(|_| ())
        },
    ),
    ("AHTBezierCurve", "all weights must be > 0", || {
        GeomEvalAHTBezierCurve::with_weights(&curve_poles(), &weights(3, 0.), 2, 0., 0.).map(|_| ())
    }),
];
