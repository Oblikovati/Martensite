// SPDX-License-Identifier: GPL-2.0-only
//! Rebind and qualify the native split body after immutable decoding.
use super::*;
use crate::surface_sketch::full_turn::FullTurnBand;
use crate::surface_sketch::full_turn::hole::{ApprovedHole, CrossedPlan, *};
use crate::surface_sketch::{history as primitive, validation};
use martensite_tkbo::body_check::ValidationReport;
use martensite_tkbrep::toptools::IndexedMapOfShape;

mod depth;
mod walls;

type Image = (PeriodicImageKey, topods::Shape);

pub(in crate::surface_sketch::full_turn) fn validate_crossed(
    prepared: &CrossedPlan,
    body: &topods::Shape,
    bindings: &[Image],
) -> Checked<validation::CheckedBody> {
    let count = prepared.holes.len();
    let cylinder = prepared.stock.cylinder;
    unique_rooted_images(body, bindings, count, cylinder)?;
    let proof = Verification::new(prepared, bindings);
    proof.check()?;
    let sweep = &prepared.stock.sweep;
    let report = validation::ordered_report(body, sweep)?;
    check_census(&report, count, cylinder)?;
    validation::finish(report, bindings, sweep)
}

fn check_census(report: &ValidationReport, count: usize, cylinder: bool) -> Checked<()> {
    let genus = i64::try_from(count + 1)
        .map_err(|e| incomplete("representable periodic aperture genus", e.to_string()))?;
    let loops = 4 + 2 * usize::from(cylinder) + 4 * count;
    if matches!(report.census.as_slice(), [census] if census.shells == 1 && census.loops == loops && census.genus == Some(genus))
    {
        return Ok(());
    }
    Err(incomplete(
        "one shell with one handle per aperture and one for the band",
        format!("{:?}", report.census),
    ))
}

struct Verification<'a> {
    prepared: &'a CrossedPlan,
    band: &'a FullTurnBand,
    bindings: &'a [Image],
    direct: Bindings,
}

impl<'a> Verification<'a> {
    fn new(prepared: &'a CrossedPlan, bindings: &'a [Image]) -> Self {
        let direct = bindings
            .iter()
            .filter_map(|(key, shape)| match key {
                PeriodicImageKey::Retained(key) => Some((*key, shape.clone())),
                _ => None,
            })
            .collect();
        Self {
            prepared,
            band: &prepared.band,
            bindings,
            direct,
        }
    }

    fn check(&self) -> Checked<()> {
        let stock = &self.prepared.stock;
        for index in [0, 2] {
            verify_face(stock, self.band.rims, &self.direct, index, None)?;
        }
        for index in 0..4 {
            let rim = rim_keys(self.band.rims, stock.sweep.first_depth, index)[0];
            verify_rim(stock, &self.direct, rim, index)?;
        }
        walls::check(self)?;
        for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
            depth::check(self, depth)?;
        }
        Ok(())
    }

    fn resolve(&self, key: PeriodicImageKey) -> Checked<&topods::Shape> {
        let i = self
            .bindings
            .binary_search_by_key(&key, |(candidate, _)| *candidate)
            .map_err(|_| incomplete("named periodic image", format!("missing {key:?}")))?;
        Ok(&self.bindings[i].1)
    }

    fn retained(&self, source: HistorySource, role: HistoryRole) -> Checked<&topods::Shape> {
        lookup(&self.direct, HistoryKey { source, role })
    }

    fn aperture_edges(
        &self,
        hole: &ApprovedHole,
        index: usize,
        depth: PeriodicDepth,
    ) -> Checked<Vec<&topods::Shape>> {
        let curve = hole.source.boundaries[index];
        let Some(crossing) = hole.crossings.iter().find(|item| item.id.curve == curve) else {
            return Ok(vec![self.retained(
                HistorySource::Curve(curve),
                primitive::edge_role(depth.index()),
            )?]);
        };
        let [before, after] = self.piece_keys(crossing.id, depth);
        Ok(vec![self.resolve(before)?, self.resolve(after)?])
    }

    fn piece_keys(
        &self,
        crossing: PeriodicCrossingId,
        depth: PeriodicDepth,
    ) -> [PeriodicImageKey; 2] {
        let root = PeriodicSourceEnd::Cut(crossing);
        [
            [PeriodicSourceEnd::Start, root],
            [root, PeriodicSourceEnd::End],
        ]
        .map(|interval| PeriodicImageKey::AperturePiece {
            curve: crossing.curve,
            depth,
            interval,
        })
    }

    fn seam_point(&self, depth: PeriodicDepth, meridian: f64) -> [f64; 3] {
        let sweep = &self.prepared.stock.sweep;
        let index = usize::from(depth.index() != sweep.first_depth);
        let [min, max] = self.prepared.bounds;
        let t = (meridian - min) / (max - min);
        let [start, finish] = [sweep.points[index], sweep.points[3 - index]];
        // Both supports are affine along a fixed angle and normal depth.
        std::array::from_fn(|i| start[i] + t * (finish[i] - start[i]))
    }
}

fn unique_rooted_images(
    body: &topods::Shape,
    bindings: &[Image],
    count: usize,
    cylinder: bool,
) -> Checked<()> {
    if bindings.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return Err(incomplete(
            "unique periodic semantic keys in total order",
            "duplicate or unsorted key",
        ));
    }
    for (kind, count) in [
        (ShapeEnum::Solid, 1),
        (ShapeEnum::Face, 4 + 4 * count),
        (ShapeEnum::Edge, 6 + 2 * usize::from(!cylinder) + 18 * count),
        (ShapeEnum::Vertex, 4 + 12 * count),
    ] {
        unique_kind(body, bindings, kind, count)?;
    }
    Ok(())
}

fn unique_kind(
    body: &topods::Shape,
    bindings: &[Image],
    kind: ShapeEnum,
    count: usize,
) -> Checked<()> {
    let mut actual = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(body, kind, &mut actual)
        .map_err(|e| native("periodic result members", e))?;
    let mut named = IndexedMapOfShape::default();
    for (key, image) in bindings.iter().filter(|(key, _)| image_kind(*key) == kind) {
        let index = actual.find_index(image);
        if index == 0 || named.find_index(image) != 0 {
            return Err(incomplete(
                "one rooted member per periodic semantic key",
                format!("{key:?}"),
            ));
        }
        named.add(image.clone());
    }
    if [actual.extent(), named.extent()] != [count, count] {
        return Err(incomplete(
            "complete periodic shape membership",
            format!(
                "{kind:?}: {} native, {} named, expected {count}",
                actual.extent(),
                named.extent()
            ),
        ));
    }
    Ok(())
}

fn image_kind(key: PeriodicImageKey) -> ShapeEnum {
    match key {
        PeriodicImageKey::Retained(key) => primitive::role_kind(key.role),
        PeriodicImageKey::AperturePiece { .. } | PeriodicImageKey::SeamSegment { .. } => {
            ShapeEnum::Edge
        }
        PeriodicImageKey::CrossingVertex { .. } => ShapeEnum::Vertex,
    }
}
