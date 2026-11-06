//! `TopoDSToStepFacetedTool`: whether a shape can be written in a faceted context.
use super::TopoDSToStepFacetedError;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomBezierSurface, GeomPlane, GeomSurface};
use martensite_tkg3d::top_abs::ShapeEnum;

/// Faceted tool.
#[derive(Clone, Copy, Debug, Default)]
pub struct TopoDSToStepFacetedTool;

/// Degree 1 in both directions with two poles in each.
fn is_flat_patch(u_degree: i32, v_degree: i32, u_poles: i32, v_poles: i32) -> bool {
    u_degree == 1 && v_degree == 1 && u_poles == 2 && v_poles == 2
}

/// The surface is a plane or a flat B-spline or Bezier patch.
fn surface_is_flat(surface: &Handle<dyn GeomSurface>) -> bool {
    if surface.cast::<GeomPlane>().is_some() {
        return true;
    }
    if let Some(spline) = surface.cast::<GeomBSplineSurface>().get() {
        return is_flat_patch(
            spline.u_degree(),
            spline.v_degree(),
            spline.nb_u_poles(),
            spline.nb_v_poles(),
        );
    }
    if let Some(bezier) = surface.cast::<GeomBezierSurface>().get() {
        return is_flat_patch(
            bezier.u_degree(),
            bezier.v_degree(),
            bezier.nb_u_poles(),
            bezier.nb_v_poles(),
        );
    }
    false
}

/// `None` keeps the search going.
fn check_pcurve(
    curve: &Handle<dyn Geom2dCurve>,
) -> Result<Option<TopoDSToStepFacetedError>, Failure> {
    if curve.cast::<Geom2dLine>().is_some() {
        // Returns from the first line it meets, leaving later faces unchecked.
        return Ok(Some(TopoDSToStepFacetedError::FacetedDone));
    }
    if let Some(spline) = curve.cast::<Geom2dBSplineCurve>().get() {
        let straight = spline.degree() == 1 && spline.nb_poles() == 2;
        return Ok((!straight).then_some(TopoDSToStepFacetedError::PCurveNotLinear));
    }
    if let Some(bezier) = curve.cast::<Geom2dBezierCurve>().get() {
        let straight = bezier.degree() == 1 && bezier.nb_poles() == 2;
        return Ok((!straight).then_some(TopoDSToStepFacetedError::PCurveNotLinear));
    }
    Ok(Some(TopoDSToStepFacetedError::PCurveNotLinear))
}

/// The verdict of the curves of a face on it; `None` when they leave the search open.
fn check_face_edges(face: &Face) -> Result<Option<TopoDSToStepFacetedError>, Failure> {
    let mut edges = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while edges.more() {
        let edge = topods::edge(edges.current())?;
        edges.next()?;
        let pcurve = BRepTool::curve_on_face(&edge, face)?;
        if let Some(verdict) = check_pcurve(&pcurve.pcurve)? {
            return Ok(Some(verdict));
        }
    }
    Ok(None)
}

impl TopoDSToStepFacetedTool {
    /// `check_topo_ds_shape(shape)`.
    pub fn check_topo_ds_shape(shape: &Shape) -> Result<TopoDSToStepFacetedError, Failure> {
        let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
        while faces.more() {
            let face = topods::face(faces.current())?;
            faces.next()?;
            if !surface_is_flat(&BRepTool::surface(&face)?) {
                return Ok(TopoDSToStepFacetedError::SurfaceNotPlane);
            }
            if let Some(verdict) = check_face_edges(&face)? {
                return Ok(verdict);
            }
        }
        Ok(TopoDSToStepFacetedError::FacetedDone)
    }
}
