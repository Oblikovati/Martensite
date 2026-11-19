// SPDX-License-Identifier: GPL-2.0-only
//! Borrowed numeric requests and immutable owned snapshots share the public thread contract.
use super::*;
use crate::predicates::roots::metric_bounds::ConicalChartReferenceQuery;
fn shared<T: Send + Sync>() {}
#[test]
fn complete_numeric_query_types_are_send_and_sync() {
    shared::<ConicalActualMemberId>();
    shared::<ConicalActualSideId>();
    shared::<ConicalActualCapId>();
    shared::<ConicalActualCapCarrier>();
    shared::<ConicalDirectedGammaSpan<'static>>();
    shared::<ConicalDirectedGammaQuery<'static>>();
    shared::<ConicalActualContourRole>();
    shared::<ConicalActualUvEdge<'static>>();
    shared::<ConicalActualUvContour<'static>>();
    shared::<ConicalActualCapQuery<'static>>();
    shared::<ConicalActualSideQuery<'static>>();
    shared::<ConicalActualPatchQuery<'static>>();
    shared::<ConicalActualAdjacent>();
}
#[test]
fn owned_actual_snapshots_and_stage_are_send_and_sync() {
    shared::<ConicalActualSideSnapshot>();
    shared::<ConicalActualUvEdgeSnapshot>();
    shared::<ConicalActualUvContourSnapshot>();
    shared::<ConicalActualPatchStage>();
}
#[test]
fn explicit_reference_and_refinement_types_are_send_and_sync() {
    shared::<ConicalChartReferenceQuery<'static>>();
    shared::<ConicalPrimitiveRefinement>();
}
