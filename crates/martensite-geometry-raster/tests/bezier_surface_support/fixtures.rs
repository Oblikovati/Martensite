//! Quadratic center lies outside the visible surface but inside its control net.
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
use super::surface_api::Look;
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
use martensite_raster_core::{PaletteColor as Color, PaletteKind as Kind};
use martensite_tkernel::{standard::Handle, storage::IoStream};
use martensite_tkg3d::geom::GeomBezierSurface;
pub fn geometry(index: usize) -> Handle<GeomBezierSurface> {
    let record = match index {
        0 => "8 0 0 1 1 -5 -5 0 -5 5 0 5 -5 0 5 5 3",
        1 => "8 0 0 2 2 -5 -5 0 -5 0 0 -5 5 0 0 -5 0 0 8 4 0 5 0 5 -5 0 5 0 0 5 5 0",
        _ => panic!("Bezier surface fixture {index}: expected 0..2"),
    };
    martensite_tkgeombase::geom_tools::read_surface(&mut IoStream::from_bytes(
        record.as_bytes().to_vec(),
    ))
    .cast()
}
#[cfg(all(target_os = "linux", feature = "x11-integration-tests"))]
pub fn look() -> Look {
    Look {
        nb_u_isos: 1,
        nb_v_isos: 1,
        bounds_color: Color::new(Kind::Jaune),
        isos_color: Color::new(Kind::Bleu),
    }
}
