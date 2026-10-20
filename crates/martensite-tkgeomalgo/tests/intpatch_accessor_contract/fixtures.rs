use martensite_tkernel::standard::Handle;
use martensite_tkg3d::{
    adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle,
    geom::{GeomPlane, GeomSurface},
    geom_adaptor::GeomAdaptorSurface,
};
use martensite_tkgeomalgo::{
    int_patch::{IntPatchArcFunction, IntPatchTheSOnBounds, IntPatchTheSurfFunction},
    int_surf::IntSurfQuadric,
};
use martensite_tkmath::gp::{Axis3, Sphere3};

pub(super) fn plane_square() -> Adaptor3dSurfaceHandle {
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    Handle::new(GeomAdaptorSurface::with_bounds(&plane, (-1.0, 1.0, -1.0, 1.0)).unwrap()).cast()
}

pub(super) fn sphere(radius: f64) -> IntSurfQuadric {
    IntSurfQuadric::from_sphere(&Sphere3::new(Axis3::default(), radius).unwrap())
}

pub(super) fn boundary_search(quadric: IntSurfQuadric) -> IntPatchTheSOnBounds {
    let surface = plane_square();
    let mut function = IntPatchArcFunction::new();
    function.set_surface(&surface);
    function.set_quadric(&quadric);
    let domain = martensite_tkg3d::adaptor3d::Adaptor3dTopolTool::with_surface(&surface).unwrap();
    let mut search = IntPatchTheSOnBounds::new();
    search
        .perform(&mut function, &domain, 1.0e-7, 1.0e-6, false)
        .unwrap();
    search
}

pub(super) fn interior_problem() -> (Adaptor3dSurfaceHandle, IntPatchTheSurfFunction) {
    let surface = plane_square();
    let mut function = IntPatchTheSurfFunction::with_surfaces(&surface, &sphere(0.75));
    function.set_tolerance(1.0e-6);
    (surface, function)
}
