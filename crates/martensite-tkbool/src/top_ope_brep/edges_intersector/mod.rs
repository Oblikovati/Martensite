//! Edge intersections, transition classification and overlap-segment reduction.
use super::TopOpeBRepPoint2d;
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorSurface,
    topods::{Edge, Face},
};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
mod context;
mod corrections;
mod curves;
mod filters;
mod perform;
mod points;
mod records;
mod reduction;
mod transitions;

/// Face contexts and intersection-point iteration; `TopOpeBRepEdgesIntersector::new()`.
pub struct TopOpeBRepEdgesIntersector {
    faces: [Face; 2],
    surfaces: [BRepAdaptorSurface; 2],
    edges: [Edge; 2],
    curves: [Geom2dAdaptorCurve; 2],
    tolerances: (f64, f64),
    tolerance_forced: bool,
    surfaces_same: bool,
    faces_same: bool,
    dimension: i32,
    points: Vec<TopOpeBRepPoint2d>,
    cursor: i32,
    selected_count: i32,
    select_keep: bool,
    point_count: i32,
    segment_count: i32,
    same_domain: bool,
    raw_cursor: i32,
    reference_orientations: [bool; 2],
}
impl Default for TopOpeBRepEdgesIntersector {
    fn default() -> Self {
        Self {
            faces: Default::default(),
            surfaces: Default::default(),
            edges: Default::default(),
            curves: Default::default(),
            tolerances: (0., 0.),
            tolerance_forced: false,
            surfaces_same: false,
            faces_same: false,
            dimension: 2,
            points: Vec::new(),
            cursor: 0,
            selected_count: 0,
            select_keep: true,
            point_count: 0,
            segment_count: 0,
            same_domain: false,
            raw_cursor: 0,
            reference_orientations: [true; 2],
        }
    }
}
impl TopOpeBRepEdgesIntersector {
    /// Starts in dimension two with null inputs; `Self::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Changes dimension only for the accepted values 1 and 2; `state.set_dimension(1)`.
    pub fn set_dimension(&mut self, dimension: i32) {
        if matches!(dimension, 1 | 2) {
            self.dimension = dimension;
        }
    }
    /// Reads the working dimension; `state.dimension()`.
    pub fn dimension(&self) -> i32 {
        self.dimension
    }
    /// Retains both caller tolerances without clamping; `state.force_tolerances(1e-5, 2e-5)`.
    pub fn force_tolerances(&mut self, first: f64, second: f64) {
        self.tolerances = (first, second);
        self.tolerance_forced = true;
    }
    /// Reads the independent tolerance outputs; `state.tolerances()`.
    pub fn tolerances(&self) -> (f64, f64) {
        self.tolerances
    }
    /// Preserves std::max ordering, including a first NaN; `state.tolerance_max()`.
    pub fn tolerance_max(&self) -> f64 {
        let (first, second) = self.tolerances;
        if first < second { second } else { first }
    }
    /// Reads whether segment results exist; `state.has_segment()`.
    pub fn has_segment(&self) -> bool {
        self.segment_count != 0
    }
    /// Reads the computed same-domain flag; `state.same_domain()`.
    pub fn same_domain(&self) -> bool {
        self.same_domain
    }
    /// Reads the count of isolated intersections; `state.nb_points()`.
    pub fn nb_points(&self) -> i32 {
        self.point_count
    }
    /// Reads the count of geometric overlap segments; `state.nb_segments()`.
    pub fn nb_segments(&self) -> i32 {
        self.segment_count
    }
    /// Release diagnostics write nothing; `state.dump("edges", 1, 2)`.
    pub fn dump(&self, _label: &str, _first: i32, _second: i32) {}
}
