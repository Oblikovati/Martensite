//! `StepToTopoDSTool` and `StepToTopoDSPointPair`: the shapes already made from the entities of a
//! representation, and the statistics of the geometry met.
use crate::step_geom::StepGeomCartesianPoint;
use crate::step_shape::StepShapeTopologicalRepresentationItem;
use martensite_tkbrep::topods::{Edge, Shape, Vertex};
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface};
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkxsbase::transfer::TransferTransientProcess;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// `StepToTopoDSPointPair`: the two ends of an edge, whichever way it is given.
///
/// ```
/// use martensite_tkdestep::{step_geom::StepGeomCartesianPoint, step_to_topo_ds::StepToTopoDSPointPair};
/// use martensite_tkernel::standard::Handle;
/// let (a, b) = (Handle::new(StepGeomCartesianPoint::new()), Handle::new(StepGeomCartesianPoint::new()));
/// assert!(StepToTopoDSPointPair::new(&a, &b) == StepToTopoDSPointPair::new(&b, &a));
/// ```
#[derive(Clone)]
pub struct StepToTopoDSPointPair {
    my_p1: Handle<StepGeomCartesianPoint>,
    my_p2: Handle<StepGeomCartesianPoint>,
}

impl StepToTopoDSPointPair {
    /// `StepToTopoDSPointPair(P1, P2)`.
    pub fn new(p1: &Handle<StepGeomCartesianPoint>, p2: &Handle<StepGeomCartesianPoint>) -> Self {
        Self {
            my_p1: p1.clone(),
            my_p2: p2.clone(),
        }
    }
    /// `GetPoint1`.
    pub fn point1(&self) -> &Handle<StepGeomCartesianPoint> {
        &self.my_p1
    }
    /// `get_point2`.
    pub fn point2(&self) -> &Handle<StepGeomCartesianPoint> {
        &self.my_p2
    }
}

impl PartialEq for StepToTopoDSPointPair {
    fn eq(&self, other: &Self) -> bool {
        (self.my_p1 == other.my_p1 && self.my_p2 == other.my_p2)
            || (self.my_p1 == other.my_p2 && self.my_p2 == other.my_p1)
    }
}

impl Eq for StepToTopoDSPointPair {}

impl Hash for StepToTopoDSPointPair {
    /// Symmetric, as the equality: the addresses are hashed in increasing order.
    fn hash<S: Hasher>(&self, state: &mut S) {
        let first = self.my_p1.object_address() as usize;
        let second = self.my_p2.object_address() as usize;
        (first.min(second), first.max(second)).hash(state);
    }
}

/// The statistics of one kind of geometry: the counts in C0, C1 and C2 or better.
#[derive(Clone, Copy, Debug, Default)]
struct ContinuityCounts {
    c0: i32,
    c1: i32,
    c2: i32,
}

impl ContinuityCounts {
    fn add(&mut self, continuity: Continuity) {
        match continuity {
            Continuity::C0 => self.c0 += 1,
            Continuity::C1 => self.c1 += 1,
            _ => self.c2 += 1,
        }
    }
}

/// `StepToTopoDSTool`. The edge and vertex maps are never iterated, so no output can depend on
/// their hash order.
#[derive(Default)]
pub struct StepToTopoDSTool {
    my_data_map: DataMap<Handle<StepShapeTopologicalRepresentationItem>, Shape>,
    my_vertex_map: HashMap<usize, (Handle<StepGeomCartesianPoint>, Vertex)>,
    my_edge_map: HashMap<StepToTopoDSPointPair, Edge>,
    my_compute_pc: bool,
    my_trans_proc: Handle<TransferTransientProcess>,
    my_surfaces: ContinuityCounts,
    my_curves_2d: ContinuityCounts,
    my_curves_3d: ContinuityCounts,
}

impl StepToTopoDSTool {
    /// `StepToTopoDSTool()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSTool(Map, TP)`.
    pub fn with_map(
        map: &DataMap<Handle<StepShapeTopologicalRepresentationItem>, Shape>,
        tp: &Handle<TransferTransientProcess>,
    ) -> Self {
        let mut tool = Self::new();
        tool.init(map, tp);
        tool
    }
    /// `init(Map, TP)`: the maps and the statistics restart from `map`.
    pub fn init(
        &mut self,
        map: &DataMap<Handle<StepShapeTopologicalRepresentationItem>, Shape>,
        tp: &Handle<TransferTransientProcess>,
    ) {
        *self = Self {
            my_data_map: map.clone(),
            my_trans_proc: tp.clone(),
            ..Self::default()
        };
    }
    /// `is_bound(TRI)`.
    pub fn is_bound(&self, tri: &Handle<StepShapeTopologicalRepresentationItem>) -> bool {
        self.my_data_map.is_bound(tri)
    }
    /// `bind(TRI, S)`: complement CKY: also feeds the transfer process.
    pub fn bind(
        &mut self,
        tri: &Handle<StepShapeTopologicalRepresentationItem>,
        shape: &Shape,
    ) -> Result<(), Failure> {
        self.my_data_map.bind(tri.clone(), shape.clone());
        TransferBRep::set_shape_result(&self.my_trans_proc, &tri.cast(), shape)
    }
    /// `find(TRI)`.
    ///
    /// # Errors
    /// a no such object failure when `tri` is not bound.
    pub fn find(
        &self,
        tri: &Handle<StepShapeTopologicalRepresentationItem>,
    ) -> Result<&Shape, Failure> {
        self.my_data_map.find(tri)
    }
    /// `clear_edge_map`.
    pub fn clear_edge_map(&mut self) {
        self.my_edge_map.clear();
    }
    /// `is_edge_bound(PP)`.
    pub fn is_edge_bound(&self, pair: &StepToTopoDSPointPair) -> bool {
        self.my_edge_map.contains_key(pair)
    }
    /// `bind_edge(PP, E)`.
    pub fn bind_edge(&mut self, pair: &StepToTopoDSPointPair, edge: &Edge) {
        self.my_edge_map.insert(pair.clone(), edge.clone());
    }
    /// `find_edge(PP)`: `None` when `pair` is not bound (a no such object failure).
    pub fn find_edge(&self, pair: &StepToTopoDSPointPair) -> Option<&Edge> {
        self.my_edge_map.get(pair)
    }
    /// `clear_vertex_map`.
    pub fn clear_vertex_map(&mut self) {
        self.my_vertex_map.clear();
    }
    /// `is_vertex_bound(PG)`.
    pub fn is_vertex_bound(&self, point: &Handle<StepGeomCartesianPoint>) -> bool {
        self.my_vertex_map
            .contains_key(&(point.object_address() as usize))
    }
    /// `bind_vertex(P, V)`: the point is kept with the vertex, so that its address stays its own.
    pub fn bind_vertex(&mut self, point: &Handle<StepGeomCartesianPoint>, vertex: &Vertex) {
        let entry = (point.clone(), vertex.clone());
        self.my_vertex_map
            .insert(point.object_address() as usize, entry);
    }
    /// `find_vertex(P)`: `None` when `point` is not bound (a no such object failure).
    pub fn find_vertex(&self, point: &Handle<StepGeomCartesianPoint>) -> Option<&Vertex> {
        self.my_vertex_map
            .get(&(point.object_address() as usize))
            .map(|entry| &entry.1)
    }
    /// `ComputePCurve(B)`.
    pub fn set_compute_pcurve(&mut self, compute: bool) {
        self.my_compute_pc = compute;
    }
    /// `ComputePCurve()`.
    pub fn compute_pcurve(&self) -> bool {
        self.my_compute_pc
    }
    /// `transient_process`.
    pub fn transient_process(&self) -> Handle<TransferTransientProcess> {
        self.my_trans_proc.clone()
    }
    /// `AddContinuity(GeomSurface)`.
    pub fn add_continuity_surface(&mut self, surface: &dyn GeomSurface) {
        self.my_surfaces.add(surface.continuity());
    }
    /// `AddContinuity(GeomCurve)`.
    pub fn add_continuity_curve(&mut self, curve: &dyn GeomCurve) {
        self.my_curves_3d.add(curve.continuity());
    }
    /// `AddContinuity(Geom2dCurve)`.
    pub fn add_continuity_curve_2d(&mut self, curve: &dyn Geom2dCurve) {
        self.my_curves_2d.add(curve.continuity());
    }
    /// `C0Surf`.
    pub fn c0_surf(&self) -> i32 {
        self.my_surfaces.c0
    }
    /// `C1Surf`.
    pub fn c1_surf(&self) -> i32 {
        self.my_surfaces.c1
    }
    /// `C2Surf`.
    pub fn c2_surf(&self) -> i32 {
        self.my_surfaces.c2
    }
    /// `C0Cur2`.
    pub fn c0_cur2(&self) -> i32 {
        self.my_curves_2d.c0
    }
    /// `C1Cur2`.
    pub fn c1_cur2(&self) -> i32 {
        self.my_curves_2d.c1
    }
    /// `C2Cur2`.
    pub fn c2_cur2(&self) -> i32 {
        self.my_curves_2d.c2
    }
    /// `C0Cur3`.
    pub fn c0_cur3(&self) -> i32 {
        self.my_curves_3d.c0
    }
    /// `C1Cur3`.
    pub fn c1_cur3(&self) -> i32 {
        self.my_curves_3d.c1
    }
    /// `C2Cur3`.
    pub fn c2_cur3(&self) -> i32 {
        self.my_curves_3d.c2
    }
}
