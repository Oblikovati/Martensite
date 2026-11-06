//! The rigid displacement of a cartesian transformation operator.
use super::StepToGeom;
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomCartesianTransformationOperator2d, StepGeomCartesianTransformationOperator3d,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dPoint;
use martensite_tkg3d::geom::GeomPoint;
use martensite_tkmath::gp::{Axis2d, Axis3, Direction2, Direction3, Transform2, Transform3};

impl StepToGeom {
    /// `MakeTransformation2d`: `None` without an origin; only the rigid displacement counts.
    pub fn make_transformation_2d(
        scto: &Handle<StepGeomCartesianTransformationOperator2d>,
        factors: &StepDataFactors,
    ) -> Result<Option<Transform2>, Failure> {
        let live = step_live(scto, "StepToGeom::MakeTransformation2d")?;
        let origin = Self::make_cartesian_point_2d(&live.local_origin(), factors)?;
        let Some(origin) = origin.get() else {
            return Ok(None);
        };
        // sln 23.10.2001. CTS23496: on problems with the direction the default is used
        let mut d1 = Direction2::X;
        let axis1 = live.axis1();
        if axis1.is_some() {
            let made = Self::make_direction_2d(&axis1)?;
            if let Some(direction) = made.get() {
                d1 = direction.dir2d()?;
            }
        }
        let mut transform = Transform2::default();
        transform.set_transformation_to(Axis2d::new(origin.pnt2d(), d1));
        Ok(Some(transform.inverted()?))
    }
    /// `MakeTransformation3d`: `None` without an origin.
    pub fn make_transformation_3d(
        scto: &Handle<StepGeomCartesianTransformationOperator3d>,
        factors: &StepDataFactors,
    ) -> Result<Option<Transform3>, Failure> {
        let live = step_live(scto, "StepToGeom::MakeTransformation3d")?;
        let origin = Self::make_cartesian_point(&live.local_origin(), factors)?;
        let Some(origin) = origin.get() else {
            return Ok(None);
        };
        // sln 23.10.2001. CTS23496: on problems with the direction the default is used
        let axis1 = live.axis1();
        let axis2 = live.axis2();
        let axis3 = live.axis3();
        let d1 = Self::direction_or(&axis1, axis1.is_some(), Direction3::X)?;
        let d2 = Self::direction_or(&axis2, axis2.is_some(), Direction3::Y)?;
        // The cross product is only taken when it is needed: parallel axes raise in it.
        let made3 = if axis3.is_some() {
            Self::make_direction(&axis3)?
        } else {
            Handle::default()
        };
        let d3 = match made3.get() {
            Some(direction) => direction.dir()?,
            None => d1.crossed(d2)?,
        };
        let mut transform = Transform3::default();
        transform.set_transformation_to(Axis3::new(origin.pnt(), d3, d1)?);
        if live.has_scale() {
            transform.set_scale_factor(live.scale())?;
        }
        //:n8 abv 16 Feb 99: tr8_as2_db.stp: reverse for accordance with LV tool
        Ok(Some(transform.inverted()?))
    }
}
