//! Owned work accounting uses closed-form surfaces and explicit cancellation hooks.
#[path = "support/self_search/mod.rs"]
mod fixtures;
use fixtures::*;
use martensite_tkernel::message::{ProgressIndicator, ProgressIndicatorHooks, ProgressScope};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::adaptor3d::{Adaptor3dSurfaceBase, h_surface_tool::Adaptor3dSurfaceHandle};
use martensite_tkgeomalgo::int_patch::*;
use std::sync::atomic::{AtomicUsize, Ordering};

struct CancelAfterPoll {
    polls: AtomicUsize,
    allowed: usize,
}
impl ProgressIndicatorHooks for CancelAfterPoll {
    fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {}
    fn user_break(&self) -> bool {
        self.polls.fetch_add(1, Ordering::SeqCst) >= self.allowed
    }
}
fn cancellation(allowed: usize) -> ProgressIndicator {
    ProgressIndicator::new(Box::new(CancelAfterPoll {
        polls: AtomicUsize::new(0),
        allowed,
    }))
}
#[test]
fn sampled_empty_search_retains_no_global_exclusion_claim() {
    let surface = plane_patch();
    let poly = IntPatchPolyhedron::with_subdivisions(&surface, 1, 1).unwrap();
    let mut search = IntPatchPrmPrmIntersection::new();
    search
        .perform_self_polyhedron((&surface, &poly, &domain(&surface)), tolerances())
        .unwrap();
    let receipt = search.self_search_receipt();
    assert_eq!(receipt.discovery, SelfSearchDiscovery::SampledPolyhedron);
    assert!(receipt.discovery_finished);
    assert_eq!(receipt.termination, SelfSearchTermination::Finished);
    assert!(search.is_empty().unwrap());
}
#[test]
fn cancelled_before_discovery_stays_cancelled_without_work() {
    let surface = plane_patch();
    let progress = cancellation(0);
    let mut search = IntPatchPrmPrmIntersection::new();
    assert!(
        search
            .perform_self_with_progress(
                &surface,
                &domain(&surface),
                tolerances(),
                &progress.start()
            )
            .is_err()
    );
    let receipt = search.self_search_receipt();
    assert_eq!(receipt.termination, SelfSearchTermination::Cancelled);
    assert!(!receipt.discovery_finished);
    assert_eq!(receipt.starts.visited, 0);
}
#[test]
fn failed_surface_preserves_error_and_frontend_receipt() {
    let surface: Adaptor3dSurfaceHandle = Handle::new(Adaptor3dSurfaceBase::default()).cast();
    let valid = plane_patch();
    let mut search = IntPatchIntersection::new();
    assert!(
        search
            .perform_self(&surface, &domain(&valid), 1e-7, 1e-7)
            .is_err()
    );
    assert_eq!(
        search.self_search_receipt().termination,
        SelfSearchTermination::Failed
    );
    assert!(!search.self_search_receipt().discovery_finished);
}
#[test]
fn receipt_is_owned_and_replaced_per_invocation() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<SurfaceSelfSearchReceipt>();
    let surface = plane_patch();
    let mut search = IntPatchPrmPrmIntersection::new();
    let first = cancelled_receipt(&mut search, &surface);
    let second = cancelled_receipt(&mut search, &surface);
    assert_eq!(first.invocation, 1);
    assert_eq!(second.invocation, 2);
    assert_eq!(first.termination, SelfSearchTermination::Cancelled);
}
fn cancelled_receipt(
    search: &mut IntPatchPrmPrmIntersection,
    surface: &Adaptor3dSurfaceHandle,
) -> SurfaceSelfSearchReceipt {
    let progress = cancellation(0);
    assert!(
        search
            .perform_self_with_progress(surface, &domain(surface), tolerances(), &progress.start())
            .is_err()
    );
    search.self_search_receipt().clone()
}

#[test]
fn known_crossing_is_independent_of_search_receipt() {
    let surface = crossing_patch();
    let a = surface.value(1. / 6., 0.5).unwrap();
    let b = surface.value(5. / 6., 0.5).unwrap();
    assert!(a.distance(&b) < 1e-14);
    assert!((a.x() - 1.).abs() < 1e-14 && a.z().abs() < 1e-14);
    let mut search = IntPatchPrmPrmIntersection::new();
    search
        .perform_self(&surface, &domain(&surface), tolerances())
        .unwrap();
    let receipt = search.self_search_receipt();
    assert!(
        search.nb_lines().unwrap() > 0,
        "the interior crossing has a contact line"
    );
    assert!(receipt.starts.visited > 0);
    assert!(receipt.calls_with_unobserved_inner_work > 0);
    assert_eq!(receipt.termination, SelfSearchTermination::Incomplete);
}
#[test]
fn cancellation_after_discovery_retains_scheduled_work() {
    let surface = crossing_patch();
    let progress = cancellation(3);
    let mut search = IntPatchPrmPrmIntersection::new();
    assert!(
        search
            .perform_self_with_progress(
                &surface,
                &domain(&surface),
                tolerances(),
                &progress.start()
            )
            .is_err()
    );
    let receipt = search.self_search_receipt();
    assert!(receipt.discovery_finished);
    assert!(receipt.sections.scheduled + receipt.tangent_zones.scheduled > 0);
    assert_eq!(receipt.termination, SelfSearchTermination::Cancelled);
}

#[test]
fn cancellation_during_marching_keeps_actual_step_and_work_counts() {
    let surface = crossing_patch();
    let progress = cancellation(20);
    let mut search = IntPatchPrmPrmIntersection::new();
    assert!(
        search
            .perform_self_with_progress(
                &surface,
                &domain(&surface),
                tolerances(),
                &progress.start()
            )
            .is_err()
    );
    let receipt = search.self_search_receipt();
    assert!(receipt.marching_steps > 0);
    assert!(receipt.walks.cancelled > 0);
    assert_eq!(receipt.termination, SelfSearchTermination::Cancelled);
}

#[test]
fn frontend_retains_nested_receipt_after_cancellation() {
    let surface = crossing_patch();
    let progress = cancellation(3);
    let mut search = IntPatchIntersection::new();
    assert!(
        search
            .perform_self_with_progress(&surface, &domain(&surface), 1e-7, 1e-7, &progress.start())
            .is_err()
    );
    let receipt = search.self_search_receipt();
    assert!(receipt.discovery_finished);
    assert!(receipt.sections.scheduled + receipt.tangent_zones.scheduled > 0);
    assert_eq!(receipt.termination, SelfSearchTermination::Cancelled);
}
#[test]
fn frontend_preserves_sampled_inner_uncertainty_and_contacts() {
    let surface = crossing_patch();
    let mut search = IntPatchIntersection::new();
    search
        .perform_self(&surface, &domain(&surface), 1e-7, 1e-7)
        .unwrap();
    assert!(search.nb_lines().unwrap() > 0);
    assert!(
        search
            .self_search_receipt()
            .calls_with_unobserved_inner_work
            > 0
    );
    assert_eq!(
        search.self_search_receipt().termination,
        SelfSearchTermination::Incomplete
    );
}
#[test]
fn analytic_shortcut_is_distinct_from_sampled_search() {
    let surface = analytic_plane();
    let mut search = IntPatchIntersection::new();
    search
        .perform_self(&surface, &domain(&surface), 1e-7, 1e-7)
        .unwrap();
    assert_eq!(
        search.self_search_receipt().discovery,
        SelfSearchDiscovery::AnalyticPolicy
    );
    assert_eq!(
        search.self_search_receipt().termination,
        SelfSearchTermination::Finished
    );
    assert_eq!(search.self_search_receipt().starts.visited, 0);
}
