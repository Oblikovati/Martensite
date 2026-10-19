//! Retained development-test Cartesian circle fixture using only public APIs.
#![allow(dead_code)]
/// A direct Cartesian circle fixture; `modeling_circle(Point2::default(), 2.0)`.
pub fn modeling_circle(
    center: martensite_tkmath::gp::Point2,
    radius: f64,
) -> martensite_tkernel::standard::Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve> {
    use martensite_tkernel::standard::Handle;
    use martensite_tkg2d::geom2d::Geom2dCircle;
    use martensite_tkmath::gp::{Axis22d, Direction2};
    let frame = Axis22d::with_directions(center, Direction2::X, Direction2::Y).unwrap();
    Handle::new(Geom2dCircle::new(frame, radius).unwrap()).cast()
}
