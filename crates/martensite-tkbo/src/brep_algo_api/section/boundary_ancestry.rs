//! Face/face records omit boundary edges contributed by edge/face common blocks (#613).
use super::BRepAlgoAPISection;
use crate::bop_ds::{DataStructure, PaveBlockHandle};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::{collections::BTreeSet, sync::Arc};

pub(super) fn unique_boundary_pair(
    section: &BRepAlgoAPISection,
    structure: &DataStructure,
    edge: &Shape,
) -> Result<Option<(Shape, Shape)>, Failure> {
    if !section.is_done() {
        return Ok(None);
    }
    let Some(roots) = &section.completed_section_operands else {
        return Ok(None);
    };
    let Some(origins) = section
        .builder()
        .and_then(|builder| builder.origins().seek(edge))
    else {
        return Ok(None);
    };
    let incidences = boundary_incidences(structure, edge, origins)?;
    if incidences.is_empty() || !result_contains_edge(section, edge)? {
        return Ok(None);
    }
    let operands = roots.face_sets(structure)?;
    let Some([first, second]) = unique_pair(structure, &operands, incidences)? else {
        return Ok(None);
    };
    Ok(Some((
        structure.shape(first)?.clone(),
        structure.shape(second)?.clone(),
    )))
}

fn boundary_incidences(
    structure: &DataStructure,
    edge: &Shape,
    origins: &List<Shape>,
) -> Result<BTreeSet<(i32, i32)>, Failure> {
    let mut incidences = BTreeSet::new();
    for origin in origins.iter() {
        let source = structure.index(origin);
        if source < 0 || structure.is_new_shape(source) {
            continue;
        }
        for block in structure.pave_blocks(source)? {
            for face in common_faces_for_image(structure, edge, block)? {
                incidences.insert((source, face));
            }
        }
    }
    Ok(incidences)
}

fn common_faces_for_image(
    structure: &DataStructure,
    edge: &Shape,
    block: &PaveBlockHandle,
) -> Result<Vec<i32>, Failure> {
    let Some(common) = structure.common_block(block) else {
        return Ok(Vec::new());
    };
    let (representative, faces) = {
        let common = common.lock().map_err(|_| poisoned("common block"))?;
        if common.faces().is_empty() {
            return Ok(Vec::new());
        }
        (Arc::clone(common.pave_block1()?), common.faces().to_vec())
    };
    // Section construction publishes the first pave block's edge for the entire group.
    let image = representative
        .lock()
        .map_err(|_| poisoned("pave block"))?
        .edge();
    if image < 0 || !edge.is_same(structure.shape(image)?) {
        return Ok(Vec::new());
    }
    Ok(faces)
}

fn poisoned(object: &str) -> Failure {
    Failure::with_message(
        "Standard_ProgramError",
        &format!("section boundary ancestry: poisoned {object}; expected readable incidence"),
    )
}

fn result_contains_edge(section: &BRepAlgoAPISection, edge: &Shape) -> Result<bool, Failure> {
    let mut images = Explorer::new(section.shape(), ShapeEnum::Edge, ShapeEnum::Shape)?;
    while images.more() {
        if edge.is_same(images.current()) {
            return Ok(true);
        }
        images.next()?;
    }
    Ok(false)
}

fn unique_pair(
    structure: &DataStructure,
    operands: &[BTreeSet<i32>; 2],
    incidences: BTreeSet<(i32, i32)>,
) -> Result<Option<[i32; 2]>, Failure> {
    let mut pair = None;
    for (source, common_face) in incidences {
        for common_side in [0, 1] {
            if !operands[common_side].contains(&common_face) {
                continue;
            }
            for &owner in &operands[1 - common_side] {
                if owner == common_face || !structure.is_sub_shape(source, owner)? {
                    continue;
                }
                let mut candidate = [owner; 2];
                candidate[common_side] = common_face;
                if pair.is_some_and(|other| candidate != other) {
                    return Ok(None);
                }
                pair = Some(candidate);
            }
        }
    }
    Ok(pair)
}
