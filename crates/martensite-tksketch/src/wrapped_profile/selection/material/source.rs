//! Exact planar snapshots and original-boundary bindings precede all witness decisions.
use super::*;
pub(super) fn bind<'a>(
    requests: &'a [Request],
    prepared: &RelationInputs<'_>,
) -> Checked<Vec<&'a Request>> {
    let tolerance = super::super::admission::admit(requests)
        .map_err(|cause| MaterialFailure::Admission(Box::new(cause)))?;
    match prepared.planar {
        PlanarSource::Graph(planar) => bind_graph(requests, prepared.prepared, planar)?,
        PlanarSource::Directed(selection) => {
            selection
                .rebind(requests, tolerance)
                .map_err(|cause| MaterialFailure::Admission(Box::new(cause)))?;
            membership(requests.len(), requests.len(), prepared.prepared.len())?;
            for (original, fitted) in requests.iter().zip(prepared.prepared) {
                match_support(original, fitted)?;
            }
        }
    }
    planar_clearance::bind_originals(prepared.boundary, requests, prepared.prepared)
        .map_err(|cause| MaterialFailure::Clearance(Box::new(cause)))?;
    let mut result: Vec<_> = requests.iter().collect();
    result.sort_by_key(|r| r.region);
    Ok(result)
}
fn bind_graph(
    requests: &[Request],
    fitted: &[Request],
    planar: &PlanarSelectionEvidence,
) -> Checked<()> {
    membership(requests.len(), planar.members.len(), fitted.len())?;
    for (original, fitted) in requests.iter().zip(fitted) {
        let id = original.region;
        let evidence = planar.members.get(&id).ok_or(MaterialFailure::Source(id))?;
        if !evidence.matches_member(original) {
            return Err(MaterialFailure::Source(id));
        }
        match_support(original, fitted)?;
    }
    if let Some(region) = planar.integral_mismatch(requests) {
        return Err(MaterialFailure::Source(region));
    }
    Ok(())
}

fn membership(requested: usize, planar: usize, prepared: usize) -> Checked<()> {
    if requested != planar || requested != prepared {
        return Err(MaterialFailure::Members {
            requested,
            planar,
            prepared,
        });
    }
    Ok(())
}

fn match_support(original: &Request, fitted: &Request) -> Checked<()> {
    if original.region != fitted.region || context(original) != context(fitted) {
        return Err(MaterialFailure::Source(original.region));
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
