//! Exact finite-curve fixtures shared by the original extrema and view regressions.
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCircle, GeomCurve, GeomLine, GeomTrimmedCurve};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};

/// Finite parallel lines for bug26269_1; `let curves = parallel_lines();`.
pub fn parallel_lines() -> [Handle<dyn GeomCurve>; 2] {
    let curves = [Point3::default(), Point3::new(2., 2., 0.)].map(|origin| {
        Handle::new(GeomLine::from_point(origin, Direction3::X)).cast::<dyn GeomCurve>()
    });
    trimmed_pair(curves, [(0., 1.); 2])
}

/// Disjoint concentric arcs for bug26269_2; `let curves = concentric_arcs();`.
pub fn concentric_arcs() -> [Handle<dyn GeomCurve>; 2] {
    trimmed_pair(circle_pair([10., 5.]), [(0., 1.), (3., 4.)])
}

/// Asymmetric concentric half-arcs for bug29712_34; `let curves = asymmetric_half_arcs();`.
pub fn asymmetric_half_arcs() -> [Handle<dyn GeomCurve>; 2] {
    let pi = std::f64::consts::PI;
    trimmed_pair(circle_pair([100., 50.]), [(0., pi), (-2., 0.)])
}

/// Opposite concentric half-arcs for bug29712_35; `let curves = opposite_half_arcs();`.
pub fn opposite_half_arcs() -> [Handle<dyn GeomCurve>; 2] {
    let pi = std::f64::consts::PI;
    trimmed_pair(circle_pair([100., 50.]), [(0., pi), (pi, 2. * pi)])
}

fn circle_pair(radii: [f64; 2]) -> [Handle<dyn GeomCurve>; 2] {
    radii.map(|radius| {
        Handle::new(GeomCircle::new(Axis2::default(), radius).unwrap()).cast::<dyn GeomCurve>()
    })
}

fn trimmed_pair(
    curves: [Handle<dyn GeomCurve>; 2],
    ranges: [(f64, f64); 2],
) -> [Handle<dyn GeomCurve>; 2] {
    std::array::from_fn(|index| {
        let (first, last) = ranges[index];
        Handle::new(GeomTrimmedCurve::new(&curves[index], first, last, true, true).unwrap()).cast()
    })
}
