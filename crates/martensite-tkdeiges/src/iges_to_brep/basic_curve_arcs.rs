//! Circular arcs, in space and in the plane.
use super::IgesToBRepBasicCurve;
use super::basic_curve::{trimmed_2d, trimmed_3d};
use crate::iges_geom::IgesGeomCircularArc;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve};
use martensite_tkg3d::geom::{GeomCircle, GeomCurve};
use martensite_tkmath::elclib;
use martensite_tkmath::gp::{Axis2, Axis2d, Circle3, Direction2, Direction3, Point2, Point3, Xyz};
use std::f64::consts::PI;

/// The frame and the ends of an arc in space.
struct ArcPlacement {
    frame: Axis2,
    start: Point3,
    end: Point3,
}

impl IgesToBRepBasicCurve {
    fn positioned_arc(&self, live: &IgesGeomCircularArc) -> Result<ArcPlacement, Failure> {
        let mut location = live.location()?;
        location.set_translation_part(Xyz::new(0.0, 0.0, 0.0));
        let main_axis = location.apply(Xyz::new(1.0, 0.0, 0.0));
        Ok(ArcPlacement {
            frame: Axis2::with_x_direction(
                live.transformed_center()?,
                live.transformed_axis()?,
                Direction3::from_xyz(main_axis)?,
            )?,
            start: live.transformed_start_point()?,
            end: live.transformed_end_point()?,
        })
    }
    fn plain_arc(live: &IgesGeomCircularArc) -> Result<ArcPlacement, Failure> {
        let z = live.z_plane();
        let at_z = |point: Point2| Point3::new(point.x(), point.y(), z);
        Ok(ArcPlacement {
            frame: Axis2::with_x_direction(
                at_z(live.center()),
                live.axis(),
                Direction3::new(1.0, 0.0, 0.0)?,
            )?,
            start: at_z(live.start_point()),
            end: at_z(live.end_point()),
        })
    }
    /// The parameters of a trimmed arc, `None` for a whole circle starting at zero.
    fn arc_range(
        &self,
        live: &IgesGeomCircularArc,
        (t1, mut t2): (f64, f64),
        distance: f64,
    ) -> Result<Option<(f64, f64)>, Failure> {
        let closed = live.is_closed();
        let eps = self.get_eps_geom();
        if closed && t1 >= eps {
            t2 = t1 + 2.0 * PI;
        }
        if !closed && (t1 - t2).abs() <= precision::p_confusion() {
            // Micro-arc: t2 - t1 is the distance of the ends over the radius.
            t2 = t1 + distance / live.radius();
        }
        if closed && t1 < eps {
            return Ok(None);
        }
        if t2 < t1 {
            t2 += 2.0 * PI;
        }
        Ok(Some((t1, t2)))
    }
    /// The circle or its arc; `tool.transfer_circular_arc(&arc)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_circular_arc(
        &self,
        start: &Handle<IgesGeomCircularArc>,
    ) -> Result<Handle<dyn GeomCurve>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        let placement = if self.applies_position(live) {
            self.positioned_arc(live)?
        } else {
            Self::plain_arc(live)?
        };
        let radius = live.radius();
        let circle = Circle3::new(placement.frame, radius)?;
        let curve: Handle<dyn GeomCurve> =
            Handle::new(GeomCircle::new(placement.frame, radius)?).cast();
        let parameters = (
            elclib::parameter(&circle, placement.start)?,
            elclib::parameter(&circle, placement.end)?,
        );
        let distance = placement.start.distance(&placement.end);
        match self.arc_range(live, parameters, distance)? {
            Some((t1, t2)) => trimmed_3d(curve, t1, t2),
            None => Ok(curve),
        }
    }
    /// The circle or its arc in the plane; a position that leaves the plane is not applied, with a
    /// warning; `tool.transfer_2d_circular_arc(&arc)?`.
    ///
    /// # Errors
    /// A failure of the entity or of the construction of the curve.
    pub fn transfer_2d_circular_arc(
        &mut self,
        start: &Handle<IgesGeomCircularArc>,
    ) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        let Some(live) = start.get() else {
            self.null_entity(start);
            return Ok(Handle::default());
        };
        self.set_epsilon(1.0e-3);
        if !live
            .transformed_axis()?
            .is_parallel(live.axis(), self.get_epsilon())
        {
            self.set_mode_transfer(true);
            self.warn_key(start, "IGES_1165");
        }
        let positioned = self.applies_position(live);
        let center = live.center();
        let mut origin = Xyz::new(center.x(), center.y(), 0.0);
        let mut main_axis = Xyz::new(1.0, 0.0, 0.0);
        if positioned {
            let mut location = live.location()?;
            origin = location.apply(origin);
            location.set_translation_part(Xyz::new(0.0, 0.0, 0.0));
            main_axis = location.apply(main_axis);
        }
        let frame = Axis2d::new(
            Point2::new(origin.x(), origin.y()),
            Direction2::new(main_axis.x(), main_axis.y())?,
        );
        let circle = Geom2dCircle::from_axis(frame, live.radius(), true)?;
        let (begin, end) = self.arc_ends_2d(live, positioned, &circle)?;
        let plane = circle.circ2d()?;
        let parameters = (
            elclib::parameter(&plane, begin)?,
            elclib::parameter(&plane, end)?,
        );
        let curve: Handle<dyn Geom2dCurve> = Handle::new(circle).cast();
        match self.arc_range(live, parameters, begin.distance(end))? {
            Some((t1, t2)) => trimmed_2d(curve, t1, t2),
            None => Ok(curve),
        }
    }
    fn arc_ends_2d(
        &self,
        live: &IgesGeomCircularArc,
        positioned: bool,
        circle: &Geom2dCircle,
    ) -> Result<(Point2, Point2), Failure> {
        if !positioned {
            return Ok((live.start_point(), live.end_point()));
        }
        // A flipped axis runs the circle the other way.
        if live
            .transformed_axis()?
            .is_opposite(live.axis(), self.get_epsilon())
        {
            circle.reverse()?;
        }
        let (begin, end) = (
            live.transformed_start_point()?,
            live.transformed_end_point()?,
        );
        Ok((
            Point2::new(begin.x(), begin.y()),
            Point2::new(end.x(), end.y()),
        ))
    }
}
