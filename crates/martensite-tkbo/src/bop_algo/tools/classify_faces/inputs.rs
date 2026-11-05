//! One coordinator-owned BVH query prepares each solid's original ordered candidates.
use super::FaceBoxes;
use crate::bop_tools::BoxSelector;
use martensite_tkbrep::{
    topexp,
    topods::{Shape, Solid},
    toptools::ShapeMapHasher,
};
use martensite_tkernel::{
    ncollection::{IndexedMap, List},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::{BoundingBox3, box3_to_bvh};
use martensite_tkmath::bvh::BvhSetTraverse;

type ShapeIndex = IndexedMap<Shape, ShapeMapHasher>;

/// Own the exact ordered query result and identity maps; `candidates.faces.len()`.
pub(super) struct SolidFaceCandidates {
    pub solid: Solid,
    pub bounds: BoundingBox3,
    pub edges: ShapeIndex,
    pub faces: Vec<Shape>,
    pub empty_solid: bool,
    pub candidate_edges: martensite_tkbrep::toptools::IndexedDataMapOfShapeListOfShape,
}

/// Query the existing built BVH once for the actual solid; `prepare_candidates(&solid, &bounds, &internal, &mut boxes)?`.
pub(super) fn prepare_candidates(
    solid: &Solid,
    bounds: &BoundingBox3,
    own_internal: &List<Shape>,
    boxes: &mut FaceBoxes,
) -> Result<Option<SolidFaceCandidates>, Failure> {
    let selected = selected_indices(bounds, boxes)?;
    if selected.is_empty() {
        return Ok(None);
    }
    let (edges, faces, empty_solid) = solid_members(solid, own_internal)?;
    let faces = candidate_shapes(selected, &faces, &boxes.faces);
    Ok(Some(candidate_record(
        solid,
        bounds,
        edges,
        faces,
        empty_solid,
    )))
}

fn candidate_record(
    solid: &Solid,
    bounds: &BoundingBox3,
    edges: ShapeIndex,
    faces: Vec<Shape>,
    empty_solid: bool,
) -> SolidFaceCandidates {
    let candidate_edges = selected_edge_map(&faces, empty_solid);
    SolidFaceCandidates {
        solid: solid.clone(),
        bounds: bounds.clone(),
        edges,
        faces,
        empty_solid,
        candidate_edges,
    }
}

fn selected_edge_map(
    faces: &[Shape],
    empty_solid: bool,
) -> martensite_tkbrep::toptools::IndexedDataMapOfShapeListOfShape {
    if empty_solid {
        return Default::default();
    }
    // Both serial and parallel consumers reuse this one identity-incidence map.
    super::candidate_edge_faces(faces)
}

fn selected_indices(bounds: &BoundingBox3, boxes: &mut FaceBoxes) -> Result<Vec<i32>, Failure> {
    let mut selector = BoxSelector::default();
    selector.set_box(box3_to_bvh(bounds)?);
    selector.set_bvh_set(Some(&mut boxes.tree));
    selector.select()?;
    Ok(selector.indices().iter().copied().collect())
}

fn solid_members(
    solid: &Solid,
    own_internal: &List<Shape>,
) -> Result<(ShapeIndex, ShapeIndex, bool), Failure> {
    let mut edges = ShapeIndex::with_hasher(ShapeMapHasher, 1);
    topexp::map_shapes_of_type(solid, ShapeEnum::Edge, &mut edges)?;
    let mut faces = ShapeIndex::with_hasher(ShapeMapHasher, 1);
    topexp::map_shapes_of_type(solid, ShapeEnum::Face, &mut faces)?;
    let empty = faces.is_empty();
    for face in own_internal.iter() {
        faces.add(face.clone());
    }
    Ok((edges, faces, empty))
}

fn candidate_shapes(selected: Vec<i32>, members: &ShapeIndex, faces: &[Shape]) -> Vec<Shape> {
    let mut indices: Vec<i32> = selected
        .into_iter()
        .filter(|index| !members.contains(&faces[*index as usize]))
        .collect();
    indices.sort_unstable();
    indices
        .into_iter()
        .map(|index| faces[index as usize].clone())
        .collect()
}
