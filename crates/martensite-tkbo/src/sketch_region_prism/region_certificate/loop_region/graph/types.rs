// SPDX-License-Identifier: GPL-2.0-only
//! Request and certificate of the plane graph region proof.
use super::super::types::{AuthoredLoopUse, LoopRegionOptions};
use super::area::GraphArea;
use crate::sketch_region::ArrangementSource;
use crate::sketch_region_contact_transport::PairedContactTransaction;

/// Borrowed inputs: the actual contact transaction and the caller vertex ids at native
/// parameters 0 and 1 of every source, in source order.
pub(crate) struct GraphRegionRequest<'a, 'b> {
    pub bridge: &'a PairedContactTransaction,
    pub sources: &'b [ArrangementSource<'a>],
    pub ends: &'b [[u64; 2]],
    pub options: LoopRegionOptions,
}

/// One certified bounded face: a simple counterclockwise orbit (the face on its left) and
/// the components whose outer walks are its holes.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GraphFaceProof {
    /// Directed source uses of the outer boundary, counterclockwise.
    pub uses: Vec<AuthoredLoopUse>,
    /// The connected component of the boundary.
    pub component: usize,
    /// Components nested directly in this face; their outer walks are the holes.
    pub holes: Vec<usize>,
    /// Certified chart area: outer area minus hole areas.
    pub area: GraphArea,
}

/// One connected component of the plane graph.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GraphComponentProof {
    /// Its outer walk, clockwise (the unbounded side on its left).
    pub outer: Vec<AuthoredLoopUse>,
    /// The bounded face (index into `faces`) it lies in directly, if any.
    pub container: Option<usize>,
}

/// Complete plane graph certificate; it publishes no body, history or names.
#[derive(Clone, PartialEq)]
pub(crate) struct GraphRegionCertificate {
    pub programs: Option<std::sync::Arc<super::super::program_trace::GraphIntegralEvidence>>,
    /// Every bounded face, ordered by its first dart in source order.
    pub faces: Vec<GraphFaceProof>,
    /// Every connected component, ordered by its first source.
    pub components: Vec<GraphComponentProof>,
    /// Certified vertex incidence contacts.
    pub contacts: usize,
    /// Pair domains certified by common-carrier route.
    pub carrier_pairs: usize,
    /// Pair domains certified in total.
    pub pair_domains: usize,
    /// Actual shared census work retained for publication.
    pub work: crate::sketch_region_certification::CensusWorkReceipt,
}

impl std::fmt::Debug for GraphRegionCertificate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = f.debug_struct("GraphRegionCertificate");
        out.field("faces", &self.faces)
            .field("components", &self.components)
            .field("contacts", &self.contacts)
            .field("carrier_pairs", &self.carrier_pairs)
            .field("pair_domains", &self.pair_domains)
            .field("work", &self.work);
        if let Some(programs) = &self.programs {
            out.field("programs", programs);
        }
        out.finish()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProgramGraphFailure {
    pub failure: super::super::types::LoopRegionFailure,
    pub options: martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramOptions,
    pub consumed: u64,
}

impl ProgramGraphFailure {
    pub(crate) fn numerical_cause(
        &self,
    ) -> Option<&martensite_tkmath::predicates::roots::metric_bounds::IntegralProgramError> {
        use super::super::integral_program::IntegralLedgerError;
        let super::super::types::LoopRegionCause::IntegralProgram(error) = &self.failure.cause
        else {
            return None;
        };
        let IntegralLedgerError::Math(cause) = error.cause.as_ref() else {
            return None;
        };
        Some(cause)
    }
    pub(crate) fn offender(&self) -> Option<crate::sketch_region::RegionSourceId> {
        let super::super::types::LoopRegionCause::IntegralProgram(error) = &self.failure.cause
        else {
            return None;
        };
        error.offender()
    }
}
