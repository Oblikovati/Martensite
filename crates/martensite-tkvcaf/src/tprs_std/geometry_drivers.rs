//! `TPrsStdGeometryDriver` (the geometry of a `TDataXtdGeometry` or of a named shape as
//! an `AisPoint`, `AisLine`, `AisCircle` or the `AisShape` of an ellipse edge) and
//! `TPrsStdNamedShapeDriver` (the `AisShape` of a named shape).
use super::TPrsStdDriver;
use super::construction_drivers::{point_object, presentation_driver, refresh_kept_object};
use martensite_tkbrep::topods::Shape;
use martensite_tkcaf::tdata_xtd::{TDataXtdGeometry, TDataXtdGeometryEnum};
use martensite_tkcaf::tnaming::{TNamingNamedShape, TNamingTool};
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCircle, GeomLine};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::gp::{Circle3, Ellipse3, Line3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use martensite_tkv3d::ais::{
    AisCircle, AisInteractiveObject, AisLine, AisShape, AisShapeParameters,
};
use martensite_tkv3d::prs_dim::prs_dim_units_api;
use martensite_tkv3d::prs_mgr::{PrsMgrPresentableObject, PrsMgrPresentableObjectMembers};
use martensite_tkv3d::select_mgr::SelectMgrSelectableObjectMembers;

presentation_driver!(
    /// `TPrsStdGeometryDriver`: the presentation of a point, line, circle or ellipse.
    TPrsStdGeometryDriver,
    "TPrsStdGeometryDriver"
);
presentation_driver!(
    /// `TPrsStdNamedShapeDriver`: the `AisShape` of the label's named shape.
    TPrsStdNamedShapeDriver,
    "TPrsStdNamedShapeDriver"
);

/// The red of every geometry presentation.
fn red() -> Color {
    Color::named(NamedColor::RED)
}

impl TPrsStdGeometryDriver {
    /// The geometry type: the `TDataXtdGeometry`'s, else the named shape's; `None`
    /// without either attribute.
    fn geometry_type(label: &TdfLabel) -> Result<Option<TDataXtdGeometryEnum>, Failure> {
        let mut geometry: Handle<TDataXtdGeometry> = Handle::default();
        if label.find_attribute_as(&TDataXtdGeometry::get_id(), &mut geometry)? {
            return Ok(Some(
                martensite_tklcaf::live(&geometry, "TPrsStdGeometryDriver")?.get_type(),
            ));
        }
        let mut named: Handle<TNamingNamedShape> = Handle::default();
        if label.find_attribute_as(&TNamingNamedShape::get_id(), &mut named)? {
            return Ok(Some(TDataXtdGeometry::type_of_label(label)?));
        }
        Ok(None)
    }
}

impl TPrsStdDriver for TPrsStdGeometryDriver {
    /// `Update`: red point, red infinite line, red circle or the red edge of an ellipse.
    fn update(
        &self,
        label: &TdfLabel,
        ais: &mut Handle<dyn AisInteractiveObject>,
    ) -> Result<bool, Failure> {
        let Some(kind) = Self::geometry_type(label)? else {
            return Ok(false);
        };
        let object = match kind {
            TDataXtdGeometryEnum::Point => point_presentation(label, ais)?,
            TDataXtdGeometryEnum::Line => line_presentation(label, ais)?,
            TDataXtdGeometryEnum::Circle => circle_presentation(label, ais)?,
            TDataXtdGeometryEnum::Ellipse => ellipse_presentation(label, ais)?,
            _ => None,
        };
        let Some(object) = object else {
            return Ok(false);
        };
        *ais = object;
        Ok(true)
    }
}

/// `TDataXtdPOINT`: a red `AisPoint`.
fn point_presentation(
    label: &TdfLabel,
    ais: &Handle<dyn AisInteractiveObject>,
) -> Result<Option<Handle<dyn AisInteractiveObject>>, Failure> {
    let mut point = Point3::default();
    if !TDataXtdGeometry::point_of_label(label, &mut point)? {
        return Ok(None);
    }
    let object: Handle<dyn AisInteractiveObject> = point_object(ais, point)?.cast();
    martensite_tklcaf::live(&object, "TPrsStdGeometryDriver")?.set_color(red())?;
    Ok(Some(object))
}

/// `TDataXtdLINE`: a red infinite `AisLine`.
fn line_presentation(
    label: &TdfLabel,
    ais: &Handle<dyn AisInteractiveObject>,
) -> Result<Option<Handle<dyn AisInteractiveObject>>, Failure> {
    let mut line = Line3::default();
    if !TDataXtdGeometry::line_of_label(label, &mut line)? {
        return Ok(None);
    }
    let geometry = Handle::new(GeomLine::from_lin(line));
    let kept = ais.cast::<AisLine>();
    let object = match kept.get() {
        Some(object) => {
            object.set_line(&geometry);
            refresh_kept_object(object)?;
            kept.clone()
        }
        None => AisLine::new(&geometry, prs_dim_units_api()),
    };
    object.set_color(red())?;
    object.set_infinite_state(true);
    Ok(Some(object.cast()))
}

/// `TDataXtdCIRCLE`: a red `AisCircle`.
fn circle_presentation(
    label: &TdfLabel,
    ais: &Handle<dyn AisInteractiveObject>,
) -> Result<Option<Handle<dyn AisInteractiveObject>>, Failure> {
    let mut circle = Circle3::default();
    if !TDataXtdGeometry::circle_of_label(label, &mut circle)? {
        return Ok(None);
    }
    let geometry = Handle::new(GeomCircle::from_circ(circle));
    let kept = ais.cast::<AisCircle>();
    let object = match kept.get() {
        Some(object) => {
            object.set_circle(&geometry);
            refresh_kept_object(object)?;
            kept.clone()
        }
        None => AisCircle::new(&geometry),
    };
    object.set_color(red())?;
    Ok(Some(object.cast()))
}

/// `TDataXtdELLIPSE`: the red `AisShape` of the edge of the ellipse.
fn ellipse_presentation(
    label: &TdfLabel,
    ais: &Handle<dyn AisInteractiveObject>,
) -> Result<Option<Handle<dyn AisInteractiveObject>>, Failure> {
    let mut ellipse = Ellipse3::default();
    if !TDataXtdGeometry::ellipse_of_label(label, &mut ellipse)? {
        return Ok(None);
    }
    let mut maker = BRepBuilderAPIMakeEdge::from_elips(ellipse)?;
    if !maker.is_done() {
        return Ok(None);
    }
    let edge = maker.shape()?.clone();
    let kept = ais.cast::<AisShape>();
    let object = match kept.get() {
        Some(object) => {
            object.reset_transformation()?;
            object.set(&edge);
            object.set_to_update_all();
            object.update_selection(-1);
            kept.clone()
        }
        None => AisShape::new(&edge),
    };
    object.set_color(red())?;
    Ok(Some(object.cast()))
}

/// `Shape::Infinite()` (the shape is not null here).
fn shape_is_infinite(shape: &Shape) -> bool {
    shape.tshape().get().is_some_and(|tshape| tshape.infinite())
}

impl TPrsStdDriver for TPrsStdNamedShapeDriver {
    /// `Update`: the `AisShape` of `TNamingTool::GetShape` (false for an empty named
    /// shape). A kept object takes the new shape only when it differs (the native
    /// `OPTIM_UPDATE` branch is compiled out). Any non-null incoming object causes the
    /// result to receive the shape's infinite state, including a wrong-type replacement.
    fn update(
        &self,
        label: &TdfLabel,
        ais: &mut Handle<dyn AisInteractiveObject>,
    ) -> Result<bool, Failure> {
        let mut named: Handle<TNamingNamedShape> = Handle::default();
        if !label.find_attribute_as(&TNamingNamedShape::get_id(), &mut named)? {
            return Ok(false);
        }
        let shape = TNamingTool::get_shape(&named)?;
        if shape.is_null() {
            return Ok(false);
        }
        let kept = ais.cast::<AisShape>();
        let object = match kept.get() {
            Some(object) => {
                if object.shape() != shape {
                    object.reset_transformation()?;
                    object.set(&shape);
                    object.update_selection(-1);
                    object.set_to_update_all();
                }
                kept.clone()
            }
            None => AisShape::new(&shape),
        };
        // At, SetInfiniteState belongs to the non-null input branch, not
        // only the successful downcast branch. Null input retains AisShape defaults.
        if !ais.is_null() {
            object.set_infinite_state(shape_is_infinite(&shape));
        }
        *ais = object.cast();
        Ok(true)
    }
}
