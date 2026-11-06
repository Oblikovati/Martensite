//! The 2D geometry of the edges of a face, written into their surface_curve entities.
use super::TopoDSToStepTool;
use crate::geom_to_step::{GeomToStepMakeCurve, empty_step_name, shared_array1};
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::{step_array1_item, step_array1_set, step_live, step_new_array1};
use crate::step_geom::{
    StepGeomCurve, StepGeomGeometricRepresentationContextAndParametricRepresentationContext,
    StepGeomPcurve, StepGeomPcurveOrSurface, StepGeomSeamCurve, StepGeomSurface,
    StepGeomSurfaceCurve,
};
use crate::step_repr::{StepReprDefinitionalRepresentation, StepReprRepresentationItem};
use crate::step_shape::StepShapeEdgeCurve;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Face};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkg2d::geom2d::{
    Geom2dBezierCurve, Geom2dCurve, Geom2dHyperbola, Geom2dParabola, Geom2dTrimmedCurve,
};
use martensite_tkg3d::geom::{
    GeomConicalSurface, GeomCylindricalSurface, GeomRectangularTrimmedSurface,
    GeomSphericalSurface, GeomSurface, GeomToroidalSurface,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::geom_convert::Units;
use martensite_tkgeombase::geom2d_convert;
use martensite_tkmath::convert::ParameterisationType;

/// Hyperbolas and parabolas trimmed on an elementary surface, trimmed curves and Bezier curves as
/// B-splines: what `the unit conversion` and STEP accept.
fn prepared_pcurve(
    pcurve: Handle<dyn Geom2dCurve>,
    range: (f64, f64),
    surface: &Handle<dyn GeomSurface>,
) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let mut curve = pcurve;
    let on_elementary = surface.cast::<GeomSphericalSurface>().is_some()
        || surface.cast::<GeomCylindricalSurface>().is_some()
        || surface.cast::<GeomConicalSurface>().is_some()
        || surface.cast::<GeomToroidalSurface>().is_some();
    let conic =
        curve.cast::<Geom2dHyperbola>().is_some() || curve.cast::<Geom2dParabola>().is_some();
    if conic && on_elementary {
        curve = Handle::new(Geom2dTrimmedCurve::new(
            &curve, range.0, range.1, true, true,
        )?)
        .cast();
    }
    if curve.cast::<Geom2dTrimmedCurve>().is_some() || curve.cast::<Geom2dBezierCurve>().is_some() {
        curve =
            geom2d_convert::curve_to_bspline_curve(&curve, ParameterisationType::TgtThetaOver2)?
                .cast();
    }
    Ok(curve)
}

/// The pcurve entity on `surface_entity`.
fn pcurve_entity(
    curve: &Handle<dyn Geom2dCurve>,
    surface_entity: &Handle<StepGeomSurface>,
    factors: &StepDataFactors,
) -> Result<Handle<StepGeomPcurve>, Failure> {
    let made = GeomToStepMakeCurve::from_geom2d(curve, factors)?.value()?;
    let items = shared_array1::<Handle<StepReprRepresentationItem>>(vec![made.cast()]);
    let context = Handle::new(
        StepGeomGeometricRepresentationContextAndParametricRepresentationContext::new(),
    );
    let identifier = Handle::new(HAsciiString::new("2D SPACE"));
    context.init_int(&identifier, &empty_step_name(), 2);
    let name = empty_step_name();
    let representation = Handle::new(StepReprDefinitionalRepresentation::new());
    representation.init(&name, &items, &context.cast());
    let pcurve = Handle::new(StepGeomPcurve::new());
    pcurve.init(&name, surface_entity, &representation);
    Ok(pcurve)
}

/// Records `pcurve` on the surface_curve: :a8 abv 13 Feb 98: allow seam to have two different
/// pcurves.
fn record_pcurve(
    surface_curve: &Handle<StepGeomSurfaceCurve>,
    pcurve: &Handle<StepGeomPcurve>,
) -> Result<(), Failure> {
    const WHERE: &str = "TopoDSToStepMakeStepFace";
    let live = step_live(surface_curve, WHERE)?;
    let mut geom = live.associated_geometry();
    if geom.is_null() {
        geom = step_new_array1::<StepGeomPcurveOrSurface>(1, 2)?;
    }
    let mut select = StepGeomPcurveOrSurface::default();
    select.set_value(&pcurve.cast())?;
    let is_seam = surface_curve.cast::<StepGeomSeamCurve>().is_some();
    if step_array1_item(&geom, 1, WHERE)?.is_null() {
        step_array1_set(&geom, 1, select.clone(), WHERE)?;
        if is_seam {
            // c est au moins ca
            step_array1_set(&geom, 2, select, WHERE)?;
        }
    } else if step_array1_item(&geom, 2, WHERE)?.is_null() || is_seam {
        step_array1_set(&geom, 2, select, WHERE)?;
    }
    live.set_associated_geometry(&geom);
    Ok(())
}

/// The pcurve of one edge on the face, when the edge has both.
fn translate_edge_pcurve(
    edge: &Edge,
    face: &Face,
    surface: &Handle<dyn GeomSurface>,
    surface_entity: &Handle<StepGeomSurface>,
    tool: &TopoDSToStepTool,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let found = BRepTool::curve_on_face(edge, face)?;
    // The edge 2D Geometry degenerates in 3D
    // The edge 2D geometry is not mapped onto any Step entity
    // (ProStep agreement)
    let Some(stored) = found.pcurve.get() else {
        return Ok(());
    };
    if BRepTool::degenerated(edge)? {
        return Ok(());
    }
    // for writing VERTEX_LOOP
    if !tool.is_bound(edge) {
        return Ok(());
    }
    let edge_curve: Handle<StepShapeEdgeCurve> = tool.find(edge)?.cast();
    let geometry: Handle<StepGeomCurve> =
        step_live(&edge_curve, "TopoDSToStepMakeStepFace")?.edge_geometry();
    if geometry.is_null() {
        return Ok(());
    }
    // Copy the Curve2d which might be changed
    let copy: Handle<dyn Geom2dCurve> = stored.copy()?.cast();
    let curve = prepared_pcurve(copy, (found.first, found.last), surface)?;
    // if the Surface is a RectangularTrimmedSurface,
    // use the BasisSurface.
    //   CKY  23 SEP 1996: on reste en Radian car on code des Radians
    //    sauf que ca ne marche pas bien ...
    let basis = match surface.cast::<GeomRectangularTrimmedSurface>().get() {
        Some(trimmed) => trimmed.basis_surface(),
        None => surface.clone(),
    };
    let mapped = Units::radian_to_degree(
        &curve,
        &basis,
        factors.length_factor(),
        factors.factor_radian_degree(),
    )?;
    let pcurve = pcurve_entity(&mapped, surface_entity, factors)?;
    record_pcurve(&geometry.cast(), &pcurve)
}

/// Translate the Edge 2D Geometry (pcurves): exploration of all the edges in the current face.
pub(super) fn translate_face_pcurves(
    forward_face: &Face,
    surface: &Handle<dyn GeomSurface>,
    surface_entity: &Handle<StepGeomSurface>,
    tool: &TopoDSToStepTool,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let mut edges = Explorer::new(forward_face, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while edges.more() {
        let edge = topods::edge(edges.current())?;
        translate_edge_pcurve(&edge, forward_face, surface, surface_entity, tool, factors)?;
        edges.next()?;
    }
    Ok(())
}
