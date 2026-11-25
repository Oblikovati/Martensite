//! Scope common-block ancestry to the operands of the retained section build.
use crate::bop_ds::DataStructure;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::collections::BTreeSet;

pub(super) struct SectionOperandRoots {
    roots: Vec<i32>,
    first_count: usize,
}

impl SectionOperandRoots {
    pub(super) fn capture(
        structure: &DataStructure,
        first: &List<Shape>,
        second: &List<Shape>,
        built: &List<Shape>,
    ) -> Option<Self> {
        let roots: Vec<_> = first
            .iter()
            .chain(second.iter())
            .map(|root| structure.index(root))
            .collect();
        if roots.contains(&-1) || !matches_build_roots(&roots, structure, built) {
            return None;
        }
        Some(Self {
            roots,
            first_count: first.len(),
        })
    }

    pub(super) fn face_sets(
        &self,
        structure: &DataStructure,
    ) -> Result<[BTreeSet<i32>; 2], Failure> {
        let (first, second) = self.roots.split_at(self.first_count);
        Ok([
            face_indices(first, structure)?,
            face_indices(second, structure)?,
        ])
    }
}

fn matches_build_roots(roots: &[i32], structure: &DataStructure, built: &List<Shape>) -> bool {
    let indexed = |shape| structure.index(shape);
    let intersected = structure.arguments().iter().map(indexed);
    let built = built.iter().map(indexed);
    if intersected.clone().eq(roots.iter().copied()) && built.clone().eq(roots.iter().copied()) {
        return true;
    }
    // A supplied filler can have been intersected with the same roots in another order.
    let requested: BTreeSet<_> = roots.iter().copied().collect();
    intersected.collect::<BTreeSet<_>>() == requested && built.collect::<BTreeSet<_>>() == requested
}

fn face_indices(roots: &[i32], structure: &DataStructure) -> Result<BTreeSet<i32>, Failure> {
    let mut pending = roots.to_vec();
    let mut seen = BTreeSet::new();
    let mut faces = BTreeSet::new();
    while let Some(index) = pending.pop() {
        if !seen.insert(index) {
            continue;
        }
        let info = structure.shape_info(index)?;
        // The BOPDS retains the original topology even if source handles are edited later.
        if info.shape_type() == ShapeEnum::Face {
            faces.insert(index);
        } else if info.shape_type() < ShapeEnum::Face {
            pending.extend_from_slice(info.sub_shapes());
        }
    }
    Ok(faces)
}
