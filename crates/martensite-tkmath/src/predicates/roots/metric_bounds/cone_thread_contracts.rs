// SPDX-License-Identifier: GPL-2.0-only
//! Compile-time ownership contracts for the cone operation's public Math inputs and receipts.
use super::conical_profile::{ConeReceipt, ConicalDistanceProof};
use super::*;
fn assert_send_sync<T: Send + Sync>() {}
#[test]
fn filled_material_types_are_send_sync() {
    assert_send_sync::<ConicalMaterialSpan<'static>>();
    assert_send_sync::<ConicalMaterialQuery<'static>>();
    assert_send_sync::<ConicalMaterialEvidence>();
    assert_send_sync::<ConicalMaterialError>();
}
#[test]
fn source_and_request_types_are_send_sync() {
    assert_send_sync::<ConicalSourceFailure>();
    assert_send_sync::<ConicalDistanceProof>();
    assert_send_sync::<ConicalDepthFailure>();
    assert_send_sync::<ConeReceipt>();
    assert_send_sync::<ConeClearanceOptions>();
    assert_send_sync::<ConeRequest<'static>>();
    assert_send_sync::<ConeRequestError>();
    assert_send_sync::<ConeRequestReceipt>();
}
#[test]
fn boundary_correspondence_types_are_send_sync() {
    assert_send_sync::<ConicalBoundaryQuery<'static>>();
    assert_send_sync::<ConicalBoundaryStage>();
    assert_send_sync::<ConicalBoundaryEvidence>();
    assert_send_sync::<ConicalBoundaryError>();
    assert_send_sync::<ConicalBoundaryFailure>();
    assert_send_sync::<ConicalBoundaryProofFailure>();
    assert_send_sync::<ConicalBoundaryFailureKind>();
}
#[test]
fn original_profile_types_are_send_sync() {
    assert_send_sync::<ConicalOriginalSpan<'static>>();
    assert_send_sync::<ConicalOriginalFailure>();
}
#[test]
fn joined_reference_profile_types_are_send_sync() {
    assert_send_sync::<ConicalJoinSpan<'static>>();
    assert_send_sync::<ConicalJoinsQuery<'static>>();
    assert_send_sync::<ConicalJoinsFailure>();
    assert_send_sync::<ConicalJoinProofFailure>();
    assert_send_sync::<ConicalJoinFailureKind>();
    assert_send_sync::<ConicalJoinsEvidence>();
}
#[test]
fn reference_body_types_are_send_sync() {
    assert_send_sync::<ConicalReferenceMember<'static>>();
    assert_send_sync::<ConicalReferenceQuery<'static>>();
    assert_send_sync::<ConicalReferenceEvidence>();
    assert_send_sync::<ConicalReferenceError>();
    assert_send_sync::<ConicalReferenceFailure>();
}
#[test]
fn source_sector_types_are_send_sync() {
    assert_send_sync::<ConicalSectorWedge>();
    assert_send_sync::<ConicalSectorFailure>();
    assert_send_sync::<ConicalSectorQuery<'static>>();
    assert_send_sync::<ConicalSectorEvidence>();
}
#[test]
fn decoded_storage_types_are_send_sync() {
    assert_send_sync::<ConicalStorageQuery<'static>>();
    assert_send_sync::<ConicalStorageFailure>();
    assert_send_sync::<ConicalStorageEvidence>();
    assert_send_sync::<ConicalStorageError>();
}
#[test]
fn authored_flux_types_are_send_sync() {
    assert_send_sync::<ConicalAuthoredContour<'static>>();
    assert_send_sync::<ConicalAuthoredFluxQuery<'static>>();
    assert_send_sync::<ConicalAuthoredFlux>();
    assert_send_sync::<ConicalAuthoredFluxError>();
    assert_send_sync::<ConicalAuthoredFluxStage>();
}
#[test]
fn reference_volume_diagnostic_types_are_send_sync() {
    assert_send_sync::<ConicalReferenceVolumeTerm>();
    assert_send_sync::<ConicalReferenceVolumeFailure>();
}
