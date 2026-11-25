// SPDX-License-Identifier: GPL-2.0-only
//! Bind split images by exact endpoint ancestry, independent of Boolean output order.
use super::*;
use crate::surface_sketch::full_turn::history::lookup;
use crate::surface_sketch::history as primitive_history;
use martensite_tkbrep::topexp;

pub(super) type SplitBindings = Vec<(PeriodicImageKey, topods::Shape)>;

pub(super) fn bind(
    proposal: images::Candidate,
    stock: &Bindings,
    aperture: &Bindings,
    band: &FullTurnBand,
    hole: &CylinderBandHole,
    crossings: &[PeriodicCrossing; 2],
) -> Checked<(topods::Shape, SplitBindings)> {
    let inputs = SplitInputs {
        proposal,
        stock,
        aperture,
        band,
        hole,
        crossings,
    };
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
    Ok((inputs.proposal.body, result))
}

struct SplitInputs<'a> {
    proposal: images::Candidate,
    stock: &'a Bindings,
    aperture: &'a Bindings,
    band: &'a FullTurnBand,
    hole: &'a CylinderBandHole,
    crossings: &'a [PeriodicCrossing; 2],
}

impl SplitInputs<'_> {
    fn retained(&self) -> Checked<Bindings> {
        let stock = self.stock.iter().filter(|(key, _)| !split_stock(*key));
        let aperture = self
            .aperture
            .iter()
            .filter(|(key, _)| !self.split_hole(*key));
        let mut direct = stock
            .map(|(key, image)| self.proposal.image(*key, image))
            .collect::<Checked<Bindings>>()?;
        for (key, image) in aperture {
            if matches!(key.source, HistorySource::Region(_)) {
                self.proposal.deleted_cap(*key, image)?;
            } else {
                direct.push(self.proposal.image(*key, image)?);
            }
        }
        root_face_orientations(&self.proposal.body, &mut direct)?;
        Ok(direct)
    }

    fn split_hole(&self, key: HistoryKey) -> bool {
        let source =
            |crossing: &PeriodicCrossing| key.source == HistorySource::Curve(crossing.id.curve);
        is_depth_edge(key.role) && self.crossings.iter().any(source)
    }

    fn bind_depth(
        &self,
        depth: PeriodicDepth,
        direct: &Bindings,
        output: &mut SplitBindings,
    ) -> Checked<()> {
        let lower = self.bind_crossing(depth, direct, output, self.crossings[0].id)?;
        let upper = self.bind_crossing(depth, direct, output, self.crossings[1].id)?;
        let roots = [lower, upper];
        let seam_key = HistoryKey {
            source: HistorySource::Region(self.band.region),
            role: primitive_history::edge_role(depth.index()),
        };
        let parent = lookup(self.stock, seam_key)?;
        let native = self.proposal.crossing_vertices::<2>(parent, seam_key)?;
        same_crossings(&native, &roots)?;
        self.bind_seams(depth, direct, output, parent, roots, seam_key)
    }

    fn bind_crossing(
        &self,
        depth: PeriodicDepth,
        direct: &Bindings,
        output: &mut SplitBindings,
        id: PeriodicCrossingId,
    ) -> Checked<topods::Shape> {
        let key = HistoryKey {
            source: HistorySource::Curve(id.curve),
            role: primitive_history::edge_role(depth.index()),
        };
        let parent = lookup(self.aperture, key)?;
        let [root] = self.proposal.crossing_vertices(parent, key)?;
        let native = self.proposal.split_edges(parent, key)?;
        let ends = self.aperture_ends(id, depth, direct)?;
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

    fn aperture_ends<'a>(
        &self,
        id: PeriodicCrossingId,
        depth: PeriodicDepth,
        direct: &'a Bindings,
    ) -> Checked<[&'a topods::Shape; 2]> {
        let index = self
            .hole
            .boundaries
            .iter()
            .position(|curve| *curve == id.curve)
            .ok_or_else(|| incomplete("authored aperture crossing curve", format!("{id:?}")))?;
        let [a, b] = [index, (index + 1) % 4].map(|position| {
            lookup(
                direct,
                HistoryKey {
                    source: HistorySource::Vertex(self.hole.corners[position]),
                    role: primitive_history::vertex_role(depth.index()),
                },
            )
        });
        Ok([a?, b?])
    }

    fn bind_seams(
        &self,
        depth: PeriodicDepth,
        direct: &Bindings,
        output: &mut SplitBindings,
        parent: &topods::Shape,
        roots: [topods::Shape; 2],
        key: HistoryKey,
    ) -> Checked<()> {
        let native = self.proposal.split_edges(parent, key)?;
        let name = |rim| {
            lookup(
                direct,
                HistoryKey {
                    source: HistorySource::Curve(rim),
                    role: primitive_history::vertex_role(depth.index()),
                },
            )
        };
        let [lower_rim, upper_rim] = self.band.rims.map(name);
        let endpoints = [[lower_rim?, &roots[0]], [&roots[1], upper_rim?]];
        let names = seam_endpoints(self.band.rims, self.crossings);
        for ([first, last], ends) in endpoints.into_iter().zip(names) {
            let segment = oriented_edge(&native, first, last)?;
            output.push((
                PeriodicImageKey::SeamSegment {
                    region: self.band.region,
                    depth,
                    ends,
                },
                segment,
            ));
        }
        Ok(())
    }
}

pub(in crate::surface_sketch::full_turn) fn seam_endpoints(
    rims: [SurfaceCurveId; 2],
    crossings: &[PeriodicCrossing; 2],
) -> [[PeriodicSeamEnd; 2]; 2] {
    [
        [
            PeriodicSeamEnd::Rim(rims[0]),
            PeriodicSeamEnd::Crossing(crossings[0].id),
        ],
        [
            PeriodicSeamEnd::Crossing(crossings[1].id),
            PeriodicSeamEnd::Rim(rims[1]),
        ],
    ]
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

fn same_crossings(reported: &[topods::Shape; 2], exact: &[topods::Shape; 2]) -> Checked<()> {
    if !exact[0].is_same(&exact[1])
        && exact
            .iter()
            .all(|root| reported.iter().any(|image| image.is_same(root)))
    {
        return Ok(());
    }
    Err(incomplete(
        "same generated crossing vertices from stock seam and aperture",
        "incompatible Boolean ancestry",
    ))
}

fn oriented_edge(
    candidates: &[topods::Shape; 2],
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
