//! Preserve cylinder arithmetic while supplying the shared native sweep pipeline.
use super::{admission::Admitted, frame, sweep::Sweep};

pub(super) fn prepare(admitted: &Admitted) -> Sweep {
    let rectangle = admitted.rectangle;
    let first_depth = usize::from(admitted.radii[1] < admitted.radii[0]);
    Sweep {
        region: rectangle.region,
        corners: rectangle.corners,
        boundaries: rectangle.boundaries,
        options: admitted.options,
        origin: rectangle.support.origin,
        axis: rectangle.support.axis,
        angle: admitted.angle,
        points: points(admitted, first_depth),
        first_depth,
        expected_volume: admitted.expected_volume,
        surface_area: surface_area(admitted),
        coordinate_error: admitted.coordinate_error,
    }
}
fn points(admitted: &Admitted, first_depth: usize) -> [[f64; 3]; 4] {
    let inner = admitted.radii[first_depth];
    let outer = admitted.radii[1 - first_depth];
    let [low, high] = admitted.rectangle.z;
    [(inner, low), (outer, low), (outer, high), (inner, high)]
        .map(|(radius, z)| frame::point(&admitted.rectangle.support, admitted.start, radius, z))
}
fn surface_area(admitted: &Admitted) -> f64 {
    let height = admitted.rectangle.z[1] - admitted.rectangle.z[0];
    let thickness = admitted.depths[1] - admitted.depths[0];
    let sum = admitted.radii[0] + admitted.radii[1];
    admitted.angle * sum * (height + thickness) + 2. * height * thickness
}
