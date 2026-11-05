//! `BRepOffsetAPIMakePipe`: the builder-API face of `BRepFillPipe`, a profile swept
//! along a spine wire.
use martensite_tkbool::brep_fill::BRepFillPipe;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Shape, Wire};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::geom_fill::GeomFillTrihedron;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSweep;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeShapeBase,
};

/// `BRepOffsetAPIMakePipe`: sweeps the profile along the spine at construction; done
/// unless the swept shape is a single (empty) shape.
///
/// ```
/// use martensite_tkmath::gp::Point3;
/// use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakePipe;
/// use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire};
/// let spine_edge = BRepBuilderAPIMakeEdge::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 5.0))?.edge()?;
/// let spine = BRepBuilderAPIMakeWire::from_edge(&spine_edge)?.wire()?;
/// let profile = BRepBuilderAPIMakeEdge::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0))?.edge()?;
/// let pipe = BRepOffsetAPIMakePipe::new(&spine, profile.as_ref())?;
/// assert!(pipe.is_done());
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub struct BRepOffsetAPIMakePipe {
    base: BRepBuilderAPIMakeShapeBase,
    my_pipe: BRepFillPipe,
}

impl BRepBuilderAPIMakeShape for BRepOffsetAPIMakePipe {
    fn make_shape_base(&self) -> &BRepBuilderAPIMakeShapeBase {
        &self.base
    }

    fn make_shape_base_mut(&mut self) -> &mut BRepBuilderAPIMakeShapeBase {
        &mut self.base
    }

    /// The swept shape; not done when it has no sub-shape.
    fn build(&mut self, _range: &ProgressRange<'_>) -> Result<(), Failure> {
        self.base.my_shape = self.my_pipe.shape().clone();
        let mut the_map = IndexedMapOfShape::default();
        topexp::map_shapes(&self.base.my_shape, &mut the_map, true, true);
        if the_map.extent() == 1 {
            self.base.set_not_done();
        } else {
            self.base.set_done();
        }
        Ok(())
    }

    /// The shapes swept from `S`.
    fn generated(&mut self, shape: &Shape) -> Result<&List<Shape>, Failure> {
        self.base.my_generated = self.my_pipe.generated(shape);
        Ok(&self.base.my_generated)
    }
}

impl BRepPrimAPIMakeSweep for BRepOffsetAPIMakePipe {
    /// The profile at the start of the spine.
    fn first_shape(&mut self) -> Result<Shape, Failure> {
        Ok(self.my_pipe.first_shape().clone())
    }

    /// The profile at the end of the spine.
    fn last_shape(&mut self) -> Result<Shape, Failure> {
        Ok(self.my_pipe.last_shape().clone())
    }
}

impl BRepOffsetAPIMakePipe {
    /// The corrected Frenet sweep, built;
    /// `BRepOffsetAPIMakePipe::new(&spine, &profile)?`.
    pub fn new(spine: &Wire, profile: &Shape) -> Result<Self, Failure> {
        Self::with_mode(spine, profile, GeomFillTrihedron::IsCorrectedFrenet, false)
    }

    /// The sweep in the
    /// Frenet, corrected Frenet or discrete mode, trying a C1 surface for a C0 one on
    /// request, built; `BRepOffsetAPIMakePipe::with_mode(&spine, &profile, GeomFillTrihedron::is_frenet, false)?`.
    pub fn with_mode(
        spine: &Wire,
        profile: &Shape,
        mode: GeomFillTrihedron,
        force_approx_c1: bool,
    ) -> Result<Self, Failure> {
        let mut maker = Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            my_pipe: BRepFillPipe::with_profile(spine, profile, mode, force_approx_c1, false)?,
        };
        maker.build(&ProgressRange::default())?;
        Ok(maker)
    }

    /// The underlying sweep; `pipe.pipe()`.
    pub fn pipe(&self) -> &BRepFillPipe {
        &self.my_pipe
    }

    /// The face swept by a profile edge along a spine
    /// edge, or the edge swept by a profile vertex; a null shape for another profile
    /// type; `pipe.generated_by(&spine_edge, &profile_edge)?`.
    pub fn generated_by(&self, s_spine: &Shape, s_profile: &Shape) -> Result<Shape, Failure> {
        match s_profile.shape_type()? {
            ShapeEnum::Edge => {
                let face = self
                    .my_pipe
                    .face(&topods::edge(s_spine)?, &topods::edge(s_profile)?)?;
                Ok(face.into_shape())
            }
            ShapeEnum::Vertex => {
                let edge = self
                    .my_pipe
                    .edge(&topods::edge(s_spine)?, &topods::vertex(s_profile)?)?;
                Ok(edge.into_shape())
            }
            _ => Ok(Shape::default()),
        }
    }

    /// The approximation error of the swept surfaces;
    /// `pipe.error_on_surface()`.
    pub fn error_on_surface(&self) -> f64 {
        self.my_pipe.error_on_surface()
    }
}
