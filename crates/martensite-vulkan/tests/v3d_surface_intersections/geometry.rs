use super::*;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkg3d::geom::GeomRectangularTrimmedSurface;
use martensite_tkgeomalgo::geom_int::{GeomIntIntSS, GeomIntIntSSApprox};

pub(super) fn trimmed_inputs() -> Result<[Handle<dyn GeomSurface>; 2], BoxError> {
    let first = crate::dataset_shape::read_dataset_shape("OCC22720-f1.brep")?;
    let second = crate::dataset_shape::read_dataset_shape("OCC22720-f2.brep")?;
    let first = BRepTool::surface(&topods::face(&first)?)?;
    let second = BRepTool::surface(&topods::face(&second)?)?;
    let trimmed = Handle::new(GeomRectangularTrimmedSurface::new(
        &first,
        (0., 2.),
        (0., 2.),
        (true, true),
    )?)
    .cast();
    Ok([trimmed, second])
}

pub(super) fn actual_intersections(
    surfaces: &[Handle<dyn GeomSurface>; 2],
) -> Result<bindings::IntersectionBindings, BoxError> {
    let operation = GeomIntIntSS::with_surfaces(
        &surfaces[0],
        &surfaces[1],
        martensite_tkernel::precision::confusion(),
        GeomIntIntSSApprox::default(),
    )?;
    if !operation.is_done() {
        return Err("No intersections found!".into());
    }
    Ok(bindings::IntersectionBindings::actual(&operation)?)
}

pub(super) fn torus_inputs() -> Result<[Handle<dyn GeomSurface>; 2], BoxError> {
    Ok([
        crate::dataset_shape::read_dataset_surface("OCC604a.draw")?,
        crate::dataset_shape::read_dataset_surface("OCC604b.draw")?,
    ])
}
