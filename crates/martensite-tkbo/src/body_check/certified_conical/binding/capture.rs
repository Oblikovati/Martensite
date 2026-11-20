// SPDX-License-Identifier: GPL-2.0-only
//! Explicit source identities cover every actual native face exactly once.
use super::*;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use std::collections::BTreeMap;
pub(super) fn family(
    bindings: &[ConicalNativeMember],
) -> Result<ConicalNativeGeometry, ConicalBindingError> {
    let mut ids = BTreeSet::new();
    let mut members = Vec::new();
    for binding in bindings {
        if !ids.insert(binding.id) {
            return Err(error(
                Some(binding.id),
                None,
                None,
                native::rejected("duplicate source member"),
            ));
        }
        members.push(member(binding)?);
    }
    members.sort_by_key(|member| member.id);
    let adjacent = adjacency(&members)?;
    let mut side_orientations = BTreeMap::new();
    for side in members.iter().flat_map(|member| &member.sides) {
        side_orientations.entry(side.id).or_insert(side.orientation);
    }
    Ok(ConicalNativeGeometry {
        members,
        adjacent,
        side_orientations,
    })
}
fn member(binding: &ConicalNativeMember) -> Result<MemberGeometry, ConicalBindingError> {
    face_coverage(binding).map_err(|cause| error(Some(binding.id), None, None, cause))?;
    let sides = capture_sides(binding)?;
    let caps = capture_caps(binding, &sides)?;
    let mut materials: Vec<_> = binding
        .caps
        .iter()
        .map(|cap| (cap.id.0, cap.material.source_bits()))
        .collect();
    materials.sort_by_key(|(id, _)| *id);
    Ok(MemberGeometry {
        id: binding.id,
        sides,
        caps,
        materials,
    })
}
fn face_coverage(binding: &ConicalNativeMember) -> Result<(), Failure> {
    let actual = native::children(&binding.shape, martensite_tkg3d::top_abs::ShapeEnum::Face)?;
    let supplied: Vec<_> = binding
        .sides
        .iter()
        .map(|(_, face)| face)
        .chain(binding.caps.iter().map(|cap| &cap.face))
        .collect();
    if actual.len() != supplied.len() || actual.is_empty() {
        return Err(native::rejected("source/native face count differs"));
    }
    oriented_faces(&actual, &supplied)?;
    Ok(())
}
fn adjacency(
    members: &[MemberGeometry],
) -> Result<Vec<ConicalActualAdjacent>, ConicalBindingError> {
    let sides: Vec<_> = members
        .iter()
        .flat_map(|member| member.sides.iter().map(move |side| (member.id, side)))
        .collect();
    let mut adjacent = Vec::new();
    append_adjacency(&sides, &mut adjacent)?;
    Ok(adjacent)
}

fn capture_sides(binding: &ConicalNativeMember) -> Result<Vec<native::Side>, ConicalBindingError> {
    let mut sides = Vec::new();
    for (id, face) in &binding.sides {
        sides.push(
            native::capture_side(id.0, face)
                .map_err(|cause| error(Some(binding.id), Some(*id), None, cause))?,
        );
    }
    sides.sort_by_key(|side| side.id);
    if sides.windows(2).any(|pair| pair[0].id == pair[1].id) {
        return Err(error(
            Some(binding.id),
            None,
            None,
            native::rejected("duplicate source side"),
        ));
    }
    Ok(sides)
}

fn capture_caps(
    binding: &ConicalNativeMember,
    sides: &[native::Side],
) -> Result<Vec<native::Cap>, ConicalBindingError> {
    let rows = native::SideRows::capture(sides);
    let mut caps = Vec::new();
    for cap in &binding.caps {
        let curves: Vec<_> = cap
            .curves
            .iter()
            .map(|(curve, side)| (*curve, side.0))
            .collect();
        caps.push(
            native::capture_cap(cap.id.0, &cap.face, &rows, &cap.material, &curves)
                .map_err(|cause| error(Some(binding.id), None, Some(cap.id), cause))?,
        );
    }
    caps.sort_by_key(|cap| cap.id);
    unique_caps(binding, &caps)?;
    Ok(caps)
}

fn oriented_faces(actual: &[Shape], supplied: &[&Face]) -> Result<(), Failure> {
    let mut faces = IndexedMapOfShape::default();
    let mut counts = Vec::<[usize; 4]>::new();
    for &face in supplied {
        let key = faces.add(Shape::clone(face));
        if key > counts.len() {
            counts.push([0; 4]);
        }
        counts[key - 1][face.orientation() as usize] += 1;
    }
    for face in actual {
        let key = faces.find_index(face);
        if key == 0 || counts[key - 1][face.orientation() as usize] != 1 {
            return Err(native::rejected(
                "actual native face lacks unique oriented source binding",
            ));
        }
    }
    Ok(())
}

fn append_adjacency(
    sides: &[(ConicalActualMemberId, &native::Side)],
    adjacent: &mut Vec<ConicalActualAdjacent>,
) -> Result<(), ConicalBindingError> {
    for ([first, second], shared) in actual_shared_edges(sides) {
        let (owner, first) = sides[first];
        let (other, second) = sides[second];
        append_pair(owner, first, other, second, shared, adjacent)?;
    }
    Ok(())
}

struct PairSlots {
    slots: [usize; 2],
    multiple: bool,
}

fn actual_shared_edges(
    sides: &[(ConicalActualMemberId, &native::Side)],
) -> BTreeMap<[usize; 2], PairSlots> {
    let mut edges = IndexedMapOfShape::default();
    let mut uses = Vec::<Vec<[usize; 2]>>::new();
    let mut pairs = BTreeMap::<[usize; 2], PairSlots>::new();
    for (index, (_, side)) in sides.iter().enumerate() {
        for (slot, edge) in side.edges.iter().enumerate() {
            let key = edges.add(edge.clone());
            if key > uses.len() {
                uses.push(Vec::new());
            }
            record_incidence(&mut uses[key - 1], &mut pairs, [index, slot]);
        }
    }
    pairs
}

fn record_incidence(
    uses: &mut Vec<[usize; 2]>,
    pairs: &mut BTreeMap<[usize; 2], PairSlots>,
    [index, slot]: [usize; 2],
) {
    for &[other, previous_slot] in uses.iter().filter(|entry| entry[0] < index) {
        pairs
            .entry([other, index])
            .and_modify(|shared| shared.multiple = true)
            .or_insert(PairSlots {
                slots: [previous_slot, slot],
                multiple: false,
            });
    }
    uses.push([index, slot]);
}

fn require_same_owner(
    owner: ConicalActualMemberId,
    other: ConicalActualMemberId,
    first: &native::Side,
) -> Result<(), ConicalBindingError> {
    if owner != other {
        return Err(error(
            Some(owner),
            Some(ConicalActualSideId(first.id)),
            None,
            native::rejected("native ruling shared across source members"),
        ));
    }
    Ok(())
}

fn append_pair(
    owner: ConicalActualMemberId,
    first: &native::Side,
    other: ConicalActualMemberId,
    second: &native::Side,
    shared: PairSlots,
    adjacent: &mut Vec<ConicalActualAdjacent>,
) -> Result<(), ConicalBindingError> {
    let slots = shared_slots(shared).map_err(|cause| {
        error(
            Some(owner),
            Some(ConicalActualSideId(first.id)),
            None,
            cause,
        )
    })?;
    require_same_owner(owner, other, first)?;
    adjacent.push(ConicalActualAdjacent {
        sides: [
            ConicalActualSideId(first.id),
            ConicalActualSideId(second.id),
        ],
        u_slots: slots,
    });
    Ok(())
}

fn shared_slots(shared: PairSlots) -> Result<[u8; 2], Failure> {
    if shared.multiple || shared.slots.iter().any(|slot| *slot < 2) {
        return Err(native::rejected(
            "actual sides have unsupported shared non-ruling incidence",
        ));
    }
    Ok(shared.slots.map(|slot| (slot - 2) as u8))
}

fn unique_caps(
    binding: &ConicalNativeMember,
    caps: &[native::Cap],
) -> Result<(), ConicalBindingError> {
    if caps.windows(2).any(|pair| pair[0].id == pair[1].id) {
        return Err(error(
            Some(binding.id),
            None,
            None,
            native::rejected("duplicate source cap"),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexed_incidence_preserves_pair_order_and_refuses_a_second_shared_edge() {
        let mut pairs = BTreeMap::new();
        record_incidence(&mut vec![[0, 3]], &mut pairs, [2, 2]);
        record_incidence(&mut vec![[0, 2]], &mut pairs, [1, 3]);
        assert_eq!(pairs.keys().copied().collect::<Vec<_>>(), [[0, 1], [0, 2]]);
        assert_eq!(
            shared_slots(pairs.remove(&[0, 1]).unwrap()).unwrap(),
            [0, 1]
        );
        record_incidence(&mut vec![[0, 2]], &mut pairs, [2, 3]);
        assert!(shared_slots(pairs.remove(&[0, 2]).unwrap()).is_err());
    }
}
