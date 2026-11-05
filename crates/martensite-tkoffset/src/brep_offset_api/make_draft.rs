//! `BRepOffsetAPIMakeDraft`: the builder-API face of `BRepFillDraft`, a draft surface
//! swept from a wire along a direction with an angle.
use martensite_tkbool::brep_fill::{BRepFillDraft, TransitionStyle};
use martensite_tkbrep::topods::{Shape, Shell};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkmath::gp::Direction3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeShapeBase, BRepBuilderAPITransitionMode,
};

/// `BRepOffsetAPIMakeDraft`: the draft of a shape, built by one of the `perform`
/// limits (a length, a surface or a shape); done when the draft is.
///
/// ```
/// use martensite_tkmath::gp::{Direction3, Point3};
/// use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakeDraft;
/// use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape};
/// let square = BRepBuilderAPIMakePolygon::from_four_points(
///     Point3::new(-1.0, -1.0, 0.0), Point3::new(1.0, -1.0, 0.0),
///     Point3::new(1.0, 1.0, 0.0), Point3::new(-1.0, 1.0, 0.0), true)?.wire()?;
/// let mut draft = BRepOffsetAPIMakeDraft::new(square.as_ref(), Direction3::new(0.0, 0.0, 1.0)?, 0.1)?;
/// draft.perform_length(3.0)?;
/// assert!(draft.is_done());
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub struct BRepOffsetAPIMakeDraft {
    base: BRepBuilderAPIMakeShapeBase,
    my_draft: BRepFillDraft,
}

impl BRepBuilderAPIMakeShape for BRepOffsetAPIMakeDraft {
    fn make_shape_base(&self) -> &BRepBuilderAPIMakeShapeBase {
        &self.base
    }

    fn make_shape_base_mut(&mut self) -> &mut BRepBuilderAPIMakeShapeBase {
        &mut self.base
    }

    /// The faces of the draft generated from `S`.
    fn generated(&mut self, shape: &Shape) -> Result<&List<Shape>, Failure> {
        self.my_draft.generated(shape)
    }
}

impl BRepOffsetAPIMakeDraft {
    /// The draft of the shape (a wire, or
    /// the free boundary of a face or a shell), not done;
    /// `BRepOffsetAPIMakeDraft::new(&shape, dir, 0.1)?`.
    pub fn new(shape: &Shape, dir: Direction3, angle: f64) -> Result<Self, Failure> {
        let mut maker = Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            my_draft: BRepFillDraft::new(shape, dir, angle)?,
        };
        maker.base.set_not_done();
        Ok(maker)
    }

    /// `set_options(Style, AngleMin, AngleMax)` (usually the right corner,
    /// 0.01 and 3): a round corner, a right corner for any other style;
    /// `draft.set_options(BRepBuilderAPITransitionMode::RightCorner, 0.01, 3.0)`.
    pub fn set_options(
        &mut self,
        style: BRepBuilderAPITransitionMode,
        angle_min: f64,
        angle_max: f64,
    ) {
        let fill_style = if style == BRepBuilderAPITransitionMode::RoundCorner {
            TransitionStyle::Round
        } else {
            TransitionStyle::Right
        };
        self.my_draft.set_options(fill_style, angle_min, angle_max);
    }

    /// `set_draft(IsInternal)` (usually false); `draft.set_draft(false)`.
    pub fn set_draft(&mut self, is_internal: bool) {
        self.my_draft.set_draft(is_internal);
    }

    /// The draft up to the length; `draft.perform_length(2.0)?`.
    pub fn perform_length(&mut self, length_max: f64) -> Result<(), Failure> {
        self.my_draft.perform_length(length_max)?;
        self.take_done_shape();
        Ok(())
    }

    /// `perform(Surface, KeepInsideSurface)` (in the reference keeping the inside): the draft up
    /// to the surface; `draft.perform_surface(&surface, true)?`.
    pub fn perform_surface(
        &mut self,
        surface: &Handle<dyn GeomSurface>,
        keep_inside_surface: bool,
    ) -> Result<(), Failure> {
        self.my_draft
            .perform_surface(surface, keep_inside_surface)?;
        self.take_done_shape();
        Ok(())
    }

    /// `perform(StopShape, KeepOutSide)` (in the reference keeping the outside): the draft up
    /// to the shape; `draft.perform_shape(&stop, true)?`.
    pub fn perform_shape(
        &mut self,
        stop_shape: &Shape,
        keep_out_side: bool,
    ) -> Result<(), Failure> {
        self.my_draft.perform_shape(stop_shape, keep_out_side)?;
        self.take_done_shape();
        Ok(())
    }

    /// Done with the draft shape when the draft is done (a failed draft keeps the
    /// previous state).
    fn take_done_shape(&mut self) {
        if self.my_draft.is_done() {
            self.base.set_done();
            self.base.my_shape = self.my_draft.shape();
        }
    }

    /// The shell of the draft surfaces; `draft.shell()`.
    pub fn shell(&self) -> Shell {
        self.my_draft.shell()
    }
}
