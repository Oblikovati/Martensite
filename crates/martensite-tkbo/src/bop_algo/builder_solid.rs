//! `BOPAlgoBuilderSolid`: builds solids from a set of faces.
use super::builder_area::progress::{legacy_unbound_area_result, observe_user_break};
use super::tools::ShapeBoxMap;
use super::{AlertShellSplitterFailed, BuilderArea, ProgressCancellation, ShellSplitter};
use crate::int_tools::Context;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{Compound, Edge, Face, Shape, ShapeIterator, Shell};
use martensite_tkbrep::toptools::IndexedDataMapOfShapeListOfShape;
use martensite_tkernel::message::AlertHandle;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::ncollection::{DataMap, Map};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[path = "builder_solid/progress.rs"]
mod progress;
pub(crate) use progress::perform_solid_with_progress;
#[path = "builder_solid/areas.rs"]
mod areas;
#[path = "builder_solid/internal.rs"]
mod internal;

/// Builds solids from faces: closed shells by the shell splitter, holes put into the
/// closest growth solid containing them, then the internal faces put into the solids.
///
/// ```
/// use martensite_tkbo::bop_algo::BuilderSolid;
/// let mut builder = BuilderSolid::new();
/// builder.perform().unwrap();
/// assert!(builder.areas().is_empty());
/// ```
pub struct BuilderSolid {
    area: BuilderArea,
    boxes: ShapeBoxMap,
}

impl Default for BuilderSolid {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for BuilderSolid {
    type Target = BuilderArea;
    fn deref(&self) -> &BuilderArea {
        &self.area
    }
}

impl DerefMut for BuilderSolid {
    fn deref_mut(&mut self) -> &mut BuilderArea {
        &mut self.area
    }
}

impl BuilderSolid {
    /// An empty builder (`BOPAlgoBuilderSolid()`); `BuilderSolid::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains the supplied nullable allocation-policy identity in the base options.
    /// `BuilderSolid::with_allocator(None)`.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        Self {
            area: BuilderArea::with_allocator(allocator),
            boxes: DataMap::with_hasher(martensite_tkbrep::toptools::ShapeMapHasher, 1),
        }
    }

    /// The bounding boxes of the built solids (`GetBoxesMap`); `builder.boxes_map()`.
    pub fn boxes_map(&self) -> &ShapeBoxMap {
        &self.boxes
    }

    /// Builds the solids from the faces (`Perform`); `builder.perform()?`.
    pub fn perform(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        legacy_unbound_area_result(perform_solid_with_progress(self, &range))
    }

    /// Execute this concrete solid builder with borrowed progress and typed raw completion.
    ///
    /// `builder.perform_with_progress(&indicator.start())` retains cancellation separately
    /// from the native report. Completed execution does not certify geometry or publication;
    /// cancelled, failed and incomplete receiver state remains diagnostic only.
    /// Inputs retain shared topology ownership: raw building may change input flags.
    /// This entry is not an isolated or non-destructive publication operation.
    pub fn perform_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> super::ProgressCompletion {
        // Concrete dispatch prevents an inherited area entry from bypassing solid preparation.
        perform_solid_with_progress(self, range)
    }

    fn prepare_solid(&mut self) -> bool {
        self.area.options.clear();
        if self.area.shapes.is_empty() {
            return false;
        }
        if self.area.context.is_null() {
            self.area.context = Handle::new(Context::default());
        }
        self.boxes.clear(false);
        // Native: a compound of the faces is made here and never used.
        true
    }

    /// Collects the faces to avoid (`PerformShapesToAvoid`): faces with a free edge and
    /// faces met twice on a non-seam edge, repeated until no more are found.
    fn perform_shapes_to_avoid(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.area.shapes_to_avoid.clear(false);
        let scope = ProgressScope::unnamed(range, 1., false);
        loop {
            if observe_user_break(&self.area.options, &scope, observed) {
                return Ok(());
            }
            if !self.avoid_free_faces(&self.unavoided_edge_faces()?)? {
                return Ok(());
            }
        }
    }

    fn unavoided_edge_faces(&self) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
        let mut edge_faces = IndexedDataMapOfShapeListOfShape::default();
        for face in self
            .area
            .shapes
            .iter()
            .filter(|face| !self.area.shapes_to_avoid.contains(face))
        {
            topexp::map_shapes_and_ancestors(
                face,
                ShapeEnum::Edge,
                ShapeEnum::Face,
                &mut edge_faces,
            )?;
        }
        Ok(edge_faces)
    }

    /// One pass of `PerformShapesToAvoid`; true when a face was added.
    fn avoid_free_faces(
        &mut self,
        edge_faces: &IndexedDataMapOfShapeListOfShape,
    ) -> Result<bool, Failure> {
        let mut found = false;
        for index in 1..=edge_faces.extent() {
            let edge = Edge::unchecked(edge_faces.find_key(index)?.clone());
            if BRepTool::degenerated(&edge)? {
                continue;
            }
            let faces: Vec<&Shape> = edge_faces.find_from_index(index)?.iter().collect();
            let Some(first) = faces.first().copied() else {
                continue;
            };
            let internal = edge.orientation() == Orientation::Internal;
            if faces.len() == 1 {
                if internal {
                    continue;
                }
                found = true;
                self.area.shapes_to_avoid.add(first.clone());
            } else if faces.len() == 2 && faces[1].is_same(first) {
                if BRepTool::is_closed_on_face(&edge, &Face::unchecked(first.clone()))? || internal
                {
                    continue;
                }
                found = true;
                self.area.shapes_to_avoid.add(first.clone());
                self.area.shapes_to_avoid.add(faces[1].clone());
            }
        }
        Ok(found)
    }

    /// Builds the closed shells with the shell splitter and the internal shells from the
    /// faces left over (`PerformLoops`).
    fn perform_loops(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.area.loops.clear();
        let handle = self.context_handle();
        let context = internal::required_context(&handle)?;
        let scope = ProgressScope::with_literal(range, "Building shells", 10., false);
        if !self.build_boundary_shells(context, &scope, observed)? {
            return Ok(());
        }
        self.post_shells(context, &scope, observed)
    }

    fn build_boundary_shells(
        &mut self,
        context: &Context,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        let mut splitter = self.shell_splitter(context)?;
        let completion =
            super::shell_splitter::perform_shell_with_progress(&mut splitter, &scope.next(9.));
        if matches!(completion, super::ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        if splitter.options().has_errors() {
            self.shell_splitter_warning(&splitter, scope, observed)?;
            return Ok(false);
        }
        legacy_unbound_area_result(completion)?;
        self.append_boundary_shells(&splitter);
        Ok(true)
    }

    fn append_boundary_shells(&mut self, splitter: &ShellSplitter) {
        for shell in splitter.shells().iter() {
            self.area.loops.append(shell.clone());
        }
    }

    fn shell_splitter(&mut self, context: &Context) -> Result<ShellSplitter, Failure> {
        let mut splitter = ShellSplitter::new();
        for face in self.area.shapes.iter() {
            if context.is_infinite_face(&Face::unchecked(face.clone()))? {
                self.area.loops.append(shell_of(face)?);
                continue;
            }
            if !self.area.shapes_to_avoid.contains(face) {
                splitter.add_start_element(face);
            }
        }
        splitter
            .options_mut()
            .set_run_parallel(self.area.options.run_parallel());
        Ok(splitter)
    }

    fn shell_splitter_warning(
        &mut self,
        splitter: &ShellSplitter,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        if scope.more() {
            return self.warn_shell_splitter_failed(splitter);
        }
        observed.record_cancelled();
        Ok(())
    }

    fn post_shells(
        &mut self,
        context: &Context,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let mut used = self.loop_faces();
        if observe_user_break(&self.area.options, scope, observed) {
            return Ok(());
        }
        self.avoid_unused_faces(context, &mut used)?;
        if observe_user_break(&self.area.options, scope, observed) {
            return Ok(());
        }
        self.make_internal_loops(scope, observed)
    }

    fn loop_faces(&self) -> Map<Shape> {
        let mut used = Map::new(1);
        for shell in self.area.loops.iter() {
            for face in ShapeIterator::new(shell, true, true) {
                used.add(face);
            }
        }
        used
    }

    fn warn_shell_splitter_failed(&mut self, splitter: &ShellSplitter) -> Result<(), Failure> {
        let builder = BRepBuilder::default();
        let mut faces = Compound::default();
        builder.make_compound(&mut faces);
        let mut faces = faces.into_shape();
        for face in splitter.start_elements().iter() {
            builder.add(&mut faces, face)?;
        }
        let warning: AlertHandle = Arc::new(AlertShellSplitterFailed::new(&faces));
        self.area.options.add_warning(&warning);
        Ok(())
    }

    /// Adds to the shapes to avoid every finite face in neither a shell nor the shapes to
    /// avoid.
    fn avoid_unused_faces(
        &mut self,
        context: &Context,
        used: &mut Map<Shape>,
    ) -> Result<(), Failure> {
        for face in self.area.shapes_to_avoid.iter() {
            used.add(face.clone());
        }
        for face in self.area.shapes.iter() {
            if !context.is_infinite_face(&Face::unchecked(face.clone()))? && !used.contains(face) {
                self.area.shapes_to_avoid.add(face.clone());
            }
        }
        Ok(())
    }

    /// Groups the faces to avoid into connected internal shells (`myLoopsInternal`).
    fn make_internal_loops(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.area.loops_internal.clear();
        let avoid: Vec<Shape> = self.area.shapes_to_avoid.iter().cloned().collect();
        let edge_faces = internal_loop_edges(&avoid)?;
        self.append_internal_loops(&avoid, &edge_faces, scope, observed)
    }

    fn append_internal_loops(
        &mut self,
        avoid: &[Shape],
        edges: &IndexedDataMapOfShapeListOfShape,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let mut added: Map<Shape> = Map::new(1);
        for face in avoid {
            if observe_user_break(&self.area.options, scope, observed) {
                return Ok(());
            }
            if added.add(face.clone()) {
                self.area
                    .loops_internal
                    .append(grow_internal_shell(face, edges, &mut added)?);
            }
        }
        Ok(())
    }

    fn context_handle(&self) -> Handle<Context> {
        self.area.context.clone()
    }
}

/// A shell of one face (an infinite face kept as is).
fn shell_of(face: &Shape) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let mut shell = Shell::default();
    builder.make_shell(&mut shell);
    let mut shell = shell.into_shape();
    builder.add(&mut shell, face)?;
    Ok(shell)
}

/// A shell of `seed` and every face connected to it by an edge.
fn grow_internal_shell(
    seed: &Shape,
    edge_faces: &IndexedDataMapOfShapeListOfShape,
    added: &mut Map<Shape>,
) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let mut shell = shell_of(seed)?;
    let mut position = 0;
    while let Some(face) = ShapeIterator::new(&shell, true, true).nth(position) {
        position += 1;
        for edge in Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)? {
            for other in edge_faces.find_from_key(&edge)?.iter() {
                if added.add(other.clone()) {
                    builder.add(&mut shell, other)?;
                }
            }
        }
    }
    shell.set_closed(BRepTool::is_closed(&shell)?)?;
    Ok(shell)
}

fn internal_loop_edges(avoid: &[Shape]) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
    let mut edges = IndexedDataMapOfShapeListOfShape::default();
    for face in avoid {
        topexp::map_shapes_and_ancestors(face, ShapeEnum::Edge, ShapeEnum::Face, &mut edges)?;
    }
    Ok(edges)
}

#[cfg(test)]
#[path = "builder_solid/progress_tests.rs"]
mod progress_tests;
