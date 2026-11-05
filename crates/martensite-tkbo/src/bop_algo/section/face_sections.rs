//! Original face sections and edge/face common blocks retain their construction order.
use super::*;
/// The section vertices, the new or interfered In vertices and the section edges of the
/// faces. Example: `add_face_sections_checked(ds, sections, cancelled)?`.
pub(super) fn add_face_sections_checked(
    structure: &DataStructure,
    sections: &mut Shape,
    mut cancelled: impl FnMut() -> bool,
) -> Result<bool, Failure> {
    for index in 0..structure.nb_source_shapes() {
        if structure.shape_info(index)?.shape_type() != ShapeEnum::Face {
            continue;
        }
        if cancelled() {
            return Ok(false);
        }
        append_source_face_sections(structure, index, sections)?;
    }
    Ok(true)
}
fn append_source_face_sections(
    structure: &DataStructure,
    index: i32,
    sections: &mut Shape,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    let info = structure.face_info(index)?;
    for vertex in info.vertices_section().iter() {
        builder.add(sections, structure.shape(*vertex)?)?;
    }
    for &vertex in info.vertices_in().iter() {
        if vertex >= 0 && (structure.is_new_shape(vertex) || structure.has_interference_for(vertex))
        {
            builder.add(sections, structure.shape(vertex)?)?;
        }
    }
    for block in info.pave_blocks_section().iter() {
        let edge = block.lock().expect("pave-block lock").edge();
        builder.add(sections, structure.shape(edge)?)?;
    }
    Ok(())
}

/// The edges of the common blocks lying on faces; `add_face_common_edges(ds, sections)?`.
pub(super) fn add_face_common_edges(
    structure: &DataStructure,
    sections: &mut Shape,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    for blocks in structure.pave_blocks_pool() {
        for block in blocks {
            let Some(common) = structure.common_block(block) else {
                continue;
            };
            let first = {
                let common = common.lock().expect("common-block lock");
                if common.faces().is_empty() {
                    continue;
                }
                std::sync::Arc::clone(common.pave_block1()?)
            };
            let edge = first.lock().expect("pave-block lock").edge();
            builder.add(sections, structure.shape(edge)?)?;
        }
    }
    Ok(())
}
