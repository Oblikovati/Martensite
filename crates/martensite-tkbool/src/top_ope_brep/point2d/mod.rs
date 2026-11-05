//! Intersection records with independent per-edge and segment bookkeeping.
use super::{TopOpeBRepHctxee2d, TopOpeBRepHctxff2d, TopOpeBRepP2DStatus};
use crate::top_ope_brep_ds::{TopOpeBRepDSConfig, TopOpeBRepDSTransition};
use martensite_tkbrep::topods::Vertex;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::int_res2d::IntersectionPoint;
use martensite_tkmath::gp::{Point2, Point3};
mod indexed;
mod properties;

#[derive(Default)]
struct EdgePoint {
    is_vertex: bool,
    vertex: Vertex,
    transition: TopOpeBRepDSTransition,
    parameter: f64,
}
/// An intersection point with two independently classified edges; `TopOpeBRepPoint2d::new()`.
pub struct TopOpeBRepPoint2d {
    pint: Option<IntersectionPoint>,
    edges: [EdgePoint; 2],
    point_of_segment: bool,
    ancestors: (i32, i32),
    has_ancestors: bool,
    status: TopOpeBRepP2DStatus,
    index: i32,
    point: Point3,
    point2d: Point2,
    keep: bool,
    config: TopOpeBRepDSConfig,
    tolerance: f64,
    context_ff: Handle<TopOpeBRepHctxff2d>,
    context_ee: Handle<TopOpeBRepHctxee2d>,
}
impl Default for TopOpeBRepPoint2d {
    fn default() -> Self {
        Self {
            pint: None,
            edges: [EdgePoint::default(), EdgePoint::default()],
            point_of_segment: false,
            ancestors: (0, 0),
            has_ancestors: false,
            status: TopOpeBRepP2DStatus::Unknown,
            index: 0,
            point: Point3::default(),
            point2d: Point2::default(),
            keep: true,
            config: TopOpeBRepDSConfig::UnshGeometry,
            tolerance: 0.,
            context_ff: Handle::default(),
            context_ee: Handle::default(),
        }
    }
}
impl TopOpeBRepPoint2d {
    /// Starts with zero parameters and a retained unknown point; `Self::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Copies the geometric intersection point and marks its presence; `point.set_pint(&intersection)`.
    pub fn set_pint(&mut self, point: &IntersectionPoint) {
        self.pint = Some(*point);
    }
    /// Checks whether an intersection point has been assigned; `point.has_pint()`.
    pub fn has_pint(&self) -> bool {
        self.pint.is_some()
    }
    /// Reads the assigned intersection or its exact native failure; `point.pint()?`.
    pub fn pint(&self) -> Result<&IntersectionPoint, Failure> {
        self.pint
            .as_ref()
            .ok_or_else(|| Failure::new("TopOpeBRepPoint2d::Pint"))
    }
    /// Stores both ancestor indices and sets NEW only if both are nonzero; `point.set_segment_ancestors(3, 7)`.
    pub fn set_segment_ancestors(&mut self, first: i32, second: i32) {
        self.ancestors = (first, second);
        self.has_ancestors = first != 0 && second != 0;
        if self.has_ancestors {
            self.status = TopOpeBRepP2DStatus::New;
        }
    }
    /// Reads the flag and both outputs even when the flag is false; `point.segment_ancestors()`.
    pub fn segment_ancestors(&self) -> (bool, i32, i32) {
        (self.has_ancestors, self.ancestors.0, self.ancestors.1)
    }
    /// The release-mode diagnostic writes nothing; `point.dump(0, 0)`.
    pub fn dump(&self, _first_edge: i32, _second_edge: i32) {}
}
