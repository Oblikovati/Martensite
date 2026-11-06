//! Owned input binding is separate from support injectivity and represented-face validity.
use super::{boundary, budget::*, carrier, decode, incidence, types::*};
use martensite_tkbrep::brep::face_input_capture::FaceInputSnapshot;
use martensite_tkg3d::geom::{StoredOpenExtrusionFailure, StoredOpenExtrusionInput};

/// Which actual representation failed admission; `OpenSupportInputRole::BoundaryCarrier`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenSupportInputRole {
    /// The actual stored basis of the selected face surface.
    SurfaceBasis,
    /// The actual spline carried by the declared VFirst cap occurrence.
    BoundaryCarrier,
}
/// Why no input binding was published; this never asserts a valid face.
#[derive(Debug)]
pub enum OpenSupportInputFailure {
    /// Existing finite decoder, rectangle, incidence or exact placement refusal.
    Face(ClosedFaceCompletion),
    /// The selected actual stored spline is unsupported or inconsistent.
    Spline {
        /// Representation's role in this input binding.
        role: OpenSupportInputRole,
        /// Invocation-local topology owner associated with the representation.
        owner: usize,
        /// Original storage/evaluator admission failure.
        cause: Box<StoredOpenExtrusionFailure>,
    },
}
/// Bound inputs for two separate open support operations; no face certificate is provided.
#[derive(Clone, Debug)]
pub struct PlacedOpenSupportInputs {
    pub(super) support: StoredOpenExtrusionInput,
    pub(super) carrier: StoredOpenExtrusionInput,
    pub(super) carrier_owner: usize,
    pub(super) carrier_occurrence: usize,
    pub(super) reference_parameter: f64,
}
impl PlacedOpenSupportInputs {
    /// Read the actual surface basis and its unmodified direction; `inputs.support()`.
    pub fn support(&self) -> &StoredOpenExtrusionInput {
        &self.support
    }
    /// Read the selected cap's actual coefficients and exact-checked local direction; `inputs.carrier()`.
    pub fn carrier(&self) -> &StoredOpenExtrusionInput {
        &self.carrier
    }
    /// Read the invocation-local cap owner and edge-use index; `inputs.carrier_binding()`.
    pub fn carrier_binding(&self) -> (usize, usize) {
        (self.carrier_owner, self.carrier_occurrence)
    }
    /// Read VFirst without translating stored poles; `inputs.reference_parameter()`.
    pub fn reference_parameter(&self) -> f64 {
        self.reference_parameter
    }
}
/// Immutable capture result, including its entire original input and actual finite work.
#[derive(Debug)]
pub struct OpenSupportInputReport {
    input: FaceInputSnapshot,
    limits: ClosedFacePolicy,
    work: ClosedFaceWork,
    outcome: Result<PlacedOpenSupportInputs, OpenSupportInputFailure>,
}
impl OpenSupportInputReport {
    /// Read the exact immutable source, not a later live handle; `report.input()`.
    pub fn input(&self) -> &FaceInputSnapshot {
        &self.input
    }
    /// Read the unchanged finite limits; `report.limits()`.
    pub fn limits(&self) -> ClosedFacePolicy {
        self.limits
    }
    /// Read actual decoder and exact identity work; `report.work()`.
    pub fn work(&self) -> ClosedFaceWork {
        self.work
    }
    /// Read input admission or the original typed refusal; `report.inputs()` is not face validity.
    pub fn inputs(&self) -> Result<&PlacedOpenSupportInputs, &OpenSupportInputFailure> {
        self.outcome.as_ref()
    }
}
/// Bind open-support inputs using existing rectangle/incidence and exact V3 placement checks.
/// `capture_placed_open_support_inputs(input, ClosedFacePolicy::placed_piecewise_boundary(), ||false)`.
/// No metric, seam/carrier identity, support injectivity or enclosing-body check is implied.
pub fn capture_placed_open_support_inputs(
    input: FaceInputSnapshot,
    limits: ClosedFacePolicy,
    mut cancelled: impl FnMut() -> bool,
) -> OpenSupportInputReport {
    let mut budget = Budget {
        policy: limits,
        work: ClosedFaceWork::default(),
        cancel: &mut cancelled,
    };
    let outcome = extract(&input, &mut budget);
    finished_report(input, limits, budget.work, outcome)
}
fn finished_report(
    input: FaceInputSnapshot,
    limits: ClosedFacePolicy,
    work: ClosedFaceWork,
    outcome: Result<PlacedOpenSupportInputs, OpenSupportInputFailure>,
) -> OpenSupportInputReport {
    OpenSupportInputReport {
        input,
        limits,
        work,
        outcome,
    }
}
struct BindingSource<'a> {
    owner: usize,
    strip: &'a decode::Strip,
    bounds: &'a boundary::Boundary,
}
fn extract<F: FnMut() -> bool>(
    input: &FaceInputSnapshot,
    budget: &mut Budget<'_, F>,
) -> Result<PlacedOpenSupportInputs, OpenSupportInputFailure> {
    if budget.policy.version != 3 {
        return Err(version(budget.policy.version));
    }
    let strip = decode::decode(input, budget).map_err(OpenSupportInputFailure::Face)?;
    let bounds = boundary::certify(&strip, budget).map_err(OpenSupportInputFailure::Face)?;
    incidence::certify(&strip, &bounds, budget).map_err(OpenSupportInputFailure::Face)?;
    bind(
        BindingSource {
            owner: input.root().owner,
            strip: &strip,
            bounds: &bounds,
        },
        budget,
    )
}
fn bind<F: FnMut() -> bool>(
    source: BindingSource<'_>,
    budget: &mut Budget<'_, F>,
) -> Result<PlacedOpenSupportInputs, OpenSupportInputFailure> {
    let support = admit_support(source.owner, source.strip, source.bounds)?;
    let (carrier, carrier_owner, carrier_occurrence) =
        carrier::open_inputs::capture(source.strip, source.bounds, budget)?;
    budget.poll().map_err(OpenSupportInputFailure::Face)?;
    Ok(PlacedOpenSupportInputs {
        support,
        carrier,
        carrier_owner,
        carrier_occurrence,
        reference_parameter: source.bounds.bounds[2],
    })
}
fn admit_support(
    owner: usize,
    strip: &decode::Strip,
    bounds: &boundary::Boundary,
) -> Result<StoredOpenExtrusionInput, OpenSupportInputFailure> {
    strip
        .basis
        .clone()
        .into_open_extrusion_input(strip.direction, [bounds.bounds[2], bounds.bounds[3]])
        .map_err(|cause| OpenSupportInputFailure::Spline {
            role: OpenSupportInputRole::SurfaceBasis,
            owner,
            cause: Box::new(cause),
        })
}
fn version(actual: u32) -> OpenSupportInputFailure {
    OpenSupportInputFailure::Face(unsupported(
        0,
        "open input placement policy",
        actual.to_string(),
        "explicit placed version 3 limits",
    ))
}
