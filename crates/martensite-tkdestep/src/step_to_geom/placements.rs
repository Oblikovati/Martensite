//! Points, directions, vectors and axis placements.
use super::StepToGeom;
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomAxis1Placement, StepGeomAxis2Placement2d, StepGeomAxis2Placement3d,
    StepGeomCartesianPoint, StepGeomDirection, StepGeomSuParameters, StepGeomVector,
};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dAxisPlacement, Geom2dCartesianPoint, Geom2dDirection, Geom2dPoint,
    Geom2dVectorWithMagnitude,
};
use martensite_tkg3d::geom::{
    GeomAxis1Placement, GeomAxis2Placement, GeomCartesianPoint, GeomDirection, GeomPoint,
    GeomVectorWithMagnitude,
};
use martensite_tkmath::gp::{
    self, Axis2, Direction2, Direction3, Point3, Vector2, Vector3, Xy, Xyz,
};

impl StepToGeom {
    /// `make_cartesian_point`: null for a null entity or one that is not 3D.
    pub fn make_cartesian_point(
        sp: &Handle<StepGeomCartesianPoint>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomCartesianPoint>, Failure> {
        let Some(live) = sp.get() else {
            return Ok(Handle::default());
        };
        if live.nb_coordinates() != 3 {
            return Ok(Handle::default());
        }
        let lf = factors.length_factor();
        let x = live.coordinates_value(1)? * lf;
        let y = live.coordinates_value(2)? * lf;
        let z = live.coordinates_value(3)? * lf;
        Ok(Handle::new(GeomCartesianPoint::new(x, y, z)))
    }
    /// `MakeCartesianPoint2d`: null for an entity that is not 2D.
    pub fn make_cartesian_point_2d(
        sp: &Handle<StepGeomCartesianPoint>,
        _factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dCartesianPoint>, Failure> {
        let live = step_live(sp, "StepToGeom::MakeCartesianPoint2d")?;
        if live.nb_coordinates() != 2 {
            return Ok(Handle::default());
        }
        let (x, y) = (live.coordinates_value(1)?, live.coordinates_value(2)?);
        Ok(Handle::new(Geom2dCartesianPoint::new(x, y)))
    }
    /// `make_direction`: null when a ratio is missing or infinite, or the magnitude is null.
    pub fn make_direction(
        sd: &Handle<StepGeomDirection>,
    ) -> Result<Handle<GeomDirection>, Failure> {
        let live = step_live(sd, "StepToGeom::MakeDirection")?;
        if live.nb_direction_ratios() < 3 {
            return Ok(Handle::default());
        }
        let x = live.direction_ratios_value(1)?;
        let y = live.direction_ratios_value(2)?;
        let z = live.direction_ratios_value(3)?;
        // 5.08.2021. Unstable test bugs xde bug24759: Y is very large value - FPE in SquareModulus
        if precision::is_infinite(x) || precision::is_infinite(y) || precision::is_infinite(z) {
            return Ok(Handle::default());
        }
        // sln 22.10.2001. CTS23496: Direction is not created if it has null magnitude
        if Xyz::new(x, y, z).square_modulus() <= gp::resolution() * gp::resolution() {
            return Ok(Handle::default());
        }
        Ok(Handle::new(GeomDirection::new(x, y, z)?))
    }
    /// `MakeDirection2d`: null when a ratio is missing or the magnitude is null.
    pub fn make_direction_2d(
        sd: &Handle<StepGeomDirection>,
    ) -> Result<Handle<Geom2dDirection>, Failure> {
        let live = step_live(sd, "StepToGeom::MakeDirection2d")?;
        if live.nb_direction_ratios() < 2 {
            return Ok(Handle::default());
        }
        let x = live.direction_ratios_value(1)?;
        let y = live.direction_ratios_value(2)?;
        // sln 23.10.2001. CTS23496: Direction is not created if it has null magnitude
        if Xy::new(x, y).square_modulus() <= gp::resolution() * gp::resolution() {
            return Ok(Handle::default());
        }
        Ok(Handle::new(Geom2dDirection::new(x, y)?))
    }
    /// `make_vector_with_magnitude`: null when the direction cannot be made.
    pub fn make_vector_with_magnitude(
        sv: &Handle<StepGeomVector>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomVectorWithMagnitude>, Failure> {
        let live = step_live(sv, "StepToGeom::MakeVectorWithMagnitude")?;
        // sln 22.10.2001. CTS23496: Vector is not created if direction have not been successfully
        // created
        let direction = Self::make_direction(&live.orientation())?;
        let Some(direction) = direction.get() else {
            return Ok(Handle::default());
        };
        let scaled = direction.dir()?.xyz() * live.magnitude() * factors.length_factor();
        Ok(Handle::new(GeomVectorWithMagnitude::from_vec(
            Vector3::from(scaled),
        )))
    }
    /// `MakeVectorWithMagnitude2d`: null when the direction cannot be made.
    pub fn make_vector_with_magnitude_2d(
        sv: &Handle<StepGeomVector>,
    ) -> Result<Handle<Geom2dVectorWithMagnitude>, Failure> {
        let live = step_live(sv, "StepToGeom::MakeVectorWithMagnitude2d")?;
        let direction = Self::make_direction_2d(&live.orientation())?;
        let Some(direction) = direction.get() else {
            return Ok(Handle::default());
        };
        let scaled = direction.dir2d()?.xy() * live.magnitude();
        Ok(Handle::new(Geom2dVectorWithMagnitude::from_vec2d(
            Vector2::from(scaled),
        )))
    }
    /// The direction of an optional entity, or `fallback` when it cannot be made
    /// (sln 22.10.2001. CTS23496: the default direction is used on problems).
    pub(super) fn direction_or(
        direction: &Handle<StepGeomDirection>,
        present: bool,
        fallback: Direction3,
    ) -> Result<Direction3, Failure> {
        if !present {
            return Ok(fallback);
        }
        let made = Self::make_direction(direction)?;
        match made.get() {
            Some(live) => live.dir(),
            None => Ok(fallback),
        }
    }
    /// `make_axis1_placement`: null when the location cannot be made.
    pub fn make_axis1_placement(
        sa: &Handle<StepGeomAxis1Placement>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomAxis1Placement>, Failure> {
        let live = step_live(sa, "StepToGeom::MakeAxis1Placement")?;
        let point = Self::make_cartesian_point(&live.location(), factors)?;
        let Some(point) = point.get() else {
            return Ok(Handle::default());
        };
        let axis = Self::direction_or(&live.axis(), live.has_axis(), Direction3::Z)?;
        Ok(Handle::new(GeomAxis1Placement::from_location(
            point.pnt(),
            axis,
        )))
    }
    /// The frame of an axis2_placement_3d: an X direction parallel to the axis is ignored.
    fn placement_frame(
        origin: Point3,
        main: Direction3,
        x_direction: Option<Direction3>,
    ) -> Result<Axis2, Failure> {
        match x_direction {
            Some(x) if !main.is_parallel(x, precision::angular()) => {
                Axis2::with_x_direction(origin, main, x)
            }
            _ => Axis2::new(origin, main),
        }
    }
    /// `make_axis2_placement(StepGeomAxis2Placement3d)`: null when the location cannot be made.
    pub fn make_axis2_placement(
        sa: &Handle<StepGeomAxis2Placement3d>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomAxis2Placement>, Failure> {
        let live = step_live(sa, "StepToGeom::MakeAxis2Placement")?;
        let point = Self::make_cartesian_point(&live.location(), factors)?;
        let Some(point) = point.get() else {
            return Ok(Handle::default());
        };
        // sln 22.10.2001. CTS23496: on problems with the direction the default is used
        let main = Self::direction_or(&live.axis(), live.has_axis(), Direction3::Z)?;
        let mut x_direction = None;
        if live.has_ref_direction() {
            let made = Self::make_direction(&live.ref_direction())?;
            if let Some(direction) = made.get() {
                x_direction = Some(direction.dir()?);
            }
        }
        let frame = Self::placement_frame(point.pnt(), main, x_direction)?;
        Ok(Handle::new(GeomAxis2Placement::new(frame)))
    }
    /// `make_axis2_placement(StepGeomSuParameters)`: the placement of kinematic su parameters.
    pub fn make_axis2_placement_su(
        sp: &Handle<StepGeomSuParameters>,
    ) -> Result<Handle<GeomAxis2Placement>, Failure> {
        let live = step_live(sp, "StepToGeom::MakeAxis2Placement")?;
        let (a, b, c) = (live.a(), live.b(), live.c());
        let (sin_alpha, cos_alpha) = martensite_cmath::sincos(live.alpha());
        let (sin_beta, cos_beta) = martensite_cmath::sincos(live.beta());
        let (sin_gamma, cos_gamma) = martensite_cmath::sincos(live.gamma());
        let origin = Point3::new(
            a * cos_gamma + b * sin_gamma * sin_alpha,
            a * sin_gamma - b * cos_gamma * sin_alpha,
            c + b * cos_alpha,
        );
        let main = Direction3::new(sin_gamma * sin_alpha, -cos_gamma * sin_alpha, cos_alpha)?;
        let x_direction = Direction3::new(
            cos_gamma * cos_beta - sin_gamma * cos_alpha * sin_beta,
            sin_gamma * cos_beta + cos_gamma * cos_alpha * sin_beta,
            sin_alpha * sin_beta,
        )?;
        Ok(Handle::new(GeomAxis2Placement::new(
            Axis2::with_x_direction(origin, main, x_direction)?,
        )))
    }
    /// `make_axis_placement`: null when the location cannot be made.
    pub fn make_axis_placement(
        sa: &Handle<StepGeomAxis2Placement2d>,
        factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dAxisPlacement>, Failure> {
        let live = step_live(sa, "StepToGeom::MakeAxisPlacement")?;
        let point = Self::make_cartesian_point_2d(&live.location(), factors)?;
        let Some(point) = point.get() else {
            return Ok(Handle::default());
        };
        // sln 23.10.2001. CTS23496: on problems with the direction the default is used
        let mut x_direction = Direction2::X;
        if live.has_ref_direction() {
            let made = Self::make_direction_2d(&live.ref_direction())?;
            if let Some(direction) = made.get() {
                x_direction = direction.dir2d()?;
            }
        }
        Ok(Handle::new(Geom2dAxisPlacement::from_location(
            point.pnt2d(),
            x_direction,
        )))
    }
}

/// The 3D placement of a conic's select: `CaseNum == 2`, else a null handle.
pub(super) fn select_placement_3d(
    select: &crate::step_geom::StepGeomAxis2Placement,
    factors: &StepDataFactors,
) -> Result<Handle<GeomAxis2Placement>, Failure> {
    if select.case_num(&select.value()) != 2 {
        return Ok(Handle::default());
    }
    StepToGeom::make_axis2_placement(&select.value().cast(), factors)
}

/// The 2D placement of a conic's select: `CaseNum == 1`, else a null handle.
pub(super) fn select_placement_2d(
    select: &crate::step_geom::StepGeomAxis2Placement,
    factors: &StepDataFactors,
) -> Result<Handle<Geom2dAxisPlacement>, Failure> {
    if select.case_num(&select.value()) != 1 {
        return Ok(Handle::default());
    }
    StepToGeom::make_axis_placement(&select.value().cast(), factors)
}
