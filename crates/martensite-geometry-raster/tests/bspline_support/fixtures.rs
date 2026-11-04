//! Closed forms, repeated knots, unclamped knots and a periodic control net.
use super::spline_api::Look;
use martensite_raster_core::{MarkerShape, PaletteColor as Color, PaletteKind as Kind};
use martensite_tkernel::{standard::Handle, storage::IoStream};
use martensite_tkg2d::geom2d::Geom2dBSplineCurve;
use martensite_tkg3d::geom::GeomBSplineCurve;
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
pub const LABELS: [&str; 4] = ["degree1", "c0", "unclamped", "periodic"];
const SPACE: [&str; 4] = [
    "7 0 0 1 3 3 0 0 0 2 4 0 4 0 0 0 2 1 1 2 2",
    "7 0 0 2 5 3 0 0 0 1 2 0 2 2 0 3 -2 0 4 0 0 0 3 1 2 2 3",
    "7 0 0 2 5 8 0 0 0 2 4 0 4 0 0 6 -4 0 8 0 0 -2 1 -1 1 0 1 1 1 2 1 3 1 4 1 5 1",
    "7 0 1 2 4 5 -5 0 0 0 5 0 5 0 0 0 -5 0 0 1 1 1 2 1 3 1 4 1",
];
const PLANE: [&str; 4] = [
    "7 0 0 1 3 3 0 0 2 4 4 0 0 2 1 1 2 2",
    "7 0 0 2 5 3 0 0 1 2 2 2 3 -2 4 0 0 3 1 2 2 3",
    "7 0 0 2 5 8 0 0 2 4 4 0 6 -4 8 0 -2 1 -1 1 0 1 1 1 2 1 3 1 4 1 5 1",
    "7 0 1 2 4 5 -5 0 0 5 5 0 0 -5 0 1 1 1 2 1 3 1 4 1",
];
pub fn space(index: usize) -> Handle<GeomBSplineCurve> {
    martensite_tkgeombase::geom_tools::read_curve(&mut IoStream::from_bytes(
        SPACE[index].as_bytes().to_vec(),
    ))
    .cast()
}
pub fn plane(index: usize) -> Handle<Geom2dBSplineCurve> {
    martensite_tkgeombase::geom_tools::read_curve2d(&mut IoStream::from_bytes(
        PLANE[index].as_bytes().to_vec(),
    ))
    .cast()
}
pub fn look() -> Look {
    Look {
        curve_color: Color::new(Kind::Jaune),
        poles_color: Color::new(Kind::Rouge),
        knots_color: Color::new(Kind::Violet),
        knots_shape: MarkerShape::Losange,
        knots_size: 5,
        show_poles: true,
        show_knots: true,
    }
}
