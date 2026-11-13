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

## Remaining gates

P2 must certify angular position/derivatives, stored radial and ruling residuals,
weight positivity, endpoint behavior and demotion, with bounded dyadic arithmetic
and closed-form oracles. It must replace fixture-selected sizing with a priori
canonical sizing and return typed incomplete results on exhausted work.

P3 must establish planar admission, clearance, regularity and the topology-
preserving homotopy. P4 must bind those proofs to the frozen coefficients,
certify represented measures and history, and verify isolation, deterministic
replay in two processes and concurrency. P5 covers glyph scale and oblique
frames. P6 removes superseded paths after their replacement gate passes.

General cone profiles, arcs and higher-degree or rational authored splines are
not covered by this fixture. Primitive cylinder/cone tests remain passing.
