//! Lines, polylines and the curve dispatch.
use super::StepToGeom;
use crate::geom_to_step::array1_of;
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomBoundedCurve, StepGeomCartesianTransformationOperator2d,
    StepGeomCartesianTransformationOperator3d, StepGeomConic, StepGeomCurve, StepGeomCurveReplica,
    StepGeomLine, StepGeomOffsetCurve3d, StepGeomPolyline, StepGeomSurfaceCurve,
    StepGeomTrimmedCurve,
};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dCurve, Geom2dLine, Geom2dPoint, Geom2dVector,
};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomCurve, GeomLine, GeomOffsetCurve, GeomPoint, GeomVector,
};
use martensite_tkmath::gp::{Direction2, Direction3, Point2, Point3};

/// The knots and multiplicities of a polyline of `count` points: a degree-one clamped curve.
fn polyline_knots(count: i32) -> (Array1<'static, f64>, Array1<'static, i32>) {
    let knots = (0..count).map(f64::from).collect();
    let mut multiplicities = vec![1; count as usize];
    multiplicities[0] = 2;
    multiplicities[count as usize - 1] = 2;
    (array1_of(knots), array1_of(multiplicities))
}

impl StepToGeom {
    /// `MakeLine`: sln 22.10.2001. CTS23496: null when the direction cannot be made.
    pub fn make_line(
        sc: &Handle<StepGeomLine>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomLine>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeLine")?;
        let point = Self::make_cartesian_point(&live.pnt(), factors)?;
        let Some(point) = point.get() else {
            return Ok(Handle::default());
        };
        let direction = Self::make_vector_with_magnitude(&live.dir(), factors)?;
        let Some(direction) = direction.get() else {
            return Ok(Handle::default());
        };
        let vector = direction.vec();
        if vector.square_magnitude() < precision::confusion() * precision::confusion() {
            return Ok(Handle::default());
        }
        Ok(Handle::new(GeomLine::from_point(
            point.pnt(),
            Direction3::from_vector(vector)?,
        )))
    }
    /// `MakeLine2d`: sln 23.10.2001. CTS23496: null when the direction cannot be made.
    pub fn make_line_2d(
        sc: &Handle<StepGeomLine>,
        factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dLine>, Failure> {
        let live = step_live(sc, "StepToGeom::MakeLine2d")?;
        let point = Self::make_cartesian_point_2d(&live.pnt(), factors)?;
        let Some(point) = point.get() else {
            return Ok(Handle::default());
        };
        let direction = Self::make_vector_with_magnitude_2d(&live.dir())?;
        let Some(direction) = direction.get() else {
            return Ok(Handle::default());
        };
        let unit = Direction2::from_vector(direction.vec2d())?;
        Ok(Handle::new(Geom2dLine::from_point(point.pnt2d(), unit)))
    }
    /// `make_polyline`: a degree-one B-spline curve; null with fewer than two points.
    pub fn make_polyline(
        spl: &Handle<StepGeomPolyline>,
        factors: &StepDataFactors,
    ) -> Result<Handle<GeomBSplineCurve>, Failure> {
        let Some(live) = spl.get() else {
            return Ok(Handle::default());
        };
        let count = live.nb_points();
        if count <= 1 {
            return Ok(Handle::default());
        }
        let mut poles: Vec<Point3> = Vec::with_capacity(count as usize);
        for rank in 1..=count {
            let made = Self::make_cartesian_point(&live.points_value(rank)?, factors)?;
            let Some(point) = made.get() else {
                return Ok(Handle::default());
            };
            poles.push(point.pnt());
        }
        let (knots, multiplicities) = polyline_knots(count);
        let curve = GeomBSplineCurve::new(&array1_of(poles), &knots, &multiplicities, 1, false)?;
        Ok(Handle::new(curve))
    }
    /// `MakePolyline2d`: a degree-one B-spline curve; null with fewer than two points.
    pub fn make_polyline_2d(
        spl: &Handle<StepGeomPolyline>,
        factors: &StepDataFactors,
    ) -> Result<Handle<Geom2dBSplineCurve>, Failure> {
        let Some(live) = spl.get() else {
            return Ok(Handle::default());
        };
        let count = live.nb_points();
        if count <= 1 {
            return Ok(Handle::default());
        }
        let mut poles: Vec<Point2> = Vec::with_capacity(count as usize);
        for rank in 1..=count {
            let made = Self::make_cartesian_point_2d(&live.points_value(rank)?, factors)?;
            let Some(point) = made.get() else {
                return Ok(Handle::default());
            };
            poles.push(point.pnt2d());
        }
        let (knots, multiplicities) = polyline_knots(count);
        let curve = Geom2dBSplineCurve::new(&array1_of(poles), &knots, &multiplicities, 1, false)?;
        Ok(Handle::new(curve))
    }
    /// A curve_replica: :n7 abv 16 Feb 99; protects against cyclic references and a wrong operator.
    fn make_curve_replica(
        replica: &StepGeomCurveReplica,
        curve: &Handle<StepGeomCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let parent = replica.parent_curve();
        let operator = replica
            .transformation()
            .cast::<StepGeomCartesianTransformationOperator3d>();
        if operator.is_null() || parent == *curve {
            return Ok(Handle::default());
        }
        let made = Self::make_curve(&parent, factors)?;
        let Some(live) = made.get() else {
            return Ok(Handle::default());
        };
        let Some(transform) = Self::make_transformation_3d(&operator, factors)? else {
            return Ok(Handle::default());
        };
        live.transform(transform)?;
        Ok(made)
    }
    /// An offset_curve_3d: :o2 abv 17 Feb 99; protects against a loop.
    fn make_offset_curve(
        offset: &StepGeomOffsetCurve3d,
        curve: &Handle<StepGeomCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let basis = offset.basis_curve();
        if basis == *curve {
            return Ok(Handle::default());
        }
        let made = Self::make_curve(&basis, factors)?;
        if made.is_null() {
            return Ok(Handle::default());
        }
        let direction = Self::make_direction(&offset.ref_direction())?;
        let Some(direction) = direction.get() else {
            return Ok(Handle::default());
        };
        let built = GeomOffsetCurve::new(&made, -offset.distance(), direction.dir()?, false)?;
        Ok(Handle::new(built).cast())
    }
    /// `make_curve`: null for a null entity or a curve that cannot be converted.
    pub fn make_curve(
        sc: &Handle<StepGeomCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        if sc.is_null() {
            return Ok(Handle::default());
        }
        let line = sc.cast::<StepGeomLine>();
        if line.is_some() {
            return Ok(Self::make_line(&line, factors)?.cast());
        }
        let trimmed = sc.cast::<StepGeomTrimmedCurve>();
        if trimmed.is_some() {
            return Ok(Self::make_trimmed_curve(&trimmed, factors)?.cast());
        }
        let conic = sc.cast::<StepGeomConic>();
        if conic.is_some() {
            return Ok(Self::make_conic(&conic, factors)?.cast());
        }
        let bounded = sc.cast::<StepGeomBoundedCurve>();
        if bounded.is_some() {
            return Ok(Self::make_bounded_curve(&bounded, factors)?.cast());
        }
        if let Some(replica) = sc.cast::<StepGeomCurveReplica>().get() {
            return Self::make_curve_replica(replica, sc, factors);
        }
        if let Some(offset) = sc.cast::<StepGeomOffsetCurve3d>().get() {
            return Self::make_offset_curve(offset, sc, factors);
        }
        if let Some(surface_curve) = sc.cast::<StepGeomSurfaceCurve>().get() {
            //:o5 abv 17 Feb 99
            return Self::make_curve(&surface_curve.curve3d(), factors);
        }
        Ok(Handle::default())
    }
    /// A curve_replica in 2D, with the same protections.
    fn make_curve_replica_2d(
        replica: &StepGeomCurveReplica,
        curve: &Handle<StepGeomCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let parent = replica.parent_curve();
        let operator = replica
            .transformation()
            .cast::<StepGeomCartesianTransformationOperator2d>();
        if operator.is_null() || parent == *curve {
            return Ok(Handle::default());
        }
        let made = Self::make_curve_2d(&parent, factors)?;
        let Some(live) = made.get() else {
            return Ok(Handle::default());
        };
        let Some(transform) = Self::make_transformation_2d(&operator, factors)? else {
            return Ok(Handle::default());
        };
        live.transform(transform)?;
        Ok(made)
    }
    /// `make_curve2d`: null for a curve that cannot be converted.
    pub fn make_curve_2d(
        sc: &Handle<StepGeomCurve>,
        factors: &StepDataFactors,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        step_live(sc, "StepToGeom::MakeCurve2d")?;
        let line = sc.cast::<StepGeomLine>();
        if line.is_some() {
            return Ok(Self::make_line_2d(&line, factors)?.cast());
        }
        let conic = sc.cast::<StepGeomConic>();
        if conic.is_some() {
            return Ok(Self::make_conic_2d(&conic, factors)?.cast());
        }
        let bounded = sc.cast::<StepGeomBoundedCurve>();
        if bounded.is_some() {
            return Ok(Self::make_bounded_curve_2d(&bounded, factors)?.cast());
        }
        if let Some(replica) = sc.cast::<StepGeomCurveReplica>().get() {
            return Self::make_curve_replica_2d(replica, sc, factors);
        }
        Ok(Handle::default())
    }
}
