//! Cone adapters return mathematical and native inputs to the shared operation driver.
use super::*;
use crate::surface_sketch::wrapped_profile::operation::stages::{OwnedFamily, SupportStages};
pub(super) struct ConeStages<'a> {
    pub request: &'a WrappedConeNormalRequest,
}
impl SupportStages for ConeStages<'_> {
    type Original = DevelopedRegionRequest;
    type Prepared = Prepared;
    type Selection = Selection;
    type Context = storage::Work;
    fn context(&self) -> storage::Work {
        storage::Work::new(self.request.proof.arithmetic)
    }
    type Stored = storage::Stored;
    type Geometry = Geometry;
    type Limits = ();
    type Budget = Budget;
    type Error = ConeFailure;
    fn limits(&self, _: &[DevelopedRegionRequest]) -> ConeChecked<()> {
        Ok(())
    }
    fn select(
        &self,
        originals: &[DevelopedRegionRequest],
        context: &mut storage::Work,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<Selection> {
        prepare::select(self.request, originals, context, cancelled)
    }
    fn prepared(selection: &Selection) -> &[Prepared] {
        &selection.prepared
    }
    fn construct(&self, prepared: &Prepared) -> ConeChecked<Constructed> {
        native::construct(prepared, self.request.support)
            .map_err(|cause| Self::assembly(prepared.source.region, cause))
    }
    fn encoding(cause: crate::surface_sketch::failure::SurfaceFailureCause) -> ConeFailure {
        ConeFailure::new(
            None,
            None,
            WrappedFailureClass::Native,
            ConeCause::Encoding(cause),
        )
    }
    fn assembly(region: SurfaceRegionId, cause: Failure) -> ConeFailure {
        ConeFailure::new(
            Some(region),
            None,
            WrappedFailureClass::Native,
            ConeCause::Native(Box::new(cause)),
        )
    }
    fn native(region: SurfaceRegionId, cause: Refusal) -> ConeFailure {
        ConeFailure::new(
            Some(region),
            None,
            WrappedFailureClass::Incomplete,
            ConeCause::NativeQualification(Box::new(cause)),
        )
    }
    fn coverage(cause: coverage::CoverageFailure) -> ConeFailure {
        ConeFailure::new(
            None,
            None,
            WrappedFailureClass::Incomplete,
            ConeCause::Coverage(Box::new(cause)),
        )
    }
    fn missing(region: SurfaceRegionId) -> ConeFailure {
        ConeFailure::new(
            Some(region),
            None,
            WrappedFailureClass::Incomplete,
            ConeCause::Missing(region),
        )
    }
    fn history(
        &self,
        originals: &[DevelopedRegionRequest],
        selection: &Selection,
        keys: &[BindingKey],
    ) -> ConeChecked<history::HistoryEvidence> {
        history::certify_developed(originals, &selection.prepared, keys).map_err(|cause| {
            ConeFailure::new(
                None,
                None,
                WrappedFailureClass::Incomplete,
                ConeCause::History(Box::new(cause)),
            )
        })
    }
    fn store(
        &self,
        original: &DevelopedRegionRequest,
        prepared: &Prepared,
        view: coverage::MemberView,
        context: &mut storage::Work,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<storage::Stored> {
        storage::capture(original, prepared, view, context, cancelled)
    }
    fn bind(
        &self,
        originals: &[DevelopedRegionRequest],
        selection: &Selection,
        stored: &BTreeMap<SurfaceRegionId, storage::Stored>,
    ) -> ConeChecked<()> {
        storage::bind(originals, selection, stored)
    }
    fn geometry(
        &self,
        original: &DevelopedRegionRequest,
        prepared: &Prepared,
        stored: storage::Stored,
        context: &mut storage::Work,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<(Geometry, Shape)> {
        measures::geometry(self.request, original, prepared, stored, context, cancelled)
    }
    fn budget(
        &self,
        _originals: &[DevelopedRegionRequest],
        family: &OwnedFamily<Selection, Geometry>,
        _: (),
        context: &mut storage::Work,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<Budget> {
        measures::budget(self.request, family, context, cancelled)
    }
}
