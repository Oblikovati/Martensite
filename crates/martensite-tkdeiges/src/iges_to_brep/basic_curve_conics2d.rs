//! Conic arcs in the plane; a position that leaves the plane is not applied, with a warning.
use super::IgesToBRepBasicCurve;
use super::basic_curve::trimmed_2d;
use super::basic_curve_conics::conic_is_circular;
use crate::iges_geom::IgesGeomConicArc;
use martensite_tkernel::message::Msg;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dCircle, Geom2dCurve, Geom2dEllipse, Geom2dHyperbola, Geom2dParabola,
};
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{Axis2d, Direction2, Direction3, Point2, Point3};
use std::f64::consts::PI;

/// The geometric definition of a conic arc in the plane.
struct ConicPlacement2d {
    frame: Axis2d,
    start: Point2,
    end: Point2,
    minor: f64,
    major: f64,
}

impl IgesToBRepBasicCurve {
    fn conic_placement_2d(&self, live: &IgesGeomConicArc) -> Result<ConicPlacement2d, Failure> {
        let mut center = Point3::new(0.0, 0.0, 0.0);
        let mut main_axis = Direction3::new(1.0, 0.0, 0.0)?;
        let (mut minor, mut major) = (0.0, 0.0);
        let (start, end) = if self.applies_position(live) {
            live.transformed_definition(&mut center, &mut main_axis, &mut minor, &mut major)?;
            let (begin, finish) = (
                live.transformed_start_point()?,
                live.transformed_end_point()?,
            );
            (
                Point2::new(begin.x(), begin.y()),
                Point2::new(finish.x(), finish.y()),
            )
        } else {
            live.definition(&mut center, &mut main_axis, &mut minor, &mut major)?;
            (live.start_point(), live.end_point())
        };
        Ok(ConicPlacement2d {
            frame: Axis2d::new(
                Point2::new(center.x(), center.y()),
                Direction2::new(main_axis.x(), main_axis.y())?,
            ),
            start,
            end,
            minor,
            major,
        })
    }
    /// A flipped axis runs the curve the other way.
    fn flip_when_opposite(
        &self,
        live: &IgesGeomConicArc,
        curve: &dyn Geom2dCurve,
    ) -> Result<(), Failure> {
        if live
            .transformed_axis()?
            .is_opposite(live.axis(), self.get_epsilon())
        {
            curve.reverse()?;
        }
        Ok(())
    }
    /// The curve trimmed unless the arc is degenerate, which is a warning and the whole curve.
    fn cyclic_trim_2d(
        &self,
        start: &Handle<IgesGeomConicArc>,
        curve: Handle<dyn Geom2dCurve>,
        (t1, mut t2): (f64, f64),
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let tolerance = precision::p_confusion();
        if t2 < t1 && t1 - t2 > tolerance {
            t2 += 2.0 * PI;
        }
        if (t1 - t2).abs() <= tolerance {
            self.warn_key(start, "IGES_1160");
            return Ok(curve);
        }
        trimmed_2d(curve, t1, t2)
    }
    /// The curve trimmed in increasing parameters, the whole curve with a warning when degenerate.
    fn open_trim_2d(
        &self,
        start: &Handle<IgesGeomConicArc>,
        curve: Handle<dyn Geom2dCurve>,
        t1: f64,
        t2: f64,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        if (t1 - t2).abs() <= precision::p_confusion() {
            self.warn_key(start, "IGES_1160");
            return Ok(curve);
        }
        trimmed_2d(curve, t1.min(t2), t1.max(t2))
    }
    fn conic_circle_2d(
        &self,
        start: &Handle<IgesGeomConicArc>,
        live: &IgesGeomConicArc,
        placement: &ConicPlacement2d,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let circle = Geom2dCircle::from_axis(placement.frame, placement.minor, true)?;
        self.flip_when_opposite(live, &circle)?;
        let plane = circle.circ2d()?;
        let curve: Handle<dyn Geom2dCurve> = Handle::new(circle).cast();
        if live.is_closed() {
            return Ok(curve);
        }
        let parameters = (
            elclib::parameter(&plane, placement.start)?,
            elclib::parameter(&plane, placement.end)?,
        );
        self.cyclic_trim_2d(start, curve, parameters)
    }
    fn conic_parabola_2d(
        &self,
        start: &Handle<IgesGeomConicArc>,
        live: &IgesGeomConicArc,
        placement: &ConicPlacement2d,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        // PTV 26.03.2002: the definition gives the focal parameter.
        let parabola = Geom2dParabola::from_axis(placement.frame, placement.minor / 2.0, true)?;
        self.flip_when_opposite(live, &parabola)?;
        let plane = parabola.parab2d()?;
        let t1 = elclib::parameter(&plane, placement.start)?;
        let t2 = elclib::parameter(&plane, placement.end)?;
        self.open_trim_2d(start, Handle::new(parabola).cast(), t1, t2)
    }
    fn conic_ellipse_2d(
        &self,
        start: &Handle<IgesGeomConicArc>,
        live: &IgesGeomConicArc,
        placement: &ConicPlacement2d,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let ellipse =
            Geom2dEllipse::from_axis(placement.frame, placement.major, placement.minor, true)?;
        self.flip_when_opposite(live, &ellipse)?;
        let plane = ellipse.elips2d()?;
        let curve: Handle<dyn Geom2dCurve> = Handle::new(ellipse).cast();
        if live.is_closed() {
            return Ok(curve);
        }
        let parameters = (
            elclib::parameter(&plane, placement.start)?,
            elclib::parameter(&plane, placement.end)?,
        );
        self.cyclic_trim_2d(start, curve, parameters)
    }
    fn conic_hyperbola_2d(
        &self,
        start: &Handle<IgesGeomConicArc>,
        live: &IgesGeomConicArc,
        placement: &ConicPlacement2d,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let hyperbola =
            Geom2dHyperbola::from_axis(placement.frame, placement.major, placement.minor, true)?;
        self.flip_when_opposite(live, &hyperbola)?;
        let plane = hyperbola.hypr2d()?;
        let t1 = elclib::parameter(&plane, placement.start)?;
        let t2 = elclib::parameter(&plane, placement.end)?;
        self.open_trim_2d(start, Handle::new(hyperbola).cast(), t1, t2)
    }
    /// The conic of the entity in the plane; a fail when the coefficients define none;
    /// `tool.transfer_2d_conic_arc(&conic)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_2d_conic_arc(
        &mut self,
        start: &Handle<IgesGeomConicArc>,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        if live.computed_form_number() == 0 {
            self.fail_key(start, "IGES_1155");
            return Ok(Handle::default());
        }
        self.set_epsilon(1.0e-3);
        if !live
            .transformed_axis()?
            .is_parallel(live.axis(), self.get_epsilon())
        {
            self.set_mode_transfer(true);
            let mut message = Msg::new();
            message.set_text(b"The Trsf is not compatible with a transfer2d, it will not applied.");
            self.send_warning(start, &message);
        }
        let placement = self.conic_placement_2d(live)?;
        if live.is_from_parabola() && !live.is_from_ellipse() {
            return self.conic_parabola_2d(start, live, &placement);
        }
        if !live.is_from_ellipse() {
            return self.conic_hyperbola_2d(start, live, &placement);
        }
        if conic_is_circular(live) {
            return self.conic_circle_2d(start, live, &placement);
        }
        self.conic_ellipse_2d(start, live, &placement)
    }
}
