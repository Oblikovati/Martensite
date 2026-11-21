//! Reuse native semantic-reference encoding, with coefficient-preserving readers.
use super::*;
use martensite_tkbrep::{
    bintools::{CurveFrameReadPolicy, LocationTransformReadPolicy, SurfaceFrameReadPolicy},
    topods,
};
pub(super) fn encode(attempt: &Attempt, body: &Shape, face: &Face) -> Checked<Vec<u8>> {
    attempt.encoding(encoding::encode(
        body,
        &[(attempt.id, face.clone().into_shape())],
    ))
}
pub(super) fn decode(attempt: &Attempt, bytes: &[u8]) -> Checked<DetachedCylinderSupport> {
    let (body, mut bindings) = attempt.encoding(encoding::decode_with_geometry_policies(
        bytes,
        &[attempt.id],
        SurfaceFrameReadPolicy::PreserveStoredV1,
        LocationTransformReadPolicy::PreserveStoredRigidV1,
        CurveFrameReadPolicy::PreserveStoredV1,
    ))?;
    let (_, selected) = bindings
        .pop()
        .ok_or_else(|| attempt.reject(AttachmentFailureCause::Membership { occurrences: 0 }))?;
    let face = attempt.native("selected face reference", topods::face(&selected))?;
    admission::selection(attempt, &body, &face)?;
    Ok(DetachedCylinderSupport {
        body,
        face,
        id: attempt.id,
        revision: attempt.revision,
    })
}
pub(super) fn restore(
    record: CylinderAttachmentRecord,
    attempt: Attempt,
) -> Checked<CylinderFaceAttachment> {
    let detached = decode(&attempt, &record.brep)?;
    let replayed = encode(&attempt, &detached.body, &detached.face)?;
    compare(&attempt, &record.brep, &replayed)?;
    let validation = admission::validation(&attempt, &detached.body, &record.resolution)?;
    Ok(CylinderFaceAttachment { record, validation })
}
pub(super) fn compare(attempt: &Attempt, expected: &[u8], actual: &[u8]) -> Checked<()> {
    if expected == actual {
        return Ok(());
    }
    let offset = expected
        .iter()
        .zip(actual)
        .position(|(a, b)| a != b)
        .unwrap_or(expected.len().min(actual.len()));
    Err(attempt.reject(AttachmentFailureCause::Replay {
        offset,
        expected_length: expected.len(),
        actual_length: actual.len(),
        expected: expected.get(offset).copied(),
        actual: actual.get(offset).copied(),
    }))
}
