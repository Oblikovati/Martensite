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
}
impl WrappedConeNormalResult {
    /// Retained cone-domain behavior; `assert_eq!(result.domain_policy(), policy)`.
    pub fn domain_policy(&self) -> WrappedConeDomainPolicy {
        self.family.family.selection.domain.policy()
    }
    /// Executed V2 domain receipt; legacy results return `None`.
    /// Example: `let center = result.chart_domain_receipt().map(|proof| proof.fit_center());`.
    pub fn chart_domain_receipt(
        &self,
    ) -> Option<&martensite_tkmath::predicates::roots::metric_bounds::ConicalChartDomainEvidence>
    {
        self.family.family.selection.domain.chart()
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
