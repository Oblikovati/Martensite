//! Shared corners collect both incident curve parents before a single record is emitted.
use super::*;
pub(super) fn region(request: &Request, records: &mut Records) -> Checked<()> {
    region_images(request, records)?;
    let mut corners: BTreeMap<CornerSource, Parents> = BTreeMap::new();
    for piece in request.loops.iter().flatten() {
        span_records(request.region, piece, records)?;
        corner_parents(piece, &mut corners);
    }
    for (corner, parents) in corners {
        corner_records(request.region, corner, &parents, records)?;
    }
    Ok(())
}
fn all_parents(request: &Request) -> Parents {
    let mut parents = BTreeSet::from([HistorySource::Region(request.region)]);
    for piece in request.loops.iter().flatten() {
        parents.insert(HistorySource::Curve(piece.span.curve));
        authored_corner(piece.start, &mut parents);
        authored_corner(piece.end, &mut parents);
    }
    parents
}
fn region_images(request: &Request, records: &mut Records) -> Checked<()> {
    let images = Images {
        owner: request.region,
        source: Source::Region(request.region),
        parents: all_parents(request),
    };
    images.emit(
        records,
        &[
            HistoryRole::Solid,
            HistoryRole::FirstDepthFace,
            HistoryRole::LastDepthFace,
        ],
        HistoryRelation::Derived,
    )
}
fn span_records(
    owner: SurfaceRegionId,
    piece: &AuthoredPiece,
    records: &mut Records,
) -> Checked<()> {
    let images = span_images(owner, piece);
    let relation = if piece.span.level == 0 {
        HistoryRelation::Derived
    } else {
        HistoryRelation::Split
    };
    images.emit(
        records,
        &[HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge],
        relation,
    )?;
    images.emit(records, &[HistoryRole::SideFace], HistoryRelation::Derived)
}
fn span_images(owner: SurfaceRegionId, piece: &AuthoredPiece) -> Images {
    let mut parents = BTreeSet::from([
        HistorySource::Region(owner),
        HistorySource::Curve(piece.span.curve),
    ]);
    authored_corner(piece.start, &mut parents);
    authored_corner(piece.end, &mut parents);
    Images {
        owner,
        source: Source::Span(piece.span),
        parents,
    }
}
fn corner_parents(piece: &AuthoredPiece, corners: &mut BTreeMap<CornerSource, Parents>) {
    for corner in [piece.start, piece.end] {
        let parents = corners.entry(corner).or_default();
        parents.insert(HistorySource::Curve(piece.span.curve));
        authored_corner(corner, parents);
    }
}
fn authored_corner(corner: CornerSource, parents: &mut Parents) {
    if let CornerSource::Authored(vertex) = corner {
        parents.insert(HistorySource::Vertex(vertex));
    }
}
fn corner_records(
    owner: SurfaceRegionId,
    corner: CornerSource,
    parents: &Parents,
    records: &mut Records,
) -> Checked<()> {
    let images = corner_images(owner, corner, parents);
    images.emit(
        records,
        &[
            HistoryRole::FirstDepthVertex,
            HistoryRole::LastDepthVertex,
            HistoryRole::NormalEdge,
        ],
        corner_relation(corner),
    )
}
fn corner_images(owner: SurfaceRegionId, corner: CornerSource, parents: &Parents) -> Images {
    let mut parents = parents.clone();
    parents.insert(HistorySource::Region(owner));
    Images {
        owner,
        source: Source::Corner(corner),
        parents,
    }
}
fn corner_relation(corner: CornerSource) -> HistoryRelation {
    match corner {
        CornerSource::Authored(_) => HistoryRelation::Derived,
        CornerSource::Split { .. } => HistoryRelation::Created,
    }
}
struct Images {
    owner: SurfaceRegionId,
    source: Source,
    parents: Parents,
}
impl Images {
    fn record(&self, role: HistoryRole, relation: HistoryRelation) -> HistoryRecord {
        HistoryRecord {
            result: BindingKey {
                source: self.source,
                role,
            },
            owner: self.owner,
            relation,
            parents: self.parents.iter().copied().collect(),
        }
    }

    fn emit(
        &self,
        records: &mut Records,
        roles: &[HistoryRole],
        relation: HistoryRelation,
    ) -> Checked<()> {
        for role in roles {
            insert(records, self.record(*role, relation))?;
        }
        Ok(())
    }
}
