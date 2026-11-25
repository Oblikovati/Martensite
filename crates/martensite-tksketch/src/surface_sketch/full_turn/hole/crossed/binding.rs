// SPDX-License-Identifier: GPL-2.0-only
//! Bind split images by exact endpoint ancestry, independent of Boolean output order.
use super::*;
use crate::surface_sketch::full_turn::history::lookup;
use crate::surface_sketch::history as primitive_history;
use martensite_tkbrep::topexp;
use std::collections::BTreeMap;

pub(super) type SplitBindings = Vec<(PeriodicImageKey, topods::Shape)>;

pub(super) fn bind(
    plan: &CrossedPlan,
    native: plan::NativeCut,
) -> Checked<(topods::Shape, SplitBindings)> {
    let inputs = SplitInputs { plan, native };
    let direct = inputs.retained()?;
    let mut result = Vec::new();
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        inputs.bind_depth(depth, &direct, &mut result)?;
    }
    result.extend(
        direct
            .into_iter()
            .map(|(key, shape)| (PeriodicImageKey::Retained(key), shape)),
    );
    result.sort_by_key(|(key, _)| *key);
    Ok((inputs.native.proposal.body, result))
}

struct SplitInputs<'a> {
    plan: &'a CrossedPlan,
    native: plan::NativeCut,
}

impl SplitInputs<'_> {
    fn retained(&self) -> Checked<Bindings> {
        let proposal = &self.native.proposal;
        let stock = self
            .native
            .stock
            .iter()
            .filter(|(key, _)| !split_stock(*key));
        let mut direct = stock
            .map(|(key, image)| proposal.image(*key, image))
            .collect::<Checked<Bindings>>()?;
        for (hole, raw) in self.plan.holes.iter().zip(&self.native.holes) {
            for (key, image) in raw.iter().filter(|(key, _)| !split_hole(hole, *key)) {
                retain_aperture(proposal, &mut direct, *key, image)?;
            }
        }
        root_face_orientations(&proposal.body, &mut direct)?;
        Ok(direct)
    }

    fn bind_depth(
        &self,
        depth: PeriodicDepth,
        direct: &Bindings,
        output: &mut SplitBindings,
    ) -> Checked<()> {
        let roots = self.bind_crossings(depth, direct, output)?;
        let seam_key = HistoryKey {
            source: HistorySource::Region(self.plan.band.region),
            role: primitive_history::edge_role(depth.index()),
        };
        let parent = lookup(&self.native.stock, seam_key)?;
        let native = self.native.proposal.crossing_vertices_count(
            parent,
            seam_key,
            self.plan.crossing_count(),
        )?;
        same_crossings(&native, &roots)?;
        self.bind_seams(depth, direct, output, parent, &roots, seam_key)
    }

    fn bind_crossings(
        &self,
        depth: PeriodicDepth,
        direct: &Bindings,
        output: &mut SplitBindings,
    ) -> Checked<BTreeMap<PeriodicCrossingId, topods::Shape>> {
        let mut roots = BTreeMap::new();
        for (hole, raw) in self.plan.holes.iter().zip(&self.native.holes) {
            let Some(crossings) = &hole.crossings else {
                continue;
            };
            for crossing in crossings.roots {
                let root =
                    self.bind_crossing(depth, direct, output, &hole.source, raw, crossing.id)?;
                roots.insert(crossing.id, root);
            }
        }
        Ok(roots)
    }

    fn bind_crossing(
        &self,
        depth: PeriodicDepth,
        direct: &Bindings,
        output: &mut SplitBindings,
        hole: &tool::Contour,
        raw: &Bindings,
        id: PeriodicCrossingId,
    ) -> Checked<topods::Shape> {
        let key = HistoryKey {
            source: HistorySource::Curve(id.curve),
            role: primitive_history::edge_role(depth.index()),
        };
        let parent = lookup(raw, key)?;
        let [root] = self.native.proposal.crossing_vertices(parent, key)?;
        let native = self.native.proposal.split_edges(parent, key)?;
        let ends = aperture_ends(hole, id, depth, direct)?;
        let pieces = [
            oriented_edge(&native, ends[0], &root)?,
            oriented_edge(&native, &root, ends[1])?,
        ];
        let intervals = [
            [PeriodicSourceEnd::Start, PeriodicSourceEnd::Cut(id)],
            [PeriodicSourceEnd::Cut(id), PeriodicSourceEnd::End],
        ];
        output.extend(intervals.into_iter().zip(pieces).map(|(interval, shape)| {
            (
                PeriodicImageKey::AperturePiece {
                    curve: id.curve,
                    depth,
                    interval,
                },
                shape,
            )
        }));
        output.push((
            PeriodicImageKey::CrossingVertex {
                depth,
                crossing: id,
            },
            root.clone(),
        ));
        Ok(root)
    }

    fn bind_seams(
        &self,
        depth: PeriodicDepth,
        direct: &Bindings,
        output: &mut SplitBindings,
        parent: &topods::Shape,
        roots: &BTreeMap<PeriodicCrossingId, topods::Shape>,
        key: HistoryKey,
    ) -> Checked<()> {
        let native = self.seam_images(parent, key)?;
        for rail in &self.plan.rails {
            let [first, last] = rail.ends.map(|end| seam_vertex(direct, roots, depth, end));
            let segment = oriented_edge(&native, first?, last?)?;
            output.push((
                PeriodicImageKey::SeamSegment {
                    region: self.plan.band.region,
                    depth,
                    ends: rail.ends,
                },
                segment,
            ));
        }
        Ok(())
    }
    fn seam_images(&self, parent: &topods::Shape, key: HistoryKey) -> Checked<Vec<topods::Shape>> {
        let count = self.plan.rails.len();
        if count == 1 {
            let (_, image) = self.native.proposal.image(key, parent)?;
            return Ok(vec![image]);
        }
        self.native.proposal.split_edges_count(parent, key, count)
    }
}

fn retain_aperture(
    proposal: &images::Candidate,
    direct: &mut Bindings,
    key: HistoryKey,
    image: &topods::Shape,
) -> Checked<()> {
    if matches!(key.source, HistorySource::Region(_)) {
        return proposal.deleted_cap(key, image);
    }
    direct.push(proposal.image(key, image)?);
    Ok(())
}

fn split_hole(hole: &ApprovedHole, key: HistoryKey) -> bool {
    is_depth_edge(key.role)
        && hole.crossings.as_ref().is_some_and(|crossings| {
            crossings
                .roots
                .iter()
                .any(|crossing| key.source == HistorySource::Curve(crossing.id.curve))
        })
}

fn aperture_ends<'a>(
    hole: &tool::Contour,
    id: PeriodicCrossingId,
    depth: PeriodicDepth,
    direct: &'a Bindings,
) -> Checked<[&'a topods::Shape; 2]> {
    let index = hole
        .boundaries
        .iter()
        .position(|curve| *curve == id.curve)
        .ok_or_else(|| incomplete("authored aperture crossing curve", format!("{id:?}")))?;
    let [a, b] = [index, (index + 1) % 4].map(|position| {
        lookup(
            direct,
            HistoryKey {
                source: HistorySource::Vertex(hole.corners[position]),
                role: primitive_history::vertex_role(depth.index()),
            },
        )
    });
    Ok([a?, b?])
}

fn seam_vertex<'a>(
    direct: &'a Bindings,
    roots: &'a BTreeMap<PeriodicCrossingId, topods::Shape>,
    depth: PeriodicDepth,
    end: PeriodicSeamEnd,
) -> Checked<&'a topods::Shape> {
    match end {
        PeriodicSeamEnd::Rim(curve) => lookup(
            direct,
            HistoryKey {
                source: HistorySource::Curve(curve),
                role: primitive_history::vertex_role(depth.index()),
            },
        ),
        PeriodicSeamEnd::Crossing(id) => roots
            .get(&id)
            .ok_or_else(|| incomplete("bound periodic crossing", format!("{id:?}"))),
    }
}

fn split_stock(key: HistoryKey) -> bool {
    matches!(key.source, HistorySource::Region(_)) && is_depth_edge(key.role)
}

fn is_depth_edge(role: HistoryRole) -> bool {
    matches!(
        role,
        HistoryRole::FirstDepthEdge | HistoryRole::LastDepthEdge
    )
}

fn same_crossings(
    reported: &[topods::Shape],
    exact: &BTreeMap<PeriodicCrossingId, topods::Shape>,
) -> Checked<()> {
    let all_bound = reported
        .iter()
        .all(|image| exact.values().any(|root| image.is_same(root)));
    if reported.len() == exact.len() && all_bound {
        return Ok(());
    }
    Err(incomplete(
        "same generated crossing vertices from stock seam and apertures",
        "incompatible Boolean ancestry",
    ))
}

fn oriented_edge(
    candidates: &[topods::Shape],
    first: &topods::Shape,
    last: &topods::Shape,
) -> Checked<topods::Shape> {
    let mut result = None;
    for image in candidates {
        let edge = topods::edge(image).map_err(|e| native("periodic split edge", e))?;
        let (a, b) = topexp::vertices(&edge, true);
        if first.is_same(&a) && last.is_same(&b) {
            result = single(result, image.clone())?;
        } else if first.is_same(&b) && last.is_same(&a) {
            result = single(result, image.reversed())?;
        }
    }
    result.ok_or_else(|| {
        incomplete(
            "periodic split edge with exactly bound ends",
            "no matching native image",
        )
    })
}

fn single(prior: Option<topods::Shape>, next: topods::Shape) -> Checked<Option<topods::Shape>> {
    if prior.is_some() {
        return Err(incomplete(
            "one split image for bound endpoints",
            "multiple matching native images",
        ));
    }
    Ok(Some(next))
}
