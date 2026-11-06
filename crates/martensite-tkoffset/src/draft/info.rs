//! The modification records of the draft: per face, edge and vertex.
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Edge, Face, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkmath::gp::Point3;

/// `DraftErrorStatus`: the step of the draft that failed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(i32)]
pub enum DraftErrorStatus {
    /// `DraftErrorStatus::NoError`.
    #[default]
    NoError,
    /// `DraftErrorStatus::FaceRecomputation`.
    FaceRecomputation,
    /// `DraftErrorStatus::EdgeRecomputation`.
    EdgeRecomputation,
    /// `DraftErrorStatus::VertexRecomputation`.
    VertexRecomputation,
}

/// `DraftEdgeInfo`: the new curve of an edge, its faces and p-curves.
///
/// ```
/// use martensite_tkoffset::draft::DraftEdgeInfo;
/// let info = DraftEdgeInfo::with_new_geometry(true);
/// assert!(info.new_geometry());
/// assert_eq!(info.tolerance(), 0.0);
/// ```
#[derive(Clone, Default)]
pub struct DraftEdgeInfo {
    my_new_geom: bool,
    my_geom: Handle<dyn GeomCurve>,
    my_first_f: Face,
    my_secon_f: Face,
    my_first_pc: Handle<dyn Geom2dCurve>,
    my_secon_pc: Handle<dyn Geom2dCurve>,
    my_root_face: Face,
    my_tgt: bool,
    my_pt: Point3,
    my_tol: f64,
}

impl DraftEdgeInfo {
    /// No new geometry; `DraftEdgeInfo::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// `DraftEdgeInfo(HasNewGeometry)`; `DraftEdgeInfo::with_new_geometry(true)`.
    pub fn with_new_geometry(has_new_geometry: bool) -> Self {
        Self {
            my_new_geom: has_new_geometry,
            ..Self::default()
        }
    }

    /// The first, then a second different face; the tolerance grows to the
    /// face tolerance; `info.add(&face)?`.
    pub fn add(&mut self, f: &Face) -> Result<(), Failure> {
        if self.my_first_f.is_null() {
            self.my_first_f = f.clone();
        } else if !self.my_first_f.is_same(f) && self.my_secon_f.is_null() {
            self.my_secon_f = f.clone();
        }
        self.my_tol = self.my_tol.max(BRepTool::face_tolerance(f)?);
        Ok(())
    }

    /// The face whose draft modified this edge; `info.set_root_face(&f)`.
    pub fn set_root_face(&mut self, f: &Face) {
        self.my_root_face = f.clone();
    }

    /// The edge is a tangency edge through the fixed point `P`;
    /// `info.tangent(p)`.
    pub fn tangent(&mut self, p: Point3) {
        self.my_tgt = true;
        self.my_pt = p;
    }

    /// Whether the edge is a tangency edge, with its fixed point (the
    /// default point otherwise); `info.is_tangent()`.
    pub fn is_tangent(&self) -> (bool, Point3) {
        (self.my_tgt, self.my_pt)
    }

    /// An empty one; `info.new_geometry()`.
    pub fn new_geometry(&self) -> bool {
        self.my_new_geom
    }

    /// `set_new_geometry(NewGeom)`; `info.set_new_geometry(true)`.
    pub fn set_new_geometry(&mut self, new_geom: bool) {
        self.my_new_geom = new_geom;
    }

    /// The new curve (null before the draft computes it); `info.geometry()`.
    pub fn geometry(&self) -> &Handle<dyn GeomCurve> {
        &self.my_geom
    }

    /// An empty one; `info.first_face()`.
    pub fn first_face(&self) -> &Face {
        &self.my_first_f
    }

    /// An empty one; `info.second_face()`.
    pub fn second_face(&self) -> &Face {
        &self.my_secon_f
    }

    /// The p-curve on the first face; `info.first_pc()`.
    pub fn first_pc(&self) -> &Handle<dyn Geom2dCurve> {
        &self.my_first_pc
    }

    /// The p-curve on the second face; `info.second_pc()`.
    pub fn second_pc(&self) -> &Handle<dyn Geom2dCurve> {
        &self.my_secon_pc
    }

    /// An empty one; `*info.change_geometry() = curve`.
    pub fn change_geometry(&mut self) -> &mut Handle<dyn GeomCurve> {
        &mut self.my_geom
    }

    /// An empty one; `*info.change_first_pc() = pcurve`.
    pub fn change_first_pc(&mut self) -> &mut Handle<dyn Geom2dCurve> {
        &mut self.my_first_pc
    }

    /// An empty one; `*info.change_second_pc() = pcurve`.
    pub fn change_second_pc(&mut self) -> &mut Handle<dyn Geom2dCurve> {
        &mut self.my_secon_pc
    }

    /// An empty one; `info.root_face()`.
    pub fn root_face(&self) -> &Face {
        &self.my_root_face
    }

    /// `tolerance(tol)`; `info.set_tolerance(1e-7)`.
    pub fn set_tolerance(&mut self, tol: f64) {
        self.my_tol = tol;
    }

    /// An empty one; `info.tolerance()`.
    pub fn tolerance(&self) -> f64 {
        self.my_tol
    }
}

/// `DraftFaceInfo`: the new surface of a face, and the two faces a postponed face
/// (a cylinder or an extrusion drafted later) is computed between.
///
/// ```
/// use martensite_tkernel::standard::Handle;
/// use martensite_tkoffset::draft::DraftFaceInfo;
/// let info = DraftFaceInfo::with_surface(&Handle::default(), true);
/// assert!(info.new_geometry() && info.geometry().is_null());
/// ```
#[derive(Clone, Default)]
pub struct DraftFaceInfo {
    my_new_geom: bool,
    my_geom: Handle<dyn GeomSurface>,
    my_root_face: Face,
    my_f1: Face,
    my_f2: Face,
    my_curv: Handle<dyn GeomCurve>,
}

impl DraftFaceInfo {
    /// An empty one; `DraftFaceInfo::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// The basis of a trimmed surface;
    /// `DraftFaceInfo::with_surface(&s, true)`.
    pub fn with_surface(s: &Handle<dyn GeomSurface>, has_new_geometry: bool) -> Self {
        let t = s.clone().cast::<GeomRectangularTrimmedSurface>();
        let my_geom = match t.get() {
            Some(t) => t.basis_surface(),
            None => s.clone(),
        };
        Self {
            my_new_geom: has_new_geometry,
            my_geom,
            ..Self::default()
        }
    }

    /// `root_face(F)`; `info.set_root_face(&f)`.
    pub fn set_root_face(&mut self, f: &Face) {
        self.my_root_face = f.clone();
    }

    /// An empty one; `info.new_geometry()`.
    pub fn new_geometry(&self) -> bool {
        self.my_new_geom
    }

    /// The first, then the second face; `info.add(&f)`.
    pub fn add(&mut self, f: &Face) {
        if self.my_f1.is_null() {
            self.my_f1 = f.clone();
        } else if self.my_f2.is_null() {
            self.my_f2 = f.clone();
        }
    }

    /// An empty one; `info.first_face()`.
    pub fn first_face(&self) -> &Face {
        &self.my_f1
    }

    /// An empty one; `info.second_face()`.
    pub fn second_face(&self) -> &Face {
        &self.my_f2
    }

    /// An empty one; `info.geometry()`.
    pub fn geometry(&self) -> &Handle<dyn GeomSurface> {
        &self.my_geom
    }

    /// An empty one; `*info.change_geometry() = surface`.
    pub fn change_geometry(&mut self) -> &mut Handle<dyn GeomSurface> {
        &mut self.my_geom
    }

    /// An empty one; `info.root_face()`.
    pub fn root_face(&self) -> &Face {
        &self.my_root_face
    }

    /// The circle of a postponed face; `*info.change_curve() = circle`.
    pub fn change_curve(&mut self) -> &mut Handle<dyn GeomCurve> {
        &mut self.my_curv
    }

    /// An empty one; `info.curve()`.
    pub fn curve(&self) -> &Handle<dyn GeomCurve> {
        &self.my_curv
    }
}

/// `DraftVertexInfo`: the new point of a vertex and its parameters on its edges, with
/// the edge iterator of the reference class (the parameter queries move it).
///
/// ```
/// use martensite_tkbrep::topods::Edge;
/// use martensite_tkoffset::draft::DraftVertexInfo;
/// let mut info = DraftVertexInfo::new();
/// info.init_edge_iterator();
/// assert!(!info.more_edge());
/// assert!(info.parameter(&Edge::default()).is_err());
/// ```
#[derive(Clone, Default)]
pub struct DraftVertexInfo {
    my_geom: Point3,
    my_edges: Vec<Shape>,
    my_params: Vec<f64>,
    my_it_ed: usize,
}

impl DraftVertexInfo {
    /// An empty one; `DraftVertexInfo::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// The edge with the parameter RealLast() unless it is there (the
    /// iterator stops on it, or ends); `info.add(&edge)`.
    pub fn add(&mut self, e: &Edge) {
        self.my_it_ed = 0;
        while self.my_it_ed < self.my_edges.len() {
            if e.is_same(&self.my_edges[self.my_it_ed]) {
                break;
            }
            self.my_it_ed += 1;
        }
        if self.my_it_ed >= self.my_edges.len() {
            self.my_edges.push(e.clone().into_shape());
            self.my_params.push(f64::MAX);
        }
    }

    /// An empty one; `info.geometry()`.
    pub fn geometry(&self) -> Point3 {
        self.my_geom
    }

    /// An empty one; `*info.change_geometry() = p`.
    pub fn change_geometry(&mut self) -> &mut Point3 {
        &mut self.my_geom
    }

    /// The index of the edge, the iterator left on it (or at the end).
    fn seek(&mut self, e: &Edge) -> Option<usize> {
        self.my_it_ed = 0;
        while self.my_it_ed < self.my_edges.len() {
            if self.my_edges[self.my_it_ed].is_same(e) {
                return Some(self.my_it_ed);
            }
            self.my_it_ed += 1;
        }
        None
    }

    /// The parameter on the edge; domain_error for another edge
    /// (the iterator is left on the edge, or at the end); `info.parameter(&edge)?`.
    pub fn parameter(&mut self, e: &Edge) -> Result<f64, Failure> {
        match self.seek(e) {
            Some(i) => Ok(self.my_params[i]),
            None => Err(Failure::with_message(
                "Standard_DomainError",
                "Draft_VertexInfo::Parameter",
            )),
        }
    }

    /// As [`Self::parameter`], to write;
    /// `*info.change_parameter(&edge)? = 0.5`.
    pub fn change_parameter(&mut self, e: &Edge) -> Result<&mut f64, Failure> {
        match self.seek(e) {
            Some(i) => Ok(&mut self.my_params[i]),
            None => Err(Failure::with_message(
                "Standard_DomainError",
                "Draft_VertexInfo::ChangeParameter",
            )),
        }
    }

    /// An empty one; `info.init_edge_iterator()`.
    pub fn init_edge_iterator(&mut self) {
        self.my_it_ed = 0;
    }

    /// The current edge; no_such_object past the end; `info.edge()?`.
    pub fn edge(&self) -> Result<Edge, Failure> {
        let current = self.my_edges.get(self.my_it_ed).ok_or_else(|| {
            Failure::with_message("Standard_NoSuchObject", "Draft_VertexInfo::Edge")
        })?;
        topods::edge(current)
    }

    /// An empty one; `info.next_edge()`.
    pub fn next_edge(&mut self) {
        if self.my_it_ed < self.my_edges.len() {
            self.my_it_ed += 1;
        }
    }

    /// An empty one; `info.more_edge()`.
    pub fn more_edge(&self) -> bool {
        self.my_it_ed < self.my_edges.len()
    }
}
