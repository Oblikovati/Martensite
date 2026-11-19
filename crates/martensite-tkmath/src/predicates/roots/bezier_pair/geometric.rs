// SPDX-License-Identifier: GPL-2.0-only
//! Exact geometric witnesses complement the unchanged algebraic pair solver.
use super::super::{RootIsolationError, error::poll};
use super::*;
use crate::predicates::{NonFiniteCoordinate, OrientationSign};
mod admission;
mod binding;
mod diagnostics;
mod execution;
mod predicates;
mod report;
mod sector;
mod types;
mod view;
pub use report::BezierPairGeometricReport;
pub use types::{
    BezierPairGeometricCompletion, BezierPairGeometricFailure, BezierPairGeometricKind,
};
pub use view::{BezierPairGeometricCertificate, CertifiedBezierEndpointContact};
type Checked<T> = Result<T, BezierPairGeometricFailure>;
type Point = [f64; 2];
#[derive(Clone, Debug)]
struct Vector {
    from: Point,
    to: Point,
}
struct SectorWitness {
    vectors: Vec<Vector>,
    first: usize,
    signs: Vec<OrientationSign>,
}
struct HullWitness {
    edge: [Point; 2],
    side: usize,
    signs: [Vec<OrientationSign>; 2],
}
#[derive(Debug)]
struct Endpoint {
    parameters: [u8; 2],
    point: Point,
}
enum GeometricProof {
    Box {
        axis: usize,
        bounds: [[f64; 2]; 2],
    },
    Monotone(SectorWitness),
    Endpoint {
        sector: SectorWitness,
        contact: Endpoint,
    },
    Hull(HullWitness),
}
impl CapturedBezierPair {
    /// Try exact polynomial geometric coverage without inventing algebraic solver evidence.
    /// Example: `let report=query.certify_geometric(policy,||false);`.
    pub fn certify_geometric(
        &self,
        policy: BezierPairPolicy,
        mut cancelled: impl FnMut() -> bool,
    ) -> BezierPairGeometricReport {
        let outcome = execution::execute(self, policy, &mut cancelled);
        BezierPairGeometricReport {
            source: self.clone(),
            policy,
            outcome,
        }
    }
}
#[cfg(test)]
mod tests;
