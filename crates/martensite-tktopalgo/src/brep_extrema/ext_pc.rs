//! `BRepExtrema_ExtPC`: the extrema between a vertex and an edge.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topods::{Edge, Vertex};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkgeombase::extrema::ExtPC;
use martensite_tkmath::gp::Point3;

/// `BRepExtrema_ExtPC`: `Extrema_ExtPC` on the 3D
/// curve of an edge over its range, with the parameter tolerance of the edge.
///
/// The persistent extrema member retains its adaptor owner and previously calculated
/// endpoint coordinates across initialization. Replaying earlier calls would instead
/// recalculate those historical values after shared source geometry changes.
///
/// ```
/// use martensite_tktopalgo::brep_extrema::BRepExtremaExtPC;
/// let ext = BRepExtremaExtPC::new();
/// assert!(!ext.is_done());
/// assert!(ext.nb_ext().is_err());
/// ```
#[derive(Clone)]
pub struct BRepExtremaExtPC {
    my_hc: Handle<BRepAdaptorCurve>,
    my_ext_pc: ExtPC<'static>,
}

impl Default for BRepExtremaExtPC {
    fn default() -> Self {
        Self::new()
    }
}

impl BRepExtremaExtPC {
    /// `BRepExtrema_ExtPC()`: no edge; `BRepExtremaExtPC::new()`.
    pub fn new() -> Self {
        Self {
            my_hc: Handle::default(),
            my_ext_pc: ExtPC::new(),
        }
    }

    /// `BRepExtrema_ExtPC(V, E)`: `Initialize(E)` then `Perform(V)`;
    /// `BRepExtremaExtPC::with_vertex_edge(&vertex, &edge)?`.
    pub fn with_vertex_edge(vertex: &Vertex, edge: &Edge) -> Result<Self, Failure> {
        let mut ext: Self = Self::new();
        ext.initialize(edge)?;
        ext.perform(vertex)?;
        Ok(ext)
    }

    /// `Initialize(E)`: the adaptor of the edge, the parameter tolerance (the curve
    /// resolution of min(edge tolerance, Confusion), at least PConfusion) and the edge
    /// range; an edge without a 3D curve or curve on surface changes nothing;
    /// `ext.initialize(&edge)?`.
    pub fn initialize(&mut self, edge: &Edge) -> Result<(), Failure> {
        if !BRepTool::is_geometric_edge(edge)? {
            return Ok(());
        }
        self.my_hc = Handle::new(BRepAdaptorCurve::with_edge(edge)?);
        let tol: f64 = BRepTool::edge_tolerance(edge)?.min(precision::confusion());
        let tol: f64 = self.my_hc.resolution(tol)?.max(precision::p_confusion());
        let range: (f64, f64) = BRepTool::range(edge)?;
        let curve: Handle<dyn Adaptor3dCurve> = self.my_hc.clone().cast();
        self.my_ext_pc
            .initialize_shared(&curve, range.0, range.1, tol)
    }

    /// `Perform(V)`: the extrema from the vertex point, when an edge was initialized;
    /// `ext.perform(&vertex)?`.
    pub fn perform(&mut self, vertex: &Vertex) -> Result<(), Failure> {
        if self.my_hc.is_null() {
            return Ok(());
        }
        let point: Point3 = BRepTool::pnt(vertex)?;
        self.my_ext_pc.perform(point)
    }

    /// `IsDone()`: true when the distances are found; `ext.is_done()`.
    pub fn is_done(&self) -> bool {
        self.my_ext_pc.is_done()
    }

    /// `NbExt()`: the number of extremum distances (StdFail_NotDone when not done);
    /// `ext.nb_ext()?`.
    pub fn nb_ext(&self) -> Result<i32, Failure> {
        self.my_ext_pc.nb_ext()
    }

    /// `IsMin(N)`: whether the `N`-th extremum distance is a minimum; `ext.is_min(1)?`.
    pub fn is_min(&self, n: i32) -> Result<bool, Failure> {
        self.my_ext_pc.is_min(n)
    }

    /// `SquareDistance(N)`: the `N`-th extremum square distance;
    /// `ext.square_distance(1)?`.
    pub fn square_distance(&self, n: i32) -> Result<f64, Failure> {
        self.my_ext_pc.square_distance(n)
    }

    /// `Parameter(N)`: the parameter on the edge of the `N`-th extremum;
    /// `ext.parameter(1)?`.
    pub fn parameter(&self, n: i32) -> Result<f64, Failure> {
        Ok(self.my_ext_pc.point(n)?.parameter())
    }

    /// `Point(N)`: the point on the edge of the `N`-th extremum; `ext.point(1)?`.
    pub fn point(&self, n: i32) -> Result<Point3, Failure> {
        Ok(*self.my_ext_pc.point(n)?.value())
    }

    /// `TrimmedSquareDistances(dist1, dist2, pnt1, pnt2)`: the square distances from
    /// the vertex to the points of the first and last parameters, and those points;
    /// `let (d1, d2, p1, p2) = ext.trimmed_square_distances();`.
    pub fn trimmed_square_distances(&self) -> (f64, f64, Point3, Point3) {
        self.my_ext_pc.trimmed_square_distances()
    }
}
