# Oblique publication review

The source review used main merge-base
`77053b4cd0f6e223d94f20b30929df15466dfc09`, with the preceding numerical layer
at `1db0f3fe`. Independent Standards and Spec reviewers inspected the physical
operation, then the #453 scheduling and #454 trace-tolerance deltas. Reviews
were read-only and ran no Cargo commands. The coordinator ran qualification.

## Standards

The initial review found that correspondence failures recorded only an expected
shape and discarded the observed mismatch. The corrected variants retain
bounded coefficient/scale data, datum structure, binding identity, archive
lengths and first differing byte, or missing region identity. Messages include
those values and their expected shape. Region, curve and depth context is
retained for trace-tolerance failures. The reviewer verified the correction.

The #453 guard uses the same authored-ID key as its sort and preserves prior
admission order. The #454 gate reuses outward arithmetic and each original
retained tolerance. Neither introduces public surface, unsafe code, a
production panic or an adjacent refactor. Both new public report types have
compile-time Send/Sync controls. The reviewed new production and test functions
remain within 20 lines, and files remain below 500 lines in the approximate
source-metrics check. No remaining Standards blocker or actionable heuristic
finding was reported.

## Spec

The reviewer found no confirmed source-correctness defect in the physical
operation. Canonical allocation precedes construction; decoded placement bits
and shared datum identity are checked; stripping the datum must reproduce the
canonical archive exactly. Physical native checks and estimates run on decoded
solids. Flux transport includes determinant drift against the unchanged ideal
volume and re-enforces the original policy.

The native-representation boundary needed explicit documentation. The proof
covers the located graph and direct location evaluation. Re-normalized analytic
copies require additional correspondence evidence. The reviewer verified the
public and feature documentation after that clarification.

The #453 fix changes actual wrapped proof input ordering, preserving source
identity and admission precedence while leaving counters and limits unchanged.
Its guards are equivalent to the reviewed stable sorts. The #454 bound
`M*r + 2*E` correctly cancels the common exact translation, retains independent
evaluation error, and checks each cap against its original edge/face limit.
The point-domain and frozen-correspondence premises remain explicit. No Spec
finding remained in either delta.

## Qualification and integration boundary

The [qualification record](P5-OBLIQUE-QUALIFICATION.md) contains executed checks,
independent fixture expectations, process replay, isolation and measured costs.
Full-text runtime controls passed after proof ordering changed. The small
single-trial full-text RSS difference has not established either a regression
or strict memory parity; scale-performance qualification remains open.

The affected-package PR gate, hosted feature checks, sanitizer qualification,
remaining application/conical work and P6 replacement cleanup are still
required. These reviews do not close the complete feature or authorize a
version change, release tag or publication.
