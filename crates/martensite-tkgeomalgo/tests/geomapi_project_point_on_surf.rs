//! The tight-bound initialization regression of `GeomAPI_ProjectPointOnSurf`.
#![allow(non_snake_case, clippy::excessive_precision)]
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomCylindricalSurface, GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkgeomalgo::geom_api::ProjectPointOnSurf;
use martensite_tkgeombase::extrema::ExtAlgo;
use martensite_tkmath::gp::{self, Point3};

// calling Init() with explicit UV bounds then Perform() must not throw.
// The bounds passed to Init() are intentionally tighter (U:[0,3]) than the trimmed surface
// (U:[0,4]) to reproduce the original bug scenario.
#[test]
fn GeomAPI_ProjectPointOnSurfTest__Bug867_InitWithTightBoundsNoException() {
    let aCyl: Handle<dyn GeomSurface> =
        Handle::new(GeomCylindricalSurface::new(gp::xoy().axis3(), 10.0).unwrap()).cast();
    let aTrimSurf: Handle<dyn GeomSurface> = Handle::new(
        GeomRectangularTrimmedSurface::new(&aCyl, (0.0, 4.0), (0.0, 2.0), (true, true)).unwrap(),
    )
    .cast();

    let aPoint = Point3::new(30.0, 30.0, 30.0);

    let mut aProjector = ProjectPointOnSurf::new();
    // EXPECT_NO_THROW
    let result = aProjector
        .init_surface(&aTrimSurf, (0.0, 3.0, 0.0, 2.0), ExtAlgo::Grad)
        .and_then(|()| aProjector.perform(aPoint));
    assert!(result.is_ok());
    // The original bug was that Init()+Perform() threw an exception.
    // Whether a projection exists within the given UV bounds is not the concern here.
}
