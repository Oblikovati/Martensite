//! Seam pcurve reconstruction from BOPToolsAlgoTools3D.
mod origin;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topods::{Edge, Face},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkmath::gp::Vector2;
pub use origin::split_seam_from_origin;
fn install_pair(
    edge: &Edge,
    face: &Face,
    curve: &Handle<dyn Geom2dCurve>,
    range: (f64, f64),
    translation: Vector2,
    first_original: bool,
) -> Result<(), Failure> {
    let original = trimmed_copy(curve, range)?;
    let translated: Handle<dyn Geom2dCurve> =
        trimmed_copy(curve, range)?.translated(translation)?.cast();
    let pair = if first_original {
        (&original, &translated)
    } else {
        (&translated, &original)
    };
    BRepBuilder::default().update_edge_pcurves_on_face(
        edge,
        pair,
        face,
        BRepTool::edge_tolerance(edge)?,
    )
}
fn trimmed_copy(
    curve: &Handle<dyn Geom2dCurve>,
    range: (f64, f64),
) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let copy: Handle<dyn Geom2dCurve> = curve.copy()?.cast();
    Ok(Handle::new(Geom2dTrimmedCurve::new(
        &copy, range.0, range.1, true, true,
    )?)
    .cast())
}

mod periodic;
pub use periodic::split_seam_on_face;
