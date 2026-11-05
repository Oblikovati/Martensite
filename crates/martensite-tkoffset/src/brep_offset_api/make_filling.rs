//! `BRepOffsetAPIMakeFilling`: the builder-API face of `BRepFillFilling`, an n-sided
//! plate surface through edge, face and point constraints.
use martensite_tkbool::brep_fill::BRepFillFilling;
use martensite_tkbrep::topods::{Edge, Face, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeShapeBase,
};

/// The resolution, constraint and approximation parameters of the filling, with the
/// reference defaults (degree 3, 15 points per curve, 2 iterations, isotropic, 1e-5,
/// 1e-4, 0.01, 0.1, degree 8 and 9 segments).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FillingParameters {
    /// `degree`.
    pub degree: i32,
    /// `NbPtsOnCur`.
    pub nb_pts_on_cur: i32,
    /// `NbIter`.
    pub nb_iter: i32,
    /// `Anisotropie`.
    pub anisotropie: bool,
    /// `Tol2d`.
    pub tol2d: f64,
    /// `tol3d`.
    pub tol3d: f64,
    /// `TolAng`.
    pub tol_ang: f64,
    /// `TolCurv`.
    pub tol_curv: f64,
    /// `MaxDeg`.
    pub max_deg: i32,
    /// `MaxSegments`.
    pub max_segments: i32,
}

impl Default for FillingParameters {
    fn default() -> Self {
        Self {
            degree: 3,
            nb_pts_on_cur: 15,
            nb_iter: 2,
            anisotropie: false,
            tol2d: 0.00001,
            tol3d: 0.0001,
            tol_ang: 0.01,
            tol_curv: 0.1,
            max_deg: 8,
            max_segments: 9,
        }
    }
}

/// `BRepOffsetAPIMakeFilling`: collects the constraints, then `build` makes the face;
/// `is_done` is the filling's flag (the command flag is never set, so `shape()` raises
/// not_done when it has to build).
///
/// ```
/// use martensite_tkoffset::brep_offset_api::{BRepOffsetAPIMakeFilling, FillingParameters};
/// use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
/// let filling = BRepOffsetAPIMakeFilling::new(FillingParameters::default());
/// assert!(!filling.is_done());
/// ```
pub struct BRepOffsetAPIMakeFilling {
    base: BRepBuilderAPIMakeShapeBase,
    my_filling: BRepFillFilling,
}

impl BRepBuilderAPIMakeShape for BRepOffsetAPIMakeFilling {
    fn make_shape_base(&self) -> &BRepBuilderAPIMakeShapeBase {
        &self.base
    }

    fn make_shape_base_mut(&mut self) -> &mut BRepBuilderAPIMakeShapeBase {
        &mut self.base
    }

    /// Whether the filling is built.
    fn is_done(&self) -> bool {
        self.my_filling.is_done()
    }

    /// The filling and its face.
    fn build(&mut self, _range: &ProgressRange<'_>) -> Result<(), Failure> {
        self.my_filling.build()?;
        self.base.my_shape = self.my_filling.face().into_shape();
        Ok(())
    }

    /// The new edge made from a boundary edge.
    fn generated(&mut self, shape: &Shape) -> Result<&List<Shape>, Failure> {
        Ok(self.my_filling.generated(shape))
    }
}

impl BRepOffsetAPIMakeFilling {
    /// `BRepOffsetAPIMakeFilling(Degree, ..., MaxSegments)`;
    /// `BRepOffsetAPIMakeFilling::new(FillingParameters::default())`.
    pub fn new(parameters: FillingParameters) -> Self {
        let p = parameters;
        Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            my_filling: BRepFillFilling::new(
                p.degree,
                p.nb_pts_on_cur,
                p.nb_iter,
                p.anisotropie,
                p.tol2d,
                p.tol3d,
                p.tol_ang,
                p.tol_curv,
                p.max_deg,
                p.max_segments,
            ),
        }
    }

    /// `set_constr_param(Tol2d, Tol3d, TolAng, TolCurv)`;
    /// `filling.set_constr_param(1e-5, 1e-4, 0.01, 0.1)`.
    pub fn set_constr_param(&mut self, tol2d: f64, tol3d: f64, tol_ang: f64, tol_curv: f64) {
        self.my_filling
            .set_constr_param(tol2d, tol3d, tol_ang, tol_curv);
    }

    /// `set_resol_param(Degree, NbPtsOnCur, NbIter, Anisotropie)`;
    /// `filling.set_resol_param(3, 15, 2, false)`.
    pub fn set_resol_param(
        &mut self,
        degree: i32,
        nb_pts_on_cur: i32,
        nb_iter: i32,
        anisotropie: bool,
    ) {
        self.my_filling
            .set_resol_param(degree, nb_pts_on_cur, nb_iter, anisotropie);
    }

    /// `set_approx_param(MaxDeg, MaxSegments)`; `filling.set_approx_param(8, 9)`.
    pub fn set_approx_param(&mut self, max_deg: i32, max_segments: i32) {
        self.my_filling.set_approx_param(max_deg, max_segments);
    }

    /// The initial surface of the plate;
    /// `filling.load_init_surface(&face)`.
    pub fn load_init_surface(&mut self, surf: &Face) {
        self.my_filling.load_init_surface(surf);
    }

    /// An edge constraint (a boundary by default); its
    /// index; `filling.add_edge(&edge, Continuity::C0, true)`.
    pub fn add_edge(&mut self, constr: &Edge, order: Continuity, is_bound: bool) -> i32 {
        self.my_filling.add_edge(constr, order, is_bound)
    }

    /// An edge constraint on a supporting face;
    /// `filling.add_edge_on_face(&edge, &face, Continuity::G1, true)`.
    pub fn add_edge_on_face(
        &mut self,
        constr: &Edge,
        support: &Face,
        order: Continuity,
        is_bound: bool,
    ) -> i32 {
        self.my_filling
            .add_edge_on_face(constr, support, order, is_bound)
    }

    /// A free face constraint; `filling.add_face(&face, Continuity::G1)`.
    pub fn add_face(&mut self, support: &Face, order: Continuity) -> i32 {
        self.my_filling.add_face(support, order)
    }

    /// A point constraint; `filling.add_point(p)?`.
    pub fn add_point(&mut self, point: Point3) -> Result<i32, Failure> {
        self.my_filling.add_point(point)
    }

    /// A point constraint on a face;
    /// `filling.add_point_on_face(0.5, 0.5, &face, Continuity::G1)?`.
    pub fn add_point_on_face(
        &mut self,
        u: f64,
        v: f64,
        support: &Face,
        order: Continuity,
    ) -> Result<i32, Failure> {
        self.my_filling.add_point_on_face(u, v, support, order)
    }

    /// The greatest distance from the constraints to the face;
    /// `filling.g0_error()?`.
    pub fn g0_error(&self) -> Result<f64, Failure> {
        self.my_filling.g0_error()
    }

    /// The greatest angle to the constraint surfaces; `filling.g1_error()?`.
    pub fn g1_error(&self) -> Result<f64, Failure> {
        self.my_filling.g1_error()
    }

    /// The greatest curvature difference; `filling.g2_error()?`.
    pub fn g2_error(&self) -> Result<f64, Failure> {
        self.my_filling.g2_error()
    }

    /// `G0Error(Index)`; `filling.g0_error_of(1)?`.
    pub fn g0_error_of(&mut self, index: i32) -> Result<f64, Failure> {
        self.my_filling.g0_error_of(index)
    }

    /// `G1Error(Index)`; `filling.g1_error_of(1)?`.
    pub fn g1_error_of(&mut self, index: i32) -> Result<f64, Failure> {
        self.my_filling.g1_error_of(index)
    }

    /// `G2Error(Index)`; `filling.g2_error_of(1)?`.
    pub fn g2_error_of(&mut self, index: i32) -> Result<f64, Failure> {
        self.my_filling.g2_error_of(index)
    }
}
