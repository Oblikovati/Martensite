//! `BRepExtrema_ExtCC`: the extrema between two edges.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topods::Edge;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkgeombase::extrema::ExtCC;
use martensite_tkmath::gp::Point3;

/// One edge given to the native `Extrema_ExtCC` member: its adaptor, range and
/// parameter tolerance.
#[derive(Clone)]
struct EdgeCurve {
    curve: Handle<BRepAdaptorCurve>,
    range: (f64, f64),
    tol: f64,
}

impl EdgeCurve {
    /// The adaptor of a geometric edge, its range and the curve resolution of
    /// min(edge tolerance, Confusion), at least PConfusion.
    fn of(edge: &Edge) -> Result<Self, Failure> {
        let curve = BRepAdaptorCurve::with_edge(edge)?;
        let tol: f64 = BRepTool::edge_tolerance(edge)?.min(precision::confusion());
        let tol: f64 = curve.resolution(tol)?.max(precision::p_confusion());
        Ok(Self {
            curve: Handle::new(curve),
            range: BRepTool::range(edge)?,
            tol,
        })
    }

    /// `SetCurve(rank, C, Uinf, Usup)` then `SetTolerance(rank, Tol)`.
    fn load(&self, ext: &mut ExtCC<'_>, rank: i32) -> Result<(), Failure> {
        let curve: Handle<dyn Adaptor3dCurve> = self.curve.clone().cast();
        ext.set_curve_range_shared(rank, &curve, self.range.0, self.range.1)?;
        ext.set_tolerance(rank, self.tol)
    }
}

/// The `Extrema_ExtCC()` of the native member: tolerances 1e-10.
fn default_extrema<'a>() -> ExtCC<'a> {
    ExtCC::new(1.0e-10, 1.0e-10)
}

/// `BRepExtrema_ExtCC`: `Extrema_ExtCC` between
/// the curves of two edges over their ranges, with their parameter tolerances.
///
/// The persistent member owns both adaptors while keeping its calculated results
/// across Initialize. Historical call replay would recalculate those results after
/// source geometry changes; the general algorithm's accumulated parameters and the
/// endpoint values of infinite ranges must also survive between calls.
///
/// ```
/// use martensite_tktopalgo::brep_extrema::BRepExtremaExtCC;
/// let ext = BRepExtremaExtCC::new();
/// assert!(!ext.is_done());
/// assert!(ext.is_parallel().is_err());
/// ```
#[derive(Clone)]
pub struct BRepExtremaExtCC {
    my_ext_cc: ExtCC<'static>,
}

impl Default for BRepExtremaExtCC {
    fn default() -> Self {
        Self::new()
    }
}

impl BRepExtremaExtCC {
    /// `BRepExtrema_ExtCC()`: no edge; `BRepExtremaExtCC::new()`.
    pub fn new() -> Self {
        Self {
            my_ext_cc: default_extrema(),
        }
    }

    /// `BRepExtrema_ExtCC(E1, E2)`: `Initialize(E2)` then `Perform(E1)`;
    /// `BRepExtremaExtCC::with_edges(&e1, &e2)?`.
    pub fn with_edges(e1: &Edge, e2: &Edge) -> Result<Self, Failure> {
        let mut ext: Self = Self::new();
        ext.initialize(e2)?;
        ext.perform(e1)?;
        Ok(ext)
    }

    /// `Initialize(E2)`: the second edge of the extrema; an edge without a 3D curve or
    /// curve on surface changes nothing; `ext.initialize(&e2)?`.
    pub fn initialize(&mut self, e2: &Edge) -> Result<(), Failure> {
        if !BRepTool::is_geometric_edge(e2)? {
            return Ok(());
        }
        let second: EdgeCurve = EdgeCurve::of(e2)?;
        second.load(&mut self.my_ext_cc, 2)
    }

    /// `Perform(E1)`: the extrema between `e1` and the initialized edge; an edge without
    /// a 3D curve or curve on surface changes nothing; Standard_NullObject when no
    /// second edge was initialized; `ext.perform(&e1)?`.
    pub fn perform(&mut self, e1: &Edge) -> Result<(), Failure> {
        if !BRepTool::is_geometric_edge(e1)? {
            return Ok(());
        }
        let first: EdgeCurve = EdgeCurve::of(e1)?;
        first.load(&mut self.my_ext_cc, 1)?;
        self.my_ext_cc.perform()
    }

    /// `IsDone()`: true when the distances are found; `ext.is_done()`.
    pub fn is_done(&self) -> bool {
        self.my_ext_cc.is_done()
    }

    /// `NbExt()`: the number of extremum distances (StdFail_NotDone when not done);
    /// `ext.nb_ext()?`.
    pub fn nb_ext(&self) -> Result<i32, Failure> {
        self.my_ext_cc.nb_ext()
    }

    /// `IsParallel()`: true when the edges are parallel (StdFail_NotDone when not
    /// done); `ext.is_parallel()?`.
    pub fn is_parallel(&self) -> Result<bool, Failure> {
        self.my_ext_cc.is_parallel()
    }

    /// `SquareDistance(N)`: the `N`-th extremum square distance;
    /// `ext.square_distance(1)?`.
    pub fn square_distance(&self, n: i32) -> Result<f64, Failure> {
        self.my_ext_cc.square_distance(n)
    }

    /// `ParameterOnE1(N)`: the parameter on the first edge of the `N`-th extremum;
    /// `ext.parameter_on_e1(1)?`.
    pub fn parameter_on_e1(&self, n: i32) -> Result<f64, Failure> {
        Ok(self.my_ext_cc.points(n)?.0.parameter())
    }

    /// `PointOnE1(N)`: the point on the first edge of the `N`-th extremum;
    /// `ext.point_on_e1(1)?`.
    pub fn point_on_e1(&self, n: i32) -> Result<Point3, Failure> {
        Ok(*self.my_ext_cc.points(n)?.0.value())
    }

    /// `ParameterOnE2(N)`: the parameter on the second edge of the `N`-th extremum;
    /// `ext.parameter_on_e2(1)?`.
    pub fn parameter_on_e2(&self, n: i32) -> Result<f64, Failure> {
        Ok(self.my_ext_cc.points(n)?.1.parameter())
    }

    /// `PointOnE2(N)`: the point on the second edge of the `N`-th extremum;
    /// `ext.point_on_e2(1)?`.
    pub fn point_on_e2(&self, n: i32) -> Result<Point3, Failure> {
        Ok(*self.my_ext_cc.points(n)?.1.value())
    }

    /// `TrimmedSquareDistances(dist11, distP12, distP21, distP22, P11, P12, P21, P22)`:
    /// the square distances between the end points of the two ranges and those points,
    /// as `([d11, d12, d21, d22], [P11, P12, P21, P22])`;
    /// `let (distances, points) = ext.trimmed_square_distances();`.
    pub fn trimmed_square_distances(&self) -> ([f64; 4], [Point3; 4]) {
        self.my_ext_cc.trimmed_square_distances()
    }
}
