//! The stage owns its versioned key policy; legacy one-way comparison is replay-only.
use crate::bop_algo::FaceGroupingVersion;
use crate::bop_tools::ShapeSet;
use martensite_tkernel::ncollection::{DefaultHashPolicy, HashPolicy};

#[derive(Clone, Copy, Debug, Default)]
pub(super) struct EdgeOccurrencePolicy(pub(super) FaceGroupingVersion);
impl HashPolicy<ShapeSet> for EdgeOccurrencePolicy {
    fn hash(&self, key: &ShapeSet) -> usize {
        DefaultHashPolicy.hash(key)
    }
    fn equal(&self, first: &ShapeSet, second: &ShapeSet) -> bool {
        match self.0 {
            FaceGroupingVersion::LegacyOccurrencesV1 => DefaultHashPolicy.equal(first, second),
            FaceGroupingVersion::ExactOccurrencesV2 => first.same_occurrences(second),
        }
    }
}
