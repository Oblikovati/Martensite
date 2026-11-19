// SPDX-License-Identifier: GPL-2.0-only
//! Explicit source identities cover every actual native face exactly once.
use super::*;
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
    Ok(ConicalNativeGeometry { members, adjacent })
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
fn shared(first: &native::Side, second: &native::Side) -> Result<Option<[u8; 2]>, Failure> {
    let pairs: Vec<_> = first
        .edges
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            second
                .edges
                .iter()
                .enumerate()
                .filter_map(move |(j, b)| a.is_same(b).then_some([i, j]))
        })
        .collect();
    shared_slots(pairs)
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
    let mut caps = Vec::new();
    for cap in &binding.caps {
        let curves: Vec<_> = cap
            .curves
            .iter()
            .map(|(curve, side)| (*curve, side.0))
            .collect();
        caps.push(
            native::capture_cap(cap.id.0, &cap.face, &sides, &cap.material, &curves)
                .map_err(|cause| error(Some(binding.id), None, Some(cap.id), cause))?,
        );
    }
    caps.sort_by_key(|cap| cap.id);
    unique_caps(binding, &caps)?;
    Ok(caps)
}

fn oriented_faces(actual: &[Shape], supplied: &[&Face]) -> Result<(), Failure> {
    for face in actual {
        if supplied
            .iter()
            .filter(|candidate| {
                candidate.is_same(face) && candidate.orientation() == face.orientation()
            })
            .count()
            != 1
        {
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
    for (i, (owner, first)) in sides.iter().enumerate() {
        for (other, second) in &sides[i + 1..] {
            append_pair(*owner, first, *other, second, adjacent)?;
        }
    }
    Ok(())
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
    adjacent: &mut Vec<ConicalActualAdjacent>,
) -> Result<(), ConicalBindingError> {
    if let Some(slots) = shared(first, second).map_err(|cause| {
        error(
            Some(owner),
            Some(ConicalActualSideId(first.id)),
            None,
            cause,
        )
    })? {
        require_same_owner(owner, other, first)?;
        adjacent.push(ConicalActualAdjacent {
            sides: [
                ConicalActualSideId(first.id),
                ConicalActualSideId(second.id),
            ],
            u_slots: slots,
        });
    }
    Ok(())
}

fn shared_slots(pairs: Vec<[usize; 2]>) -> Result<Option<[u8; 2]>, Failure> {
    if pairs.is_empty() {
        return Ok(None);
    }
    if pairs.len() != 1 || pairs[0].iter().any(|slot| *slot < 2) {
        return Err(native::rejected(
            "actual sides have unsupported shared non-ruling incidence",
        ));
    }
    Ok(Some(pairs[0].map(|slot| (slot - 2) as u8)))
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
