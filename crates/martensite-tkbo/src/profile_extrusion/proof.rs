// SPDX-License-Identifier: GPL-2.0-only
//! Certification through the plane graph region proof, which generalizes the loop proof. Each curve becomes proper Bézier pieces (arcs and circles at most a quarter
//! turn per piece); the proof certifies that pieces meet only at shared vertices, orders
//! the branches at every vertex, and derives every bounded face, its holes and its area.
use super::admitted::AdmittedProfile;
use super::decline::DeclineCause;
use super::loops::ProfileLoop;
use super::proof_pieces::PieceTable;
use super::topology::native_plane;
use crate::sketch_region_prism::{
    AuthoredLoopUse, GraphFaceProof, GraphRegionCertificate, ProfileProofCause,
    ProfileProofRefusal, certify_profile_graph, certify_profile_graph_version_with_cancel,
    certify_profile_graph_with_cancel,
};

/// One certified face: its counterclockwise outer loop, clockwise holes and certified area
/// `(representative, absolute error)`.
pub(crate) struct CertifiedFace {
    pub outer: ProfileLoop,
    pub holes: Vec<ProfileLoop>,
    pub area: (f64, f64),
}

/// Every bounded face of a profile, ordered by the caller index of its outer loop's first
/// curve (forward use first), and the proof's evidence counts.
pub(crate) struct CertifiedRegions {
    pub faces: Vec<CertifiedFace>,
    pub version: super::PlanarProofVersion,
    /// Pair domains certified in total.
    pub pair_domains: usize,
    /// Pair domains certified by the common-carrier route.
    pub carrier_pairs: usize,
    /// Connected components of the curve graph.
    pub components: usize,
    pub work: super::PlanarProofWorkReceipt,
    pub programs: Option<std::sync::Arc<crate::sketch_region_prism::GraphIntegralEvidence>>,
    pub parents: Vec<(u64, super::ProfileCurveId, usize)>,
}

pub(crate) struct RegionProofFailure {
    pub cause: DeclineCause,
    pub root_failure: Option<super::PlanarPairRootFailure>,
    pub work: Option<super::PlanarProofWorkReceipt>,
    pub cancelled: bool,
    pub program: Option<Box<crate::sketch_region_prism::ProgramGraphFailure>>,
    pub program_source: Option<super::ProfileCurveId>,
}
/// Certify every face under the unchanged or explicitly requested pair-work policy.
pub(crate) fn certify_regions(
    profile: &AdmittedProfile,
    pair_bisections: Option<u16>,
) -> Result<CertifiedRegions, RegionProofFailure> {
    let table = PieceTable::new(profile);
    let input = graph_input(profile, &table, pair_bisections)?;
    let certificate =
        certify_profile_graph(input).map_err(|refusal| graph_refusal(refusal, &table))?;
    Ok(assemble_certificate(certificate, &table))
}
pub(crate) fn certify_regions_with_cancel(
    profile: &AdmittedProfile,
    pair_bisections: Option<u16>,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedRegions, RegionProofFailure> {
    let table = PieceTable::new(profile);
    let input = graph_input(profile, &table, pair_bisections)?;
    let certificate = certify_profile_graph_with_cancel(input, cancelled)
        .map_err(|refusal| graph_refusal(refusal, &table))?;
    Ok(assemble_certificate(certificate, &table))
}
pub(crate) fn certify_regions_version_with_cancel(
    profile: &AdmittedProfile,
    pair_bisections: Option<u16>,
    version: super::PlanarProofVersion,
    cancelled: impl FnMut() -> bool + Send,
) -> Result<CertifiedRegions, RegionProofFailure> {
    if version == super::PlanarProofVersion::CarrierV2 {
        return certify_regions_with_cancel(profile, pair_bisections, cancelled);
    }
    let table = PieceTable::new(profile);
    let input = graph_input(profile, &table, pair_bisections)?;
    let result = match version.integral_options() {
        Some(options) => crate::sketch_region_prism::certify_profile_program_with_cancel(
            input,
            version.request_version(),
            options,
            cancelled,
        ),
        None => {
            certify_profile_graph_version_with_cancel(input, version.request_version(), cancelled)
        }
    };
    let certificate = result.map_err(|refusal| graph_refusal(refusal, &table))?;
    let mut certified = assemble_certificate(certificate, &table);
    certified.version = version;
    Ok(certified)
}
fn assemble_certificate(
    certificate: GraphRegionCertificate,
    table: &PieceTable,
) -> CertifiedRegions {
    let mut faces = faces(&certificate, table);
    faces.sort_by_key(|face| {
        let first = face.outer.uses[0];
        (table.caller_index(first.curve), first.reversed)
    });
    CertifiedRegions {
        faces,
        version: super::PlanarProofVersion::CarrierV2,
        pair_domains: certificate.pair_domains,
        carrier_pairs: certificate.carrier_pairs,
        components: certificate.components.len(),
        work: certificate.work,
        parents: if certificate.programs.is_some() {
            table.integral_parents()
        } else {
            Vec::new()
        },
        programs: certificate.programs,
    }
}
fn graph_input(
    profile: &AdmittedProfile,
    table: &PieceTable,
    pair_bisections: Option<u16>,
) -> Result<crate::sketch_region_prism::ProfileGraphInput, RegionProofFailure> {
    let plane = native_plane(&profile.frame).map_err(|failure| RegionProofFailure {
        cause: incomplete(format!("declared plane: {failure}")),
        root_failure: None,
        work: None,
        cancelled: false,
        program: None,
        program_source: None,
    })?;
    let mut input = table.graph_input(plane, profile.resolution.weld);
    input.pair_bisections = pair_bisections;
    Ok(input)
}
fn graph_refusal(refusal: ProfileProofRefusal, table: &PieceTable) -> RegionProofFailure {
    let work = refusal.work;
    let program = refusal.program.clone();
    let program_source = program
        .as_ref()
        .and_then(|failure| failure.offender())
        .map(|source| table.curve_of(source.0));
    let root_failure = pair_root_failure(&refusal, table);
    RegionProofFailure {
        program,
        program_source,
        root_failure,
        cancelled: refusal.cancelled,
        cause: declined(refusal, table),
        work,
    }
}

fn faces(certificate: &GraphRegionCertificate, table: &PieceTable) -> Vec<CertifiedFace> {
    let face = |proof: &GraphFaceProof| CertifiedFace {
        outer: table.caller_loop(&walk(&proof.uses)),
        holes: proof
            .holes
            .iter()
            .map(|k| table.caller_loop(&walk(&certificate.components[*k].outer)))
            .collect(),
        area: (
            proof.area.representative(),
            proof.area.absolute_error_bound(),
        ),
    };
    certificate.faces.iter().map(face).collect()
}

fn walk(uses: &[AuthoredLoopUse]) -> Vec<(u64, bool)> {
    uses.iter().map(|u| (u.source.0, u.reversed)).collect()
}

fn incomplete(reason: String) -> DeclineCause {
    DeclineCause::IncompleteCertification { reason }
}

fn declined(refusal: ProfileProofRefusal, table: &PieceTable) -> DeclineCause {
    let caller = |vertex: u64| table.caller_vertex(vertex);
    match refusal.cause {
        ProfileProofCause::UnexpectedContact { edges, point } => DeclineCause::SelfIntersecting {
            curves: edges.map(|edge| table.curve_of(edge)),
            point: Some(point),
        },
        ProfileProofCause::TangentBranches { vertex } => match caller(vertex) {
            Some(vertex) => DeclineCause::TangentBranches { vertex },
            None => incomplete(format!("tangent branches at split point {vertex}")),
        },
        ProfileProofCause::Pinched { vertex, hole } => match caller(vertex) {
            Some(vertex) => DeclineCause::PinchedRegion { vertex, hole },
            None => incomplete(format!("pinched boundary at split point {vertex}")),
        },
        ProfileProofCause::Incomplete(reason) => incomplete(reason),
    }
}

fn pair_root_failure(
    refusal: &ProfileProofRefusal,
    table: &PieceTable,
) -> Option<super::PlanarPairRootFailure> {
    refusal
        .root_failure
        .as_ref()
        .map(|(sources, cause)| super::PlanarPairRootFailure {
            curves: sources.map(|source| table.curve_of(source)),
            cause: cause.clone(),
        })
}
