//! Explicit cone-domain behavior keeps previously accepted replay inputs selectable.
use super::*;
/// Select the cone-domain proof without changing planar or integration behavior.
/// Example: `let policy = WrappedConeDomainPolicy::PositiveXCommonChartV2;`.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WrappedConeDomainPolicy {
    /// Original quarter-turn sector, sine floor and common reference chart.
    #[default]
    LegacySectorV1,
    /// Positive-X common chart with bounded positive sine and strict angular width.
    PositiveXCommonChartV2,
    /// The same quantitative chart with an explicitly selected 4096-source family cap.
    PositiveXFamilyChartV3,
    /// A single Positive-X chart centered on the selected family, within one physical period.
    PositiveXRecenteredChartV4,
    /// One independently centered Positive-X chart per disconnected region.
    PositiveXRegionAtlasV5,
    /// Local Positive-X charts over exact subintervals of connected or disconnected contours.
    PositiveXConnectedAtlasV6,
}
impl WrappedConeNormalResult {
    /// Retained cone-domain behavior; `assert_eq!(result.domain_policy(), policy)`.
    pub fn domain_policy(&self) -> WrappedConeDomainPolicy {
        self.family.family.selection.domain.policy()
    }
    /// Executed common chart receipt, if there was one chart for the whole request.
    /// Example: `let center = result.chart_domain_receipt().map(|proof| proof.fit_center());`.
    pub fn chart_domain_receipt(
        &self,
    ) -> Option<&martensite_tkmath::predicates::roots::metric_bounds::ConicalChartDomainEvidence>
    {
        self.family.family.selection.domain.chart()
    }
    /// Chart for one selected region, including atlas members.
    /// The connected atlas certifies each fitted interval individually.
    /// Example: `result.region_chart_domain_receipt(SurfaceRegionId(1))`.
    pub fn region_chart_domain_receipt(
        &self,
        region: SurfaceRegionId,
    ) -> Option<&martensite_tkmath::predicates::roots::metric_bounds::ConicalChartDomainEvidence>
    {
        let family = &self.family.family;
        family
            .members
            .contains_key(&region)
            .then(|| family.selection.domain.region_chart(region))
            .flatten()
    }
}
/// Select an explicit domain proof for a complete canonical cone operation.
/// Example: `normal_extrude_cone_profile_with_domain_policy(&request, policy)?;`.
pub fn normal_extrude_cone_profile_with_domain_policy(
    request: &WrappedConeNormalRequest,
    policy: WrappedConeDomainPolicy,
) -> Checked<WrappedConeNormalResult> {
    normal_extrude_cone_profile_with_domain_policy_and_cancel(request, policy, || false)
}
/// Cancel the selected complete cone-domain operation without partial publication.
/// Example: `normal_extrude_cone_profile_with_domain_policy_and_cancel(&request, policy, || stop())`.
pub fn normal_extrude_cone_profile_with_domain_policy_and_cancel(
    request: &WrappedConeNormalRequest,
    policy: WrappedConeDomainPolicy,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<WrappedConeNormalResult> {
    operation::cone::execute_with_domain(request, policy, &mut cancelled)
}
#[cfg(test)]
mod tests;
