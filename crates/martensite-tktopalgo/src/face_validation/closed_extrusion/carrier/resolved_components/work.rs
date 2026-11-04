//! One whole-face allowance is shared across all actual component/contact operations.
use super::*;
pub(super) fn remaining(report: &ToleranceResolvedFaceReport) -> ResolvedResult<u32> {
    let limit = u64::from(report.policy.max_incidence_work);
    if report.incidence_work > limit {
        return Err(ToleranceResolvedFaceCompletion::Work {
            limit: report.policy.max_incidence_work,
            actual: report.incidence_work,
        });
    }
    Ok((limit - report.incidence_work) as u32)
}
pub(super) fn record(report: &mut ToleranceResolvedFaceReport, used: u32) -> ResolvedResult<()> {
    report.incidence_work += u64::from(used);
    remaining(report).map(|_| ())
}

pub(super) fn pair_allowance(
    report: &ToleranceResolvedFaceReport,
    pair: [&GivenVertexNeighborhoodReport; 2],
) -> ResolvedResult<[u32; 2]> {
    let total = u64::from(pair[0].work()) + u64::from(pair[1].work());
    let prior = u32::try_from(total).map_err(|_| ToleranceResolvedFaceCompletion::Work {
        limit: report.policy.max_incidence_work,
        actual: total,
    })?;
    let allowance =
        remaining(report)?
            .checked_add(prior)
            .ok_or(ToleranceResolvedFaceCompletion::Work {
                limit: report.policy.max_incidence_work,
                actual: report.incidence_work + total,
            })?;
    Ok([prior, allowance])
}
