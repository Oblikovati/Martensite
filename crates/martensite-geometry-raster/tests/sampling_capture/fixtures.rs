//! Finite closed-form curves and nontrivial piecewise/surface iso dispatch paths.
use martensite_tkernel::{standard::Handle, storage::IoStream};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dIsoCurve, Adaptor3dSurface},
    geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface},
};
use martensite_tkgeombase::geom_tools;
use martensite_tkmath::geom_abs::IsoType;

pub enum Geometry {
    Spatial(GeomAdaptorCurve),
    Planar(Box<Geom2dAdaptorCurve>),
    Iso(Adaptor3dIsoCurve, IsoType, f64, (f64, f64)),
}
impl Geometry {
    pub fn spatial(&self) -> bool {
        !matches!(self, Self::Planar(_))
    }
}
type CurveRecord = (&'static str, &'static str, &'static str, (f64, f64));
pub type CaptureCase = (String, Geometry, i32);
const RECORDS: [CurveRecord; 5] = [
    ("line", "1 0 0 0 1 0 0", "1 0 0 1 0", (-10., 10.)),
    (
        "circle",
        "2 0 0 0 0 0 1 1 0 0 0 1 0 5",
        "2 0 0 1 0 0 1 5",
        (0., std::f64::consts::TAU),
    ),
    (
        "ellipse",
        "3 0 0 0 0 0 1 1 0 0 0 1 0 7 3",
        "3 0 0 1 0 0 1 7 3",
        (0., std::f64::consts::TAU),
    ),
    (
        "bezier",
        "6 0 2 0 0 0 1 2 0 2 0 0",
        "6 0 2 0 0 1 2 2 0",
        (0., 1.),
    ),
    (
        "c0-spline",
        "7 0 0 2 5 3 0 0 0 1 2 0 2 2 0 3 -2 0 4 0 0 0 3 1 2 2 3",
        "7 0 0 2 5 3 0 0 1 2 2 2 3 -2 4 0 0 3 1 2 2 3",
        (0., 2.),
    ),
];
pub fn curve_matrix() -> Vec<CaptureCase> {
    let mut cases = Vec::new();
    for record in RECORDS {
        append_curve_modes(&mut cases, record);
    }
    cases
}
fn append_curve_modes(cases: &mut Vec<CaptureCase>, (label, spatial, planar, range): CurveRecord) {
    for mode in [0, 1] {
        cases.push((
            format!("spatial-{label}-mode{mode}"),
            spatial_curve(spatial, range),
            mode,
        ));
        cases.push((
            format!("planar-{label}-mode{mode}"),
            planar_curve(planar, range),
            mode,
        ));
    }
}
fn spatial_curve(record: &str, range: (f64, f64)) -> Geometry {
    let curve = geom_tools::read_curve(&mut stream(record));
    Geometry::Spatial(GeomAdaptorCurve::with_range(&curve, range.0, range.1).unwrap())
}
fn planar_curve(record: &str, range: (f64, f64)) -> Geometry {
    let curve = geom_tools::read_curve2d(&mut stream(record));
    Geometry::Planar(Box::new(
        Geom2dAdaptorCurve::with_range(&curve, range.0, range.1).unwrap(),
    ))
}
type SurfaceRecord = (&'static str, &'static str, IsoType, f64, (f64, f64));
const SURFACES: [SurfaceRecord; 3] = [
    (
        "plane",
        "1 0 0 0 0 0 1 1 0 0 0 1 0",
        IsoType::IsoU,
        1.,
        (-10., 10.),
    ),
    (
        "bezier",
        "8 0 0 1 1 0 0 0 0 10 0 10 0 0 10 10 5",
        IsoType::IsoU,
        0.25,
        (0., 1.),
    ),
    (
        "bspline",
        "9 0 0 0 0 2 1 4 2 3 2 0 0 0 0 1 0 1 0 0 1 1 1 2 0 0 2 1 0 3 0 0 3 1 0 0 3 1 1 2 3 0 2 1 2",
        IsoType::IsoV,
        0.25,
        (0., 2.),
    ),
];
pub fn iso_matrix() -> Vec<CaptureCase> {
    let mut cases = Vec::new();
    for (label, record, kind, parameter, range) in SURFACES {
        for mode in [0, 1] {
            let surface = geom_tools::read_surface(&mut stream(record));
            let adaptor = GeomAdaptorSurface::with_surface(&surface).unwrap();
            let iso = Adaptor3dIsoCurve::with_surface(
                &Handle::new(adaptor).cast::<dyn Adaptor3dSurface>(),
            );
            cases.push((
                format!("iso-{label}-mode{mode}"),
                Geometry::Iso(iso, kind, parameter, range),
                mode,
            ));
        }
    }
    cases
}
fn stream(record: &str) -> IoStream {
    IoStream::from_bytes(record.as_bytes().to_vec())
}
