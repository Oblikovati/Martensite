// SPDX-License-Identifier: GPL-2.0-only
//! Compose semantic bindings through a single non-destructive difference.
use super::*;
use martensite_tkbo::{bop_algo::Operation, brep_algo_api::BRepAlgoAPIBooleanOperation};
use martensite_tkbrep::{topexp, topods::ShapeIterator, toptools::IndexedMapOfShape};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use std::{collections::BTreeMap, fmt::Debug};

pub(super) struct Candidate {
    pub(super) body: topods::Shape,
    cut: BRepAlgoAPIBooleanOperation,
    members: BTreeMap<ShapeEnum, IndexedMapOfShape>,
}

pub(super) fn subtract(stock: &topods::Shape, hole: &topods::Shape) -> Checked<Candidate> {
    subtract_all(stock, [hole.clone()])
}

pub(super) fn subtract_all(
    stock: &topods::Shape,
    holes: impl IntoIterator<Item = topods::Shape>,
) -> Checked<Candidate> {
    let mut cut = BRepAlgoAPIBooleanOperation::new();
    cut.set_operation(Operation::Cut);
    cut.set_non_destructive(true);
    cut.set_arguments(&List::from_iter([stock.clone()]));
    cut.set_tools(&List::from_iter(holes));
    cut.build()
        .map_err(|e| native("subtract admitted aperture", e))?;
    if cut.has_errors() {
        return Err(incomplete(
            "completed aperture subtraction",
            "native Boolean errors",
        ));
    }
    let body = one_solid(cut.shape())?;
    let members = rooted_members(&body)?;
    Ok(Candidate { body, cut, members })
}

fn rooted_members(body: &topods::Shape) -> Checked<BTreeMap<ShapeEnum, IndexedMapOfShape>> {
    let mut all = BTreeMap::new();
    for kind in [
        ShapeEnum::Solid,
        ShapeEnum::Face,
        ShapeEnum::Edge,
        ShapeEnum::Vertex,
    ] {
        let mut map = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(body, kind, &mut map)
            .map_err(|e| native("index aperture result images", e))?;
        all.insert(kind, map);
    }
    Ok(all)
}

pub(super) fn bind(
    candidate: Candidate,
    stock: Bindings,
    hole: Bindings,
) -> Checked<(topods::Shape, Bindings)> {
    let mut result = stock
        .into_iter()
        .map(|(key, shape)| candidate.image(key, &shape))
        .collect::<Checked<Bindings>>()?;
    for (key, shape) in hole {
        if matches!(key.source, HistorySource::Region(_)) {
            candidate.deleted_cap(key, &shape)?;
        } else {
            result.push(candidate.image(key, &shape)?);
        }
    }
    result.sort_by_key(|(key, _)| *key);
    root_face_orientations(&candidate.body, &mut result)?;
    Ok((candidate.body, result))
}

impl Candidate {
    pub(super) fn image(
        &self,
        key: HistoryKey,
        parent: &topods::Shape,
    ) -> Checked<(HistoryKey, topods::Shape)> {
        self.image_by_role(key, key.role, parent)
            .map(|image| (key, image))
    }

    pub(super) fn image_by_role(
        &self,
        key: impl Debug + Copy,
        role: HistoryRole,
        parent: &topods::Shape,
    ) -> Checked<topods::Shape> {
        let members = self
            .members
            .get(&role_kind(role))
            .ok_or_else(|| incomplete("indexed aperture result kind", format!("{key:?}")))?;
        let modified = self
            .cut
            .modified(parent)
            .map_err(|e| native("aperture modified history", e))?;
        let images = std::iter::once(parent)
            .chain(&modified)
            .map(|image| members.find_index(image));
        let one = unique_member(images, key)?;
        self.require_unsplit(key, parent)?;
        let image = members
            .find_key(one)
            .map_err(|e| native("bound aperture image", e))?;
        Ok(image.clone())
    }

    fn require_unsplit(&self, key: impl Debug, parent: &topods::Shape) -> Checked<()> {
        let generated = self
            .cut
            .generated(parent)
            .map_err(|e| native("aperture generated history", e))?;
        let unexpected = generated
            .iter()
            .any(|image| self.members.values().any(|map| map.find_index(image) != 0));
        if unexpected {
            return Err(incomplete(
                "unsplit source at a clear native seam",
                format!("{key:?}"),
            ));
        }
        Ok(())
    }

    pub(super) fn deleted_cap(&self, key: impl Debug, image: &topods::Shape) -> Checked<()> {
        if self
            .cut
            .is_deleted(image)
            .map_err(|e| native("aperture deleted history", e))?
        {
            return Ok(());
        }
        Err(incomplete(
            "consumed aperture tool cap or solid",
            format!("{key:?}"),
        ))
    }

    pub(super) fn split_edges(
        &self,
        parent: &topods::Shape,
        key: impl Debug,
    ) -> Checked<[topods::Shape; 2]> {
        fixed(self.split_edges_count(parent, key, 2)?)
    }

    pub(super) fn split_edges_count(
        &self,
        parent: &topods::Shape,
        key: impl Debug,
        count: usize,
    ) -> Checked<Vec<topods::Shape>> {
        let modified = self
            .cut
            .modified(parent)
            .map_err(|e| native("periodic split edges", e))?;
        self.rooted_images(modified, ShapeEnum::Edge, key, count)
    }

    pub(super) fn crossing_vertices<const N: usize>(
        &self,
        parent: &topods::Shape,
        key: impl Debug,
    ) -> Checked<[topods::Shape; N]> {
        fixed(self.crossing_vertices_count(parent, key, N)?)
    }

    pub(super) fn crossing_vertices_count(
        &self,
        parent: &topods::Shape,
        key: impl Debug,
        count: usize,
    ) -> Checked<Vec<topods::Shape>> {
        let generated = self
            .cut
            .generated(parent)
            .map_err(|e| native("periodic split vertices", e))?;
        self.rooted_images(generated, ShapeEnum::Vertex, key, count)
    }

    fn rooted_images(
        &self,
        images: Vec<topods::Shape>,
        kind: ShapeEnum,
        key: impl Debug,
        count: usize,
    ) -> Checked<Vec<topods::Shape>> {
        let members = self.members.get(&kind).ok_or_else(|| {
            incomplete("indexed periodic split kind", format!("{key:?}: {kind:?}"))
        })?;
        let mut report = IndexedMapOfShape::default();
        for image in images {
            if members.find_index(&image) != 0 {
                report.add(image);
            }
        }
        if report.extent() != count {
            return Err(incomplete(
                "complete periodic split images",
                format!(
                    "{key:?}: {} {kind:?} images, expected {count}",
                    report.extent()
                ),
            ));
        }
        (1..=count)
            .map(|i| {
                report
                    .find_key(i)
                    .cloned()
                    .map_err(|e| native("bound periodic split image", e))
            })
            .collect()
    }
}

fn fixed<const N: usize>(images: Vec<topods::Shape>) -> Checked<[topods::Shape; N]> {
    images.try_into().map_err(|items: Vec<_>| {
        incomplete(
            "complete periodic split history",
            format!("{} images, expected {N}", items.len()),
        )
    })
}

fn unique_member(indices: impl Iterator<Item = usize>, key: impl Debug) -> Checked<usize> {
    let mut one = 0;
    for image in indices.filter(|&index| index != 0) {
        if one != 0 && one != image {
            return Err(incomplete(
                "unique image of an admitted unsplit aperture source",
                format!("{key:?}"),
            ));
        }
        one = image;
    }
    if one != 0 {
        return Ok(one);
    }
    Err(incomplete(
        "retained source image after aperture subtraction",
        format!("{key:?}"),
    ))
}

fn one_solid(proposal: &topods::Shape) -> Checked<topods::Shape> {
    let kind = proposal
        .shape_type()
        .map_err(|e| native("aperture output kind", e))?;
    if kind == ShapeEnum::Solid {
        return Ok(proposal.clone());
    }
    let mut members = ShapeIterator::new(proposal, true, true);
    let first = members
        .value()
        .map_err(|e| native("aperture result member", e))?
        .clone();
    members.next();
    let member_kind = first
        .shape_type()
        .map_err(|e| native("aperture member kind", e))?;
    if kind != ShapeEnum::Compound || members.more() || member_kind != ShapeEnum::Solid {
        return Err(incomplete(
            "one solid from an aperture cut",
            format!("{kind:?} with {member_kind:?} member"),
        ));
    }
    Ok(first)
}
