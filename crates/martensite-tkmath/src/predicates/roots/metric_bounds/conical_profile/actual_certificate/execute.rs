// SPDX-License-Identifier: GPL-2.0-only
//! One global remaining grant covers every side, adjacency, product and cap domain.
use super::ConicalActualPatchStage as Stage;
use super::*;
/// Certify the complete actual family numeric inventory in one remaining grant.
/// Example: `let proof=certify_conical_actual_patches(query,||false)?;`.
pub fn certify_conical_actual_patches(
    query: ConicalActualPatchQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalActualPatchCertificate, ConicalActualPatchError> {
    let mut position = error::FailurePosition::default();
    admission::admit(&query, &mut position)
        .map_err(|v| position.error(v, Stage::Binding, query.options, 0))?;
    let mut arithmetic = Arithmetic::new(sector_api::policy(query.options), &mut cancelled);
    run(&query, &mut arithmetic, &mut position)
}
fn run(
    query: &ConicalActualPatchQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
    position: &mut error::FailurePosition,
) -> Result<ConicalActualPatchCertificate, ConicalActualPatchError> {
    let sides = capture_all(query, arithmetic, position)?;
    position.clear();
    let support = support(query, arithmetic, position)?;
    let result = barriers::certify(&sides, &support.sine, &support.cosine, arithmetic);
    let barriers = stage(result, Stage::SideCap, query, arithmetic, position)?;
    union_all(query, &sides, &barriers, support, arithmetic, position)?;
    position.clear();
    let refine = query
        .sides
        .iter()
        .all(|side| side.domain.version() != ConicalChartDomainVersion::OriginalV2);
    let result = hierarchy::certify(&sides, query.adjacent, refine, arithmetic);
    let hierarchy = stage(result, Stage::Hierarchy, query, arithmetic, position)?;
    let result = caps::certify(
        query,
        &support.sine,
        &support.cosine,
        refine,
        arithmetic,
        position,
    );
    let caps = stage(result, Stage::Cap, query, arithmetic, position)?;
    position.clear();
    final_checkpoint(query, arithmetic, position)?;
    Ok(evidence(
        query,
        sides,
        caps,
        hierarchy,
        arithmetic.operations(),
    ))
}
fn union_all(
    query: &ConicalActualPatchQuery<'_>,
    sides: &[capture::SideCapture],
    barriers: &barriers::Barriers,
    support: &GammaSpanCapture,
    arithmetic: &mut Arithmetic<'_>,
    position: &mut error::FailurePosition,
) -> Result<(), ConicalActualPatchError> {
    let (sine, cosine) = (&support.sine, &support.cosine);
    let indices = sides
        .iter()
        .enumerate()
        .map(|(i, s)| (s.source.id, i))
        .collect::<BTreeMap<_, _>>();
    for pair in query.adjacent {
        position.side(pair.sides[0]);
        let locations = pair.sides.map(|id| indices[&id]);
        let [a, b] = locations.map(|index| &sides[index]);
        let result = if locations.iter().all(|&index| {
            query.sides[index].domain.version() != ConicalChartDomainVersion::OriginalV2
        }) {
            unions::certify_retained(pair, [a, b], barriers, arithmetic)
        } else {
            unions::certify(pair, a, b, barriers, sine, cosine, arithmetic)
        };
        let result = result.map_err(|cause| ConicalActualPatchFailure::SidePair {
            sides: pair.sides,
            cause: Box::new(cause),
        });
        stage(result, Stage::Adjacent, query, arithmetic, position)?;
    }
    Ok(())
}
fn evidence(
    query: &ConicalActualPatchQuery<'_>,
    sides: Vec<capture::SideCapture>,
    caps: Vec<caps::CapCapture>,
    hierarchy: hierarchy::HierarchyStats,
    operations: u32,
) -> ConicalActualPatchCertificate {
    let members = sides
        .iter()
        .map(|s| s.source.member)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let coverage = coverage(query, sides.len(), caps.len(), &hierarchy);
    ConicalActualPatchCertificate {
        source_bits: binding::bits(query),
        gamma: query.gamma.clone(),
        sides: sides.into_iter().map(|s| s.source).collect(),
        caps: caps.into_iter().map(|s| s.source).collect(),
        adjacent: query.adjacent.to_vec(),
        members,
        options: query.options,
        operations,
        coverage,
    }
}

fn capture_all(
    query: &ConicalActualPatchQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
    position: &mut error::FailurePosition,
) -> Result<Vec<capture::SideCapture>, ConicalActualPatchError> {
    let mut sides = Vec::new();
    for side in query.sides {
        position.side(side.id);
        let result = capture::side(side, query, arithmetic);
        sides.push(stage(result, Stage::Capture, query, arithmetic, position)?);
    }
    Ok(sides)
}
fn stage<T>(
    result: Outcome<T>,
    stage: ConicalActualPatchStage,
    query: &ConicalActualPatchQuery<'_>,
    arithmetic: &Arithmetic<'_>,
    position: &error::FailurePosition,
) -> Result<T, ConicalActualPatchError> {
    result.map_err(|v| position.error(v, stage, query.options, arithmetic.operations()))
}
fn support<'a>(
    query: &ConicalActualPatchQuery<'a>,
    arithmetic: &Arithmetic<'_>,
    position: &error::FailurePosition,
) -> Result<&'a GammaSpanCapture, ConicalActualPatchError> {
    query.gamma.span_capture(query.sides[0].id).ok_or_else(|| {
        position.error(
            error::shape("family support", 0, "issued source"),
            Stage::Binding,
            query.options,
            arithmetic.operations(),
        )
    })
}

fn coverage(
    query: &ConicalActualPatchQuery<'_>,
    sides: usize,
    caps: usize,
    hierarchy: &hierarchy::HierarchyStats,
) -> ConicalActualPatchCoverage {
    let count = caps as u64;
    ConicalActualPatchCoverage {
        side_self_domains: sides,
        adjacent_unions: query.adjacent.len(),
        nonincident_pairs: hierarchy.excluded_pairs,
        side_cap_pairs: sides as u64 * count,
        cap_self_domains: caps,
        cap_pairs: count * (count - 1) / 2,
        hierarchy_nodes: hierarchy.nodes,
        hierarchy_visits: hierarchy.visits,
    }
}

fn final_checkpoint(
    query: &ConicalActualPatchQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
    position: &error::FailurePosition,
) -> Result<(), ConicalActualPatchError> {
    let result = arithmetic
        .check_cancelled()
        .map_err(ConicalActualPatchFailure::from);
    stage(result, Stage::Cap, query, arithmetic, position)
}
