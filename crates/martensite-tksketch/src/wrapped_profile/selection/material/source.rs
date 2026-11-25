//! Exact planar snapshots and original-boundary bindings precede all witness decisions.
use super::*;
pub(super) fn bind<'a>(
    requests: &'a [Request],
    prepared: &RelationInputs<'_>,
) -> Checked<Vec<&'a Request>> {
    super::super::admission::admit(requests)
        .map_err(|cause| MaterialFailure::Admission(Box::new(cause)))?;
    membership(requests.len(), prepared)?;
    for (original, fitted) in requests.iter().zip(prepared.prepared) {
        member(original, fitted, prepared)?;
    }
    if let Some(region) = prepared.planar.integral_mismatch(requests) {
        return Err(MaterialFailure::Source(region));
    }
    planar_clearance::bind_originals(prepared.boundary, requests, prepared.prepared)
        .map_err(|cause| MaterialFailure::Clearance(Box::new(cause)))?;
    let mut result: Vec<_> = requests.iter().collect();
    result.sort_by_key(|r| r.region);
    Ok(result)
}
fn membership(count: usize, prepared: &RelationInputs<'_>) -> Checked<()> {
    if count != prepared.planar.members.len() || count != prepared.prepared.len() {
        return Err(MaterialFailure::Members {
            requested: count,
            planar: prepared.planar.members.len(),
            prepared: prepared.prepared.len(),
        });
    }
    Ok(())
}

fn member(original: &Request, fitted: &Request, prepared: &RelationInputs<'_>) -> Checked<()> {
    let id = original.region;
    let evidence = prepared
        .planar
        .members
        .get(&id)
        .ok_or(MaterialFailure::Source(id))?;
    if id != fitted.region
        || !evidence.matches_member(original)
        || context(original) != context(fitted)
    {
        return Err(MaterialFailure::Source(id));
    }
    Ok(())
}
fn context(r: &Request) -> [u64; 5] {
    [
        r.radius.to_bits(),
        r.depths[0].to_bits(),
        r.depths[1].to_bits(),
        r.tolerance.to_bits(),
        r.max_fit_level.into(),
    ]
}
