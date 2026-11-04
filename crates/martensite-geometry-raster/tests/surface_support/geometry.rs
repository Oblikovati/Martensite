//! Actual geometric fixtures are independent from surface display controls.
use martensite_tkernel::{precision, standard::Handle, storage::IoStream};
use martensite_tkg3d::geom::{GeomPlane, GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkmath::gp::Axis3;
pub fn plane() -> Handle<dyn GeomSurface> {
    Handle::new(GeomPlane::new(Axis3::default())).cast()
}
pub fn fixture(index: usize) -> Handle<dyn GeomSurface> {
    match index {
        0 => plane(),
        1 => trim((-5., 5.)),
        2 => trim((0., precision::infinite())),
        3 => read("2 0 0 0 0 0 1 1 0 0 0 1 0 5"),
        4 => read("8 0 0 1 1 -5 -5 0 -5 5 0 5 -5 0 5 5 3"),
        5 => read(
            "9 0 0 0 0 2 1 4 2 3 2 0 0 0 0 1 0 1 0 0 1 1 1 2 0 0 2 1 0 3 0 0 3 1 0 0 3 1 1 2 3 0 2 1 2",
        ),
        _ => panic!("surface fixture {index}: expected0..6"),
    }
}
fn read(record: &str) -> Handle<dyn GeomSurface> {
    martensite_tkgeombase::geom_tools::read_surface(&mut IoStream::from_bytes(
        record.as_bytes().to_vec(),
    ))
}
fn trim(u: (f64, f64)) -> Handle<dyn GeomSurface> {
    Handle::new(GeomRectangularTrimmedSurface::new(&plane(), u, (-5., 5.), (true, true)).unwrap())
        .cast()
}
