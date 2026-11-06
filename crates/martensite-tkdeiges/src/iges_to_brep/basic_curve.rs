//! `IgesToBRepBasicCurve`: the curve entities with a direct geometric form. This file holds the
//! choice among the kinds, lines and matrices.
use super::curve_and_surface::curve_and_surface_tool;
use crate::iges_data::{IgesDataIgesEntity, IgesDataToolLocation};
use crate::iges_geom::{
    IgesGeomBSplineCurve, IgesGeomCircularArc, IgesGeomConicArc, IgesGeomCopiousData, IgesGeomLine,
    IgesGeomSplineCurve, IgesGeomTransformationMatrix,
};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine, Geom2dTrimmedCurve};
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomTransformation, GeomTrimmedCurve};
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{
    Direction2, Direction3, Line2, Line3, Point2, Point3, Transform3, Vector2, Vector3,
};

curve_and_surface_tool!(
    /// Translates lines, conics, arcs, splines and point lists.
    ///
    /// ```
    /// use martensite_tkdeiges::iges_to_brep::IgesToBRepBasicCurve;
    /// let tool = IgesToBRepBasicCurve::new();
    /// assert!(!tool.get_mode_transfer());
    /// ```
    IgesToBRepBasicCurve,
    |base| base.set_mode_transfer(false)
);

/// Clamps a parameter pair at the infinite limit.
pub(super) fn clamped_parameters(t1: f64, t2: f64) -> (f64, f64) {
    let lower = if precision::is_negative_infinite(t1) {
        -precision::infinite()
    } else {
        t1
    };
    let upper = if precision::is_positive_infinite(t2) {
        precision::infinite()
    } else {
        t2
    };
    (lower, upper)
}

/// A 3D curve trimmed between two parameters, in its own sense.
pub(super) fn trimmed_3d(
    curve: Handle<dyn GeomCurve>,
    t1: f64,
    t2: f64,
) -> Result<Handle<dyn GeomCurve>, Failure> {
    Ok(Handle::new(GeomTrimmedCurve::new(&curve, t1, t2, true, true)?).cast())
}

/// A curve in the plane trimmed between two parameters, in its own sense.
pub(super) fn trimmed_2d(
    curve: Handle<dyn Geom2dCurve>,
    t1: f64,
    t2: f64,
) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    Ok(Handle::new(Geom2dTrimmedCurve::new(&curve, t1, t2, true, true)?).cast())
}

impl IgesToBRepBasicCurve {
    /// The transformation of an entity applies only out of topology mode.
    pub(super) fn applies_position(&self, start: &IgesDataIgesEntity) -> bool {
        !self.get_mode_transfer() && start.has_transf()
    }
    /// The fail of a null entity.
    pub(super) fn null_entity<T: ?Sized + RuntimeTyped>(&self, start: &Handle<T>) {
        self.fail_key(start, "IGES_1005");
    }
    /// The curve of any basic curve entity, scaled to the file unit; null on a failure, which has
    /// its message or is the exception a transfer takes as no result;
    /// `tool.transfer_basic_curve(&entity)?`.
    ///
    /// # Errors
    /// A failure of the scaling of the curve.
    pub fn transfer_basic_curve(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        if start.is_null() {
            self.null_entity(start);
            return Ok(Handle::default());
        }
        // A curve that refuses its weights or knots is no result.
        let result = self.dispatch_basic_curve(start).unwrap_or_default();
        if let Some(live) = result.get() {
            live.scale(Point3::new(0.0, 0.0, 0.0), self.get_unit_factor())?;
        }
        Ok(result)
    }
    fn dispatch_basic_curve(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let bspline = start.cast::<IgesGeomBSplineCurve>();
        if bspline.is_some() {
            return self.transfer_bspline_curve(&bspline);
        }
        let line = start.cast::<IgesGeomLine>();
        if line.is_some() {
            return self.transfer_line(&line);
        }
        let arc = start.cast::<IgesGeomCircularArc>();
        if arc.is_some() {
            return self.transfer_circular_arc(&arc);
        }
        let conic = start.cast::<IgesGeomConicArc>();
        if conic.is_some() {
            return self.transfer_conic_arc(&conic);
        }
        let copious = start.cast::<IgesGeomCopiousData>();
        if copious.is_some() {
            return Ok(self.transfer_copious_data(&copious)?.cast());
        }
        let spline = start.cast::<IgesGeomSplineCurve>();
        if spline.is_some() {
            return Ok(self.transfer_spline_curve(&spline)?.cast());
        }
        Ok(Handle::default())
    }
    /// The curve in the plane of any basic curve entity; null on a failure;
    /// `tool.transfer_2d_basic_curve(&entity)`.
    pub fn transfer_2d_basic_curve(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Handle<dyn Geom2dCurve> {
        if start.is_null() {
            self.null_entity(start);
            return Handle::default();
        }
        // A multiplicity over degree + 1 raises.
        self.dispatch_2d_basic_curve(start).unwrap_or_default()
    }
    fn dispatch_2d_basic_curve(
        &mut self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let bspline = start.cast::<IgesGeomBSplineCurve>();
        if bspline.is_some() {
            return self.transfer_2d_bspline_curve(&bspline);
        }
        let line = start.cast::<IgesGeomLine>();
        if line.is_some() {
            return self.transfer_2d_line(&line);
        }
        let arc = start.cast::<IgesGeomCircularArc>();
        if arc.is_some() {
            return self.transfer_2d_circular_arc(&arc);
        }
        let conic = start.cast::<IgesGeomConicArc>();
        if conic.is_some() {
            return self.transfer_2d_conic_arc(&conic);
        }
        let copious = start.cast::<IgesGeomCopiousData>();
        if copious.is_some() {
            return Ok(self.transfer_2d_copious_data(&copious)?.cast());
        }
        let spline = start.cast::<IgesGeomSplineCurve>();
        if spline.is_some() {
            return Ok(self.transfer_2d_spline_curve(&spline)?.cast());
        }
        Ok(Handle::default())
    }
    /// The trimmed line between the ends of the entity; a fail when they coincide;
    /// `tool.transfer_line(&line)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_line(
        &self,
        start: &Handle<IgesGeomLine>,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        let (begin, end) = if self.applies_position(live) {
            (
                live.transformed_start_point()?,
                live.transformed_end_point()?,
            )
        } else {
            (live.start_point(), live.end_point())
        };
        if begin.is_equal(&end, precision::confusion()) {
            self.fail_key(start, "IGES_1225");
            return Ok(Handle::default());
        }
        let direction = Direction3::from_vector(Vector3::between(begin, end))?;
        let line = Line3::new(begin, direction);
        let (t1, t2) = clamped_parameters(
            elclib::parameter(&line, begin)?,
            elclib::parameter(&line, end)?,
        );
        trimmed_3d(Handle::new(GeomLine::from_lin(line)).cast(), t1, t2)
    }
    /// The trimmed line in the plane, the Z of the ends dropped; `tool.transfer_2d_line(&line)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_2d_line(
        &self,
        start: &Handle<IgesGeomLine>,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        let (begin, end) = if self.applies_position(live) {
            (
                live.transformed_start_point()?,
                live.transformed_end_point()?,
            )
        } else {
            (live.start_point(), live.end_point())
        };
        let begin = Point2::new(begin.x(), begin.y());
        let end = Point2::new(end.x(), end.y());
        if begin.is_equal(end, precision::p_confusion()) {
            self.fail_key(start, "IGES_1225");
            return Ok(Handle::default());
        }
        let direction = Direction2::from_vector(Vector2::between(begin, end))?;
        let line = Line2::new(begin, direction);
        let (t1, t2) = clamped_parameters(
            elclib::parameter(&line, begin)?,
            elclib::parameter(&line, end)?,
        );
        trimmed_2d(Handle::new(Geom2dLine::from_lin2d(line)).cast(), t1, t2)
    }
    /// The similarity of a transformation matrix, a fail when it is not one;
    /// `tool.transfer_transformation(&matrix)?`.
    ///
    /// # Errors
    /// A failure of the entity.
    pub fn transfer_transformation(
        &mut self,
        start: &Handle<IgesGeomTransformationMatrix>,
    ) -> Result<Handle<GeomTransformation>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        self.set_epsilon(1.0e-5);
        let mut similarity = Transform3::default();
        let location = live.value()?;
        if IgesDataToolLocation::convert_location(
            self.get_epsilon(),
            &location,
            &mut similarity,
            1.0,
        )? {
            return Ok(Handle::new(GeomTransformation::new(similarity)));
        }
        self.fail_key(start, "IGES_1036");
        Ok(Handle::default())
    }
}
