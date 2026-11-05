//! Faces built on restored geometry records: natural-bound faces of surfaces and of swept pipe
//! surfaces (`mkface`, `tuyau`; OCC31631).
use super::prelude::*;
use crate::dataset_shape::{read_dataset_curve, read_dataset_surface};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkgeomalgo::geom_fill::GeomFillPipe;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

/// `mkface`: the face bounded by the natural limits of `surface`.
fn natural_face(surface: &Handle<dyn GeomSurface>) -> Result<Shape, Box<dyn Error>> {
    let maker = BRepBuilderAPIMakeFace::from_surface(surface, precision::confusion())?;
    Ok(maker.face()?.into_shape())
}

/// The natural face of the surface of geometry file `name`; `surface_face("s.rle")?`.
pub(crate) fn surface_face(name: &str) -> Result<Shape, Box<dyn Error>> {
    natural_face(&read_dataset_surface(name)?)
}

/// `tuyau res path c1 c2 radius`: the pipe of `radius` along `path` between two rail curves,
/// performed with parameters; a null surface when the pipe is not done.
fn rail_pipe(
    path: &str,
    first: &str,
    second: &str,
    radius: f64,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    let (path, first, second) = (
        read_dataset_curve(path)?,
        read_dataset_curve(first)?,
        read_dataset_curve(second)?,
    );
    let mut pipe = GeomFillPipe::with_rails(&path, &first, &second, radius)?;
    pipe.perform_with_parameters(true, false)?;
    if !pipe.is_done() {
        return Ok(Handle::default());
    }
    Ok(pipe.surface())
}

/// `bug31631`: pipes between the two rails in either order give faces of the same area.
pub(crate) fn rail_pipe_faces() -> Result<(), Box<dyn Error>> {
    banner(&[
        "============",
        "0031631: Modeling Algorithms - Invalid result of Pipe builder",
        "============",
        "",
    ]);
    let (path, sup1, sup2) = ("bug31631_1.brep", "bug31631_2.brep", "bug31631_3.brep");
    let res1 = rail_pipe(path, sup1, sup2, 6.35001275)?;
    let res2 = rail_pipe(path, sup2, sup1, 6.35001275)?;
    let face1 = natural_face(&res1)?;
    let face2 = natural_face(&res2)?;
    check_mass(&face1, &MassCheck::near(MassKind::Area, 352.49))?;
    check_mass(&face2, &MassCheck::near(MassKind::Area, 352.49))?;
    checkview_3d(&face1, "bug31631.png")
}
