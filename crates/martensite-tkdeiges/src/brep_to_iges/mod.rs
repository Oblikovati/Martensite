//! Translation of shapes into IGES entities: faces as trimmed surfaces, groups for the rest.
mod br_entity;
mod br_shell;
mod br_shell_reversed;
mod br_solid;
mod br_wire;
mod br_wire_pcurve;

pub use br_entity::{BRepToIgesBREntity, BRepToIgesOriginMap};
pub use br_shell::BRepToIgesBRShell;
pub use br_solid::BRepToIgesBRSolid;
pub use br_wire::BRepToIgesBRWire;

use crate::iges_data::{IgesDataIgesEntity, iges_array1_set, iges_new_array1};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The count of subshapes of a kind that are not in a shape of another kind.
pub fn count_subshapes(shape: &Shape, kind: ShapeEnum, avoid: ShapeEnum) -> Result<i32, Failure> {
    let mut explorer = Explorer::new(shape, kind, avoid)?;
    let mut count = 0;
    while explorer.more() {
        count += 1;
        explorer.next()?;
    }
    Ok(count)
}

/// An array from 1 of the entities of a list.
pub fn entity_array(
    entities: &[Handle<IgesDataIgesEntity>],
) -> Result<Handle<HArray1<Handle<IgesDataIgesEntity>>>, Failure> {
    if entities.is_empty() {
        return Ok(Handle::default());
    }
    let array = iges_new_array1(1, entities.len() as i32)?;
    for (rank, entity) in (1..).zip(entities) {
        iges_array1_set(&array, rank, entity.clone(), "BRepToIges")?;
    }
    Ok(array)
}
