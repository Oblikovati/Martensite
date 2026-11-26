// SPDX-License-Identifier: GPL-2.0-only
//! Resolve each native result entity through its typed source and role before proof.
use super::{
    Failure, Stage, poll,
    receipt::{MixedSpatialPrismHistory, MixedSpatialPrismSource},
    source::Source,
};
use martensite_tkbo::profile_extrusion::{
    HistoryRecord, HistoryRole, HistorySource, OperationResult, ProfileCurveId, ProfileVertexId,
    RegionId, ResultEntity,
};
use martensite_tkbrep::topods::{Edge, Face, Vertex};
use martensite_tksketch3d::PointId;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub(super) struct Image {
    pub history: MixedSpatialPrismHistory,
    pub result: ResultEntity,
}

pub(super) struct Images<'a> {
    pub caps: [&'a Face; 2],
    pub curves: BTreeMap<ProfileCurveId, CurveImages<'a>>,
    pub points: BTreeMap<PointId, PointImages<'a>>,
    pub entries: Vec<Image>,
}

pub(super) struct CurveImages<'a> {
    pub caps: [&'a Edge; 2],
    pub side: &'a Face,
}

pub(super) struct PointImages<'a> {
    pub caps: [&'a Vertex; 2],
    pub lateral: &'a Edge,
}

pub(super) fn bind<'a>(
    swept: &'a OperationResult,
    source: &Source,
    region: RegionId,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Images<'a>, Failure> {
    let mut pending = PendingImages::new(swept, cancelled)?;
    let caps = pending.caps(region)?;
    let mut curves = BTreeMap::new();
    for (&curve, entry) in &source.curves {
        poll(cancelled, Stage::History)?;
        curves.insert(
            curve,
            pending.curve(curve, MixedSpatialPrismSource::Curve(entry.used))?,
        );
    }
    let mut points = BTreeMap::new();
    for &point in source.points.keys() {
        poll(cancelled, Stage::History)?;
        points.insert(point, pending.point(point)?);
    }
    let entries = pending.complete()?;
    Ok(Images {
        caps,
        curves,
        points,
        entries,
    })
}

struct PendingImages<'a> {
    swept: &'a OperationResult,
    roles: BTreeMap<(HistorySource, HistoryRole), HistoryRecord>,
    entries: Vec<Image>,
}

impl<'a> PendingImages<'a> {
    fn new(
        swept: &'a OperationResult,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, Failure> {
        let mut roles = BTreeMap::new();
        let mut entities = BTreeSet::new();
        for record in &swept.history().records {
            poll(cancelled, Stage::History)?;
            if roles
                .insert((record.source, record.role), *record)
                .is_some()
                || !entities.insert(record.result)
            {
                return Err(unbound(
                    "unique native role per source and parent per image",
                ));
            }
        }
        let count = swept.faces().len() + swept.edges().len() + swept.vertices().len();
        if roles.len() != count || !swept.history().deleted.is_empty() {
            return Err(unbound(
                "every retained native entity without deleted source entities",
            ));
        }
        Ok(Self {
            swept,
            roles,
            entries: Vec::with_capacity(count),
        })
    }

    fn caps(&mut self, region: RegionId) -> Result<[&'a Face; 2], Failure> {
        let native = HistorySource::Region(region);
        let source = MixedSpatialPrismSource::Boundary;
        Ok([
            self.face(native, source, HistoryRole::StartCap)?,
            self.face(native, source, HistoryRole::EndCap)?,
        ])
    }

    fn curve(
        &mut self,
        curve: ProfileCurveId,
        source: MixedSpatialPrismSource,
    ) -> Result<CurveImages<'a>, Failure> {
        let native = HistorySource::Curve(curve);
        let side = self.face(native, source, HistoryRole::SideFace)?;
        let caps = [
            self.edge(native, source, HistoryRole::StartEdge)?,
            self.edge(native, source, HistoryRole::EndEdge)?,
        ];
        Ok(CurveImages { caps, side })
    }

    fn point(&mut self, point: PointId) -> Result<PointImages<'a>, Failure> {
        let native = HistorySource::Vertex(ProfileVertexId(point.get().get()));
        let source = MixedSpatialPrismSource::Point(point);
        let lateral = self.edge(native, source, HistoryRole::LateralEdge)?;
        let caps = [
            self.vertex(native, source, HistoryRole::StartVertex)?,
            self.vertex(native, source, HistoryRole::EndVertex)?,
        ];
        Ok(PointImages { caps, lateral })
    }

    fn face(
        &mut self,
        native: HistorySource,
        source: MixedSpatialPrismSource,
        role: HistoryRole,
    ) -> Result<&'a Face, Failure> {
        match self.take(native, source, role)? {
            ResultEntity::Face(index) => self.swept.faces().get(index),
            _ => None,
        }
        .ok_or_else(|| unbound("a retained face for its authored role"))
    }

    fn edge(
        &mut self,
        native: HistorySource,
        source: MixedSpatialPrismSource,
        role: HistoryRole,
    ) -> Result<&'a Edge, Failure> {
        match self.take(native, source, role)? {
            ResultEntity::Edge(index) => self.swept.edges().get(index),
            _ => None,
        }
        .ok_or_else(|| unbound("a retained edge for its authored role"))
    }

    fn vertex(
        &mut self,
        native: HistorySource,
        source: MixedSpatialPrismSource,
        role: HistoryRole,
    ) -> Result<&'a Vertex, Failure> {
        match self.take(native, source, role)? {
            ResultEntity::Vertex(index) => self.swept.vertices().get(index),
            _ => None,
        }
        .ok_or_else(|| unbound("a retained vertex for its authored role"))
    }

    fn take(
        &mut self,
        native: HistorySource,
        source: MixedSpatialPrismSource,
        role: HistoryRole,
    ) -> Result<ResultEntity, Failure> {
        let HistoryRecord {
            result, relation, ..
        } = self
            .roles
            .remove(&(native, role))
            .ok_or_else(|| unbound("one retained native image for each authored source role"))?;
        self.entries.push(Image {
            history: MixedSpatialPrismHistory {
                source,
                role,
                relation,
            },
            result,
        });
        Ok(result)
    }

    fn complete(self) -> Result<Vec<Image>, Failure> {
        if !self.roles.is_empty() {
            return Err(unbound("exactly the complete authored source roles"));
        }
        Ok(self.entries)
    }
}

fn unbound(expected: &'static str) -> Failure {
    Failure::History(expected)
}
