// SPDX-License-Identifier: GPL-2.0-only
//! #544: selector refusals retain the normalized expectation and complete observed policies.
use super::{BoundaryCensus, paired_types::PairedCensusEvidence};
use martensite_tkmath::predicates::roots::bezier_pair::*;
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct GraphPolicySnapshot {
    pub pair_policy: BezierPairPolicy,
    pub exclusion_version: Option<u32>,
    pub selected_signs: BezierPairSignOptions,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GraphPolicyMismatch {
    pub requested_graph_version: u32,
    pub admitted_graph_versions: Vec<u32>,
    pub expected: Option<GraphPolicySnapshot>,
    pub actual: Option<GraphPolicySnapshot>,
}
/// Bind the actual scalar policies without running geometry or allocating on success.
/// Example: `bind_graph_policy(census, 4)?` requires the filtered-sign graph policy.
pub(crate) fn bind_graph_policy(
    census: &BoundaryCensus,
    version: u32,
) -> Result<(), Box<GraphPolicyMismatch>> {
    let expected = GraphPolicySnapshot::expected(census, version);
    let actual = census.paired.as_ref().map(GraphPolicySnapshot::actual);
    if let (Some(expected), Some(actual)) = (&expected, &actual)
        && expected.matches(actual)
    {
        return Ok(());
    }
    Err(Box::new(GraphPolicyMismatch {
        requested_graph_version: version,
        admitted_graph_versions: if version <= 4 {
            vec![2, 3, 4]
        } else if version <= 5 {
            vec![2, 3, 4, 5]
        } else {
            vec![2, 3, 4, 5, 6]
        },
        expected,
        actual,
    }))
}
impl GraphPolicySnapshot {
    fn expected(census: &BoundaryCensus, version: u32) -> Option<Self> {
        let (exclusion_version, sign_version) = match version {
            2 => (None, 1),
            3 => (Some(1), 1),
            4 | 5 => (Some(1), 2),
            6 => (Some(2), 2),
            _ => return None,
        };
        let mut signs = BezierPairSignOptions::derived();
        signs.version = sign_version;
        Some(Self {
            pair_policy: expected_pair(census, signs),
            exclusion_version,
            selected_signs: signs,
        })
    }
    fn actual(paired: &PairedCensusEvidence) -> Self {
        Self {
            pair_policy: paired.requested,
            exclusion_version: paired.exclusion_version,
            selected_signs: paired.selected_signs,
        }
    }
    fn matches(&self, actual: &Self) -> bool {
        self.exclusion_version == actual.exclusion_version
            && self.selected_signs == actual.selected_signs
            && self.pair_policy.version == actual.pair_policy.version
            && self.pair_policy.signs == actual.pair_policy.signs
            && self.pair_policy.roots == actual.pair_policy.roots
            && self.pair_policy.requested_world_error.to_bits()
                == actual.pair_policy.requested_world_error.to_bits()
    }
}
fn expected_pair(census: &BoundaryCensus, signs: BezierPairSignOptions) -> BezierPairPolicy {
    BezierPairPolicy {
        version: 2,
        roots: census.requested.roots,
        signs,
        requested_world_error: census.requested.max_world_error,
    }
}
