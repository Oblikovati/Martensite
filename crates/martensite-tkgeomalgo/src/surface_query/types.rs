//! Query ownership and operation scope are explicit; a surface proof does not validate trims.
use martensite_tkbrep::brep::face_input_capture::{
    FaceCaptureIncomplete, FaceCaptureLimits, FaceInputSnapshot,
};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::predicates::roots::surface_query::{SurfaceQueryPolicy, SurfaceQueryReport};
/// The exact requested self-intersection domain and unchanged solver settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceSelfQuery {
    /// Actual U and V adaptor bounds, without clipping or reconstruction.
    pub bounds: [[f64; 2]; 2],
    /// Actual adaptor periodicity flags.
    pub periodic: [bool; 2],
    /// Actual arc and tangency tolerance settings; neither is widened here.
    pub tolerances: [f64; 2],
    /// Actual tangent-contact option.
    pub tangency: bool,
}
/// Versioned sufficient query embedding policy; `FaceQueryPolicy::bounded()`.
#[derive(Clone, Copy, Debug)]
pub struct FaceQueryPolicy {
    /// Supported behavior version, currently one.
    pub version: u32,
    /// Finite coherent input capture limits.
    pub capture: FaceCaptureLimits,
    /// Exact numeric work limits.
    pub proof: SurfaceQueryPolicy,
}
impl FaceQueryPolicy {
    /// Select finite limits without changing existing validation policies; `FaceQueryPolicy::bounded()`.
    pub const fn bounded() -> Self {
        Self {
            version: 1,
            capture: FaceCaptureLimits::bounded(),
            proof: SurfaceQueryPolicy::bounded(),
        }
    }
}
/// A missing owned query obligation, with its actual invocation-local source owner.
#[derive(Clone, Debug, PartialEq)]
pub enum FaceQueryIncomplete {
    /// The requested behavior version is unsupported.
    Version(u32),
    /// Coherent source capture was unavailable.
    Capture(FaceCaptureIncomplete),
    /// A required source/domain relation is outside the admitted family.
    Binding {
        /// Actual invocation-local source owner.
        owner: usize,
        /// Required relation that was not established.
        relation: &'static str,
        /// Actual observed shape or value, without silently substituting it.
        actual: String,
    },
    /// The actual stored layout query failed.
    Geometry {
        /// Actual invocation-local geometry owner.
        owner: usize,
        /// Underlying arithmetic/storage failure.
        cause: Box<Failure>,
    },
    /// The exact theorem remained incomplete.
    Proof {
        /// Actual invocation-local surface owner.
        owner: usize,
        /// Exact arithmetic's missing obligation.
        cause: martensite_tkmath::predicates::roots::surface_query::SurfaceQueryCause,
    },
    /// Cancellation observed before or after held input leases.
    Cancelled,
}
/// Immutable request/source/proof join; it claims only query embedding modulo declared U aliases.
#[derive(Clone, Debug)]
pub struct FaceQueryReport {
    pub(super) request: SurfaceSelfQuery,
    pub(super) policy: FaceQueryPolicy,
    pub(super) input: Option<FaceInputSnapshot>,
    pub(super) proof: Option<SurfaceQueryReport>,
    pub(super) surface_owner: Option<usize>,
    pub(super) incomplete: Option<FaceQueryIncomplete>,
}
impl FaceQueryReport {
    /// Read actual unchanged requested solver settings; `report.request()`.
    pub fn request(&self) -> SurfaceSelfQuery {
        self.request
    }
    /// Read unchanged finite policy limits; `report.policy()`.
    pub fn policy(&self) -> FaceQueryPolicy {
        self.policy
    }
    /// Read coherent owned face input, if capture completed; `report.input()`.
    pub fn input(&self) -> Option<&FaceInputSnapshot> {
        self.input.as_ref()
    }
    /// Read the actual root-face support owner, never a coordinate-derived identity; `report.surface_owner()`.
    pub fn surface_owner(&self) -> Option<usize> {
        self.surface_owner
    }
    /// Read all executed exact work, even after incomplete proof; `report.proof()`.
    pub fn proof(&self) -> Option<&SurfaceQueryReport> {
        self.proof.as_ref()
    }
    /// Read missing obligations without treating them as valid; `report.incomplete()`.
    pub fn incomplete(&self) -> Option<&FaceQueryIncomplete> {
        self.incomplete.as_ref()
    }
    /// Observe complete query embedding only; `report.is_certified()` does not validate face trims.
    pub fn is_certified(&self) -> bool {
        self.incomplete.is_none()
            && self
                .proof
                .as_ref()
                .is_some_and(|proof| proof.certificate().is_some())
    }
}
pub(super) fn binding(
    owner: usize,
    relation: &'static str,
    actual: impl std::fmt::Debug,
) -> FaceQueryIncomplete {
    FaceQueryIncomplete::Binding {
        owner,
        relation,
        actual: format!("{actual:?}"),
    }
}
pub(super) fn require(
    condition: bool,
    owner: usize,
    relation: &'static str,
    actual: impl std::fmt::Debug,
) -> Result<(), FaceQueryIncomplete> {
    if condition {
        return Ok(());
    }
    Err(binding(owner, relation, actual))
}
