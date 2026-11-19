//! Reuse certified planar capture; select material only by authored directed boundaries.
use super::*;
use martensite_tkbo::profile_extrusion::*;
use std::collections::BTreeMap;
#[cfg(test)]
#[path = "planar_order_tests.rs"]
mod order_tests;
#[cfg(test)]
#[path = "planar_tests.rs"]
mod tests;

#[path = "planar_joins.rs"]
mod joins;
#[path = "planar_sources.rs"]
mod source;

type LoopIdentity = Vec<(ProfileCurveId, bool)>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct MaterialIdentity {
    outer: LoopIdentity,
    holes: Vec<LoopIdentity>,
}

pub(super) struct PlanarEvidence {
    pub area: Measured,
    pub selected_region: RegionId,
    #[cfg(test)]
    pub certificate: RegionCertificate,
    identity: MaterialIdentity,
    source: source::SourceSnapshot,
    integral: Option<PlanarIntegralEvidence>,
}

impl PlanarEvidence {
    pub(super) fn matches_request(&self, request: &DevelopedRegionRequest) -> bool {
        requested_identity(request).is_ok_and(|id| id == self.identity)
            && self.source.matches(request)
            && self.integral.as_ref().is_none_or(|proof| {
                combined_profile(std::slice::from_ref(request))
                    .is_ok_and(|profile| proof.matches_profile(&profile))
            })
    }
}

#[derive(Debug)]
pub(super) enum PlanarFailure {
    #[cfg(test)]
    Capture(Box<OperationDecline>),
    WorkCapture(Box<PlanarRegionWorkFailure>),
    Selection {
        expected: MaterialIdentity,
        candidates: Vec<MaterialIdentity>,
    },
    SplitSource {
        curve: SurfaceCurveId,
        level: u8,
        index: u32,
    },
    NonAuthoredVertex(CornerSource),
    EmptyMaterial,
    Join(Box<joins::JoinFailure>),
}
impl std::fmt::Display for PlanarFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Join(cause) => write!(f, "planar join: {cause}"),
            Self::WorkCapture(cause) => write!(f, "planar authored material: {cause}"),
            #[cfg(test)]
            Self::Capture(cause) => write!(f, "planar authored material: {cause}"),
            Self::Selection {
                expected,
                candidates,
            } => selection_message(f, expected, candidates),
            Self::SplitSource {
                curve,
                level,
                index,
            } => source_message(f, *curve, *level, *index),
            Self::NonAuthoredVertex(source) => vertex_message(f, *source),
            Self::EmptyMaterial => write!(
                f,
                "material loops=0; expected an outer boundary with optional holes"
            ),
        }
    }
}

fn vertex_message(f: &mut std::fmt::Formatter<'_>, source: CornerSource) -> std::fmt::Result {
    write!(f, "vertex={source:?}; expected an original authored vertex")
}

fn selection_message(
    f: &mut std::fmt::Formatter<'_>,
    expected: &MaterialIdentity,
    candidates: &[MaterialIdentity],
) -> std::fmt::Result {
    write!(
        f,
        "planar material candidates={candidates:?}; expected exactly one region with directed boundaries={expected:?}"
    )
}

fn source_message(
    f: &mut std::fmt::Formatter<'_>,
    curve: SurfaceCurveId,
    level: u8,
    index: u32,
) -> std::fmt::Result {
    write!(
        f,
        "authored curve={curve:?}, interval level={level}, index={index}; expected original interval [0,1]"
    )
}

#[cfg(test)]
pub(super) fn certify(request: &DevelopedRegionRequest) -> Result<PlanarEvidence, PlanarFailure> {
    let captured = capture(std::slice::from_ref(request), request.tolerance)?;
    select(request, &captured)
}
#[cfg(test)]
pub(super) fn capture(
    requests: &[impl AsRef<DevelopedRegionRequest>],
    tolerance: f64,
) -> Result<PlanarRegionSet, PlanarFailure> {
    let profile = combined_profile(requests)?;
    build_planar_regions(&profile, super::pipeline::resolution(tolerance))
        .map_err(|cause| PlanarFailure::Capture(Box::new(cause)))
}
#[cfg(test)]
pub(super) fn capture_with_work(
    requests: &[impl AsRef<DevelopedRegionRequest>],
    tolerance: f64,
    work: PlanarProofWorkLimit,
) -> Result<PlanarRegionSet, PlanarFailure> {
    let profile = combined_profile(requests)?;
    build_planar_regions_with_work(&profile, super::pipeline::resolution(tolerance), work)
        .map_err(|cause| PlanarFailure::WorkCapture(Box::new(cause)))
}
pub(super) fn capture_with_work_and_cancel(
    requests: &[impl AsRef<DevelopedRegionRequest>],
    tolerance: f64,
    work: PlanarProofWorkLimit,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<PlanarRegionSet, PlanarFailure> {
    let profile = combined_profile(requests)?;
    build_planar_regions_with_work_and_cancel(
        &profile,
        super::pipeline::resolution(tolerance),
        work,
        cancelled,
    )
    .map_err(|cause| PlanarFailure::WorkCapture(Box::new(cause)))
}
pub(super) fn capture_with_proof_and_cancel(
    requests: &[impl AsRef<DevelopedRegionRequest>],
    tolerance: f64,
    work: PlanarProofWorkLimit,
    version: PlanarProofVersion,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<PlanarRegionSet, PlanarFailure> {
    let profile = combined_profile(requests)?;
    build_planar_regions_with_proof_and_cancel(
        &profile,
        super::pipeline::resolution(tolerance),
        work,
        version,
        cancelled,
    )
    .map_err(|cause| PlanarFailure::WorkCapture(Box::new(cause)))
}

pub(super) fn capture_with_integration_budget_and_cancel(
    requests: &[impl AsRef<DevelopedRegionRequest>],
    tolerance: f64,
    work: PlanarProofWorkLimit,
    version: PlanarProofVersion,
    remaining: u64,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<PlanarRegionSet, PlanarFailure> {
    let profile = combined_profile(requests)?;
    build_planar_regions_with_integration_budget_and_cancel(
        &profile,
        super::pipeline::resolution(tolerance),
        work,
        version,
        remaining,
        cancelled,
    )
    .map_err(|cause| PlanarFailure::WorkCapture(Box::new(cause)))
}
impl PlanarEvidence {
    pub(super) fn integral_program_evidence(&self) -> Option<&PlanarIntegralEvidence> {
        self.integral.as_ref()
    }
}
impl std::fmt::Debug for PlanarEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = f.debug_struct("PlanarEvidence");
        output.field("area", &self.area);
        #[cfg(test)]
        output.field("certificate", &self.certificate);
        output
            .field("identity", &self.identity)
            .field("source", &self.source);
        if let Some(integral) = &self.integral {
            output.field("integral", integral);
        }
        output.finish()
    }
}

fn combined_profile(
    requests: &[impl AsRef<DevelopedRegionRequest>],
) -> Result<SketchProfile, PlanarFailure> {
    let mut combined = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: Vec::new(),
        curves: Vec::new(),
    };
    for request in requests {
        let request = request.as_ref();
        joins::admit(request).map_err(|cause| PlanarFailure::Join(Box::new(cause)))?;
        let profile = profile(request)?;
        combined.vertices.extend(profile.vertices);
        combined.curves.extend(profile.curves);
    }
    canonicalize_proof_input(&mut combined);
    Ok(combined)
}

fn canonicalize_proof_input(profile: &mut SketchProfile) {
    // #453: schedule actual work by authored IDs, after original-order admission succeeds.
    if !profile.curves.is_sorted_by_key(ProfileCurve::id) {
        profile.curves.sort_by_key(ProfileCurve::id);
    }
    if !profile.vertices.is_sorted_by_key(|vertex| vertex.id) {
        profile.vertices.sort_by_key(|vertex| vertex.id);
    }
}

fn profile(request: &DevelopedRegionRequest) -> Result<SketchProfile, PlanarFailure> {
    let mut vertices = BTreeMap::new();
    let mut curves = Vec::new();
    for piece in request.loops.iter().flatten() {
        let start = authored_vertex(piece.start)?;
        let end = authored_vertex(piece.end)?;
        vertices.insert(start, piece.poles[0]);
        curves.push(authored_curve(piece, start, end)?);
    }
    Ok(SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: vertices
            .into_iter()
            .map(|(id, point)| ProfileVertex { id, point })
            .collect(),
        curves,
    })
}

fn authored_vertex(source: CornerSource) -> Result<ProfileVertexId, PlanarFailure> {
    match source {
        CornerSource::Authored(id) => Ok(ProfileVertexId(id.0)),
        other => Err(PlanarFailure::NonAuthoredVertex(other)),
    }
}

fn authored_curve(
    piece: &AuthoredPiece,
    start: ProfileVertexId,
    end: ProfileVertexId,
) -> Result<ProfileCurve, PlanarFailure> {
    if piece.span.level != 0 || piece.span.index != 0 {
        return Err(PlanarFailure::SplitSource {
            curve: piece.span.curve,
            level: piece.span.level,
            index: piece.span.index,
        });
    }
    let id = ProfileCurveId(piece.span.curve.0);
    if piece.poles.len() == 2 {
        return Ok(ProfileCurve::Line { id, start, end });
    }
    Ok(spline(piece, start, end))
}

fn spline(piece: &AuthoredPiece, start: ProfileVertexId, end: ProfileVertexId) -> ProfileCurve {
    let id = ProfileCurveId(piece.span.curve.0);
    let degree = piece.poles.len() - 1;
    ProfileCurve::Spline {
        id,
        start,
        end,
        degree: degree as u32,
        poles: piece.poles.clone(),
        weights: None,
        knots: std::iter::repeat_n(0., degree + 1)
            .chain(std::iter::repeat_n(1., degree + 1))
            .collect(),
    }
}

pub(super) fn select(
    request: &DevelopedRegionRequest,
    captured: &PlanarRegionSet,
) -> Result<PlanarEvidence, PlanarFailure> {
    let expected = requested_identity(request)?;
    let selected = unique_region(&expected, captured)?;
    Ok(PlanarEvidence {
        area: selected.area(),
        selected_region: selected.id(),
        #[cfg(test)]
        certificate: captured.certificate(),
        identity: expected,
        source: source::SourceSnapshot::capture(request),
        integral: captured.integral_program_evidence().cloned(),
    })
}

fn unique_region<'a>(
    expected: &MaterialIdentity,
    captured: &'a PlanarRegionSet,
) -> Result<&'a PlanarRegion, PlanarFailure> {
    let candidates: Vec<_> = captured.regions().iter().map(region_identity).collect();
    let matches: Vec<_> = captured
        .regions()
        .iter()
        .zip(&candidates)
        .filter(|(_, identity)| **identity == *expected)
        .collect();
    if matches.len() != 1 {
        return Err(PlanarFailure::Selection {
            expected: expected.clone(),
            candidates,
        });
    }
    Ok(matches[0].0)
}

fn requested_identity(request: &DevelopedRegionRequest) -> Result<MaterialIdentity, PlanarFailure> {
    let mut loops = request.loops.iter().map(|contour| {
        canonical(
            contour
                .iter()
                .map(|piece| (ProfileCurveId(piece.span.curve.0), false)),
        )
    });
    let outer = loops.next().ok_or(PlanarFailure::EmptyMaterial)?;
    let mut holes: Vec<_> = loops.collect();
    holes.sort();
    Ok(MaterialIdentity { outer, holes })
}

fn region_identity(region: &PlanarRegion) -> MaterialIdentity {
    let mut holes: Vec<_> = region.holes().iter().map(loop_identity).collect();
    holes.sort();
    MaterialIdentity {
        outer: loop_identity(region.outer()),
        holes,
    }
}

fn loop_identity(contour: &ProfileLoop) -> LoopIdentity {
    canonical(contour.uses.iter().map(|use_| (use_.curve, use_.reversed)))
}

fn canonical(uses: impl Iterator<Item = (ProfileCurveId, bool)>) -> LoopIdentity {
    let mut uses: Vec<_> = uses.collect();
    let first = uses
        .iter()
        .enumerate()
        .min_by_key(|(_, entry)| **entry)
        .map_or(0, |(i, _)| i);
    uses.rotate_left(first);
    uses
}
