//! Far-from-boundary points use an independent ideal frame and expected Boolean membership.
use super::*;
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_class3d::BRepClass3dSolidClassifier;

pub(super) fn check(body: &Shape, kind: Operation, shift: f64, oblique: bool) {
    for (radius, s, state) in expected(kind, shift) {
        let point = point(radius, s, oblique);
        let actual = BRepClass3dSolidClassifier::with_point(body, &point, 1e-7)
            .unwrap()
            .state();
        assert_eq!(
            actual, state,
            "{kind:?} radius={radius} s={s} oblique={oblique}"
        );
    }
}
fn expected(kind: Operation, shift: f64) -> [(f64, f64, State); 5] {
    let added = if kind == Operation::Fuse {
        State::In
    } else {
        State::Out
    };
    [
        (5., shift + 1., State::In),
        (9.5, shift + 1., added),
        (11., shift + 1., added),
        (13., shift + 1., State::Out),
        (11., shift + 3., State::Out),
    ]
}
fn point(radius: f64, s: f64, oblique: bool) -> Point3 {
    let local = [
        radius * martensite_cmath::cos(s / 10.),
        radius * martensite_cmath::sin(s / 10.),
        2.,
    ];
    if !oblique {
        return Point3::new(local[0], local[1], local[2]);
    }
    let x = [5., -4., 1.].map(|v| v / 42_f64.sqrt());
    let y = [1., 1., -1.].map(|v| v / 3_f64.sqrt());
    let z = [1., 2., 3.].map(|v| v / 14_f64.sqrt());
    let world: [f64; 3] = std::array::from_fn(|i| {
        [1.25, -2.5, 3.75][i] + local[0] * x[i] + local[1] * y[i] + local[2] * z[i]
    });
    Point3::new(world[0], world[1], world[2])
}
