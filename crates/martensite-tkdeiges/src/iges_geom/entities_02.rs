#![allow(
    unused_assignments,
    clippy::if_same_then_else,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::manual_range_contains,
    clippy::assign_op_pattern,
    reason = "the members keep the form of the rules they state, and values a reader then ignores"
)]
use super::IgesGeomBoundary;
use crate::iges_basic::entity_shape::{iges_array1_lower, iges_direction};
use crate::iges_basic::shared_runs::{iges_shared_018, iges_shared_019, iges_shared_022};
use crate::iges_data::IgesDataIgesEntity;
use crate::{iges_entity, iges_methods};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkmath::gp::{
    Direction2, Direction3, GeneralTransform3, Point2, Point3, Xy, Xyz, dz,
};

iges_entity! {
/// defines BoundedSurface, Type `143` Form `0`
/// A bounded surface is used to communicate trimmed
/// surfaces. The surface and trimming curves are assumed
/// to be represented parametrically.
IgesGeomBoundedSurface, IgesGeomBoundedSurfaceState, "IgesGeomBoundedSurface", parent IgesDataIgesEntity, { type_: i32, surface: Handle<IgesDataIgesEntity>, boundaries: Handle<HArray1<Handle<IgesGeomBoundary>>> }
}
impl IgesGeomBoundedSurface {
    iges_methods! {
    get representation_type = type_: i32;
    get surface = surface: Handle<IgesDataIgesEntity>;
    len nb_boundaries = boundaries;
    item boundary = boundaries: Handle<IgesGeomBoundary>;
    }
    /// This method is used to set the fields of the class
    /// BoundedSurface
    /// - aType     : Type of bounded surface representation
    /// - aSurface  : Surface entity to be bounded
    /// - allBounds : Array of boundary entities
    pub fn init(
        &self,
        a_type: i32,
        a_surface: &Handle<IgesDataIgesEntity>,
        a_boundary: &Handle<HArray1<Handle<IgesGeomBoundary>>>,
    ) -> Result<(), Failure> {
        if iges_array1_lower(a_boundary, "IgesGeomBoundedSurface::init")? != 1 {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomBoundedSurface : Init",
            ));
        }
        {
            let mut state = self.state.write();
            state.type_ = a_type;
            state.surface = a_surface.clone();
            state.boundaries = a_boundary.clone();
        }
        self.init_type_and_form(143, 0);
        Ok(())
    }
}
iges_entity! {
/// defines IGESCircularArc, Type `100` Form `0`
/// A circular arc is a connected portion of a parent circle
/// which consists of more than one point. The definition space
/// coordinate system is always chosen so that the circular arc
/// remains in a plane either coincident with or parallel to
/// the XT, YT plane.
IgesGeomCircularArc, IgesGeomCircularArcState, "IgesGeomCircularArc", parent IgesDataIgesEntity, { zt: f64, center: Xy, start: Xy, end: Xy }
}
impl IgesGeomCircularArc {
    iges_methods! {
    get z_plane = zt: f64;
    }
    /// This method is used to set the fields of the class
    /// CircularArc
    /// - aZT     : Shift above the Z plane
    /// - aCenter : Center of the circle of which the arc forms a part
    /// - aStart  : Starting point of the circular arc
    /// - anEnd   : Ending point of the circular arc
    pub fn init(&self, a_zt: f64, a_center: Xy, a_start: Xy, an_end: Xy) {
        {
            let mut state = self.state.write();
            state.zt = a_zt;
            state.center = a_center;
            state.start = a_start;
            state.end = an_end;
        }
        self.init_type_and_form(100, 0);
    }
    /// returns the center of the circle of which arc forms a part
    pub fn center(&self) -> Point2 {
        let the_center = self.state.read().center;
        Point2::from(the_center)
    }
    /// returns the center of the circle of which arc forms a part
    /// after applying Transf. Matrix
    pub fn transformed_center(&self) -> Result<Point3, Failure> {
        let the_center = self.state.read().center;
        let the_zt = self.state.read().zt;
        let mut center: Xyz = Xyz::new(the_center.x(), the_center.y(), the_zt);
        if self.has_transf() {
            self.location()?.transforms(&mut center);
        }
        Ok(Point3::from(center))
    }
    /// returns the start point of the arc
    pub fn start_point(&self) -> Point2 {
        let the_start = self.state.read().start;
        Point2::from(the_start)
    }
    /// returns the start point of the arc after applying Transf. Matrix
    pub fn transformed_start_point(&self) -> Result<Point3, Failure> {
        iges_shared_019!(self, start, the_start, the_zt);
        Ok(Point3::from(start))
    }
    /// returns the end point of the arc
    pub fn end_point(&self) -> Point2 {
        let the_end = self.state.read().end;
        Point2::from(the_end)
    }
    /// returns the end point of the arc after applying Transf. Matrix
    pub fn transformed_end_point(&self) -> Result<Point3, Failure> {
        iges_shared_018!(end, self, the_end, the_zt);
        Ok(Point3::from(end))
    }
    /// returns the radius of the circle of which arc forms a part
    pub fn radius(&self) -> f64 {
        let the_start = self.state.read().start;
        let the_center = self.state.read().center;
        let x1: f64 = the_start.x();
        let y1: f64 = the_start.y();
        let x2: f64 = the_center.x();
        let y2: f64 = the_center.y();
        let (dx, dy) = (x2 - x1, y2 - y1);
        f64::sqrt(dx * dx + dy * dy)
    }
    /// returns the angle subtended by the arc at the center in radians
    pub fn angle(&self) -> Result<f64, Failure> {
        let the_center = self.state.read().center;
        let the_start = self.state.read().start;
        let the_end = self.state.read().end;
        let mut x1: f64 = 0.0;
        let mut y1: f64 = 0.0;
        let mut x2: f64 = 0.0;
        let mut y2: f64 = 0.0;
        let xc: f64 = the_center.x();
        let yc: f64 = the_center.y();
        x1 = the_start.x();
        y1 = the_start.y();
        x2 = the_end.x();
        y2 = the_end.y();
        let dir1: Direction2 = Direction2::new(x1 - xc, y1 - yc)?;
        let dir2: Direction2 = Direction2::new(x2 - xc, y2 - yc)?;
        let t: f64 = dir1.angle(dir2);
        Ok(t + (if t > 0.0 {
            0.0
        } else {
            2.0 * std::f64::consts::PI
        }))
    }
    /// Z-Axis of circle (i.e. [0,0,1])
    pub fn axis(&self) -> Direction3 {
        dz()
    }
    /// Z-Axis after applying Trans. Matrix
    pub fn transformed_axis(&self) -> Result<Direction3, Failure> {
        iges_shared_022!(axis, loc, self);
        iges_direction(axis)
    }
    /// True if StartPoint = EndPoint
    pub fn is_closed(&self) -> bool {
        let the_start = self.state.read().start;
        let the_end = self.state.read().end;
        (the_start.x() - the_end.x()).abs() < precision::p_confusion()
            && (the_start.y() - the_end.y()).abs() < precision::p_confusion()
    }
}
iges_entity! {
/// defines IGESCompositeCurve, Type `102` Form `0`
/// A composite curve is defined as an ordered list of entities
/// consisting of a point, connect point and parametrised curve
/// entities (excluding the CompositeCurve entity).
IgesGeomCompositeCurve, IgesGeomCompositeCurveState, "IgesGeomCompositeCurve", parent IgesDataIgesEntity, { entities: Handle<HArray1<Handle<IgesDataIgesEntity>>> }
}
impl IgesGeomCompositeCurve {
    iges_methods! {
    len nb_curves = entities;
    item curve = entities: Handle<IgesDataIgesEntity>;
    }
    /// This method is used to set the fields of the class
    /// CompositeCurve
    /// - allEntities : Constituent Entities of the composite curve
    pub fn init(
        &self,
        all_entities: &Handle<HArray1<Handle<IgesDataIgesEntity>>>,
    ) -> Result<(), Failure> {
        if !all_entities.is_null()
            && iges_array1_lower(all_entities, "IgesGeomCompositeCurve::init")? != 1
        {
            return Err(exceptions::dimension_mismatch(
                "IgesGeomCompositeCurve : Init",
            ));
        }
        self.state.write().entities = all_entities.clone();
        self.init_type_and_form(102, 0);
        Ok(())
    }
}
