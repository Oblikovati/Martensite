// SPDX-License-Identifier: GPL-2.0-only
//! #555: the real fresh producer shares geometry/callback results with the live path.
use super::super::tests::{Continue, Profile};
use super::*;
mod early_failures;
mod recapture;
mod unified;
#[derive(Default)]
struct Trace {
    checkpoints: Vec<OwnedPrismCheckpoint>,
    stop: Option<usize>,
}
impl OwnedPrismProgress for Trace {
    fn checkpoint(
        &mut self,
        point: OwnedPrismCheckpoint,
    ) -> Result<bool, martensite_tkernel::standard::Failure> {
        self.checkpoints.push(point);
        Ok(self.stop == Some(point.ordinal))
    }
}
fn request() -> OwnedProfileRequest {
    let mut profile = Profile::default();
    profile.edge(1, &[[0., 0.], [0.5, -0.125], [1., 0.]], None, [1, 2]);
    for (id, points, ends) in [
        (2, [[1., 0.], [1., 1.]], [2, 3]),
        (3, [[1., 1.], [0., 1.]], [3, 4]),
        (4, [[0., 1.], [0., 0.]], [4, 1]),
    ] {
        profile.edge(id, &points, None, ends);
    }
    let mut request = profile.request();
    request.version = super::super::admission::GRAPH_PROFILE_VERSION;
    request
}
fn live(
    request: OwnedProfileRequest,
    progress: &mut dyn OwnedPrismProgress,
) -> Result<GraphRegionCertificate, super::super::certify::ProfileProofRefusal> {
    let prepared = super::super::pipeline::prepare_owned_profile(request, progress)
        .map_err(super::super::certify::preparation_refusal)?;
    prepared
        .inspect_plane_graph(|| false)
        .map_err(|failure| super::super::certify::inspection_refusal(&prepared, failure))
}
#[test]
fn owned_graph_capture_batches_are_linear_and_match_live_certificate_bytes() {
    let request = request();
    let mut count = 0;
    let mut actual_trace = Trace::default();
    let actual = run_observed(request.clone(), &mut actual_trace, |value| count = value).unwrap();
    let mut expected_trace = Trace::default();
    let expected = live(request.clone(), &mut expected_trace).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(
        format!("{actual:?}").as_bytes(),
        format!("{expected:?}").as_bytes()
    );
    assert_eq!(actual_trace.checkpoints, expected_trace.checkpoints);
    assert_eq!(count, 6 * request.edges.len());
}
#[test]
fn owned_graph_source_revision_changes_start_fresh_capture_batches() {
    let mut first = request();
    let mut first_count = 0;
    let expected = run_observed(first.clone(), &mut Continue, |value| first_count = value).unwrap();
    first.operation = CallerOperationId(92);
    first.edges[0].revision = CallerSourceRevision(2);
    let mut next_count = 0;
    let actual = run_observed(first, &mut Continue, |value| next_count = value).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(next_count, first_count);
    assert_eq!(next_count, 24);
}
#[test]
fn owned_graph_cancellation_preserves_each_phase_first_checkpoint_and_failure() {
    let mut completed = Trace::default();
    run(request(), &mut completed).unwrap();
    let mut phases = Vec::new();
    for point in completed.checkpoints {
        if phases.contains(&point.phase) {
            continue;
        }
        phases.push(point.phase);
        compare_cancelled(point.ordinal);
    }
}
fn compare_cancelled(cutoff: usize) {
    let mut actual = Trace {
        stop: Some(cutoff),
        ..Default::default()
    };
    let mut expected = Trace {
        stop: Some(cutoff),
        ..Default::default()
    };
    assert_eq!(run(request(), &mut actual), live(request(), &mut expected));
    assert_eq!(actual.checkpoints, expected.checkpoints);
}
