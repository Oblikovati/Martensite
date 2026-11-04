//! Analytical and piecewise curve records build actual kernel geometry, not snapshots.
use martensite_geometry_raster::CurveLook;
use martensite_raster_core::{PaletteColor, PaletteKind};
use martensite_tkernel::{standard::Handle, storage::IoStream};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkgeombase::geom_tools;
pub struct Record {
    pub label: &'static str,
    spatial: &'static str,
    planar: &'static str,
}
pub const RECORDS: [Record; 6] = [
    Record {
        label: "finite-line",
        spatial: "8 -10 10 1 0 0 0 1 0 0",
        planar: "8 -10 10 1 0 0 1 0",
    },
    Record {
        label: "infinite-line",
        spatial: "1 0 0 0 1 0 0",
        planar: "1 0 0 1 0",
    },
    Record {
        label: "circle",
        spatial: "2 0 0 0 0 0 1 1 0 0 0 1 0 5",
        planar: "2 0 0 1 0 0 1 5",
    },
    Record {
        label: "ellipse",
        spatial: "3 0 0 0 0 0 1 1 0 0 0 1 0 7 3",
        planar: "3 0 0 1 0 0 1 7 3",
    },
    Record {
        label: "bezier",
        spatial: "6 0 2 0 0 0 1 2 0 2 0 0",
        planar: "6 0 2 0 0 1 2 2 0",
    },
    Record {
        label: "c0-spline",
        spatial: "7 0 0 2 5 3 0 0 0 1 2 0 2 2 0 3 -2 0 4 0 0 0 3 1 2 2 3",
        planar: "7 0 0 2 5 3 0 0 1 2 2 2 3 -2 4 0 0 3 1 2 2 3",
    },
];
pub fn spatial(index: usize) -> Handle<dyn GeomCurve> {
    geom_tools::read_curve(&mut IoStream::from_bytes(
        RECORDS[index].spatial.as_bytes().to_vec(),
    ))
}
pub fn planar(index: usize) -> Handle<dyn Geom2dCurve> {
    geom_tools::read_curve2d(&mut IoStream::from_bytes(
        RECORDS[index].planar.as_bytes().to_vec(),
    ))
}
pub fn red_look() -> CurveLook {
    CurveLook::new(PaletteColor::new(PaletteKind::Rouge))
}
