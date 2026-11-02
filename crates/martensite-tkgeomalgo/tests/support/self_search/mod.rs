use martensite_tkernel::{ncollection::Array2, standard::Handle};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dTopolTool, h_surface_tool::Adaptor3dSurfaceHandle},
    geom::{GeomBezierSurface, GeomPlane, GeomSurface},
    geom_adaptor::GeomAdaptorSurface,
};
use martensite_tkgeomalgo::int_patch::IntPatchPrmPrmTolerances;
use martensite_tkmath::gp::Point3;

pub fn tolerances() -> IntPatchPrmPrmTolerances {
    IntPatchPrmPrmTolerances {
        tol_tangency: 1e-7,
        epsilon: 1e-7,
        deflection: 0.01,
        increment: 0.01,
    }
}
pub fn patch(curve: &[(f64, f64)]) -> Adaptor3dSurfaceHandle {
    let mut poles = Array2::with_bounds(1, curve.len() as i32, 1, 2).unwrap();
    for (i, &(x, z)) in curve.iter().enumerate() {
        for j in 1..=2 {
            poles
                .set_value(i as i32 + 1, j, Point3::new(x, f64::from(j - 1), z))
                .unwrap();
        }
    }
    let geometry: Handle<dyn GeomSurface> =
        Handle::new(GeomBezierSurface::new(&poles).unwrap()).cast();
    Handle::new(GeomAdaptorSurface::with_surface(&geometry).unwrap()).cast()
}
pub fn plane_patch() -> Adaptor3dSurfaceHandle {
    patch(&[(0., 0.), (1., 0.)])
}
pub fn crossing_patch() -> Adaptor3dSurfaceHandle {
    // x=s²,z=s³-s, s=3u-1.5: u=1/6 and5/6 meet at x=1,z=0.
    patch(&[
        (2.25, -1.875),
        (-0.75, 3.875),
        (-0.75, -3.875),
        (2.25, 1.875),
    ])
}
pub fn domain(surface: &Adaptor3dSurfaceHandle) -> Adaptor3dTopolTool {
    Adaptor3dTopolTool::with_surface(surface).unwrap()
}

pub fn analytic_plane() -> Adaptor3dSurfaceHandle {
    let geometry: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Default::default())).cast();
    Handle::new(GeomAdaptorSurface::with_bounds(&geometry, (0., 1., 0., 1.)).unwrap()).cast()
}
