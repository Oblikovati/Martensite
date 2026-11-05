//! `BOPAlgoBuilderFace`: splits a face by a set of edges lying on it.
use super::alerts::alert;
use super::builder_area::progress::observe_user_break;
use super::{AlertNullInputShapes, BuilderArea, ProgressCancellation};
use crate::int_tools::Context;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{Edge, Face, Shape};
use martensite_tkbrep::toptools::IndexedDataMapOfShapeListOfShape;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use std::ops::{Deref, DerefMut};

#[path = "builder_face/areas.rs"]
mod areas;
#[path = "builder_face/internal.rs"]
mod internal;
#[path = "builder_face/loops.rs"]
mod loops;
#[path = "builder_face/progress.rs"]
mod progress;
use internal::is_inside;

/// Builds the faces bounded by split edges on one face: wires from the edges, faces from
/// the outer wires with their holes, then the internal edges put into the faces.
///
/// ```
/// use martensite_tkbo::bop_algo::BuilderFace;
/// let mut builder = BuilderFace::new();
/// builder.perform().unwrap();
/// assert!(builder.options().has_errors()); // no face was given
/// ```
pub struct BuilderFace {
    area: BuilderArea,
    face: Face,
    orientation: Orientation,
}

impl Default for BuilderFace {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for BuilderFace {
    type Target = BuilderArea;
    fn deref(&self) -> &BuilderArea {
        &self.area
    }
}

impl DerefMut for BuilderFace {
    fn deref_mut(&mut self) -> &mut BuilderArea {
        &mut self.area
    }
}

impl BuilderFace {
    /// A builder with no face and an EXTERNAL orientation (`BOPAlgoBuilderFace()`).
    /// `BuilderFace::new()`.
    pub fn new() -> Self {
        Self::with_allocator(Some(BaseAllocator::common()))
    }

    /// Retains the supplied nullable allocation-policy identity in the base options.
    /// `BuilderFace::with_allocator(None)`.
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        Self {
            area: BuilderArea::with_allocator(allocator),
            face: Face::default(),
            orientation: Orientation::External,
        }
    }

    /// Sets the face to split, kept FORWARD with its orientation saved (`SetFace`).
    /// `builder.set_face(&face)`.
    pub fn set_face(&mut self, face: &Face) {
        self.orientation = face.orientation();
        self.face = Face::unchecked(face.oriented(Orientation::Forward));
    }

    /// The FORWARD face to split (`Face`); `builder.face()`.
    pub fn face(&self) -> &Face {
        &self.face
    }

    /// The orientation of the face given to `set_face` (`Orientation`); `builder.orientation()`.
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Builds the split faces (`Perform`); `builder.perform()?`.
    pub fn perform(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        super::builder_area::progress::legacy_unbound_area_result(
            self.perform_with_progress(&range),
        )
    }

    /// Reports a missing face and creates the context (`CheckData`).
    fn check_data(&mut self) {
        if self.face.is_null() {
            self.area.options.add_error(&alert(AlertNullInputShapes));
            return;
        }
        if self.area.context.is_null() {
            self.area.context = Handle::new(Context::default());
        }
    }

    /// Collects the edges to avoid (`PerformShapesToAvoid`): edges with a free vertex and
    /// edges met twice at a vertex, repeated until no more are found.
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
            let vertex_edges = self.available_vertex_edges()?;
            if !self.avoid_free_edges(&vertex_edges)? {
                return Ok(());
            }
        }
    }

    fn available_vertex_edges(&self) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
        let mut vertex_edges = IndexedDataMapOfShapeListOfShape::default();
        for edge in self.area.shapes.iter() {
            if !self.area.shapes_to_avoid.contains(edge) {
                topexp::map_shapes_and_ancestors(
                    edge,
                    ShapeEnum::Vertex,
                    ShapeEnum::Edge,
                    &mut vertex_edges,
                )?;
            }
        }
        Ok(vertex_edges)
    }

    /// One pass of `PerformShapesToAvoid` over the vertex/edge map; true when an edge was
    /// added to the shapes to avoid.
    fn avoid_free_edges(
        &mut self,
        vertex_edges: &IndexedDataMapOfShapeListOfShape,
    ) -> Result<bool, Failure> {
        let mut found = false;
        for index in 1..=vertex_edges.extent() {
            let vertex = vertex_edges.find_key(index)?;
            let edges: Vec<&Shape> = vertex_edges.find_from_index(index)?.iter().collect();
            let Some(first) = edges.first().copied() else {
                continue;
            };
            if edges.len() == 1 {
                if BRepTool::degenerated(&Edge::unchecked(first.clone()))?
                    || vertex.orientation() == Orientation::Internal
                {
                    continue;
                }
                found = true;
                self.area.shapes_to_avoid.add(first.clone());
            } else if edges.len() == 2 && edges[1].is_same(first) {
                let (start, end) = topexp::vertices(&Edge::unchecked(first.clone()), false);
                if start.is_same(&end) {
                    continue;
                }
                found = true;
                self.area.shapes_to_avoid.add(first.clone());
                self.area.shapes_to_avoid.add(edges[1].clone());
            }
        }
        Ok(found)
    }
}

/// The context created by `CheckData`.
fn required_context(handle: &Handle<Context>) -> Result<&Context, Failure> {
    super::builder_area::required_context(handle, "BOPAlgoBuilderFace")
}
