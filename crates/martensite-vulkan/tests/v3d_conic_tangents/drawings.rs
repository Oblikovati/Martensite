use super::*;
use martensite_geometry_raster::{CurveLook, PlanarCurveDrawable};
use martensite_raster_core::{PaletteColor, PaletteKind, drawable::RasterDrawable};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use martensite_tkv3d::ais::{AisInteractiveObject, AisShape};
use std::rc::Rc;
pub(super) type NamedPlanarCurves = Vec<(String, Rc<PlanarCurveDrawable>)>;

pub(super) fn bind_planar_curve(
    bindings: &mut NamedPlanarCurves,
    name: &str,
    curve: &Handle<dyn Geom2dCurve>,
) {
    // The command creates fresh defaults, not the mutable session parameter table.
    let drawing = Rc::new(PlanarCurveDrawable::with_look(
        curve.clone(),
        CurveLook::new(PaletteColor::new(PaletteKind::Jaune)),
        30,
    ));
    drawing.set_visible(true);
    bindings.push((name.to_owned(), drawing));
}

pub(super) fn display_reembedded_curve(
    session: &ViewerSession,
    curve: &Handle<dyn Geom2dCurve>,
    source: &geometry::SpatialConics,
) -> Result<(), Failure> {
    let spatial = geom_api::to_3d(curve, &source.plane.pln())?;
    let mut maker = BRepBuilderAPIMakeEdge::from_curve(&spatial)?;
    let shape = maker.shape()?.clone();
    let object: Handle<dyn AisInteractiveObject> = AisShape::new(&shape).cast();
    session.context().display(&object, false)
}
