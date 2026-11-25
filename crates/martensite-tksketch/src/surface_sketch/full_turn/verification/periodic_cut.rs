// SPDX-License-Identifier: GPL-2.0-only
//! Rebind and qualify the native split body after immutable decoding.
use super::*;
use crate::surface_sketch::full_turn::FullTurnBand;
use crate::surface_sketch::full_turn::hole::{Cutout, seam_endpoints, *};
use crate::surface_sketch::{history as primitive, validation};
use martensite_tkbrep::toptools::IndexedMapOfShape;

mod depth;
mod walls;

type Image = (PeriodicImageKey, topods::Shape);

pub(in crate::surface_sketch::full_turn) fn validate_crossed(
    prepared: &Cutout,
    band: &FullTurnBand,
    hole: &CylinderBandHole,
    crossings: &[PeriodicCrossing; 2],
    body: &topods::Shape,
    bindings: &[Image],
) -> Checked<validation::CheckedBody> {
    unique_rooted_images(body, bindings)?;
    let proof = Verification::new(prepared, band, hole, crossings, bindings);
    proof.check()?;
    let sweep = &prepared.stock.sweep;
    let report = validation::ordered_report(body, sweep)?;
    if !matches!(report.census.as_slice(), [census] if census.shells == 1 && census.loops == 10 && census.genus == Some(2))
    {
        return Err(incomplete(
            "one genus-two periodic aperture shell",
            format!("{:?}", report.census),
        ));
    }
    validation::finish(report, bindings, sweep)
}

struct Verification<'a> {
    prepared: &'a Cutout,
    band: &'a FullTurnBand,
    hole: &'a CylinderBandHole,
    crossings: &'a [PeriodicCrossing; 2],
    bindings: &'a [Image],
    direct: Bindings,
}

impl<'a> Verification<'a> {
    fn new(
        prepared: &'a Cutout,
        band: &'a FullTurnBand,
        hole: &'a CylinderBandHole,
        crossings: &'a [PeriodicCrossing; 2],
        bindings: &'a [Image],
    ) -> Self {
        let direct = bindings
            .iter()
            .filter_map(|(key, shape)| match key {
                PeriodicImageKey::Retained(key) => Some((*key, shape.clone())),
                _ => None,
            })
            .collect();
        Self {
            prepared,
            band,
            hole,
            crossings,
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

    fn aperture_edges(&self, curve: usize, depth: PeriodicDepth) -> Checked<Vec<&topods::Shape>> {
        let Some(crossing) = self
            .crossings
            .iter()
            .find(|item| item.id.curve == self.hole.boundaries[curve])
        else {
            return Ok(vec![self.retained(
                HistorySource::Curve(self.hole.boundaries[curve]),
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

    fn point(&self, depth: PeriodicDepth, axial: f64, arc: f64) -> [f64; 3] {
        let support = &self.prepared.aperture.rectangle.support;
        let radius = self.prepared.aperture.radii[depth.index()];
        let phase = support.chart_origin + arc / support.radius;
        frame::point(support, phase, radius, axial)
    }
}

fn unique_rooted_images(body: &topods::Shape, bindings: &[Image]) -> Checked<()> {
    if bindings.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return Err(incomplete(
            "unique periodic semantic keys in total order",
            "duplicate or unsorted key",
        ));
    }
    for (kind, count) in [
        (ShapeEnum::Solid, 1),
        (ShapeEnum::Face, 8),
        (ShapeEnum::Edge, 24),
        (ShapeEnum::Vertex, 16),
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
