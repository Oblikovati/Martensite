//! Transport checked rational developed boundaries into the existing normal sweeps.
use super::*;
use martensite_tkbo::body_check::ValidationReport;
use martensite_tkbo::profile_extrusion::{
    OperationDecline, PlanarProofVersion, PlanarProofWorkLimit, PlanarProofWorkReceipt,
    PlanarRegionWorkFailure, ProfileCurveId, SketchProfile,
    build_planar_regions_with_work_and_cancel,
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedRationalMeasure, RationalIntegralError, RationalReferenceFailure,
    RationalReferenceSpan, certify_rational_face_area,
};
mod domain;
mod fit;
mod input;
mod measure;
mod proxy;
mod retained;

/// Original cause of a declined rational-source normal extrusion.
#[derive(Debug)]
#[non_exhaustive]
pub enum RationalWrappedFailure {
    /// Unsupported or malformed input before requesting any body.
    Input {
        /// Actual field or entity context.
        actual: String,
        /// Required shape of the corresponding input.
        expected: &'static str,
    },
    /// Original rational source did not produce certified planar material.
    Planar(Box<PlanarRegionWorkFailure>),
    /// Source conic admission declined.
    Analytic(Box<OperationDecline>),
    /// The original signed boundary area did not produce a bounded enclosure.
    Measure(Box<RationalIntegralError>),
    /// Reference geometry or its monotone proof remained unresolved.
    Reference(Box<RationalReferenceFailure>),
    /// The existing polynomial wrapped proof or native construction declined.
    Wrapped(Box<WrappedNormalFailure>),
}
impl std::fmt::Display for RationalWrappedFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input { actual, expected } => write!(f, "{actual}; expected {expected}"),
            Self::Planar(cause) => cause.fmt(f),
            Self::Analytic(cause) => cause.fmt(f),
            Self::Measure(cause) => cause.fmt(f),
            Self::Reference(cause) => cause.fmt(f),
            Self::Wrapped(cause) => cause.fmt(f),
        }
    }
}
impl std::error::Error for RationalWrappedFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input { .. } => None,
            Self::Planar(cause) => Some(cause.as_ref()),
            Self::Analytic(cause) => Some(cause.as_ref()),
            Self::Measure(cause) => Some(cause.as_ref()),
            Self::Reference(cause) => Some(cause.as_ref()),
            Self::Wrapped(cause) => Some(cause.as_ref()),
        }
    }
}
type Checked<T> = Result<T, RationalWrappedFailure>;

fn invalid(actual: impl Into<String>, expected: &'static str) -> RationalWrappedFailure {
    RationalWrappedFailure::Input {
        actual: actual.into(),
        expected,
    }
}

/// A completed rational-source solid with original source history and volume evidence.
pub struct RationalWrappedResult {
    retained: retained::Retained,
    validation: ValidationReport,
    source: AuthoredSource,
    authored_volume: WrappedCertifiedRange,
    boundary_flux: WrappedCertifiedRange,
    volume_discrepancy: WrappedCertifiedBound,
    spatial_error: WrappedCertifiedBound,
}

impl RationalWrappedResult {
    /// Ordered checks of the retained solid. Example: `assert!(result.validation().is_valid());`.
    pub fn validation(&self) -> &ValidationReport {
        &self.validation
    }
    /// Original rational material area evidence. Example: `let area = result.authored_area();`.
    pub fn authored_area(&self) -> WrappedCertifiedRange {
        area_range(&self.source.area)
    }
    /// Original selected curves and vertices. Example: `result.authored_profile().curves.len()`.
    pub fn authored_profile(&self) -> &SketchProfile {
        &self.source.profile
    }
    /// Original directed material selection. Example: `result.authored_selection().region`.
    pub fn authored_selection(&self) -> &WrappedRegionSelection {
        &self.source.selection
    }
    /// Work spent certifying the original planar profile. Example: `result.authored_planar_work()`.
    pub fn authored_planar_work(&self) -> PlanarProofWorkReceipt {
        self.source.planar_work
    }
    /// Error-bounded integration receipt for the original source. Example: `result.authored_area_receipt().enclosure()`.
    pub fn authored_area_receipt(&self) -> &CertifiedRationalMeasure {
        &self.source.area
    }
    /// Original normal-sweep volume enclosure. Example: `let bounds = result.authored_volume();`.
    pub fn authored_volume(&self) -> WrappedCertifiedRange {
        self.authored_volume
    }
    /// Retained boundary-flux enclosure. Example: `let bounds = result.boundary_flux();`.
    pub fn boundary_flux(&self) -> WrappedCertifiedRange {
        self.boundary_flux
    }
    /// Outward bound between source volume and retained flux. Example: `result.volume_discrepancy().upper`.
    pub fn volume_discrepancy(&self) -> WrappedCertifiedBound {
        self.volume_discrepancy
    }
    /// Combined spatial bound in model units. Example: `let bound = result.spatial_error();`.
    pub fn spatial_error(&self) -> WrappedCertifiedBound {
        self.spatial_error
    }
    /// Original curve and parameter history. Example: `let history = result.history();`.
    pub fn history(&self) -> &[WrappedHistoryRecord] {
        self.retained.history()
    }
    /// Frozen native bytes. Example: `let brep = result.encoded_brep();`.
    pub fn encoded_brep(&self) -> &[u8] {
        self.retained.encoded_brep()
    }
    /// Detached native body and original authored keys. Example: `let body = result.export_native()?;`.
    pub fn export_native(&self) -> Result<DetachedWrappedBodies, WrappedNormalFailure> {
        self.retained.export_native()
    }
}

/// Extrude a selected region with lines, Bernstein splines, or exact analytic quarters.
/// Requires one material region and increasing depths away from the cylindrical axis.
/// The reference level specifies the dyadic intervals used in the original curve parameter.
/// Example: `let body = normal_extrude_rational_cylinder(&request, 4, || false)?;`.
pub fn normal_extrude_rational_cylinder(
    request: &WrappedNormalRequest,
    reference_level: u8,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<RationalWrappedResult> {
    super::input::poll(&mut cancelled, WrappedStage::Admission).map_err(wrapped_failure)?;
    let source = input::read(&request.profile, &request.selections, request.options)?;
    let support = domain::cylinder(request)?;
    let prepared = prepare(
        source,
        request.options,
        support,
        reference_level,
        &mut cancelled,
    )?;
    let native = prepared.proxy.cylinder(request);
    let inputs = super::input::read(&native, &mut cancelled)
        .map_err(|cause| prepared.proxy.geometry_failure(cause))?;
    let proof = PlanarProofVersion::GeometricCompositeV6 {
        max_components: 64,
        max_total_subintervals: 100_000,
    };
    let retained = operation::from_inputs_with_proof(&native, inputs, proof, &mut cancelled)
        .map_err(|cause| prepared.proxy.geometry_failure(cause))?;
    prepared.finish(retained.into(), request.options.volume, &mut cancelled)
}

/// Extrude a selected region with lines, Bernstein splines, or exact analytic quarters on a cone.
/// Requires increasing depths, one chart in the positive developed halfplane, and clearance
/// from the apex throughout the sweep. Example: `normal_extrude_rational_cone(&request, 4, || false)?`.
pub fn normal_extrude_rational_cone(
    request: &WrappedConeNormalRequest,
    reference_level: u8,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Checked<RationalWrappedResult> {
    super::input::poll(&mut cancelled, WrappedStage::Admission).map_err(wrapped_failure)?;
    let source = input::read(&request.profile, &request.selections, request.options)?;
    let support = domain::cone(request, &source)?;
    let prepared = prepare(
        source,
        request.options,
        support,
        reference_level,
        &mut cancelled,
    )?;
    let native = prepared.proxy.cone(request);
    let retained = normal_extrude_cone_profile_with_cancel(&native, &mut cancelled)
        .map_err(|cause| prepared.proxy.geometry_failure(cause))?;
    prepared.finish(retained.into(), request.options.volume, &mut cancelled)
}

fn wrapped_failure(cause: WrappedNormalFailure) -> RationalWrappedFailure {
    RationalWrappedFailure::Wrapped(Box::new(cause))
}

struct Prepared {
    proxy: proxy::Proxy,
    source: AuthoredSource,
    inverse_slant_error: f64,
}

struct AuthoredSource {
    profile: SketchProfile,
    selection: WrappedRegionSelection,
    area: CertifiedRationalMeasure,
    planar_work: PlanarProofWorkReceipt,
}

fn area_range(area: &CertifiedRationalMeasure) -> WrappedCertifiedRange {
    let [lower, upper] = area.enclosure();
    WrappedCertifiedRange { lower, upper }
}

fn prepare(
    source: input::Source<'_>,
    options: WrappedNormalOptions,
    support: domain::Support,
    reference_level: u8,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Prepared> {
    let (area, planar_work) = original_area(options, &source, cancelled)?;
    let scale = support.scale();
    let limit = (options.spatial_error / (4. * scale)).next_down();
    let reference = fit::cover(reference_level, &source, limit, cancelled)?;
    support.check(&source, &reference)?;
    let proxy_options = reference_options(options, planar_work)?;
    let proxy = proxy::build(proxy_options, &source, &reference, scale)?;
    let source = AuthoredSource {
        profile: source.profile,
        selection: source.selection.clone(),
        area,
        planar_work,
    };
    let inverse_slant_error = support.inverse_slant_error(&reference);
    Ok(Prepared {
        proxy,
        source,
        inverse_slant_error,
    })
}

fn reference_options(
    mut options: WrappedNormalOptions,
    work: PlanarProofWorkReceipt,
) -> Checked<WrappedNormalOptions> {
    let available = u64::from(work.effective_pair_bisections);
    let remaining = available
        .checked_sub(work.consumed_pair_bisections)
        .ok_or_else(|| {
            invalid(
                format!("authored planar work={work:?}"),
                "consumed bisections within the requested allowance",
            )
        })?;
    options.planar_work = PlanarProofWorkLimit::PairBisections(remaining as u16);
    Ok(options)
}

impl Prepared {
    fn finish(
        self,
        mut retained: retained::Retained,
        policy: WrappedVolumePolicy,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<RationalWrappedResult> {
        let authored_volume = retained.authored_volume(
            area_range(&self.source.area),
            self.proxy.selection.region,
            self.inverse_slant_error,
        )?;
        self.proxy.history(retained.history_mut())?;
        let (validation, boundary_flux) = retained.report(self.proxy.selection.region)?;
        let volume_discrepancy = verify_volume(policy, boundary_flux, authored_volume)?;
        super::input::poll(cancelled, WrappedStage::Publication).map_err(wrapped_failure)?;
        Ok(RationalWrappedResult {
            validation: validation.clone(),
            retained,
            source: self.source,
            authored_volume,
            boundary_flux,
            volume_discrepancy,
            spatial_error: WrappedCertifiedBound {
                upper: self.proxy.spatial_error,
            },
        })
    }
}

fn original_area(
    options: WrappedNormalOptions,
    source: &input::Source<'_>,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<(CertifiedRationalMeasure, PlanarProofWorkReceipt)> {
    let partition = source
        .curves
        .iter()
        .any(|curve| curve.spans.len() > 1)
        .then(|| proxy::native_partition(source, options))
        .transpose()?;
    let (profile, selection) = partition
        .as_ref()
        .map_or((&source.profile, source.selection), |p| {
            (&p.profile, &p.selection)
        });
    let regions = build_planar_regions_with_work_and_cancel(
        profile,
        pipeline::resolution(options.spatial_error),
        options.planar_work,
        &mut *cancelled,
    )
    .map_err(|cause| RationalWrappedFailure::Planar(Box::new(cause)))?;
    input::select_cell(selection, regions.regions())?;
    let outer = source.area_wire(&source.selection.outer)?;
    let holes = source
        .selection
        .holes
        .iter()
        .map(|boundary| source.area_wire(boundary))
        .collect::<Checked<Vec<_>>>()?;
    let holes: Vec<_> = holes.iter().map(Vec::as_slice).collect();
    let area = certify_rational_face_area(&outer, &holes, Default::default(), cancelled)
        .map_err(|cause| RationalWrappedFailure::Measure(Box::new(cause)))?;
    Ok((area, regions.proof_work()))
}

fn verify_volume(
    policy: WrappedVolumePolicy,
    flux: WrappedCertifiedRange,
    volume: WrappedCertifiedRange,
) -> Checked<WrappedCertifiedBound> {
    let difference = (volume.upper - flux.lower)
        .max(flux.upper - volume.lower)
        .next_up();
    let permitted = match policy {
        WrappedVolumePolicy::Refuse(limit) => difference <= limit,
        _ => true,
    };
    if !difference.is_finite() || !permitted {
        return Err(invalid(
            format!("authored-volume discrepancy upper={difference}, policy={policy:?}"),
            "a finite certified discrepancy within any requested refusal limit",
        ));
    }
    Ok(WrappedCertifiedBound { upper: difference })
}
