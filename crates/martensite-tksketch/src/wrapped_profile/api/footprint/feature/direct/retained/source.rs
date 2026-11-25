//! Verify the exact source faces of each native tool member.
use super::*;
use martensite_tkbrep::toptools::IndexedMapOfShape;

pub(super) fn shell_faces(
    records: &[crate::surface_sketch::WrappedHistoryRecord],
    native: &DetachedWrappedBodies,
) -> Bound<BTreeMap<WrappedHistoryKey, Shape>> {
    let mut members = BTreeMap::new();
    let mut unique_roots = BTreeSet::new();
    let mut root_members = IndexedMapOfShape::default();
    let mut children = ShapeIterator::new(&native.bodies, true, true);
    while children.more() {
        let shape = children.value()?.to_shape();
        let before = root_members.extent();
        if shape.shape_type()? != ShapeEnum::Solid || root_members.add(shape) != before + 1 {
            return Err(Rejected::Authority);
        }
        children.next();
    }
    for item in records {
        if item.result.role != HistoryRole::Solid {
            continue;
        }
        let member = native
            .entities
            .get(&item.result)
            .ok_or(Rejected::Missing(item.result))?;
        let index = root_members.find_index(member);
        if index == 0 || !unique_roots.insert(index) {
            return Err(Rejected::Authority);
        }
        let mut faces = IndexedMapOfShape::default();
        martensite_tkbrep::topexp::map_shapes_of_type(member, ShapeEnum::Face, &mut faces)?;
        if members.insert(item.owner, faces).is_some() {
            return Err(Rejected::Authority);
        }
    }
    if unique_roots.len() != root_members.extent() {
        return Err(Rejected::Authority);
    }
    let mut result = BTreeMap::new();
    for item in records {
        let source = native
            .entities
            .get(&item.result)
            .ok_or(Rejected::Missing(item.result))?;
        if source.shape_type()? != ShapeEnum::Face {
            continue;
        }
        let faces = members
            .get(&item.owner)
            .ok_or(Rejected::Missing(item.result))?;
        let face = faces
            .find_key(faces.find_index(source))
            .map_err(|_| Rejected::Missing(item.result))?;
        if result.insert(item.result, face.clone()).is_some() {
            return Err(Rejected::History);
        }
    }
    Ok(result)
}
