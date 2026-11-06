//! Points, directions, vectors, axis placements and the transformation operator.
use super::{empty_step_name, geom_to_step_class};
use crate::step_data::StepDataFactors;
use crate::step_geom::{
    StepGeomAxis1Placement, StepGeomAxis2Placement2d, StepGeomAxis2Placement3d,
    StepGeomCartesianPoint, StepGeomCartesianTransformationOperator3d, StepGeomDirection,
    StepGeomVector,
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dAxisPlacement, Geom2dCartesianPoint, Geom2dDirection, Geom2dPoint, Geom2dVector,
};
use martensite_tkg3d::geom::{
    GeomAxis1Placement, GeomAxis2Placement, GeomCartesianPoint, GeomDirection, GeomPoint,
    GeomVector,
};
use martensite_tkmath::gp::{
    Axis1, Axis2, Axis2d, Axis3, Axis22d, Direction2, Direction3, Point2, Point3, Transform3,
    Vector2, Vector3, Xyz,
};

geom_to_step_class!(
    /// `GeomToStepMakeCartesianPoint`: a cartesian_point from a point.
    ///
    /// ```
    /// use martensite_tkdestep::geom_to_step::GeomToStepMakeCartesianPoint;
    /// use martensite_tkmath::gp::Point3;
    /// let made = GeomToStepMakeCartesianPoint::from_pnt(&Point3::new(2.0, 4.0, 6.0), 2.0);
    /// assert_eq!(made.value().unwrap().nb_coordinates(), 3);
    /// ```
    GeomToStepMakeCartesianPoint,
    "GeomToStepMakeCartesianPoint",
    StepGeomCartesianPoint
);

impl GeomToStepMakeCartesianPoint {
    fn with_3d(x: f64, y: f64, z: f64, factor: f64) -> Self {
        let point = StepGeomCartesianPoint::new();
        point.init_3d(&empty_step_name(), x / factor, y / factor, z / factor);
        Self::done_with(Handle::new(point))
    }
    fn with_2d(x: f64, y: f64) -> Self {
        let point = StepGeomCartesianPoint::new();
        point.init_2d(&empty_step_name(), x, y);
        Self::done_with(Handle::new(point))
    }
    /// `GeomToStepMakeCartesianPoint(gp_Pnt, aFactor)`.
    pub fn from_pnt(point: &Point3, factor: f64) -> Self {
        Self::with_3d(point.x(), point.y(), point.z(), factor)
    }
    /// `GeomToStepMakeCartesianPoint(gp_Pnt2d, aFactor)`: the factor is ignored.
    pub fn from_pnt2d(point: &Point2, _factor: f64) -> Self {
        Self::with_2d(point.x(), point.y())
    }
    /// `GeomToStepMakeCartesianPoint(GeomCartesianPoint, aFactor)`.
    pub fn from_geom(point: &Handle<GeomCartesianPoint>, factor: f64) -> Result<Self, Failure> {
        let live = live_geometry(point, "GeomToStepMakeCartesianPoint")?;
        Ok(Self::from_pnt(&live.pnt(), factor))
    }
    /// `GeomToStepMakeCartesianPoint(Geom2dCartesianPoint)`.
    pub fn from_geom2d(point: &Handle<Geom2dCartesianPoint>) -> Result<Self, Failure> {
        let live = live_geometry(point, "GeomToStepMakeCartesianPoint")?;
        Ok(Self::with_2d(live.x(), live.y()))
    }
}

/// The object of a Geom handle, or the null dereference.
pub(super) fn live_geometry<'a, T: ?Sized + martensite_tkernel::standard::RuntimeTyped>(
    handle: &'a Handle<T>,
    class: &str,
) -> Result<&'a T, Failure> {
    handle.get().ok_or_else(|| {
        martensite_tkernel::standard::exceptions::null_object(&format!(
            "{class}: null geometry handle"
        ))
    })
}

geom_to_step_class!(
    /// `GeomToStepMakeDirection`: a direction from a unit vector.
    ///
    /// ```
    /// use martensite_tkdestep::geom_to_step::GeomToStepMakeDirection;
    /// use martensite_tkmath::gp::Direction2;
    /// let made = GeomToStepMakeDirection::from_dir2d(&Direction2::default());
    /// assert_eq!(made.value().unwrap().nb_direction_ratios(), 2);
    /// ```
    GeomToStepMakeDirection,
    "GeomToStepMakeDirection",
    StepGeomDirection
);

impl GeomToStepMakeDirection {
    /// `GeomToStepMakeDirection(gp_Dir)`.
    pub fn from_dir(direction: &Direction3) -> Self {
        let made = StepGeomDirection::new();
        made.init_3d(
            &empty_step_name(),
            direction.x(),
            direction.y(),
            direction.z(),
        );
        Self::done_with(Handle::new(made))
    }
    /// `GeomToStepMakeDirection(gp_Dir2d)`.
    pub fn from_dir2d(direction: &Direction2) -> Self {
        let made = StepGeomDirection::new();
        made.init_2d(&empty_step_name(), direction.x(), direction.y());
        Self::done_with(Handle::new(made))
    }
    /// `GeomToStepMakeDirection(GeomDirection)`.
    pub fn from_geom(direction: &Handle<GeomDirection>) -> Result<Self, Failure> {
        Ok(Self::from_dir(
            &live_geometry(direction, "GeomToStepMakeDirection")?.dir()?,
        ))
    }
    /// `GeomToStepMakeDirection(Geom2dDirection)`.
    pub fn from_geom2d(direction: &Handle<Geom2dDirection>) -> Result<Self, Failure> {
        Ok(Self::from_dir2d(
            &live_geometry(direction, "GeomToStepMakeDirection")?.dir2d()?,
        ))
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeVector`: a vector from a gp or Geom vector.
    ///
    /// ```
    /// use martensite_tkdestep::{geom_to_step::GeomToStepMakeVector, step_data::StepDataFactors};
    /// use martensite_tkmath::gp::Vector3;
    /// let vector = Vector3::new(0.0, 0.0, 4.0);
    /// let made = GeomToStepMakeVector::from_vec(&vector, &StepDataFactors::new()).unwrap();
    /// assert_eq!(made.value().unwrap().magnitude(), 4.0);
    /// ```
    GeomToStepMakeVector,
    "GeomToStepMakeVector",
    StepGeomVector
);

impl GeomToStepMakeVector {
    fn with_parts(direction: &GeomToStepMakeDirection, magnitude: f64) -> Result<Self, Failure> {
        let vector = StepGeomVector::new();
        vector.init(&empty_step_name(), &direction.value()?, magnitude);
        Ok(Self::done_with(Handle::new(vector)))
    }
    /// `GeomToStepMakeVector(gp_Vec, factors)`; a null vector raises when made a direction.
    pub fn from_vec(vector: &Vector3, factors: &StepDataFactors) -> Result<Self, Failure> {
        let direction = GeomToStepMakeDirection::from_dir(&Direction3::from_vector(*vector)?);
        Self::with_parts(&direction, vector.magnitude() / factors.length_factor())
    }
    /// `GeomToStepMakeVector(gp_Vec2d, factors)`: 2D magnitudes are not scaled.
    pub fn from_vec2d(vector: &Vector2, _factors: &StepDataFactors) -> Result<Self, Failure> {
        let direction = GeomToStepMakeDirection::from_dir2d(&Direction2::from_vector(*vector)?);
        Self::with_parts(&direction, vector.magnitude())
    }
    /// `GeomToStepMakeVector(GeomVector, factors)`.
    pub fn from_geom(
        vector: &Handle<dyn GeomVector>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        Self::from_vec(
            &live_geometry(vector, "GeomToStepMakeVector")?.vec(),
            factors,
        )
    }
    /// `GeomToStepMakeVector(Geom2dVector, factors)`.
    pub fn from_geom2d(
        vector: &Handle<dyn Geom2dVector>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        Self::from_vec2d(
            &live_geometry(vector, "GeomToStepMakeVector")?.vec2d(),
            factors,
        )
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeAxis1Placement`: an axis1_placement.
    GeomToStepMakeAxis1Placement,
    "GeomToStepMakeAxis1Placement",
    StepGeomAxis1Placement
);

impl GeomToStepMakeAxis1Placement {
    fn with_parts(
        point: &GeomToStepMakeCartesianPoint,
        direction: &GeomToStepMakeDirection,
    ) -> Result<Self, Failure> {
        let axis = StepGeomAxis1Placement::new();
        axis.set_location(&point.value()?);
        axis.set_axis(&direction.value()?);
        axis.set_name(&empty_step_name());
        Ok(Self::done_with(Handle::new(axis)))
    }
    /// `GeomToStepMakeAxis1Placement(gp_Ax1, factors)`.
    pub fn from_ax1(axis: &Axis1, factors: &StepDataFactors) -> Result<Self, Failure> {
        let point =
            GeomToStepMakeCartesianPoint::from_pnt(&axis.location(), factors.length_factor());
        Self::with_parts(
            &point,
            &GeomToStepMakeDirection::from_dir(&axis.direction()),
        )
    }
    /// `GeomToStepMakeAxis1Placement(gp_Ax2d, factors)`.
    pub fn from_ax2d(axis: &Axis2d, factors: &StepDataFactors) -> Result<Self, Failure> {
        let point =
            GeomToStepMakeCartesianPoint::from_pnt2d(&axis.location(), factors.length_factor());
        Self::with_parts(
            &point,
            &GeomToStepMakeDirection::from_dir2d(&axis.direction()),
        )
    }
    /// `GeomToStepMakeAxis1Placement(GeomAxis1Placement, factors)`.
    pub fn from_geom(
        axis: &Handle<GeomAxis1Placement>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        Self::from_ax1(
            &live_geometry(axis, "GeomToStepMakeAxis1Placement")?.ax1(),
            factors,
        )
    }
    /// `GeomToStepMakeAxis1Placement(Geom2dAxisPlacement, factors)`.
    pub fn from_geom2d(
        axis: &Handle<Geom2dAxisPlacement>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        Self::from_ax2d(
            &live_geometry(axis, "GeomToStepMakeAxis1Placement")?.ax2d(),
            factors,
        )
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeAxis2Placement2d`: an axis2_placement_2d.
    GeomToStepMakeAxis2Placement2d,
    "GeomToStepMakeAxis2Placement2d",
    StepGeomAxis2Placement2d
);

impl GeomToStepMakeAxis2Placement2d {
    fn with_parts(
        point: &GeomToStepMakeCartesianPoint,
        direction: &GeomToStepMakeDirection,
    ) -> Result<Self, Failure> {
        let axis = StepGeomAxis2Placement2d::new();
        axis.set_location(&point.value()?);
        axis.set_ref_direction(&direction.value()?);
        axis.set_name(&empty_step_name());
        Ok(Self::done_with(Handle::new(axis)))
    }
    /// `GeomToStepMakeAxis2Placement2d(gp_Ax2, factors)`: the main direction of the 3D axis is
    /// the reference direction.
    pub fn from_ax2(axis: &Axis2, factors: &StepDataFactors) -> Result<Self, Failure> {
        let point =
            GeomToStepMakeCartesianPoint::from_pnt(&axis.location(), factors.length_factor());
        Self::with_parts(
            &point,
            &GeomToStepMakeDirection::from_dir(&axis.direction()),
        )
    }
    /// `GeomToStepMakeAxis2Placement2d(gp_Ax22d, factors)`.
    pub fn from_ax22d(axis: &Axis22d, factors: &StepDataFactors) -> Result<Self, Failure> {
        let point =
            GeomToStepMakeCartesianPoint::from_pnt2d(&axis.location(), factors.length_factor());
        Self::with_parts(
            &point,
            &GeomToStepMakeDirection::from_dir2d(&axis.x_direction()),
        )
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeAxis2Placement3d`: an axis2_placement_3d.
    GeomToStepMakeAxis2Placement3d,
    "GeomToStepMakeAxis2Placement3d",
    StepGeomAxis2Placement3d
);

impl GeomToStepMakeAxis2Placement3d {
    fn with_frame(
        origin: &Point3,
        main: &Direction3,
        x_direction: &Direction3,
        factor: f64,
    ) -> Result<Self, Failure> {
        let axis = StepGeomAxis2Placement3d::new();
        axis.set_location(&GeomToStepMakeCartesianPoint::from_pnt(origin, factor).value()?);
        axis.set_axis(&GeomToStepMakeDirection::from_dir(main).value()?);
        axis.set_ref_direction(&GeomToStepMakeDirection::from_dir(x_direction).value()?);
        axis.set_name(&empty_step_name());
        Ok(Self::done_with(Handle::new(axis)))
    }
    /// `GeomToStepMakeAxis2Placement3d(factors)`: the placement at the origin.
    pub fn at_origin(factors: &StepDataFactors) -> Result<Self, Failure> {
        Self::from_ax2(&Axis2::default(), factors)
    }
    /// `GeomToStepMakeAxis2Placement3d(gp_Ax2, factors)`.
    pub fn from_ax2(axis: &Axis2, factors: &StepDataFactors) -> Result<Self, Failure> {
        let factor = factors.length_factor();
        Self::with_frame(
            &axis.location(),
            &axis.direction(),
            &axis.x_direction(),
            factor,
        )
    }
    /// `GeomToStepMakeAxis2Placement3d(gp_Ax3, factors)`.
    pub fn from_ax3(axis: &Axis3, factors: &StepDataFactors) -> Result<Self, Failure> {
        let factor = factors.length_factor();
        Self::with_frame(
            &axis.location(),
            &axis.direction(),
            &axis.x_direction(),
            factor,
        )
    }
    /// `GeomToStepMakeAxis2Placement3d(gp_Trsf, factors)`: the default frame transformed.
    pub fn from_trsf(transform: &Transform3, factors: &StepDataFactors) -> Result<Self, Failure> {
        Self::from_ax2(&Axis2::default().transformed(*transform)?, factors)
    }
    /// `GeomToStepMakeAxis2Placement3d(GeomAxis2Placement, factors)`.
    pub fn from_geom(
        axis: &Handle<GeomAxis2Placement>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        Self::from_ax2(
            &live_geometry(axis, "GeomToStepMakeAxis2Placement3d")?.ax2()?,
            factors,
        )
    }
}

geom_to_step_class!(
    /// `GeomToStepMakeCartesianTransformationOperator`: an operator from a transformation.
    GeomToStepMakeCartesianTransformationOperator,
    "GeomToStepMakeCartesianTransformationOperator",
    StepGeomCartesianTransformationOperator3d
);

impl GeomToStepMakeCartesianTransformationOperator {
    /// The row of a matrix as a direction, normalized as the implicit conversion is.
    fn row_direction(row: Xyz) -> Result<Handle<StepGeomDirection>, Failure> {
        GeomToStepMakeDirection::from_dir(&Direction3::from_xyz(row)?).value()
    }
    /// `GeomToStepMakeCartesianTransformationOperator(gp_Trsf, factors)`.
    pub fn new(transform: &Transform3, factors: &StepDataFactors) -> Result<Self, Failure> {
        let inverse = transform.inverted()?;
        let inverse_matrix = inverse.matrix();
        let matrix = transform.matrix();
        let translation = (inverse.translation_part() * matrix).reversed();
        let origin = Point3::new(translation.x(), translation.y(), translation.z());
        let operator = StepGeomCartesianTransformationOperator3d::new();
        operator.set_name(&empty_step_name());
        operator.set_axis1(&Self::row_direction(inverse_matrix.row(1)?)?);
        operator.set_axis2(&Self::row_direction(inverse_matrix.row(2)?)?);
        operator.set_axis3(&Self::row_direction(inverse_matrix.row(3)?)?);
        let point = GeomToStepMakeCartesianPoint::from_pnt(&origin, factors.length_factor());
        operator.set_local_origin(&point.value()?);
        operator.set_scale(inverse.scale_factor());
        Ok(Self::done_with(Handle::new(operator)))
    }
}
