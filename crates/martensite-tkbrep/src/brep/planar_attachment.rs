// SPDX-License-Identifier: GPL-2.0-only
//! Freeze the actual planar attachment domain of one selected native face.
pub(super) mod capture;
mod cubic;
mod curve;
pub use cubic::{FaceAttachmentCubic, FaceAttachmentCubicError};

use curve::PlanarSupport;
pub use curve::{DetachedPlanarCurve, FaceAttachmentCurve, FaceAttachmentCurveError};

use crate::{
    brep::face_input_capture::{
        FaceCaptureBehavior, FaceCaptureIncomplete, FaceCaptureLimits,
        capture_face_input_with_behavior,
    },
    topods::{Face, ShapeIterator, Solid},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::UnitPlacementError;
use martensite_tkmath::predicates::roots::metric_bounds::planar_attachment::CertifiedPlanarAttachment;
pub use martensite_tkmath::predicates::roots::metric_bounds::planar_attachment::{
    PlanarAttachmentChart, PlanarAttachmentError, PlanarAttachmentLocation, PlanarAttachmentQuery,
    PlanarCubicCertificate, PlanarCubicOptions, PlanarCubicProofFailure, PlanarCurveCertificate,
    PlanarCurvePath, PlanarCurveProofFailure, PlanarMonotoneAxis, PlanarSegmentContact,
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
    /// The runtime root kind must be a native solid.
    BodyKind(ShapeEnum),
    /// The source body must contain direct shells and faces of the named kinds.
    BodyChildKind {
        /// Runtime kind of the visited child.
        actual: ShapeEnum,
        /// Required child kind.
        expected: ShapeEnum,
    },
    /// Native traversal failed before uniqueness was established.
    Traversal(martensite_tkernel::standard::Failure),
    /// Coherent source capture was incomplete.
    Capture(FaceCaptureIncomplete),
    /// A captured occurrence conflicts with the supported topology.
    Topology(usize),
    /// The local support or a required stored carrier is outside the finite affine domain.
    Representation(usize),
    /// A stored datum or the effective native map is outside the unit-scale placement domain.
    Placement(UnitPlacementError),
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
            Self::BodyKind(actual) => write!(
                output,
                "attachment source body is {actual:?}; expected Solid"
            ),
            Self::BodyChildKind { actual, expected } => write!(
                output,
                "attachment source body child is {actual:?}, expected {expected:?}"
            ),
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
            Self::Placement(cause) => write!(output, "attachment placement refused ({cause})"),
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
            Self::Placement(cause) => Some(cause),
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
    support: PlanarSupport,
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
        self.require_current(current, &mut cancelled)?;
        self.proof
            .classify(point, distance_limit, cancelled)
            .map_err(numeric_failure)
    }

    /// Check a supplied UV lift against the frozen trim and its proposed world point.
    /// Example: attachment.classify_uv(current, uv, world, tolerance, || false).
    pub fn classify_uv(
        &self,
        current: AttachmentSource,
        uv: [f64; 2],
        world: [f64; 3],
        distance_limit: f64,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarAttachmentQuery, FaceAttachmentError> {
        self.require_current(current, &mut cancelled)?;
        self.proof
            .classify_uv(uv, world, distance_limit, cancelled)
            .map_err(numeric_failure)
    }

    /// Read nominal solver coefficients from this same original affine chart.
    /// Example: attachment.solver_chart(current, accepted_uv).
    pub fn solver_chart(
        &self,
        current: AttachmentSource,
        anchor: [f64; 2],
    ) -> Result<PlanarAttachmentChart, FaceAttachmentError> {
        self.require_source(current)?;
        self.proof.solver_chart(anchor).map_err(numeric_failure)
    }

    /// Classify a directed segment against this exact finite native trim.
    /// Example: `attachment.classify_segment(current, endpoints, || false)?`.
    pub fn classify_segment(
        &self,
        current: AttachmentSource,
        endpoints: [[f64; 3]; 2],
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<PlanarSegmentContact, FaceAttachmentError> {
        self.require_current(current, &mut cancelled)?;
        self.proof
            .classify_segment(endpoints, cancelled)
            .map_err(numeric_failure)
    }

    fn require_current(
        &self,
        current: AttachmentSource,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), FaceAttachmentError> {
        poll(cancelled)?;
        self.require_source(current)
    }

    fn require_source(&self, current: AttachmentSource) -> Result<(), FaceAttachmentError> {
        if current != self.source {
            return Err(FaceAttachmentError::StaleSource {
                captured: self.source,
                current,
            });
        }
        Ok(())
    }
}

/// Bind the actual native face to a host source stamp and certify both complete linear loops.
/// The input body topology is stable under the caller's source transaction. The face and its
/// support may have one common unit-scale placement on the face occurrence. Inner locations, seams,
/// curves and touching or concave loops are outside this first domain. The owner
/// limit also bounds the shell and face visits used to establish unique membership.
/// Example: capture_planar_attachment(&body, &face, source, limits, || false).
pub fn capture_planar_attachment(
    body: &Solid,
    face: &Face,
    source: AttachmentSource,
    limits: FaceCaptureLimits,
    mut cancelled: impl FnMut() -> bool,
) -> Result<FaceAttachment, FaceAttachmentError> {
    capture_domain(
        body,
        face,
        source,
        limits,
        TrimScope::RequireHole,
        &mut cancelled,
    )
}

/// Bind one exact native face for Split with a convex finite outer loop and at most one convex hole.
/// Membership and placement are checked under the caller's stable body revision.
/// Example: `capture_planar_split_domain(&body, &face, source, limits, || false)?`.
pub fn capture_planar_split_domain(
    body: &Solid,
    face: &Face,
    source: AttachmentSource,
    limits: FaceCaptureLimits,
    mut cancelled: impl FnMut() -> bool,
) -> Result<FaceAttachment, FaceAttachmentError> {
    capture_domain(
        body,
        face,
        source,
        limits,
        TrimScope::OptionalHole,
        &mut cancelled,
    )
}

#[derive(Clone, Copy)]
enum TrimScope {
    RequireHole,
    OptionalHole,
}

fn capture_domain(
    body: &Solid,
    face: &Face,
    source: AttachmentSource,
    limits: FaceCaptureLimits,
    scope: TrimScope,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<FaceAttachment, FaceAttachmentError> {
    member(body, face, limits.owners, cancelled)?;
    let snapshot = capture_face_input_with_behavior(
        face,
        limits,
        FaceCaptureBehavior::StoredSurfaceQueriesV2,
        &mut *cancelled,
    )
    .map_err(capture_failure)?;
    let (proof, support) = capture::prove(&snapshot, scope, cancelled)?;
    member(body, face, limits.owners, cancelled)?;
    poll(cancelled)?;
    Ok(FaceAttachment {
        source,
        proof,
        support,
    })
}

pub(super) fn member(
    body: &Solid,
    face: &Face,
    limit: usize,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(), FaceAttachmentError> {
    poll(cancelled)?;
    let map = FaceAttachmentError::Traversal;
    let actual = body.shape_type().map_err(map)?;
    if actual != ShapeEnum::Solid {
        return Err(FaceAttachmentError::BodyKind(actual));
    }
    let mut census = Census {
        visited: 0,
        limit,
        found: false,
    };
    for shell in ShapeIterator::new(body, true, true) {
        census.visit(&shell, ShapeEnum::Shell, cancelled)?;
        for occurrence in ShapeIterator::new(&shell, true, true) {
            census.visit(&occurrence, ShapeEnum::Face, cancelled)?;
            if occurrence.is_equal(face) && std::mem::replace(&mut census.found, true) {
                return Err(FaceAttachmentError::FaceOccurrence);
            }
        }
    }
    census
        .found
        .then_some(())
        .ok_or(FaceAttachmentError::FaceOccurrence)
}

struct Census {
    visited: usize,
    limit: usize,
    found: bool,
}

impl Census {
    fn visit(
        &mut self,
        node: &crate::topods::Shape,
        expected: ShapeEnum,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<(), FaceAttachmentError> {
        poll(cancelled)?;
        self.visited += 1;
        if self.visited > self.limit {
            return Err(capture_failure(FaceCaptureIncomplete::Limit {
                actual: self.visited,
                maximum: self.limit,
                dimension: "source body descendants",
            }));
        }
        let actual = node.shape_type().map_err(FaceAttachmentError::Traversal)?;
        if actual != expected {
            return Err(FaceAttachmentError::BodyChildKind { actual, expected });
        }
        Ok(())
    }
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

pub(super) fn capture_failure(cause: FaceCaptureIncomplete) -> FaceAttachmentError {
    match cause {
        FaceCaptureIncomplete::Cancelled => FaceAttachmentError::Cancelled,
        cause => FaceAttachmentError::Capture(cause),
    }
}
