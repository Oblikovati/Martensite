use super::*;
use martensite_tkmath::gp::{Direction2, Point2};

#[test]
fn rotated_analytic_hole_conic_is_bounded_by_its_stored_axes() {
    let frame = Axis22d::new(Point2::new(4., 5.), Direction2::new(3., 4.).unwrap(), true).unwrap();
    let ellipse = Geom2dEllipse::new(frame, 0.5, 0.25).unwrap();
    let curve = Handle::new(ellipse).cast();
    let hull = bounds(&curve, [0., std::f64::consts::TAU]).ok().unwrap();
    for t in [0., 0.75, 1.5, 2.25, 3., 3.75, 4.5, 5.25, 6.] {
        let point = curve.value(t).unwrap();
        assert!(
            [point.x(), point.y()]
                .into_iter()
                .zip(hull.0)
                .all(|(value, [lo, hi])| lo <= value && value <= hi)
        );
    }
}
