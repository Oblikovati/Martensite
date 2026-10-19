use super::*;
#[test]
fn validation_branches_and_order() {
    check_cases(CASES);
}
const CASES: &[RejectedCase] = &[
    (
        "TBezierSurface",
        "U pole count must be odd and >= 3",
        || {
            GeomEvalTBezierSurface::new(&Array2::with_bounds(1, 2, 1, 2).unwrap(), 0., 0.)
                .map(|_| ())
        },
    ),
    (
        "TBezierSurface",
        "V pole count must be odd and >= 3",
        || {
            GeomEvalTBezierSurface::new(&Array2::with_bounds(1, 3, 1, 2).unwrap(), 0., 0.)
                .map(|_| ())
        },
    ),
    ("TBezierSurface", "alphaU must be > 0", || {
        GeomEvalTBezierSurface::new(&Array2::with_bounds(1, 3, 1, 3).unwrap(), 0., 0.).map(|_| ())
    }),
    ("TBezierSurface", "alphaV must be > 0", || {
        GeomEvalTBezierSurface::new(&Array2::with_bounds(1, 3, 1, 3).unwrap(), 1., 0.).map(|_| ())
    }),
    ("AHTBezierSurface", "algebraic degree must be >= 0", || {
        GeomEvalAHTBezierSurface::new(&surface_poles(), [-1, 2], [-1., -1.], [-1., -1.]).map(|_| ())
    }),
    ("AHTBezierSurface", "algebraic degree must be >= 0", || {
        GeomEvalAHTBezierSurface::new(&surface_poles(), [2, -1], [-1., -1.], [-1., -1.]).map(|_| ())
    }),
    ("AHTBezierSurface", "alpha must be >= 0", || {
        GeomEvalAHTBezierSurface::new(&surface_poles(), [2, 2], [-1., 0.], [-1., -1.]).map(|_| ())
    }),
    ("AHTBezierSurface", "alpha must be >= 0", || {
        GeomEvalAHTBezierSurface::new(&surface_poles(), [2, 2], [0., -1.], [-1., -1.]).map(|_| ())
    }),
    ("AHTBezierSurface", "beta must be >= 0", || {
        GeomEvalAHTBezierSurface::new(&surface_poles(), [2, 2], [0., 0.], [-1., 0.]).map(|_| ())
    }),
    ("AHTBezierSurface", "beta must be >= 0", || {
        GeomEvalAHTBezierSurface::new(&surface_poles(), [2, 2], [0., 0.], [0., -1.]).map(|_| ())
    }),
    (
        "AHTBezierSurface",
        "poles array dimensions must match basis dimensions",
        || GeomEvalAHTBezierSurface::new(&surface_poles(), [1, 1], [0., 0.], [0., 0.]).map(|_| ()),
    ),
    (
        "TBezierSurface",
        "U pole count must be odd and >= 3",
        || {
            GeomEvalTBezierSurface::with_weights(
                &Array2::with_bounds(1, 2, 1, 2).unwrap(),
                &grid_weights(2, 2, 0.),
                0.,
                0.,
            )
            .map(|_| ())
        },
    ),
    (
        "TBezierSurface",
        "V pole count must be odd and >= 3",
        || {
            GeomEvalTBezierSurface::with_weights(
                &Array2::with_bounds(1, 3, 1, 2).unwrap(),
                &grid_weights(2, 2, 0.),
                0.,
                0.,
            )
            .map(|_| ())
        },
    ),
    ("TBezierSurface", "alphaU must be > 0", || {
        GeomEvalTBezierSurface::with_weights(
            &Array2::with_bounds(1, 3, 1, 3).unwrap(),
            &grid_weights(2, 2, 0.),
            0.,
            0.,
        )
        .map(|_| ())
    }),
    ("TBezierSurface", "alphaV must be > 0", || {
        GeomEvalTBezierSurface::with_weights(
            &Array2::with_bounds(1, 3, 1, 3).unwrap(),
            &grid_weights(2, 2, 0.),
            1.,
            0.,
        )
        .map(|_| ())
    }),
    ("AHTBezierSurface", "algebraic degree must be >= 0", || {
        GeomEvalAHTBezierSurface::with_weights(
            &surface_poles(),
            &grid_weights(2, 2, 0.),
            [-1, 2],
            [-1., -1.],
            [-1., -1.],
        )
        .map(|_| ())
    }),
    ("AHTBezierSurface", "algebraic degree must be >= 0", || {
        GeomEvalAHTBezierSurface::with_weights(
            &surface_poles(),
            &grid_weights(2, 2, 0.),
            [2, -1],
            [-1., -1.],
            [-1., -1.],
        )
        .map(|_| ())
    }),
    ("AHTBezierSurface", "alpha must be >= 0", || {
        GeomEvalAHTBezierSurface::with_weights(
            &surface_poles(),
            &grid_weights(2, 2, 0.),
            [2, 2],
            [-1., 0.],
            [-1., -1.],
        )
        .map(|_| ())
    }),
    ("AHTBezierSurface", "alpha must be >= 0", || {
        GeomEvalAHTBezierSurface::with_weights(
            &surface_poles(),
            &grid_weights(2, 2, 0.),
            [2, 2],
            [0., -1.],
            [-1., -1.],
        )
        .map(|_| ())
    }),
    ("AHTBezierSurface", "beta must be >= 0", || {
        GeomEvalAHTBezierSurface::with_weights(
            &surface_poles(),
            &grid_weights(2, 2, 0.),
            [2, 2],
            [0., 0.],
            [-1., 0.],
        )
        .map(|_| ())
    }),
    ("AHTBezierSurface", "beta must be >= 0", || {
        GeomEvalAHTBezierSurface::with_weights(
            &surface_poles(),
            &grid_weights(2, 2, 0.),
            [2, 2],
            [0., 0.],
            [0., -1.],
        )
        .map(|_| ())
    }),
    (
        "AHTBezierSurface",
        "poles array dimensions must match basis dimensions",
        || {
            GeomEvalAHTBezierSurface::with_weights(
                &surface_poles(),
                &grid_weights(2, 2, 0.),
                [1, 1],
                [0., 0.],
                [0., 0.],
            )
            .map(|_| ())
        },
    ),
    (
        "TBezierSurface",
        "weights dimensions must match poles dimensions",
        || {
            GeomEvalTBezierSurface::with_weights(&surface_poles(), &grid_weights(2, 3, 0.), 1., 1.)
                .map(|_| ())
        },
    ),
    ("TBezierSurface", "all weights must be > 0", || {
        GeomEvalTBezierSurface::with_weights(&surface_poles(), &grid_weights(3, 3, 0.), 1., 1.)
            .map(|_| ())
    }),
    (
        "AHTBezierSurface",
        "weights array dimensions must match poles",
        || {
            GeomEvalAHTBezierSurface::with_weights(
                &surface_poles(),
                &grid_weights(2, 3, 0.),
                [2; 2],
                [0.; 2],
                [0.; 2],
            )
            .map(|_| ())
        },
    ),
    ("AHTBezierSurface", "all weights must be > 0", || {
        GeomEvalAHTBezierSurface::with_weights(
            &surface_poles(),
            &grid_weights(3, 3, 0.),
            [2; 2],
            [0.; 2],
            [0.; 2],
        )
        .map(|_| ())
    }),
];
