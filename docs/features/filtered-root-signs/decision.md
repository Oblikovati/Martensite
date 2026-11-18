# Decision: opt-in exact bracket filtering for graph membership

Status: approved for implementation under the maintainer's #544 delegation on
2026-11-18. Math compatibility qualification passed. Restored legacy TKBO
callback controls, component review and the full PR gate remain pending.

Actual source-pair inspection found expensive generated-membership sign chains
whose signs can sometimes be established on their existing exact root brackets.
The mixed case also contains rejected nonpoint roots and a common-root query.
Those observations rule out evaluating only accepted roots or treating a zero
interval bound as a root-zero proof.

Reuse the existing exact rational Horner loop behind a roots-private coefficient
helper. Keep its graph wrapper's original operations, errors and cancellation
polls. Record this extraction separately from the numerical policy change.

Select the filter only through retained `BezierPairSignOptions.version == 2`.
Version 1 and `derived()` retain the original eager evaluator for every
isolation. The attempted #542 default shortcut failed the full owned-operation
callback replay gate (`8238` observed versus `8268` expected) and was removed.
Version 2 alone uses the same all-root attempt for empty, point-only and mixed
isolations, reusing the exact point-sign function for singleton roots. Accept a nonpoint sign only when exact interval evaluation
is strict throughout the original bracket. An exactly zero normalized query
is the sole global-zero case.

If any root is undecided, discard the attempt and evaluate the original query on
the entire original isolation with the existing evaluator. Cancellation is an
error at `RootSignStage::Evaluation`; it cannot become an inconclusive attempt.
Keep upstream source/query degree admission and all primary-root and
exceptional-fibre computation unchanged. The Math result needs no new public fields or enum variants; its existing
effective-sign receipt retains the request.

This is one conditioned stage in the shared pair pipeline. It adds no source
cache, root refinement, tolerance adjustment, subdivision allowance or separate
wrap algorithm. The additive `PlanarProofVersion::FilteredBernsteinV4` requests it through
owned graph version 4. `BernsteinV3` remains graph version 3 with sign version 1;
default builders remain on `CarrierV2`. The private census retains its selected
sign settings and binds them to the owned request before graph consumption.
No source, degree, root-work or world-error setting is widened. Performance must be
measured on accepted identical workloads after functional qualification.
