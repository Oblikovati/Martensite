//! Support adapters contribute leaf receipts; the executor owns operation sequencing.
use super::*;
use family::control::Checkpoint;
use std::collections::BTreeMap;
pub(in crate::surface_sketch::wrapped_profile) struct QualifiedMember<G> {
    pub geometry: G,
    pub native: pipeline::NativeQualification,
}
pub(in crate::surface_sketch::wrapped_profile) struct OwnedFamily<S, G> {
    pub bytes: Vec<u8>,
    pub keys: Vec<BindingKey>,
    pub members: BTreeMap<SurfaceRegionId, QualifiedMember<G>>,
    pub selection: S,
    pub coverage: coverage::FamilyCoverage,
    pub history: history::HistoryEvidence,
}
pub(in crate::surface_sketch::wrapped_profile) struct Budgeted<S, G, B> {
    pub family: OwnedFamily<S, G>,
    pub budget: B,
}
pub(in crate::surface_sketch::wrapped_profile) trait SupportStages {
    type Original: AsRef<DevelopedRegionRequest>;
    type Prepared: AsRef<DevelopedRegionRequest>;
    type Selection;
    type Context;
    type Stored;
    type Geometry;
    type Limits;
    type Budget;
    type Error: From<Checkpoint>;
    fn context(&self) -> Self::Context;
    fn limits(&self, originals: &[Self::Original]) -> Result<Self::Limits, Self::Error>;
    fn select(
        &self,
        originals: &[Self::Original],
        context: &mut Self::Context,
        cancelled: &mut CancelPoll<'_>,
    ) -> Result<Self::Selection, Self::Error>;
    fn prepared(selection: &Self::Selection) -> &[Self::Prepared];
    fn construct(&self, prepared: &Self::Prepared) -> Result<Constructed, Self::Error>;
    fn encoding(cause: crate::surface_sketch::failure::SurfaceFailureCause) -> Self::Error;
    fn assembly(region: SurfaceRegionId, cause: Failure) -> Self::Error;
    fn native(region: SurfaceRegionId, cause: Refusal) -> Self::Error;
    fn coverage(cause: coverage::CoverageFailure) -> Self::Error;
    fn missing(region: SurfaceRegionId) -> Self::Error;
    fn history(
        &self,
        originals: &[Self::Original],
        selection: &Self::Selection,
        keys: &[BindingKey],
    ) -> Result<history::HistoryEvidence, Self::Error>;
    fn store(
        &self,
        original: &Self::Original,
        prepared: &Self::Prepared,
        view: coverage::MemberView,
        context: &mut Self::Context,
        cancelled: &mut CancelPoll<'_>,
    ) -> Result<Self::Stored, Self::Error>;
    fn bind(
        &self,
        originals: &[Self::Original],
        selection: &Self::Selection,
        stored: &BTreeMap<SurfaceRegionId, Self::Stored>,
    ) -> Result<(), Self::Error>;
    fn geometry(
        &self,
        original: &Self::Original,
        prepared: &Self::Prepared,
        stored: Self::Stored,
        context: &mut Self::Context,
        cancelled: &mut CancelPoll<'_>,
    ) -> Result<(Self::Geometry, Shape), Self::Error>;
    fn budget(
        &self,
        originals: &[Self::Original],
        family: &OwnedFamily<Self::Selection, Self::Geometry>,
        limits: Self::Limits,
        context: &mut Self::Context,
        cancelled: &mut CancelPoll<'_>,
    ) -> Result<Self::Budget, Self::Error>;
}
