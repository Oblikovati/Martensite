//! The two process-wide settings of `BRepLib`: the
//! default precision of the makers and the plane of the 2d edges.
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::GeomPlane;
use martensite_tkmath::gp::{self, Axis3};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex, PoisonError};

// `static double thePrecision = Precision::Confusion();` as IEEE bits, so that a set
// from any thread is seen by the next read, as the native (unsynchronized) static.
static THE_PRECISION: AtomicU64 = AtomicU64::new(precision::confusion().to_bits());

// `static occ::handle<Geom_Plane> thePlane;`: null until the first read.
static THE_PLANE: LazyLock<Mutex<Handle<GeomPlane>>> =
    LazyLock::new(|| Mutex::new(Handle::default()));

/// `BRepLib::Precision(P)`: sets the default precision of the makers;
/// `brep_lib::set_precision(1e-6)`.
pub fn set_precision(value: f64) {
    THE_PRECISION.store(value.to_bits(), Ordering::SeqCst);
}

/// `BRepLib::Precision()`: the default precision (Precision::Confusion() until set);
/// `assert_eq!(martensite_tktopalgo::brep_lib::precision(), 1e-7)`.
pub fn precision() -> f64 {
    f64::from_bits(THE_PRECISION.load(Ordering::SeqCst))
}

/// `BRepLib::Plane(P)`: sets the plane of the 2d makers (a null handle resets it to
/// the default at the next read); `brep_lib::set_plane(&plane)`.
pub fn set_plane(plane: &Handle<GeomPlane>) {
    *THE_PLANE.lock().unwrap_or_else(PoisonError::into_inner) = plane.clone();
}

/// `BRepLib::Plane()`: the current plane, created as `Geom_Plane(gp::XOY())` when null;
/// the returned handle shares the stored plane;
/// `assert!(martensite_tktopalgo::brep_lib::plane().is_some())`.
pub fn plane() -> Handle<GeomPlane> {
    let mut the_plane = THE_PLANE.lock().unwrap_or_else(PoisonError::into_inner);
    if the_plane.is_null() {
        *the_plane = Handle::new(GeomPlane::new(Axis3::from_axis2(gp::xoy())));
    }
    the_plane.clone()
}
