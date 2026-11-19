//! Authored keys select actual shell occurrences through native is_same only.
use super::*;
use martensite_tkbo::profile_extrusion::ProfileCurveId;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Face},
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) fn members(
    selection: &Selection,
    members: &BTreeMap<SurfaceRegionId, (Geometry, Shape)>,
    work: &storage::Work,
) -> ConeChecked<Vec<ConicalNativeMember>> {
    let mut found = Vec::new();
    for prepared in &selection.prepared {
        let region = prepared.source.region;
        let (geometry, shape) = members
            .get(&region)
            .ok_or_else(|| missing(region, "actual geometry member"))?;
        let bindings = work
            .actual_bindings
            .get(&region)
            .ok_or_else(|| missing(region, "actual source/native bindings"))?;
        found.push(member(prepared, geometry, shape, bindings)?);
    }
    Ok(found)
}
fn member(
    prepared: &Prepared,
    geometry: &Geometry,
    shape: &Shape,
    bindings: &[(BindingKey, Shape)],
) -> ConeChecked<ConicalNativeMember> {
    let actual = faces(prepared.source.region, shape)?;
    let sides = side_bindings(prepared, bindings, &actual)?;
    let curves = curves(prepared, &retained_family(prepared)?.inventory)?;
    let caps = (0..2)
        .map(|row| cap(prepared, geometry, row, bindings, &actual, &curves))
        .collect::<ConeChecked<_>>()?;
    Ok(ConicalNativeMember {
        id: ConicalActualMemberId(prepared.source.region.0),
        shape: shape.clone(),
        sides,
        caps,
    })
}
fn retained_family(prepared: &Prepared) -> ConeChecked<&prepare::directed::DirectedFamily> {
    prepared
        .directed_family
        .as_deref()
        .ok_or_else(|| missing(prepared.source.region, "directed native family"))
}
fn side_bindings(
    prepared: &Prepared,
    bindings: &[(BindingKey, Shape)],
    actual: &[Shape],
) -> ConeChecked<Vec<(ConicalActualSideId, Face)>> {
    let family = retained_family(prepared)?;
    prepared
        .source
        .loops
        .iter()
        .flatten()
        .map(|piece| {
            let key = BindingKey {
                source: Source::Span(piece.span),
                role: HistoryRole::SideFace,
            };
            Ok((
                family.inventory.side(prepared.source.region, piece.span)?,
                face(prepared.source.region, key, bindings, actual)?,
            ))
        })
        .collect()
}
fn cap(
    prepared: &Prepared,
    geometry: &Geometry,
    row: usize,
    bindings: &[(BindingKey, Shape)],
    actual: &[Shape],
    curves: &[(ProfileCurveId, ConicalActualSideId)],
) -> ConeChecked<ConicalNativeCap> {
    let region = prepared.source.region;
    let material = geometry
        .material
        .selected_material
        .as_ref()
        .ok_or_else(|| missing(region, "issued selected actual UV material"))?;
    let role = [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace][row];
    let key = BindingKey {
        source: Source::Region(region),
        role,
    };
    Ok(ConicalNativeCap {
        id: retained_family(prepared)?.inventory.cap_pair(region)?[row],
        face: face(region, key, bindings, actual)?,
        material: material.clone(),
        curves: curves.to_vec(),
    })
}

fn curves(
    prepared: &Prepared,
    inventory: &prepare::directed::Inventory,
) -> ConeChecked<Vec<(ProfileCurveId, ConicalActualSideId)>> {
    let ids = material::source_curve_ids(prepared, prepared.fits.len())?;
    ids.into_iter()
        .map(|(span, id)| {
            Ok((
                ProfileCurveId(id.0),
                inventory.side(prepared.source.region, span)?,
            ))
        })
        .collect()
}
fn faces(region: SurfaceRegionId, shape: &Shape) -> ConeChecked<Vec<Shape>> {
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)
        .map_err(|cause| native(region, cause))?;
    let mut faces = Vec::new();
    while explorer.more() {
        faces.push(explorer.current().clone());
        explorer.next().map_err(|cause| native(region, cause))?;
    }
    Ok(faces)
}
fn face(
    region: SurfaceRegionId,
    key: BindingKey,
    bindings: &[(BindingKey, Shape)],
    faces: &[Shape],
) -> ConeChecked<Face> {
    let bound: Vec<_> = bindings
        .iter()
        .filter(|(candidate, _)| *candidate == key)
        .map(|(_, shape)| shape)
        .collect();
    if bound.len() != 1 {
        return Err(missing(region, "unique actual source face key"));
    }
    let actual: Vec<_> = faces.iter().filter(|face| face.is_same(bound[0])).collect();
    if actual.len() != 1 {
        return Err(missing(region, "unique actual native face occurrence"));
    }
    topods::face(actual[0]).map_err(|cause| native(region, cause))
}
fn native(region: SurfaceRegionId, cause: Failure) -> ConeFailure {
    storage::failure(region, None, ConeCause::Native(Box::new(cause)))
}

pub(super) fn published_members(
    family: &super::super::super::stages::OwnedFamily<Selection, Geometry>,
    work: &storage::Work,
) -> ConeChecked<Vec<ConicalNativeMember>> {
    family
        .selection
        .prepared
        .iter()
        .map(|prepared| {
            let bindings = work
                .actual_bindings
                .get(&prepared.source.region)
                .ok_or_else(|| {
                    missing(prepared.source.region, "published actual source bindings")
                })?;
            published_member(family, prepared, bindings)
        })
        .collect()
}
fn published_member(
    family: &super::super::super::stages::OwnedFamily<Selection, Geometry>,
    prepared: &Prepared,
    bindings: &[(BindingKey, Shape)],
) -> ConeChecked<ConicalNativeMember> {
    let region = prepared.source.region;
    let geometry = &family
        .members
        .get(&region)
        .ok_or_else(|| missing(region, "published actual member"))?
        .geometry;
    member(prepared, geometry, root(region, bindings)?, bindings)
}
fn root(region: SurfaceRegionId, bindings: &[(BindingKey, Shape)]) -> ConeChecked<&Shape> {
    let key = BindingKey {
        source: Source::Region(region),
        role: HistoryRole::Solid,
    };
    let roots: Vec<_> = bindings
        .iter()
        .filter(|(candidate, _)| *candidate == key)
        .map(|(_, shape)| shape)
        .collect();
    if roots.len() != 1 {
        return Err(missing(region, "published actual member root"));
    }
    Ok(roots[0])
}
#[cfg(test)]
pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn retained_members(
    family: &super::super::super::stages::OwnedFamily<Selection, Geometry>,
) -> ConeChecked<Vec<ConicalNativeMember>> {
    let (_, bindings) = pipeline::decode(&family.bytes, &family.keys).map_err(|cause| {
        ConeFailure::new(
            None,
            None,
            WrappedFailureClass::Incomplete,
            ConeCause::Native(Box::new(cause)),
        )
    })?;
    family
        .selection
        .prepared
        .iter()
        .map(|prepared| published_member(family, prepared, &bindings))
        .collect()
}
