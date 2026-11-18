//! Cylinder numerical leaves implement the shared executor's receipt boundaries.
use super::*;
use crate::surface_sketch::wrapped_profile::operation::stages::{OwnedFamily, SupportStages};
use std::collections::BTreeMap;
pub(in crate::surface_sketch::wrapped_profile) struct CylinderStages {
    pub work: martensite_tkbo::profile_extrusion::PlanarProofWorkLimit,
    pub volume: CylindricalMeasureLimit,
}
impl SupportStages for CylinderStages {
    type Original = Request;
    type Prepared = Request;
    type Context = ();
    fn context(&self) {}
    type Selection = selection::QualifiedSelection;
    type Stored = pipeline::StoredMember;
    type Geometry = pipeline::MemberGeometry;
    type Limits = publication_budget::BudgetLimits;
    type Budget = publication_budget::BudgetEvidence;
    type Error = FamilyFailure;
    fn limits(&self, originals: &[Request]) -> FamilyChecked<Self::Limits> {
        publication_budget::requested_limits(originals, self.volume).map_err(budget_failure)
    }
    fn select(
        &self,
        originals: &[Request],
        _: &mut (),
        cancelled: &mut CancelPoll<'_>,
    ) -> FamilyChecked<Self::Selection> {
        selection::certify_with_work(originals, Default::default(), self.work, cancelled)
            .map_err(|cause| FamilyFailure::Selection(Box::new(cause)))
    }
    fn prepared(selection: &Self::Selection) -> &[Request] {
        &selection.prepared.prepared
    }
    fn construct(&self, request: &Request) -> FamilyChecked<Constructed> {
        native::construct(request).map_err(|cause| Self::assembly(request.region, cause))
    }
    fn encoding(cause: crate::surface_sketch::failure::SurfaceFailureCause) -> FamilyFailure {
        FamilyFailure::Encoding(Box::new(cause))
    }
    fn assembly(region: SurfaceRegionId, cause: Failure) -> FamilyFailure {
        FamilyFailure::Native {
            region: Some(region),
            cause: Box::new(cause),
        }
    }
    fn native(_: SurfaceRegionId, cause: Refusal) -> FamilyFailure {
        FamilyFailure::Member(Box::new(cause))
    }
    fn coverage(cause: coverage::CoverageFailure) -> FamilyFailure {
        FamilyFailure::Coverage(Box::new(cause))
    }
    fn missing(region: SurfaceRegionId) -> FamilyFailure {
        family::membership(region)
    }
    fn history(
        &self,
        originals: &[Request],
        selection: &Self::Selection,
        keys: &[BindingKey],
    ) -> FamilyChecked<history::HistoryEvidence> {
        history::certify(originals, selection, keys)
            .map_err(|cause| FamilyFailure::History(Box::new(cause)))
    }
    fn store(
        &self,
        original: &Request,
        prepared: &Request,
        view: coverage::MemberView,
        _: &mut (),
        cancelled: &mut CancelPoll<'_>,
    ) -> FamilyChecked<Self::Stored> {
        pipeline::store_member_with_cancel(original, prepared, view.shape, view.bindings, cancelled)
            .map_err(|cause| FamilyFailure::Member(Box::new(cause)))
    }
    fn bind(
        &self,
        originals: &[Request],
        selection: &Self::Selection,
        stored: &BTreeMap<SurfaceRegionId, Self::Stored>,
    ) -> FamilyChecked<()> {
        family::bind(originals, selection, stored)
    }
    fn geometry(
        &self,
        original: &Request,
        prepared: &Request,
        stored: Self::Stored,
        _: &mut (),
        cancelled: &mut CancelPoll<'_>,
    ) -> FamilyChecked<(Self::Geometry, Shape)> {
        pipeline::certify_geometry(original, prepared, stored, cancelled)
            .map_err(|cause| FamilyFailure::Member(Box::new(cause)))
    }
    fn budget(
        &self,
        originals: &[Request],
        family: &OwnedFamily<Self::Selection, Self::Geometry>,
        limits: Self::Limits,
        _: &mut (),
        _: &mut CancelPoll<'_>,
    ) -> FamilyChecked<Self::Budget> {
        publication_budget::certify_with_limits(originals, family, limits).map_err(budget_failure)
    }
}
