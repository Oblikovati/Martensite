//! Conic arcs in space: circle, parabola, ellipse and hyperbola.
use super::IgesToBRepBasicCurve;
use super::basic_curve::trimmed_3d;
use crate::iges_geom::IgesGeomConicArc;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola, GeomParabola};
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Ellipse3, Hyperbola3, Parabola3, Point3};
use std::f64::consts::PI;

/// The coefficient tolerance, of the dimension of the inverse square of a length.
/// An ellipse with big radii has small coefficients.
pub(super) fn squared_tolerance() -> f64 {
    precision::p_confusion() * precision::p_confusion()
}

/// The geometric definition of a conic arc in space.
struct ConicPlacement {
    frame: Axis2,
    start: Point3,
    end: Point3,
    minor: f64,
    major: f64,
}

impl IgesToBRepBasicCurve {
    fn conic_placement(&self, live: &IgesGeomConicArc) -> Result<ConicPlacement, Failure> {
        let mut center = Point3::new(0.0, 0.0, 0.0);
        let mut main_axis = Direction3::new(1.0, 0.0, 0.0)?;
        let (mut minor, mut major) = (0.0, 0.0);
        let (normal, start, end) = if self.applies_position(live) {
            live.transformed_definition(&mut center, &mut main_axis, &mut minor, &mut major)?;
            (
                live.transformed_axis()?,
                live.transformed_start_point()?,
                live.transformed_end_point()?,
            )
        } else {
            live.definition(&mut center, &mut main_axis, &mut minor, &mut major)?;
            let z = live.z_plane();
            let (begin, finish) = (live.start_point(), live.end_point());
            (
                live.axis(),
                Point3::new(begin.x(), begin.y(), z),
                Point3::new(finish.x(), finish.y(), z),
            )
        };
        Ok(ConicPlacement {
            frame: Axis2::with_x_direction(center, normal, main_axis)?,
            start,
            end,
            minor,
            major,
        })
    }
    /// The curve trimmed unless the arc is degenerate, which is a warning and the whole curve.
    fn closed_conic_trim(
        &self,
        start: &Handle<IgesGeomConicArc>,
        curve: Handle<dyn GeomCurve>,
        (t1, mut t2): (f64, f64),
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let tolerance = precision::confusion();
        if t2 < t1 && t1 - t2 > tolerance {
            t2 += 2.0 * PI;
        }
        if (t1 - t2).abs() <= tolerance {
            self.warn_key(start, "IGES_1160");
            return Ok(curve);
        }
        trimmed_3d(curve, t1, t2)
    }
    fn conic_circle(
        &self,
        start: &Handle<IgesGeomConicArc>,
        live: &IgesGeomConicArc,
        placement: &ConicPlacement,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let radius = placement.minor;
        let curve: Handle<dyn GeomCurve> =
            Handle::new(GeomCircle::new(placement.frame, radius)?).cast();
        if live.is_closed() {
            return Ok(curve);
        }
        let circle = Circle3::new(placement.frame, radius)?;
        let parameters = (
            elclib::parameter(&circle, placement.start)?,
            elclib::parameter(&circle, placement.end)?,
        );
        self.closed_conic_trim(start, curve, parameters)
    }
    fn conic_parabola(
        &self,
        start: &Handle<IgesGeomConicArc>,
        placement: &ConicPlacement,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        // PTV 26.03.2002: the definition gives the focal parameter.
        let focal = placement.minor / 2.0;
        let parabola = Parabola3::new(placement.frame, focal)?;
        let curve: Handle<dyn GeomCurve> =
            Handle::new(GeomParabola::new(placement.frame, focal)?).cast();
        let t1 = elclib::parameter(&parabola, placement.start)?;
        let t2 = elclib::parameter(&parabola, placement.end)?;
        if (t1 - t2).abs() <= precision::confusion() {
            self.warn_key(start, "IGES_1160");
            return Ok(curve);
        }
        // With t1 > t2 the course of the curve is reversed.
        trimmed_3d(curve, t1, t2)
    }
    fn conic_ellipse(
        &self,
        start: &Handle<IgesGeomConicArc>,
        live: &IgesGeomConicArc,
        placement: &ConicPlacement,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let (major, minor) = (placement.major, placement.minor);
        let curve: Handle<dyn GeomCurve> =
            Handle::new(GeomEllipse::new(placement.frame, major, minor)?).cast();
        if live.is_closed() {
            return Ok(curve);
        }
        let ellipse = Ellipse3::new(placement.frame, major, minor)?;
        let parameters = (
            elclib::parameter(&ellipse, placement.start)?,
            elclib::parameter(&ellipse, placement.end)?,
        );
        self.closed_conic_trim(start, curve, parameters)
    }
    fn conic_hyperbola(
        &self,
        start: &Handle<IgesGeomConicArc>,
        placement: &ConicPlacement,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let (major, minor) = (placement.major, placement.minor);
        let hyperbola = Hyperbola3::new(placement.frame, major, minor)?;
        let t1 = elclib::parameter(&hyperbola, placement.start)?;
        let t2 = elclib::parameter(&hyperbola, placement.end)?;
        let curve: Handle<dyn GeomCurve> =
            Handle::new(GeomHyperbola::new(placement.frame, major, minor)?).cast();
        // The precision of parameters.
        if (t1 - t2).abs() <= precision::p_confusion() {
            self.warn_key(start, "IGES_1160");
            return Ok(curve);
        }
        trimmed_3d(curve, t1.min(t2), t1.max(t2))
    }
    /// The conic of the entity; a fail when the coefficients define none;
    /// `tool.transfer_conic_arc(&conic)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_conic_arc(
        &self,
        start: &Handle<IgesGeomConicArc>,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        if live.computed_form_number() == 0 {
            self.fail_key(start, "IGES_1155");
            return Ok(Handle::default());
        }
        let placement = self.conic_placement(live)?;
        if live.is_from_parabola() && !live.is_from_ellipse() {
            return self.conic_parabola(start, &placement);
        }
        if !live.is_from_ellipse() {
            return self.conic_hyperbola(start, &placement);
        }
        if conic_is_circular(live) {
            return self.conic_circle(start, live, &placement);
        }
        self.conic_ellipse(start, live, &placement)
    }
}

/// An ellipse whose equation has equal square terms and no mixed term.
pub(super) fn conic_is_circular(live: &IgesGeomConicArc) -> bool {
    let (mut a, mut b, mut c, mut d, mut e, mut f) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    live.equation(&mut a, &mut b, &mut c, &mut d, &mut e, &mut f);
    (a - c).abs() <= squared_tolerance() && b.abs() < squared_tolerance()
}
