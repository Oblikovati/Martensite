// SPDX-License-Identifier: GPL-2.0-only
//! Freeze the actual planar attachment domain of one selected native face.
mod capture;

use crate::{
    brep::face_input_capture::{
        FaceCaptureBehavior, FaceCaptureIncomplete, FaceCaptureLimits,
        capture_face_input_with_behavior,
    },
    topexp::Explorer,
    topods::{Face, Solid},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::predicates::roots::metric_bounds::planar_attachment::CertifiedPlanarAttachment;
pub use martensite_tkmath::predicates::roots::metric_bounds::planar_attachment::{
    PlanarAttachmentError, PlanarAttachmentLocation, PlanarAttachmentQuery,
};
use std::{error::Error, fmt, num::NonZeroU64};

/// Host authority for an exact body occurrence in the source transaction.
/// The host resolves identity and establishes the revision before taking a native capture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttachmentSource {
    /// Source document identity.
    pub document: [u8; 16],
    /// Stable body identity in the document.
    pub body: NonZeroU64,
    /// Stable selected face occurrence within that body.
    pub face: NonZeroU64,
    /// Source incarnation, changed when an identity is retired or recreated.
    pub incarnation: u64,
    /// Source geometry and placement revision within the incarnation.
    pub revision: u64,
}

/// Why original native face attachment evidence was not completed.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum FaceAttachmentError {
    /// The selected located, oriented face did not occur exactly once under the source body.
    FaceOccurrence,
    /// Native traversal failed before uniqueness was established.
    Traversal(martensite_tkernel::standard::Failure),
    /// Coherent source capture was incomplete.
    Capture(FaceCaptureIncomplete),
    /// A captured occurrence conflicts with the supported topology.
    Topology(usize),
    /// The local support or a required stored carrier is outside the finite affine domain.
    Representation(usize),
    /// Certification found inconsistent trim geometry or could not finish a query.
    Numeric(PlanarAttachmentError),
    /// The host's current source stamp differs from the captured stamp.
    StaleSource {
        /// Source bound when this proof was completed.
        captured: AttachmentSource,
        /// Host source for the requested query.
        current: AttachmentSource,
    },
    /// Preparation or the query was cancelled.
    Cancelled,
}

impl fmt::Display for FaceAttachmentError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FaceOccurrence => {
                output.write_str("expected one matching located and oriented source face")
            }
            Self::Traversal(cause) => write!(
                output,
                "attachment could not traverse source faces ({cause})"
            ),
            Self::Capture(cause) => {
                write!(output, "attachment could not capture source ({cause:?})")
            }
            Self::Topology(owner) => write!(
                output,
                "attachment source owner {owner} requires supported directed topology"
            ),
            Self::Representation(owner) => write!(
                output,
                "attachment source owner {owner} requires an admitted affine representation"
            ),
            Self::Numeric(cause) => write!(output, "attachment proof did not complete ({cause})"),
            Self::StaleSource { captured, current } => write!(
                output,
                "attachment source {current:?} must match captured source {captured:?}"
            ),
            Self::Cancelled => output.write_str("planar attachment was cancelled"),
        }
    }
}

impl Error for FaceAttachmentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Traversal(cause) => Some(cause),
            Self::Numeric(cause) => Some(cause),
            _ => None,
        }
    }
}

/// Immutable evidence for the supported trims of a selected face occurrence.
#[derive(Clone, Debug)]
pub struct FaceAttachment {
    source: AttachmentSource,
    proof: CertifiedPlanarAttachment,
}

impl FaceAttachment {
    /// The exact host source bound to this proof. Example: attachment.source().
    pub fn source(&self) -> AttachmentSource {
        self.source
    }

    /// Check the source and classify a world point using the frozen original trim.
    /// Example: attachment.classify(current, [-1., 0., 0.], 0., || false).
    pub fn classify(
        &self,
        current: AttachmentSource,
        point: [f64; 3],
        distance_limit: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarAttachmentQuery, FaceAttachmentError> {
        poll(&mut cancelled)?;
        if current != self.source {
            return Err(FaceAttachmentError::StaleSource {
                captured: self.source,
                current,
            });
        }
        self.proof
            .classify(point, distance_limit, cancelled)
            .map_err(numeric_failure)
    }
}

/// Bind the actual native face to a host source stamp and certify both complete linear loops.
/// The input body topology is stable under the caller's source transaction. The face and its
/// support may have one common placement on the face occurrence. Inner locations, seams,
/// curves and touching or concave loops are outside this first domain.
/// Example: capture_planar_attachment(&body, &face, source, limits, || false).
pub fn capture_planar_attachment(
    body: &Solid,
    face: &Face,
    source: AttachmentSource,
    limits: FaceCaptureLimits,
    mut cancelled: impl FnMut() -> bool,
) -> Result<FaceAttachment, FaceAttachmentError> {
    member(body, face, limits.owners, &mut cancelled)?;
    let snapshot = capture_face_input_with_behavior(
        face,
        limits,
        FaceCaptureBehavior::StoredSurfaceQueriesV2,
        &mut cancelled,
    )
    .map_err(capture_failure)?;
    let proof = capture::prove(&snapshot, &mut cancelled)?;
    member(body, face, limits.owners, &mut cancelled)?;
    poll(&mut cancelled)?;
    Ok(FaceAttachment { source, proof })
}

fn member(
    body: &Solid,
    face: &Face,
    limit: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), FaceAttachmentError> {
    poll(cancelled)?;
    let map = FaceAttachmentError::Traversal;
    let mut explorer = Explorer::new(body, ShapeEnum::Face, ShapeEnum::Shape).map_err(map)?;
    let mut found = false;
    let mut visited = 0;
    while explorer.more() {
        poll(cancelled)?;
        visited += 1;
        if visited > limit {
            return Err(capture_failure(FaceCaptureIncomplete::Limit {
                actual: visited,
                maximum: limit,
                dimension: "source face occurrences",
            }));
        }
        if explorer.current().is_equal(face) && std::mem::replace(&mut found, true) {
            return Err(FaceAttachmentError::FaceOccurrence);
        }
        explorer.next().map_err(map)?;
    }
    found
        .then_some(())
        .ok_or(FaceAttachmentError::FaceOccurrence)
}

fn poll(cancelled: &mut impl FnMut() -> bool) -> Result<(), FaceAttachmentError> {
    if cancelled() {
        return Err(FaceAttachmentError::Cancelled);
    }
    Ok(())
}

fn numeric_failure(cause: PlanarAttachmentError) -> FaceAttachmentError {
    match cause {
        PlanarAttachmentError::Cancelled => FaceAttachmentError::Cancelled,
        cause => FaceAttachmentError::Numeric(cause),
    }
}

fn capture_failure(cause: FaceCaptureIncomplete) -> FaceAttachmentError {
    match cause {
        FaceCaptureIncomplete::Cancelled => FaceAttachmentError::Cancelled,
        cause => FaceAttachmentError::Capture(cause),
    }
}
