//! `BOPAlgoTools::ClassifyFaces` with its `BOPAlgoFillIn3DParts` task: the faces lying
//! inside each solid.
use crate::bop_tools::{BoxSet, internal_face_in_solid, is_inverted_solid};
use crate::int_tools::Context;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{Edge, Face, Shape, ShapeIterator, Solid, Vertex};
use martensite_tkbrep::toptools::{IndexedDataMapOfShapeListOfShape, ShapeMapHasher};
use martensite_tkernel::ncollection::{DataMap, IndexedDataMap, IndexedMap, List, Map};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::{BoundingBox3, box3_to_bvh};
use martensite_tkmath::bvh::BvhVec3d;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;

#[path = "classify_faces/inputs.rs"]
mod inputs;
use inputs::{SolidFaceCandidates, prepare_candidates};

#[path = "classify_faces/progress.rs"]
mod progress;
use crate::bop_algo::ProgressCancellation;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
pub use progress::{FaceClassificationJob, classify_faces_with_progress};

/// Bounding boxes already known for shapes (`theShapeBoxMap`).
pub type ShapeBoxMap = DataMap<Shape, BoundingBox3, ShapeMapHasher>;
/// Shapes and the faces classified inside them (`theInParts`).
pub type InParts = IndexedDataMap<Shape, List<Shape>, ShapeMapHasher>;
/// Own INTERNAL faces of solids (`theSolidsIF`).
pub type SolidInternalFaces = DataMap<Shape, List<Shape>, ShapeMapHasher>;

/// The faces to classify with their boxes, in a box tree (`BOPAlgoVectorOfShapeBox`).
struct FaceBoxes {
    faces: Vec<Shape>,
    tree: BoxSet<BvhVec3d>,
}

/// Classifies `faces` against `solids`, adding to `in_parts` each solid with the faces
/// inside it (`ClassifyFaces`).
/// `classify_faces(&faces, &solids, &context, &mut in_parts, &boxes, &internal)?`.
pub fn classify_faces(
    faces: &List<Shape>,
    solids: &List<Shape>,
    context: &Context,
    in_parts: &mut InParts,
    boxes: &ShapeBoxMap,
    solid_internal_faces: &SolidInternalFaces,
) -> Result<(), Failure> {
    let job = FaceClassificationJob {
        faces,
        solids,
        context,
        parts: in_parts,
        boxes,
        internal: solid_internal_faces,
        parallel: false,
    };
    let outcome = classify_faces_with_progress(job, &ProgressRange::default());
    crate::bop_algo::builder_area::progress::legacy_unbound_area_result(outcome)
}

fn empty_face_boxes(size: usize) -> FaceBoxes {
    let mut tree = BoxSet::default();
    tree.set_size(size);
    FaceBoxes {
        faces: Vec::new(),
        tree,
    }
}

fn append_face_box(
    boxes: &mut FaceBoxes,
    position: usize,
    face: &Shape,
    known: &ShapeBoxMap,
) -> Result<(), Failure> {
    let bounds = face_box(face, known)?;
    boxes.tree.add(position as i32, box3_to_bvh(&bounds)?);
    boxes.faces.push(face.clone());
    Ok(())
}

fn face_box(face: &Shape, known: &ShapeBoxMap) -> Result<BoundingBox3, Failure> {
    if let Some(bounds) = known.seek(face) {
        return Ok(bounds.clone());
    }
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(face, &mut bounds, true)?;
    Ok(bounds)
}

/// The known box of the solid, else its box, whole for an inverted solid.
fn solid_box(solid: &Solid, boxes: &ShapeBoxMap) -> Result<BoundingBox3, Failure> {
    if let Some(bounds) = boxes.seek(solid) {
        return Ok(bounds.clone());
    }
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(solid, &mut bounds, true)?;
    if !bounds.is_whole() && is_inverted_solid(solid)? {
        bounds.set_whole();
    }
    Ok(bounds)
}

fn classify_candidates_with_progress(
    input: &SolidFaceCandidates,
    context: &Context,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<List<Shape>, Failure> {
    let _: &(dyn Send + Sync) = input;
    if input.empty_solid {
        // A solid of INTERNAL faces only classifies every point IN: no real classification.
        return Ok(input.faces.iter().cloned().collect());
    }
    let blocks = ClassificationBlocks {
        solid: &input.solid,
        solid_box: &input.bounds,
        solid_edges: &input.edges,
        candidate_edges: &input.candidate_edges,
    };
    blocks.collect_inside(&input.faces, context, &scope.next(1.), observed)
}

/// The edge/face map of the candidate faces through their wires (`MapEdgesAndFaces`);
/// empty for a single candidate.
fn candidate_edge_faces(faces: &[Shape]) -> IndexedDataMapOfShapeListOfShape {
    let mut edge_faces = IndexedDataMapOfShapeListOfShape::default();
    if faces.len() < 2 {
        return edge_faces;
    }
    for face in faces {
        for wire in ShapeIterator::new(face, true, true) {
            if wire.shape_type().ok() != Some(ShapeEnum::Wire) {
                continue;
            }
            for edge in ShapeIterator::new(&wire, true, true) {
                if let Some(found) = edge_faces.change_seek(&edge) {
                    found.append(face.clone());
                    continue;
                }
                let mut found = List::default();
                found.append(face.clone());
                edge_faces.add(edge, found);
            }
        }
    }
    edge_faces
}

/// The solid and the maps the blocks of candidate faces are built and classified with.
struct ClassificationBlocks<'a> {
    solid: &'a Solid,
    solid_box: &'a BoundingBox3,
    solid_edges: &'a IndexedMap<Shape, ShapeMapHasher>,
    candidate_edges: &'a IndexedDataMapOfShapeListOfShape,
}

struct ClassificationParts {
    done: Map<Shape, ShapeMapHasher>,
    solid_edge_faces: IndexedDataMapOfShapeListOfShape,
    inside: List<Shape>,
}

impl ClassificationParts {
    fn new() -> Self {
        Self {
            done: Map::with_hasher(ShapeMapHasher, 1),
            solid_edge_faces: IndexedDataMapOfShapeListOfShape::default(),
            inside: List::default(),
        }
    }
}

impl ClassificationBlocks<'_> {
    fn collect_inside(
        &self,
        faces: &[Shape],
        context: &Context,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<List<Shape>, Failure> {
        let mut parts = ClassificationParts::new();
        let scope = ProgressScope::unnamed(range, faces.len() as f64, false);
        for face in faces {
            if progress::scope_cancelled(&scope, observed) {
                return Ok(parts.inside);
            }
            self.classify_candidate_block(face, context, &mut parts)?;
            drop(scope.next(1.));
        }
        Ok(parts.inside)
    }

    fn classify_candidate_block(
        &self,
        face: &Shape,
        context: &Context,
        parts: &mut ClassificationParts,
    ) -> Result<(), Failure> {
        let Some((block, to_classify)) = self.next_candidate_block(face, &mut parts.done)? else {
            return Ok(());
        };
        if self.block_is_internal(&to_classify, context, &mut parts.solid_edge_faces)? {
            for face in block {
                parts.inside.append(face);
            }
        }
        Ok(())
    }

    fn next_candidate_block(
        &self,
        face: &Shape,
        done: &mut Map<Shape, ShapeMapHasher>,
    ) -> Result<Option<(Vec<Shape>, Face)>, Failure> {
        if !done.add(face.clone()) {
            return Ok(None);
        }
        let (block, selected) = self.connexity_block(face, done)?;
        if !self.solid_box.is_whole() && self.block_out_of_box(&block)? {
            return Ok(None);
        }
        Ok(Some((
            block,
            selected.unwrap_or_else(|| Face::unchecked(face.clone())),
        )))
    }

    fn block_is_internal(
        &self,
        face: &Face,
        context: &Context,
        edges: &mut IndexedDataMapOfShapeListOfShape,
    ) -> Result<bool, Failure> {
        if edges.is_empty() {
            topexp::map_shapes_and_ancestors(self.solid, ShapeEnum::Edge, ShapeEnum::Face, edges)?;
        }
        internal_face_in_solid(face, self.solid, edges, precision::confusion(), context)
    }

    /// The faces connected to `start` through edges off the solid; the first face touching
    /// a solid or degenerated edge is the one to classify (`MakeConnexityBlock`).
    fn connexity_block(
        &self,
        start: &Shape,
        done: &mut Map<Shape, ShapeMapHasher>,
    ) -> Result<(Vec<Shape>, Option<Face>), Failure> {
        let mut block = vec![start.clone()];
        let mut to_classify = None;
        if self.candidate_edges.is_empty() {
            return Ok((block, to_classify));
        }
        let mut position = 0;
        while position < block.len() {
            let face = block[position].clone();
            position += 1;
            for wire in ShapeIterator::new(&face, true, true) {
                if wire.shape_type()? != ShapeEnum::Wire {
                    continue;
                }
                for edge in ShapeIterator::new(&wire, true, true) {
                    if self.solid_edges.contains(&edge)
                        || BRepTool::degenerated(&Edge::unchecked(edge.clone()))?
                    {
                        if to_classify.is_none() {
                            to_classify = Some(Face::unchecked(face.clone()));
                        }
                        continue;
                    }
                    let Some(neighbours) = self.candidate_edges.seek(&edge) else {
                        continue;
                    };
                    for neighbour in neighbours.iter() {
                        if done.add(neighbour.clone()) {
                            block.push(neighbour.clone());
                        }
                    }
                }
            }
        }
        Ok((block, to_classify))
    }

    /// Whether a vertex box of the block is out of the solid box: the block is then out.
    fn block_out_of_box(&self, block: &[Shape]) -> Result<bool, Failure> {
        for face in block {
            for vertex in Explorer::new(face, ShapeEnum::Vertex, ShapeEnum::Shape)? {
                let vertex = Vertex::unchecked(vertex);
                let mut bounds = BoundingBox3::default();
                bounds.add_point(BRepTool::pnt(&vertex)?);
                bounds.set_gap(BRepTool::vertex_tolerance(&vertex)?);
                if self.solid_box.is_out_box(&bounds) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
