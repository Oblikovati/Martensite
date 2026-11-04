// SPDX-License-Identifier: GPL-2.0-only
use super::super::rational_polynomial::ratio;
use super::*;
use std::cmp::Ordering;
/// Owned exact exclusion between two declared distinct vertex balls.
/// Example: `report.separated()` is true only for strict sphere separation.
#[derive(Clone, Debug, PartialEq)]
pub struct DisjointVertexNeighborhoodReport {
    first: GivenVertexNeighborhoodReport,
    second: GivenVertexNeighborhoodReport,
    outcome: Result<(), DisjointVertexNeighborhoodFailure>,
    policy: VertexNeighborhoodPolicy,
    work: u32,
}
/// Original refusal or nonseparation; this does not assert invalid geometry.
#[derive(Clone, Debug, PartialEq)]
pub enum DisjointVertexNeighborhoodFailure {
    /// Both operands must already have complete proper component proofs.
    PriorIncomplete {
        /// Zero is the first operand, one is the second.
        operand: usize,
        /// Actual neighborhood refusal.
        cause: VertexNeighborhoodFailure,
    },
    /// Distinct topological endpoint classes cannot share one declared identity.
    SameIdentity(usize),
    /// Exact center distance is at most the sum of the original radii.
    NotSeparated,
    /// Caller cancellation before complete exact comparison.
    Cancelled,
    /// Actual combined work or input-policy refusal.
    Exact(VertexNeighborhoodFailure),
}
impl DisjointVertexNeighborhoodReport {
    /// Read both owned operands; example: `report.inputs().0.input().vertex`.
    pub fn inputs(
        &self,
    ) -> (
        &GivenVertexNeighborhoodReport,
        &GivenVertexNeighborhoodReport,
    ) {
        (&self.first, &self.second)
    }
    /// Read strict exclusion; example: `report.separated()`.
    pub fn separated(&self) -> bool {
        self.outcome.is_ok()
    }
    /// Read unchanged combined work limits; example: `report.policy().max_work`.
    pub fn policy(&self) -> VertexNeighborhoodPolicy {
        self.policy
    }
    /// Read combined prior and comparison work; example: `report.work()`.
    pub fn work(&self) -> u32 {
        self.work
    }
    /// Read original noncompletion; example: `report.failure()`.
    pub fn failure(&self) -> Option<&DisjointVertexNeighborhoodFailure> {
        self.outcome.as_ref().err()
    }
}
/// Compare exact center distance with the exact sum of unmodified raw radii.
/// No rounded transformed point or radius sum is used. The given identities must differ.
/// Example: `certify_disjoint_vertex_neighborhoods(&first, &second, policy, || false)`.
pub fn certify_disjoint_vertex_neighborhoods(
    first: &GivenVertexNeighborhoodReport,
    second: &GivenVertexNeighborhoodReport,
    policy: VertexNeighborhoodPolicy,
    mut cancelled: impl FnMut() -> bool,
) -> DisjointVertexNeighborhoodReport {
    let mut work = isolation::Work {
        used: 0,
        policy,
        cancelled: &mut cancelled,
    };
    let outcome = budgeted_disjoint(first, second, &mut work);
    DisjointVertexNeighborhoodReport {
        first: first.clone(),
        second: second.clone(),
        outcome,
        policy,
        work: work.used,
    }
}
fn budgeted_disjoint(
    first: &GivenVertexNeighborhoodReport,
    second: &GivenVertexNeighborhoodReport,
    work: &mut isolation::Work<'_>,
) -> Result<(), DisjointVertexNeighborhoodFailure> {
    if work.policy.version != 1 {
        return Err(DisjointVertexNeighborhoodFailure::Exact(
            VertexNeighborhoodFailure::Version(work.policy.version),
        ));
    }
    work.reserve(u64::from(first.work()) + u64::from(second.work()))
        .map_err(DisjointVertexNeighborhoodFailure::Exact)?;
    work.charge()
        .map_err(DisjointVertexNeighborhoodFailure::Exact)?;
    disjoint(first, second, work.cancelled)
}

fn disjoint(
    first: &GivenVertexNeighborhoodReport,
    second: &GivenVertexNeighborhoodReport,
    cancelled: &mut dyn FnMut() -> bool,
) -> Result<(), DisjointVertexNeighborhoodFailure> {
    complete_operands(first, second)?;
    if cancelled() {
        return Err(DisjointVertexNeighborhoodFailure::Cancelled);
    }
    if first.input().vertex == second.input().vertex {
        return Err(DisjointVertexNeighborhoodFailure::SameIdentity(
            first.input().vertex,
        ));
    }
    separated(first.input(), second.input())
}
fn complete_operands(
    first: &GivenVertexNeighborhoodReport,
    second: &GivenVertexNeighborhoodReport,
) -> Result<(), DisjointVertexNeighborhoodFailure> {
    for (operand, report) in [first, second].into_iter().enumerate() {
        if let Some(cause) = report.failure() {
            return Err(DisjointVertexNeighborhoodFailure::PriorIncomplete {
                operand,
                cause: cause.clone(),
            });
        }
    }
    Ok(())
}

fn separated(
    first: &GivenVertexNeighborhoodInput,
    second: &GivenVertexNeighborhoodInput,
) -> Result<(), DisjointVertexNeighborhoodFailure> {
    let a = capture::center(first);
    let b = capture::center(second);
    let squared = a.iter().zip(&b).fold(ratio(0.), |sum, (a, b)| {
        let difference = a.subtract(b);
        sum.add(&difference.multiply(&difference))
    });
    let radii = ratio(first.radius).add(&ratio(second.radius));
    if squared.cmp(&radii.multiply(&radii)) != Ordering::Greater {
        return Err(DisjointVertexNeighborhoodFailure::NotSeparated);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn endpoint(vertex: usize, x: f64, radius: f64) -> GivenVertexNeighborhoodReport {
        let input = GivenVertexNeighborhoodInput {
            vertex,
            curve: VertexNeighborhoodCurve::Line {
                origin: [x, 0., 0.],
                direction: [0., 1., 0.],
            },
            interval: [0., 4.],
            placements: Vec::new(),
            point: [x, 0., 0.],
            point_placements: Vec::new(),
            radius,
            ends: VertexNeighborhoodEnds::First,
        };
        certify_given_vertex_neighborhood(input, VertexNeighborhoodPolicy::v1(10000), || false)
    }
    fn run(
        first: &GivenVertexNeighborhoodReport,
        second: &GivenVertexNeighborhoodReport,
        cancelled: impl FnMut() -> bool,
    ) -> DisjointVertexNeighborhoodReport {
        certify_disjoint_vertex_neighborhoods(
            first,
            second,
            VertexNeighborhoodPolicy::v1(10000),
            cancelled,
        )
    }
    #[test]
    fn distinct_located_vertices_need_strict_disjoint_balls() {
        let a = endpoint(0, 0., 1.);
        assert!(run(&a, &endpoint(1, 3., 1.), || false).separated());
        assert!(!run(&a, &endpoint(1, 2., 1.), || false).separated());
        assert!(!run(&a, &endpoint(1, 1., 1.), || false).separated());
    }
    #[test]
    fn identity_is_not_derived_from_separation() {
        let a = endpoint(0, 0., 1.);
        let b = endpoint(0, 3., 1.);
        assert_eq!(
            run(&a, &b, || false).failure(),
            Some(&DisjointVertexNeighborhoodFailure::SameIdentity(0))
        );
    }
    #[test]
    fn cancellation_is_not_a_separation_certificate() {
        let a = endpoint(0, 0., 1.);
        let b = endpoint(1, 3., 1.);
        let result = run(&a, &b, || true);
        assert_eq!(
            result.failure(),
            Some(&DisjointVertexNeighborhoodFailure::Exact(
                VertexNeighborhoodFailure::Cancelled
            ))
        );
    }
}
