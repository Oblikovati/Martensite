//! `BOPAlgoBuilder::FillSameDomainFaces` and `FillInternalVertices` (BOPAlgoBuilder_2).
use super::images::append_listed;
use super::{BOPAlgoBuilder, SameDomainShapes, required_context};
use crate::bop_algo::tools::{ConnectionMap, fill_map, make_blocks};
use crate::bop_ds::DataStructure;
use crate::bop_tools::ShapeSet;
use crate::int_tools::Context;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::{DataMap, HashPolicy, IndexedDataMap, List, Map};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::SurfaceType;

/// Faces by their set of edges (`anESetFaces`).
type EdgeSetFaces = IndexedDataMap<ShapeSet, Vec<Shape>, occurrence_keys::EdgeOccurrencePolicy>;

impl BOPAlgoBuilder {
    /// Groups the splits of intersected faces with equal edge sets that are same-domain, and
    /// makes the face of minimal DS index (or the first split) the image of the group
    /// (`FillSameDomainFaces`); `builder.fill_same_domain_faces(&ds)?`.
    pub fn fill_same_domain_faces(&mut self, structure: &DataStructure) -> Result<(), Failure> {
        let observed = crate::bop_algo::ProgressCancellation::default();
        let range = martensite_tkernel::message::ProgressRange::default();
        let completion = self.fill_same_domain_faces_progress_raw(structure, &range, &observed)?;
        crate::bop_algo::builder_area::progress::legacy_unbound_area_result(completion)
    }

    /// The solid each source face, and each split of it, belongs to: two faces of one solid
    /// cannot be same-domain (`aFaceToParent`).
    fn face_parents(&self, structure: &DataStructure) -> Result<SameDomainShapes, Failure> {
        let mut parents: SameDomainShapes = DataMap::with_hasher(ShapeMapHasher, 1);
        for index in 0..structure.nb_source_shapes() {
            let info = structure.shape_info(index)?;
            if info.shape_type() != ShapeEnum::Solid {
                continue;
            }
            for face in Explorer::new(info.shape(), ShapeEnum::Face, ShapeEnum::Shape)? {
                if !parents.is_bound(&face) {
                    parents.bind(face, info.shape().clone());
                }
            }
        }
        let mut propagation: SameDomainShapes = DataMap::with_hasher(ShapeMapHasher, 1);
        for (face, pieces) in self.images.items() {
            let Some(parent) = parents.seek(face) else {
                continue;
            };
            for piece in pieces.iter() {
                if !parents.is_bound(piece) {
                    propagation.bind(piece.clone(), parent.clone());
                }
            }
        }
        for (piece, parent) in propagation.items() {
            parents.bind(piece.clone(), parent.clone());
        }
        Ok(parents)
    }

    /// The splits (or the faces) of the intersected faces, sorted by DS index, by edge set;
    /// with the bounded planar ones.
    fn add_source_face_sets(
        &self,
        structure: &DataStructure,
        index: i32,
        sets: &mut EdgeSetFaces,
        planar: &mut Map<Shape, ShapeMapHasher>,
    ) -> Result<(), Failure> {
        let info = structure.shape_info(index)?;
        let check_planar = self.same_domain_planar_candidate(info)?;
        for piece in self.same_domain_face_pieces(info.shape()) {
            add_edge_set(&piece, sets)?;
            if check_planar {
                planar.add(piece);
            }
        }
        Ok(())
    }

    fn same_domain_planar_candidate(
        &self,
        info: &crate::bop_ds::ShapeInfo,
    ) -> Result<bool, Failure> {
        let context = required_context(&self.context)?;
        Ok(context
            .surface_adaptor(&Face::unchecked(info.shape().clone()))?
            .read()
            .get_type()?
            == SurfaceType::Plane
            && !is_open_box(info.box_bounds()))
    }

    fn same_domain_face_pieces(&self, face: &Shape) -> Vec<Shape> {
        match self.images.seek(face) {
            Some(pieces) => pieces.iter().cloned().collect(),
            None => vec![face.clone()],
        }
    }

    /// The face of minimal DS index (else the first face) represents the group; original
    /// faces become their own image.
    fn bind_same_domain_group(&mut self, structure: &DataStructure, group: &[Shape]) {
        let mut representative: Option<&Shape> = None;
        let mut minimal = i32::MAX;
        for face in group {
            let index = structure.index(face);
            if index < 0 {
                continue;
            }
            // An original face has no split but a same-domain face: it is its own split.
            let mut image = List::default();
            image.append(face.clone());
            self.images.bound(face.clone(), image);
            if index < minimal {
                minimal = index;
                representative = Some(face);
            }
        }
        let representative = representative.unwrap_or(&group[0]).clone();
        for face in group {
            self.shapes_sd.bind(face.clone(), representative.clone());
        }
    }

    /// Replaces the face images by their same-domain face and fills the origins.
    fn update_face_images_with_same_domain(
        &mut self,
        structure: &DataStructure,
    ) -> Result<(), Failure> {
        for index in 0..structure.nb_source_shapes() {
            let info = structure.shape_info(index)?;
            if info.shape_type() != ShapeEnum::Face {
                continue;
            }
            let face = info.shape().clone();
            let Some(images) = self.images.change_seek(&face) else {
                continue;
            };
            let mut updated = Vec::new();
            for image in images.iter_mut() {
                if let Some(same_domain) = self.shapes_sd.seek(image) {
                    *image = same_domain.clone();
                }
                updated.push(image.clone());
            }
            for image in updated {
                append_listed(&mut self.origins, &image, &face);
            }
        }
        Ok(())
    }
}

/// The intersected faces with face information, by increasing DS index.
fn intersected_faces(
    structure: &DataStructure,
    mut poll: impl FnMut() -> bool,
) -> Result<Option<Vec<i32>>, Failure> {
    let mut fence: Map<i32> = Map::new(1);
    let mut faces = Vec::new();
    for record in structure.interf_ff_records().iter() {
        if poll() {
            return Ok(None);
        }
        append_intersected_faces(structure, record.indices(), &mut fence, &mut faces)?;
    }
    faces.sort_unstable();
    Ok(Some(faces))
}

fn append_intersected_faces(
    structure: &DataStructure,
    indices: (i32, i32),
    fence: &mut Map<i32>,
    faces: &mut Vec<i32>,
) -> Result<(), Failure> {
    for face in [indices.0, indices.1] {
        if structure.has_face_info(face)? && fence.add(face) {
            faces.push(face);
        }
    }
    Ok(())
}

fn is_open_box(bounds: &martensite_tkmath::bnd::BoundingBox3) -> bool {
    bounds.is_open_xmin()
        || bounds.is_open_xmax()
        || bounds.is_open_ymin()
        || bounds.is_open_ymax()
        || bounds.is_open_zmin()
        || bounds.is_open_zmax()
}

/// Adds the face under its set of edges (`AddEdgeSet`).
fn add_edge_set<H: HashPolicy<ShapeSet>>(
    face: &Shape,
    sets: &mut IndexedDataMap<ShapeSet, Vec<Shape>, H>,
) -> Result<(), Failure> {
    let mut set = ShapeSet::new();
    set.add(face, ShapeEnum::Edge)?;
    if let Some(faces) = sets.change_seek(&set) {
        faces.push(face.clone());
        return Ok(());
    }
    sets.add(set, vec![face.clone()]);
    Ok(())
}

/// Links the pairs of bounded planar faces of one edge set, which are same-domain without
/// a check, and collects the other pairs for `AreFacesSameDomain`; faces of one solid are
/// never paired.
fn link_planar_pairs(
    faces: &[Shape],
    planar: &Map<Shape, ShapeMapHasher>,
    parents: &SameDomainShapes,
    links: &mut ConnectionMap<Shape, Shape, ShapeMapHasher>,
    pairs: &mut Vec<(Shape, Shape)>,
) {
    if faces.len() < 2 {
        return;
    }
    for (position, first) in faces.iter().enumerate() {
        let first_planar = planar.contains(first);
        let first_parent = parents.seek(first);
        for second in &faces[position + 1..] {
            if let (Some(a), Some(b)) = (first_parent, parents.seek(second))
                && a.is_same(b)
            {
                continue;
            }
            if first_planar && planar.contains(second) {
                fill_map(links, first.clone(), second.clone());
                continue;
            }
            pairs.push((first.clone(), second.clone()));
        }
    }
}

#[path = "same_domain_faces/query.rs"]
mod query;

#[path = "same_domain_faces/preparation.rs"]
mod preparation;
#[path = "same_domain_faces/progress.rs"]
mod progress;
#[path = "same_domain_faces/tasks.rs"]
mod tasks;

#[cfg(test)]
#[path = "same_domain_faces/contract.rs"]
mod contract;
#[cfg(test)]
#[path = "same_domain_faces/fixtures.rs"]
mod fixtures;

#[cfg(test)]
#[path = "same_domain_faces/census.rs"]
mod census;

#[path = "same_domain_faces/occurrence_keys.rs"]
mod occurrence_keys;

#[cfg(test)]
#[path = "same_domain_faces/key_collision_tests.rs"]
mod key_collision_tests;
