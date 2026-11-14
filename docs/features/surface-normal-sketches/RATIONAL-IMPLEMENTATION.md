# Rational implementation evidence

## Scope and provenance

The maintainer approved the representation, result contract and archive action
on 2026-11-13. The replacement branch is `feature/rational-wrapped-profiles`,
started from `main` at `919b8e31e`. The unfinished polynomial implementation is
archived at `775f373c`; its seam and collar proofs are no longer being extended.
No package version or release tag has changed.

The [review](REVIEW-2026-11-13.md) and
[ADR](../../architecture/decisions/rational-wrapped-profile-representation.md)
define the work. This record covers P1 on Linux x86_64. It does not qualify the
six certificate obligations or the complete application cases.

## P1: one native operation with a hole

The private `wrapped_profile::pipeline::wrap_extrude` admits an owned request,
constructs rational edges and ruled sides, freezes the body and history in one
shape table, decodes once, runs ordered native checks and estimates volume.
Its result is named `Unqualified`; there is no accepted-result constructor or
public rational operation. Tests exercise detached decoding separately.

The fixture has line, quadratic and cubic authored pieces and one rectangular
hole. Its support radius is 10, physical normal depths are -1 and 2, and the
native tolerance cap is `1e-5`. The fixture requests dyadic level 2 before
construction. This gives 32 pieces; sizing from certified error bounds remains
P2 work. Intervals and subdivision vertices retain their authored curve parent.
History uses those identities, independent of traversal rank and proof work.

Both depths use one fitted half-angle polynomial and one weight vector per
piece. Shared vertices are computed once. Native endpoints reuse those stored
vertices. Binary64 rounding, endpoint displacement and any native rationality
demotion still need O2/O3 certificates. The floating-point identity test is a
closed-form regression control, not a certificate.

The native conformance routine runs before freezing. A guard refuses if it
replaces the authored polynomial cap pcurves. This routine's estimates do not
establish O2/O3. The final refusing tolerance check remains enabled; the
implementation does not raise its cap after a native tolerance increase.

The fixture passes topology, sampled geometry, native-search self-intersection
and sampled tolerance checks. Its census is 64 vertices, 96 edges, 34 faces,
36 loops and genus 1. It retains 195 distinct history keys. Re-encoding a detached
decode matches all 59,475 frozen bytes. The new path uses the existing
`PreserveStoredV1` frame policy from #432; legacy primitive readers keep their
existing policy. Using legacy reconstruction changed four signed-zero bytes in
the cap frames, which the exact readback control detected.

Green's integral gives authored area `3.775 - 0.25 = 3.525`. The independent
ideal volume is `3.525 * (12² - 9²) / 20 = 11.10375`. Native quadrature returned
`11.103750626212612`, with relative integration estimate about `8.05e-11`.
The discrepancy is recorded separately; the quadrature estimate does not bound
the approximation discrepancy or establish O6.

A boss and pocket against a larger analytic cylindrical carrier both complete
without native Boolean errors. Each result passes the ordered body checks and
matches its independent volume target within `1e-4`: boss `29.355`, pocket
`18.25125`. These controls establish interoperability for this fixture; they do
not establish fillet support or general Boolean performance.

## Commands and measurements

`cargo test -p martensite-tksketch --quiet` passed 42 tests: 10 library tests and
32 integration tests. The library suite, including construction and validation
of both Boolean results, took 18.76 seconds; integration tests took 1.99 seconds.
Dependencies compiled as needed; unchanged dependency suites were not run.

The standalone P1 fixture passed in 3.97 seconds with peak RSS 57,456 KiB, using
the repository's debug test profile and one test thread. The executable was run
under `/usr/bin/time`; compilation was excluded. This meets the review's
five-second P1 target on this machine. Timing is measured separately from the
unit assertions so a loaded runner does not cause a clock-based test failure.
The compiler was `rustc 1.99.0 (b940084d7 2026-09-28)`, targeting
`x86_64-unknown-linux-gnu`.

`cargo xtask pr` passed formatting, source and expanded unsafe inventories,
clippy, build and the 42 affected-package tests. The first attempt stopped on
the uninitialized test-data submodule in the new worktree; initializing the
pinned dataset checkout resolved that setup failure. Only `martensite-tksketch`
was selected for package tests.

`cargo deny --all-features check` passed bans, licenses and sources but failed
advisories on the pre-existing `rustix 1.1.5` dependency, RUSTSEC-2026-0486.
`cargo audit` exited successfully with the same advisory as a warning. Bug #445
tracks it separately. This patch changes no third-party dependency version.

Run the controls with:

```sh
cargo test -p martensite-tksketch --lib wrapped_profile -- --test-threads=1
cargo test -p martensite-tksketch --test surface_sketch
```

The retained review metrics script reports no new rational file over 500 lines,
no function over 20 code lines and no nesting above two. These are approximate
brace-based measurements. The rational code contains no unsafe blocks or
suppression of dead-code warnings.

## P2 storage checkpoint

The [storage certificate](STORAGE-CERTIFICATES.md) now proves bounds from the
decoded homogeneous coefficients, including positive retained weights, radial
distance, scaled radial rulings, depth-invariant height and the combined distance
from the normalized first-curve radial sweep. The private pipeline checks native
control-net correspondence before applying those bounds. It still returns an
`Unqualified` result.

The current 32-piece fixture has certified radial, ruling, height and combined
sweep bounds below `1e-10`, within its downward-rounded epsilon/1000 allowance.
Its standalone debug run, including these certificates and the native checks,
took 4.08 seconds with peak RSS 56,256 KiB. This remains within the five-second
P1 target on the same compiler and platform. P5 scale measurements remain open.

`cargo test -p martensite-tkmath -p martensite-tksketch --quiet` passed 409 TKMath
library tests, 3,114 TKMath integration tests, 56 TKMath doctests, 15 sketch library
tests and 32 sketch integration tests. A subsequent focused test confirmed that
explicit 4096-bit precision resolves a subnormal denominator that the default
precision refuses. A 410-test library run and the final 26 storage controls also
passed, including retention of an exact endpoint that cannot fit finite
binary64. Five native storage-binding controls passed. Clippy for both packages
and all targets passed with warnings denied. No third-party dependency or package
version changed.

The full `cargo xtask pr` gate recorded above belongs to the P1 checkpoint. The
storage changes have the package checks recorded here; the complete PR gate must
run again before pushing the feature.

## P2 authored-profile checkpoint

The [profile certificate and fit planner](PROFILE-CERTIFICATES.md) compare decoded
coefficients against the original authored polynomial on each exact interval.
Position and derivative bounds now accompany the storage proof. A priori
fourth-derivative sizing chooses the geometric subdivision before construction.
The private fixture uses 19 pieces and 117 semantic bindings; its native loop,
hole, boss and pocket controls pass within the unchanged error allowances.

## P3 authored-material checkpoint

The private pipeline now reuses the established planar region builder before
fitting. It matches complete directed authored boundaries to one material face,
with exact-sign exclusions for stationary endpoints and tangent returns. The
[planar qualification note](PLANAR-QUALIFICATION.md) records the implementation,
controls and moving-endpoint homotopy. Strict derivative-cone margins now pass
for every prepared piece and adjacent pair, bound to their retained O2 sources.
Certified hierarchy pruning and cached dyadic covers now establish all 152
nonadjacent clearances before wrapped native construction. Their original
source/interval/chart records bind back to retained O2 proofs. An outward
true-circumference slab proof now excludes all nonzero periodic images when the
whole source width leaves the required clearance. A priori multiple-chart
fitting now covers long curves and parameter-seam crossings while preserving
original intervals and split-corner ancestry. The periodic boundary pipeline
now examines every required positive image under one work allowance, accounts
for true-period rounding and certifies the infinite tail. The current one-outer
planar selection excludes translated material containment by Jordan separation
and equal area. A two-turn helical strip passes; disconnected cross-component
classification remains open.

## Remaining gates

Radius/depth rounding now composes with the existing storage bound under one
allowance. Decoded shared endpoints require authored vertex identity and exact
pole agreement; a tiny-span control also covers native weight demotion. P3
must prove topology while permitting the consistent endpoint motion covered by
the profile error bound. Decoded pcurve domains, coefficient/weight bindings and raw/effective
topology tolerance caps now have direct checks; see
[retained trims](STORED-TRIMS.md). Full cap spatial residuals now have outward
same-parameter certificates against both retained topology tolerances. The
remaining ordered trim-publication checks are still open. The implemented storage certificate does
not satisfy these remaining obligations.

P3 must establish planar admission, clearance, regularity and the topology-
preserving homotopy. P4 must bind those proofs to the frozen coefficients,
complete represented-measure interpretation and history, and verify isolation, deterministic
replay in two processes and concurrency. P5 covers glyph scale and oblique
frames. P6 removes superseded paths after their replacement gate passes.

General cone profiles, arcs and higher-degree or rational authored splines are
not covered by this fixture. Primitive cylinder/cone tests remain passing.


## Conditional represented-flux checkpoint

The decoded spans now feed a certified conditional boundary-flux integrator.
Ruled sides reduce to exact one-dimensional rational quotients; cylindrical cap
UV contributions are exact polynomial integrals. The enclosure sums every
declared contribution before final outward rounding and records global/per-span
work. The native volume estimate remains separate. See
[certified flux](CERTIFIED-FLUX.md) for the derivation, source binding and controls.
Full frozen face/trim coverage and ideal-realization measure correspondence
remain required before the scalar becomes a published body measure.
