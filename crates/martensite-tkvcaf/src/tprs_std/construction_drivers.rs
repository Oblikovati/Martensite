//! The drivers of the construction attributes: `TDataXtdAxis` shows as an `AisAxis`,
//! `TDataXtdPlane` as an `AisPlane` and `TDataXtdPoint` as an `AisPoint`. An object of
//! the right type is updated in place; any other object is replaced.
use super::TPrsStdDriver;
use martensite_tkcaf::tdata_xtd::{TDataXtdAxis, TDataXtdGeometry, TDataXtdPlane, TDataXtdPoint};
use martensite_tkcaf::tnaming::{TNamingNamedShape, TNamingTool};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, TransientHandleTarget};
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomLine, GeomPlane};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::gp::{Line3, Plane3, Point3};
use martensite_tkv3d::ais::{AisAxis, AisInteractiveObject, AisPlane, AisPoint};
use martensite_tkv3d::prs_dim::prs_dim_units_api;
use martensite_tkv3d::prs_mgr::{PrsMgrPresentableObject, PrsMgrPresentableObjectMembers};
use martensite_tkv3d::select_mgr::SelectMgrSelectableObjectMembers;

/// Declares a stateless `TPrsStdDriver` subclass with its RTTI and default constructor.
macro_rules! presentation_driver {
    ($(#[$doc:meta])* $name:ident, $class_name:literal) => {
        $(#[$doc])*
        #[derive(Default)]
        pub struct $name;
        martensite_tkernel::impl_transient!($name, $class_name, dyn $crate::tprs_std::TPrsStdDriver,
            interfaces: [dyn $crate::tprs_std::TPrsStdDriver]);
        impl $name {
            #[doc = concat!("`", $class_name, "()`.")]
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
}
pub(super) use presentation_driver;

presentation_driver!(
    /// `TPrsStdAxisDriver`: the `AisAxis` of a `TDataXtdAxis`.
    TPrsStdAxisDriver,
    "TPrsStdAxisDriver"
);
presentation_driver!(
    /// `TPrsStdPlaneDriver`: the `AisPlane` of a `TDataXtdPlane`.
    TPrsStdPlaneDriver,
    "TPrsStdPlaneDriver"
);
presentation_driver!(
    /// `TPrsStdPointDriver`: the `AisPoint` of a `TDataXtdPoint`.
    TPrsStdPointDriver,
    "TPrsStdPointDriver"
);

/// True when the label holds the attribute of that ID.
pub(super) fn label_has<T: RuntimeTyped + TransientHandleTarget + ?Sized>(
    label: &TdfLabel,
    id: &martensite_tkernel::standard::Guid,
) -> Result<bool, Failure> {
    let mut found: Handle<T> = Handle::default();
    label.find_attribute_as(id, &mut found)
}

/// The update step every driver applies to an object it keeps: `ResetTransformation`,
/// `SetToUpdate` (all modes) and `UpdateSelection` (all modes).
pub(super) fn refresh_kept_object<
    O: PrsMgrPresentableObject + SelectMgrSelectableObjectMembers + ?Sized,
>(
    object: &O,
) -> Result<(), Failure> {
    object.reset_transformation()?;
    object.set_to_update_all();
    object.update_selection(-1);
    Ok(())
}

impl TPrsStdDriver for TPrsStdAxisDriver {
    /// `Update`: the label's line (false when its named shape is empty).
    fn update(
        &self,
        label: &TdfLabel,
        ais: &mut Handle<dyn AisInteractiveObject>,
    ) -> Result<bool, Failure> {
        if !label_has::<TDataXtdAxis>(label, &TDataXtdAxis::get_id())? {
            return Ok(false);
        }
        let mut named: Handle<TNamingNamedShape> = Handle::default();
        if label.find_attribute_as(&TNamingNamedShape::get_id(), &mut named)?
            && TNamingTool::get_shape(&named)?.is_null()
        {
            return Ok(false);
        }
        let mut line = Line3::default();
        if !TDataXtdGeometry::line_of_label(label, &mut line)? {
            return Ok(false);
        }
        let geometry = Handle::new(GeomLine::from_lin(line));
        match ais.downcast_ref::<AisAxis>() {
            Some(axis) => {
                axis.set_component(&geometry)?;
                refresh_kept_object(axis)?;
            }
            None => *ais = AisAxis::new(&geometry, prs_dim_units_api())?.into_upcast(),
        }
        Ok(true)
    }
}

impl TPrsStdDriver for TPrsStdPlaneDriver {
    /// `Update`: the label's plane, centered at its location.
    fn update(
        &self,
        label: &TdfLabel,
        ais: &mut Handle<dyn AisInteractiveObject>,
    ) -> Result<bool, Failure> {
        if !label_has::<TDataXtdPlane>(label, &TDataXtdPlane::get_id())? {
            return Ok(false);
        }
        let mut plane = Plane3::default();
        if !TDataXtdGeometry::plane_of_label(label, &mut plane)? {
            return Ok(false);
        }
        let geometry = Handle::new(GeomPlane::from_pln(plane));
        match ais.downcast_ref::<AisPlane>() {
            Some(object) => {
                object.set_component(&geometry);
                object.set_center(plane.location());
                refresh_kept_object(object)?;
            }
            None => {
                *ais = AisPlane::with_center(&geometry, plane.location(), false)?.into_upcast();
            }
        }
        Ok(true)
    }
}

impl TPrsStdDriver for TPrsStdPointDriver {
    /// `Update`: the label's point.
    fn update(
        &self,
        label: &TdfLabel,
        ais: &mut Handle<dyn AisInteractiveObject>,
    ) -> Result<bool, Failure> {
        if !label_has::<TDataXtdPoint>(label, &TDataXtdPoint::get_id())? {
            return Ok(false);
        }
        let mut point = Point3::default();
        if !TDataXtdGeometry::point_of_label(label, &mut point)? {
            return Ok(false);
        }
        *ais = point_object(ais, point)?.into_upcast();
        Ok(true)
    }
}

/// The `AisPoint` of a point: the kept object updated, or a new one.
pub(super) fn point_object(
    ais: &Handle<dyn AisInteractiveObject>,
    point: Point3,
) -> Result<Handle<AisPoint>, Failure> {
    let geometry = Handle::new(GeomCartesianPoint::from_pnt(point)).into_upcast();
    let kept = ais.downcast::<AisPoint>();
    match kept.get() {
        Some(object) => {
            object.set_component(&geometry);
            refresh_kept_object(object)?;
            Ok(kept)
        }
        None => AisPoint::new(&geometry),
    }
}
