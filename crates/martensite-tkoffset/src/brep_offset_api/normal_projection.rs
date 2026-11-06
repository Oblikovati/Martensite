//! `BRepOffsetAPINormalProjection`: the builder-API face of
//! `BRepAlgoNormalProjection`, edges and wires projected along the normals of a shape.
use martensite_tkbool::brep_algo::BRepAlgoNormalProjection;
use martensite_tkbrep::topods::{Edge, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeShapeBase,
};

/// `BRepOffsetAPINormalProjection`: projects the added shapes on the base shape at
/// `build`; `is_done` is the projector's flag.
///
/// ```
/// use martensite_tkoffset::brep_offset_api::BRepOffsetAPINormalProjection;
/// use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
/// let projection = BRepOffsetAPINormalProjection::new();
/// assert!(!projection.is_done());
/// ```
pub struct BRepOffsetAPINormalProjection {
    base: BRepBuilderAPIMakeShapeBase,
    my_normal_projector: BRepAlgoNormalProjection,
}

impl Default for BRepOffsetAPINormalProjection {
    fn default() -> Self {
        Self::new()
    }
}

impl BRepBuilderAPIMakeShape for BRepOffsetAPINormalProjection {
    fn make_shape_base(&self) -> &BRepBuilderAPIMakeShapeBase {
        &self.base
    }

    fn make_shape_base_mut(&mut self) -> &mut BRepBuilderAPIMakeShapeBase {
        &mut self.base
    }

    /// Whether the projection is built.
    fn is_done(&self) -> bool {
        self.my_normal_projector.is_done()
    }

    /// The projection, then done.
    fn build(&mut self, _range: &ProgressRange<'_>) -> Result<(), Failure> {
        self.my_normal_projector.build()?;
        self.base.my_shape = self.my_normal_projector.projection().clone();
        self.base.set_done();
        Ok(())
    }

    /// The projections of an added shape.
    fn generated(&mut self, shape: &Shape) -> Result<&List<Shape>, Failure> {
        self.my_normal_projector.generated(shape)
    }
}

impl BRepOffsetAPINormalProjection {
    /// No base shape and the default parameters;
    /// `BRepOffsetAPINormalProjection::new()`.
    pub fn new() -> Self {
        Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            my_normal_projector: BRepAlgoNormalProjection::new(),
        }
    }

    /// The base shape;
    /// `BRepOffsetAPINormalProjection::with_shape(&shape)`.
    pub fn with_shape(s: &Shape) -> Self {
        let mut projection = Self::new();
        projection.init(s);
        projection
    }

    /// The base shape; `projection.init(&shape)`.
    pub fn init(&mut self, s: &Shape) {
        self.my_normal_projector.init(s);
    }

    /// An edge or wire to project; `projection.add(&wire)?`.
    pub fn add(&mut self, to_proj: &Shape) -> Result<(), Failure> {
        self.my_normal_projector.add(to_proj)
    }

    /// `set_params(Tol3D, Tol2D, InternalContinuity, MaxDegree, MaxSeg)`;
    /// `projection.set_params(1e-4, 1e-3, Continuity::C1, 14, 16)`.
    pub fn set_params(
        &mut self,
        tol3d: f64,
        tol2d: f64,
        internal_continuity: Continuity,
        max_degree: i32,
        max_seg: i32,
    ) {
        self.my_normal_projector
            .set_params(tol3d, tol2d, internal_continuity, max_degree, max_seg);
    }

    /// No projection farther than the distance (a negative
    /// one for none); `projection.set_max_distance(1.0)`.
    pub fn set_max_distance(&mut self, max_dist: f64) {
        self.my_normal_projector.set_max_distance(max_dist);
    }

    /// `set_limit(FaceBoundaries)` (usually true): the projections cut by
    /// the face boundaries; `projection.set_limit(true)`.
    pub fn set_limit(&mut self, face_boundaries: bool) {
        self.my_normal_projector.set_limit(face_boundaries);
    }

    /// `compute3d(With3d)` (usually true); `projection.compute3d(true)`.
    pub fn compute3d(&mut self, with3d: bool) {
        self.my_normal_projector.compute3d(with3d);
    }

    /// The compound of the projected edges; `projection.projection()`.
    pub fn projection(&self) -> &Shape {
        self.my_normal_projector.projection()
    }

    /// The face a projected edge lies on; `projection.couple(&edge)?`.
    pub fn couple(&self, e: &Edge) -> Result<&Shape, Failure> {
        self.my_normal_projector.couple(e)
    }

    /// The added edge a projected edge comes from;
    /// `projection.ancestor(&edge)?`.
    pub fn ancestor(&self, e: &Edge) -> Result<&Shape, Failure> {
        self.my_normal_projector.ancestor(e)
    }

    /// The projected edges assembled into wires; false when they
    /// do not make wires; `projection.build_wire(&mut wires)?`.
    pub fn build_wire(&self, list_of_wire: &mut List<Shape>) -> Result<bool, Failure> {
        self.my_normal_projector.build_wire(list_of_wire)
    }
}
