mod walls;

use super::{BRepOffsetSimpleOffset, build_simple_offset_wall, compute_simple_offset_max_angle};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools::{BRepToolsModification, Modifier, ReShape};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Edge, Shape, Solid, Vertex};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_build::ShapeBuildReShape;

/// `BRepOffsetSimpleStatus`: the state of a simple offset attempt.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(i32)]
pub enum BRepOffsetSimpleStatus {
    /// `OK`.
    #[default]
    Ok = 0,
    /// `BRepOffsetSimpleStatus::NullInputShape`.
    NullInputShape = 1,
    /// `BRepOffsetSimpleStatus::ErrorOffsetComputation`.
    ErrorOffsetComputation = 2,
    /// `BRepOffsetSimpleStatus::ErrorWallFaceComputation`.
    ErrorWallFaceComputation = 3,
    /// `BRepOffsetSimpleStatus::ErrorInvalidNbShells`.
    ErrorInvalidNbShells = 4,
    /// `BRepOffsetSimpleStatus::ErrorNonClosedShell`.
    ErrorNonClosedShell = 5,
}

/// Stateful shell-producing `BRepOffsetMakeSimpleOffset` orchestration.
///
/// Shell results preserve the reference modifier history. Solid mode rebuilds walls for the
/// currently supported boundary cases.
pub struct BRepOffsetMakeSimpleOffset {
    input_shape: Shape,
    offset_value: f64,
    tolerance: f64,
    build_solid: bool,
    max_angle: f64,
    error: BRepOffsetSimpleStatus,
    done: bool,
    modifier: Modifier,
    reshape: Handle<ShapeBuildReShape>,
    result_shape: Shape,
    wall_edges: DataMap<Vertex, Edge>,
}

impl Default for BRepOffsetMakeSimpleOffset {
    fn default() -> Self {
        Self {
            input_shape: Shape::default(),
            offset_value: 0.0,
            tolerance: martensite_tkernel::precision::confusion(),
            build_solid: false,
            max_angle: 0.0,
            error: BRepOffsetSimpleStatus::Ok,
            done: false,
            modifier: Modifier::default(),
            reshape: Handle::new(ShapeBuildReShape::new()),
            result_shape: Shape::default(),
            wall_edges: DataMap::default(),
        }
    }
}

impl BRepOffsetMakeSimpleOffset {
    /// No input shape or result.
    ///
    /// ```
    /// use martensite_tkoffset::brep_offset::BRepOffsetMakeSimpleOffset;
    /// let maker = BRepOffsetMakeSimpleOffset::new();
    /// assert!(!maker.is_done());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize with an input and offset.
    ///
    /// ```
    /// use martensite_tkoffset::brep_offset::BRepOffsetMakeSimpleOffset;
    /// use martensite_tkbrep::topods::Shape;
    /// let maker = BRepOffsetMakeSimpleOffset::with_shape(&Shape::default(), 1.0);
    /// assert_eq!(maker.offset_value(), 1.0);
    /// ```
    pub fn with_shape(shape: &Shape, offset: f64) -> Self {
        let mut maker = Self::new();
        maker.input_shape = shape.clone();
        maker.offset_value = offset;
        maker
    }

    /// Update inputs and clear run state.
    pub fn initialize(&mut self, shape: &Shape, offset: f64) {
        self.input_shape = shape.clone();
        self.offset_value = offset;
        self.clear();
    }

    /// Rebuild the offset shell and retain the modifier for history queries.
    pub fn perform(&mut self) -> Result<(), Failure> {
        self.clear();
        if self.input_shape.is_null() {
            self.error = BRepOffsetSimpleStatus::NullInputShape;
            return Ok(());
        }
        self.prepare_shell_modifier()?;
        if self.error != BRepOffsetSimpleStatus::Ok {
            return Ok(());
        }
        self.restore_degenerated_edges()?;
        if self.build_solid {
            self.build_solid_result()?;
            if self.error != BRepOffsetSimpleStatus::Ok {
                return Ok(());
            }
        }
        self.done = true;
        Ok(())
    }

    fn prepare_shell_modifier(&mut self) -> Result<(), Failure> {
        self.ensure_max_angle()?;
        self.modifier.init(&self.input_shape)?;
        self.run_shell_mapping()?;
        self.capture_result_shape()
    }

    fn ensure_max_angle(&mut self) -> Result<(), Failure> {
        if self.max_angle == 0.0 {
            self.compute_max_angle()?;
        }
        Ok(())
    }

    fn run_shell_mapping(&mut self) -> Result<(), Failure> {
        let mapping =
            BRepOffsetSimpleOffset::new(&self.input_shape, self.offset_value, self.tolerance)?;
        let mapping: Handle<dyn BRepToolsModification> = Handle::new(mapping).cast();
        self.modifier.perform(&mapping, &ProgressRange::default())?;
        if !self.modifier.is_done() {
            self.error = BRepOffsetSimpleStatus::ErrorOffsetComputation;
        }
        Ok(())
    }

    fn capture_result_shape(&mut self) -> Result<(), Failure> {
        if self.error != BRepOffsetSimpleStatus::Ok {
            return Ok(());
        }
        self.result_shape = self.modifier.modified_shape(&self.input_shape)?;
        Ok(())
    }

    fn restore_degenerated_edges(&self) -> Result<(), Failure> {
        let mut explorer = Explorer::new(&self.input_shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
        let builder = BRepBuilder::default();
        while explorer.more() {
            mark_degenerated_image(
                &self.modifier,
                &builder,
                Edge::unchecked(explorer.current().clone()),
            )?;
            explorer.next()?;
        }
        Ok(())
    }

    /// `clear()` is protected in the reference; Rust exposes it to reset run state and history,
    /// preserving inputs, output and the build-solid flag.
    ///
    /// ```
    /// use martensite_tkoffset::brep_offset::BRepOffsetMakeSimpleOffset;
    /// let mut maker = BRepOffsetMakeSimpleOffset::new();
    /// maker.clear();
    /// assert!(!maker.is_done());
    /// ```
    pub fn clear(&mut self) {
        self.done = false;
        self.error = BRepOffsetSimpleStatus::Ok;
        self.max_angle = 0.0;
        self.reshape
            .get()
            .expect("reshape handle is initialized")
            .clear();
        self.wall_edges.clear(true);
    }

    /// `compute_max_angle()` is protected in the reference; Rust exposes it to prime the cached
    /// largest angle between adjacent face normals before `safe_offset`.
    ///
    /// ```
    /// use martensite_tkoffset::brep_offset::BRepOffsetMakeSimpleOffset;
    /// let mut maker = BRepOffsetMakeSimpleOffset::new();
    /// maker.compute_max_angle()?;
    /// # Ok::<(), martensite_tkernel::standard::Failure>(())
    /// ```
    pub fn compute_max_angle(&mut self) -> Result<(), Failure> {
        let measured = compute_simple_offset_max_angle(&self.input_shape)?;
        if measured > self.max_angle {
            self.max_angle = measured;
        }
        Ok(())
    }

    fn build_solid_result(&mut self) -> Result<(), Failure> {
        let shell = match walls::make_solid(
            &self.input_shape,
            &self.result_shape,
            &self.modifier,
            &self.reshape,
            &mut self.wall_edges,
        )? {
            Ok(shell) => shell,
            Err(status) => {
                self.error = status;
                return Ok(());
            }
        };
        self.result_shape = make_solid(&shell)?;
        Ok(())
    }

    /// The reference text for the current status.
    pub fn error_message(&self) -> &'static str {
        match self.error {
            BRepOffsetSimpleStatus::Ok => "",
            BRepOffsetSimpleStatus::NullInputShape => "Null input shape",
            BRepOffsetSimpleStatus::ErrorOffsetComputation => "Error during offset construction",
            BRepOffsetSimpleStatus::ErrorWallFaceComputation => "Error during building wall face",
            BRepOffsetSimpleStatus::ErrorInvalidNbShells => "Result contains two or more shells",
            BRepOffsetSimpleStatus::ErrorNonClosedShell => "Result shell is not closed",
        }
    }

    /// Estimate a distance from max vertex tolerance.
    pub fn safe_offset(&mut self, expected_tolerance: f64) -> Result<f64, Failure> {
        if self.input_shape.is_null() {
            return Ok(0.0);
        }
        if self.max_angle == 0.0 {
            self.compute_max_angle()?;
        }
        let max_tolerance = BRepTool::max_tolerance(&self.input_shape, ShapeEnum::Vertex)?;
        let estimate = (expected_tolerance - max_tolerance) / (2.0 * self.max_angle);
        Ok(if estimate < 0.0 { 0.0 } else { estimate })
    }

    /// Return the offset image after reshape history.
    pub fn generated(&self, shape: &Shape) -> Result<Shape, Failure> {
        let generated = self.modifier.modified_shape(shape)?;
        if generated.is_null() {
            return Ok(generated);
        }
        self.reshape
            .get()
            .expect("reshape handle is initialized")
            .apply(&generated, ShapeEnum::Shape)
    }

    /// Return a reshape replacement or a null shape.
    pub fn modified(&self, shape: &Shape) -> Result<Shape, Failure> {
        let (status, result) = self
            .reshape
            .get()
            .expect("reshape handle is initialized")
            .status(shape, false)?;
        Ok(if status == 0 {
            Shape::default()
        } else {
            result
        })
    }

    /// An empty one; `maker.error()`.
    pub fn error(&self) -> BRepOffsetSimpleStatus {
        self.error
    }

    /// An empty one; `maker.is_done()`.
    pub fn is_done(&self) -> bool {
        self.done
    }

    /// An empty one; `maker.result_shape()`.
    pub fn result_shape(&self) -> &Shape {
        &self.result_shape
    }

    /// An empty one; `maker.build_solid_flag()`.
    pub fn build_solid_flag(&self) -> bool {
        self.build_solid
    }

    /// `set_build_solid_flag(flag)`; `maker.set_build_solid_flag(true)`.
    pub fn set_build_solid_flag(&mut self, build_solid: bool) {
        self.build_solid = build_solid;
    }

    /// An empty one; `maker.offset_value()`.
    pub fn offset_value(&self) -> f64 {
        self.offset_value
    }

    /// `set_offset_value(offset)`; `maker.set_offset_value(1.0)`.
    pub fn set_offset_value(&mut self, offset: f64) {
        self.offset_value = offset;
    }

    /// An empty one; `maker.tolerance()`.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// `set_tolerance(tolerance)`; `maker.set_tolerance(1e-7)`.
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }
}

fn make_solid(shell: &martensite_tkbrep::topods::Shell) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let mut solid = Solid::default();
    builder.make_solid(&mut solid);
    builder.add(&mut solid, shell)?;
    Ok(solid.into_shape())
}

fn mark_degenerated_image(
    modifier: &Modifier,
    builder: &BRepBuilder,
    edge: Edge,
) -> Result<(), Failure> {
    if !BRepTool::degenerated(&edge)? {
        return Ok(());
    }
    let image = Edge::unchecked(modifier.modified_shape(&edge)?);
    builder.degenerated(&image, true)
}
