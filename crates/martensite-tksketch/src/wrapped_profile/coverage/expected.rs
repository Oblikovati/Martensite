//! Expected incidence uses authored span/corner labels, never native traversal positions.
use super::*;
pub(super) fn region(request: &DevelopedRegionRequest, role: HistoryRole) -> BindingKey {
    BindingKey {
        source: Source::Region(request.region),
        role,
    }
}
pub(super) fn span(span: CurveSpan, role: HistoryRole) -> BindingKey {
    BindingKey {
        source: Source::Span(span),
        role,
    }
}
pub(super) fn corner(corner: CornerSource, role: HistoryRole) -> BindingKey {
    BindingKey {
        source: Source::Corner(corner),
        role,
    }
}
pub(super) fn definitions(
    request: &DevelopedRegionRequest,
) -> Checked<BTreeMap<BindingKey, ShapeEnum>> {
    let mut result = BTreeMap::new();
    insert(
        &mut result,
        region(request, HistoryRole::Solid),
        ShapeEnum::Solid,
    )?;
    for role in [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace] {
        insert(&mut result, region(request, role), ShapeEnum::Face)?;
    }
    for piece in request.loops.iter().flatten() {
        piece_definitions(piece, &mut result)?;
    }
    Ok(result)
}
fn piece_definitions(
    piece: &AuthoredPiece,
    result: &mut BTreeMap<BindingKey, ShapeEnum>,
) -> Checked<()> {
    for (role, kind) in [
        (HistoryRole::FirstDepthEdge, ShapeEnum::Edge),
        (HistoryRole::LastDepthEdge, ShapeEnum::Edge),
        (HistoryRole::SideFace, ShapeEnum::Face),
    ] {
        insert(result, span(piece.span, role), kind)?;
    }
    for (role, kind) in [
        (HistoryRole::FirstDepthVertex, ShapeEnum::Vertex),
        (HistoryRole::LastDepthVertex, ShapeEnum::Vertex),
        (HistoryRole::NormalEdge, ShapeEnum::Edge),
    ] {
        insert(result, corner(piece.start, role), kind)?;
    }
    Ok(())
}
fn insert(
    result: &mut BTreeMap<BindingKey, ShapeEnum>,
    key: BindingKey,
    kind: ShapeEnum,
) -> Checked<()> {
    if result.insert(key, kind).is_some() {
        return Err(fail(
            key,
            CoverageCause::DuplicateBinding {
                key,
                role: "declared",
            },
        ));
    }
    Ok(())
}
pub(super) fn faces(
    request: &DevelopedRegionRequest,
    handedness: DevelopedMapHandedness,
) -> Vec<BoundUse> {
    let mut uses = vec![
        BoundUse::new(
            region(request, HistoryRole::FirstDepthFace),
            Orientation::Reversed,
        ),
        BoundUse::forward(region(request, HistoryRole::LastDepthFace)),
    ];
    uses.extend(request.loops.iter().flatten().map(|p| {
        BoundUse::new(
            span(p.span, HistoryRole::SideFace),
            handedness.boundary_orientation(),
        )
    }));
    uses
}
pub(super) fn side(piece: &AuthoredPiece) -> Vec<BoundUse> {
    vec![
        BoundUse::forward(span(piece.span, HistoryRole::FirstDepthEdge)),
        BoundUse::forward(corner(piece.end, HistoryRole::NormalEdge)),
        BoundUse::new(
            span(piece.span, HistoryRole::LastDepthEdge),
            Orientation::Reversed,
        ),
        BoundUse::new(
            corner(piece.start, HistoryRole::NormalEdge),
            Orientation::Reversed,
        ),
    ]
}
pub(super) fn cap(
    request: &DevelopedRegionRequest,
    role: HistoryRole,
    handedness: DevelopedMapHandedness,
) -> Vec<Vec<BoundUse>> {
    request
        .loops
        .iter()
        .map(|contour| cap_contour(contour, role, handedness))
        .collect()
}
fn cap_contour(
    contour: &[AuthoredPiece],
    role: HistoryRole,
    handedness: DevelopedMapHandedness,
) -> Vec<BoundUse> {
    if handedness.reverses() {
        return contour
            .iter()
            .rev()
            .map(|p| BoundUse::new(span(p.span, role), Orientation::Reversed))
            .collect();
    }
    contour
        .iter()
        .map(|p| BoundUse::forward(span(p.span, role)))
        .collect()
}

pub(super) fn ends(piece: &AuthoredPiece, role: HistoryRole) -> Vec<BoundUse> {
    vec![
        BoundUse::forward(corner(piece.start, role)),
        BoundUse::new(corner(piece.end, role), Orientation::Reversed),
    ]
}
pub(super) fn normal_ends(source: CornerSource) -> Vec<BoundUse> {
    vec![
        BoundUse::forward(corner(source, HistoryRole::FirstDepthVertex)),
        BoundUse::new(
            corner(source, HistoryRole::LastDepthVertex),
            Orientation::Reversed,
        ),
    ]
}
