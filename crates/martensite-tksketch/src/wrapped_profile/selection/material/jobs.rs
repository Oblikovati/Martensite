//! Every positive image needs both containment directions for every ordered region pair.
use super::*;
pub(super) fn plan(
    regions: &[&Request],
    periodic: &CertifiedCylindricalPeriodicClearance,
    limit: u32,
) -> Checked<(Vec<Job>, Vec<CylindricalPeriodicProbe>)> {
    let count = periodic.images().len() as u64 * regions.len() as u64 * regions.len() as u64 * 2;
    if count > u64::from(limit) {
        return Err(MaterialFailure::Limit {
            role: "periodic probes",
            actual: count,
            maximum: limit.into(),
        });
    }
    let mut jobs = Vec::new();
    let mut probes = Vec::new();
    for image in periodic.images() {
        image_jobs(regions, image.translation().turn(), &mut jobs, &mut probes)?;
    }
    Ok((jobs, probes))
}
fn image_jobs(
    regions: &[&Request],
    turn: u32,
    jobs: &mut Vec<Job>,
    probes: &mut Vec<CylindricalPeriodicProbe>,
) -> Checked<()> {
    for a in regions {
        for b in regions {
            append(a, b, turn, CylindricalProbeSense::Subtract, jobs, probes)?;
            append(b, a, turn, CylindricalProbeSense::Add, jobs, probes)?;
        }
    }
    Ok(())
}
fn append(
    source: &Request,
    target: &Request,
    turn: u32,
    sense: CylindricalProbeSense,
    jobs: &mut Vec<Job>,
    probes: &mut Vec<CylindricalPeriodicProbe>,
) -> Checked<()> {
    let (witness, point) = witness(source)?;
    jobs.push(Job {
        source: source.region,
        target: target.region,
        witness,
    });
    probes.push(CylindricalPeriodicProbe { turn, point, sense });
    Ok(())
}
pub(super) fn witness(region: &Request) -> Checked<(SurfaceCurveId, [f64; 2])> {
    let piece = region
        .loops
        .first()
        .and_then(|c| c.first())
        .ok_or(MaterialFailure::Source(region.region))?;
    let point = piece
        .poles
        .first()
        .copied()
        .ok_or(MaterialFailure::Source(region.region))?;
    Ok((piece.span.curve, point))
}
