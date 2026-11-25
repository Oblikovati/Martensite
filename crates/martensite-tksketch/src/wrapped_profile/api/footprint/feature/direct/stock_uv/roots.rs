use super::*;

#[derive(Clone, Copy)]
pub(super) struct Weld {
    pub(super) face: CylinderBodyRef,
    pub(super) sense: Orientation,
    pub(super) region: SurfaceRegionId,
    pub(super) gap: f64,
}

pub(super) struct Roots {
    pub(super) band: CylinderBodyRef,
    pub(super) chart: stock::StockChart,
    pub(super) cap: Option<f64>,
    pub(super) faces: BTreeSet<CylinderBodyRef>,
    pub(super) welds: BTreeMap<CylinderBodyRef, Weld>,
}

impl Roots {
    pub(super) fn capture(
        attempt: Attempt,
        input: &Input<'_>,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<Self> {
        poll(cancelled)?;
        input.resolution.admit().map_err(|(field, value)| {
            attempt.fail(
                STAGE,
                CylinderFeatureCause::Resolution(martensite_tkbo::body_check::ResolutionRejected {
                    field,
                    value,
                }),
            )
        })?;
        let [u, v] = input.chart.sheet;
        if input.chart.sew.to_bits() != input.resolution.sew.to_bits()
            || [u, v]
                .iter()
                .any(|r| r.iter().any(|v| !v.is_finite()) || r[0] >= r[1])
        {
            return Err(Refusal::Limit(input.band));
        }
        let mut regions = authored_regions(input.records, cancelled)?;
        let (faces, edges) =
            history_roots(attempt, input.band, &mut regions, input.history, cancelled)?;
        if input.band.kind != ShapeEnum::Face || !faces.contains(&input.band) {
            return Err(Refusal::Role(input.band));
        }
        if let Some((&key, _)) = regions.first_key_value() {
            return Err(Refusal::Authored(key));
        }
        let mut welds = BTreeMap::new();
        for weld in input.welded {
            poll(cancelled)?;
            let edge = weld.edge();
            let face = weld.face();
            let region = *edges.get(&edge).ok_or(Refusal::Incidence(edge))?;
            if edge.kind != ShapeEnum::Edge || !faces.contains(&face) {
                return Err(Refusal::Role(face));
            }
            let record = Weld {
                face,
                sense: weld.direction(),
                region,
                gap: weld.gap_upper(),
            };
            if welds.insert(edge, record).is_some() {
                return Err(Refusal::Incidence(edge));
            }
        }
        if welds.is_empty() || welds.len() != edges.len() {
            return Err(Refusal::Incidence(input.band));
        }
        let cap = match input.resolution.cap {
            martensite_tkbo::profile_extrusion::ToleranceCap::Refuse(cap) => Some(cap),
            _ => None,
        };
        Ok(Self {
            band: input.band,
            chart: input.chart,
            cap,
            faces,
            welds,
        })
    }
}

fn authored_regions(
    records: &[WrappedHistoryRecord],
    cancelled: &mut CancelPoll<'_>,
) -> Checked<BTreeMap<WrappedHistoryKey, SurfaceRegionId>> {
    let mut regions = BTreeMap::new();
    for record in records {
        poll(cancelled)?;
        if record.result.role == HistoryRole::FirstDepthEdge
            && regions.insert(record.result, record.owner).is_some()
        {
            return Err(Refusal::Authored(record.result));
        }
        if regions.len() > proof::PIECE_LIMIT {
            return Err(Refusal::Authored(record.result));
        }
    }
    Ok(regions)
}

type HistoryRoots = (
    BTreeSet<CylinderBodyRef>,
    BTreeMap<CylinderBodyRef, SurfaceRegionId>,
);

fn history_roots(
    attempt: Attempt,
    band: CylinderBodyRef,
    authored: &mut BTreeMap<WrappedHistoryKey, SurfaceRegionId>,
    history: &CylinderFeatureHistory,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<HistoryRoots> {
    let mut faces = BTreeSet::new();
    let mut edges = BTreeMap::new();
    for source in &history.removed {
        poll(cancelled)?;
        match *source {
            CylinderFeatureSource::SupportFace(id) if id == attempt.attachment => {
                return Err(Refusal::Role(band));
            }
            CylinderFeatureSource::Tool(key) if key.role == HistoryRole::FirstDepthEdge => {
                return Err(Refusal::Authored(key));
            }
            _ => {}
        }
    }
    for (&id, parents) in &history.images {
        poll(cancelled)?;
        for parent in parents {
            match parent.source {
                CylinderFeatureSource::SupportFace(attachment) => {
                    if parents.len() != 1
                        || attachment != attempt.attachment
                        || id.kind != ShapeEnum::Face
                        || parent.relation != CylinderFeatureRelation::Modified
                    {
                        return Err(Refusal::Role(id));
                    }
                    faces.insert(id);
                }
                CylinderFeatureSource::Tool(key) if key.role == HistoryRole::FirstDepthEdge => {
                    if parents.len() != 1
                        || id.kind != ShapeEnum::Edge
                        || parent.relation != CylinderFeatureRelation::Unchanged
                    {
                        return Err(Refusal::Authored(key));
                    }
                    let owner = authored.remove(&key).ok_or(Refusal::Authored(key))?;
                    edges.insert(id, owner);
                }
                _ => {}
            }
        }
    }
    Ok((faces, edges))
}
