# Verification plan

This file separates required evidence from executed results. The initial
baseline is `main` at `4b76a823959260a0bca4506be9a9c45b575aa878`.

## Claims and evidence

| Claim | Minimum evidence | Owner |
| --- | --- | --- |
| Cylindrical depth follows the chosen normal | Independent points and carrier checks on both radial caps and side faces, both normal signs and depth intervals spanning zero | Implementation owner |
| The actual body matches the region | Closed-form sector volume compared with an independently integrated B-rep; per-face geometry, trims, oriented incidence and manifold census (six faces, twelve edges, eight vertices); positive signed volume and reported integration uncertainty | Implementation owner |
| A seam crossing preserves the feature | The same region across zero angle, translated by one period, and in a shifted frame; compare geometric invariants and source history | Implementation owner |
| History is complete and identity is stable | Every result face/edge/vertex has a recorded parent; edit coordinates with fixed IDs; reject duplicate IDs | Implementation owner |
| Bad domains cannot publish success | Nonfinite values, zero/negative radius, axis crossing, tiny dimensions, full/multiple turns, near-closure, coordinate precision loss and arithmetic overflow | Implementation owner |
| Results own their geometry | Mutate the input after construction and mutate detached exports; earlier result metadata and fresh exports remain unchanged | Implementation owner |
| Concurrency and replay are deterministic | Compile-time `Send + Sync`; same inputs in separate processes and parallel versus sequential execution yield identical native geometry, trim and semantic-binding bytes as well as metadata | Implementation owner |
| Existing planar behaviour and cost are preserved | Existing planar fixtures unchanged; compare identical planar workloads against the baseline, recording time and peak memory | Coordinator |
| Repository checks pass | Focused tests, then `cargo xtask pr` for changed and affected packages; review against the pinned merge-base with main | Coordinator |

The generic body checker supplies sampled geometry and native-search
self-intersection evidence. Tests must not relabel those checks as exact. A
closed-form volume computed from the request alone does not validate the built
solid. Display tessellation is excluded from all geometry acceptance checks.

## Execution record

Geometry contract review passed on its
second attempt. The review required explicit native closure margins, unchanged
chart direction under normal reversal, complete semantic history, immutable
publication, and validation of the decoded representation. These requirements
remain implementation acceptance checks.

The first implementation check runs an ordinary and a seam-crossing rectangle
through the existing revolution builder. A successful construction is followed
by carrier, trim, incidence and history checks; construction alone is not a
passing feature result.

Record the exact source revision, platform, command, test count, terminal exit,
measured errors and exclusions here after each meaningful validation step.
Keep throwaway measurement code out of the final change. Retain findings and
measurements here when its investigation is complete.

### Native construction feasibility

On Linux x86_64, the registered `surface_sketch` test target passed its first
native control: one test with two sector placements (ordinary start angle 0.2
and seam-crossing start angle -0.3), zero failures, 0.04 seconds test execution
after compilation. Both used radii 9 and 12, axial height 4 and angular span 0.7.
The independently expected positive volume is 88.2.

Both bodies passed the existing ordered body checker, had six faces, twelve
edges and eight vertices, and agreed with the volume oracle. The explicit
Gauss-Kronrod volume call reported a finite nonnegative convergence indicator
within 1e-9. This is numerical evidence, not a certified enclosure.

The control ran from the #416 worktree based on contract commit `0347012c` with
an uncommitted test, using `cargo test -p martensite-tkbo --test surface_sketch`,
two build jobs, incremental compilation disabled, opt-level 1 and debug
information disabled. Existing per-package optimization overrides remained active.
The local log is `/tmp/martensite416-native-smoke.log`.
This establishes builder feasibility only. Public admission, semantic history,
encoding fidelity and export isolation still require their own passing tests.

### Focused operation and review checks

The first complete public suite passed 16 tests. Review then found that one
carrier-direction check compared a dot-product residual with an angular
limit. The implementation now measures the unoriented angle in radians;
private controls distinguish aligned, antiparallel, within-limit and
outside-limit directions and reject degenerate/nonfinite directions.

After that correction, all 18 public tests and four private regression tests
passed on Linux x86_64. The public suite includes half-turn, greater-than-half-
turn and resolvable near-full-turn sectors, an oblique translated frame and a
small cylinder with independently expected volume 8.82e-8. It compares actual
native geometry, trims and explicit semantic references byte for byte in two
processes and across eight workers performing four operations each. Native
body exports remain isolated when their surfaces, curves and p-curves change.
The private controls also reject swapped same-kind semantic bindings and
check the volume integrator's absolute/relative error-unit transition.

Commands were `cargo test -p martensite-tkbo --test surface_sketch` and
`cargo test -p martensite-tkbo --lib surface_sketch`, with the same compilation
settings as the feasibility control. Both exited zero. The local logs are
`/tmp/martensite416-remediation-public.log` and
`/tmp/martensite416-remediation-private.log`; test execution took 0.25 and
0.05 seconds respectively, excluding compilation. The reviewed implementation
snapshot is recorded locally in `/tmp/martensite416-gate2-remediated-files.json`
with fingerprint `5e5d9a61a5e758f9229ec883cb149539c40ea509fcd05c510ce7bb62b3e05d63`.

All 326 build-driver tests passed after the new typed sanitizer control was
registered. The README example compiled and executed successfully. The local
sanitizer startup control exited 139 before entering `main`; this is a failure,
not evidence about the new operation. Hosted ThreadSanitizer, the affected
package gate and final performance qualification remain required.

### Toolkit placement and cost investigation

The operation was first implemented inside TKBO. An eight-pair planar control
showed median elapsed time of 1.738 seconds before the addition and 1.793 seconds
after it, with median peak resident memory of 23,076 and 24,142 KiB. These were
100-operation samples after five warmups, pinned to one CPU on a shared Linux
host. A subsequent memory comparison differed by only 84 KiB. Inspection also
rewrote the benchmark executables after the first timings, so later recorded
binary hashes cannot identify the initial timed artifacts. These measurements
are diagnostic and do not establish a performance regression or a passing gate.

The independent surface operation now belongs to the downstream
`martensite-tksketch` crate. TKBO and its existing dependencies remain unchanged.
This placement gives the feature its own test suite and avoids changing the
compiled TKBO module for existing consumers. The module move preserved all 23
reviewed source and test files apart from namespace imports and import order.
Existing package versions and lockfile entries remain unchanged; the lockfile
adds the new package only. The mandatory expanded unsafe inventory still checks
the workspace, even when package tests are selected narrowly.

After relocation, `cargo test -p martensite-tksketch --test surface_sketch`
passed all 18 public tests in 0.18 seconds, and
`cargo test -p martensite-tksketch --lib surface_sketch` passed all four private
controls in 0.02 seconds, excluding compilation. Both exited zero using the
same explicit opt-level 1, two-job, debug-disabled settings recorded above.
`cargo deny --all-features check` and `cargo audit` both exited zero; the former
reported only existing duplicate-version warnings and the latter reported no
vulnerabilities among 152 dependencies. Logs are under
`/tmp/martensite416-tksketch-{public,private,deny,audit}.log` on the verification
host. The final placement review accepted the downstream public API boundary.

The actual `cargo xtask ci plan` comparison against pinned main
`4b76a823959260a0bca4506be9a9c45b575aa878` selected exactly
`martensite-build` and `martensite-tksketch` for testing, with sanitizer coverage
enabled. It exited zero; the local record is
`/tmp/martensite416-ci-plan.log`. No base-toolkit suite was selected.
The sanitizer command registry changed, so the existing selection policy also
requires all nine focused sanitizer controls for this PR. Future changes
confined to the sketch toolkit select its own control.

The final planar compatibility control rebuilt the same 100-operation circle
extrusion workload with the original compiler, opt-level 1, debug-disabled
settings and dependency-feature graph. Its TKBO compiler fingerprint matched
byte for byte. All 20 reachable local package directories and existing lockfile
entries were unchanged against both pinned main and `v0.13.0`.

The rebuilt executable had identical allocated ELF sections, addresses and
contents, including executable code and writable data. Its four load-segment
layouts also matched. The code-section SHA-256 was
`a1367b728955bb6ff3e50c51f8a0331878b7cf6404b71c776fdb677eb4fac487`;
the combined allocated-section SHA-256 was
`0cb677090e6986cfd612226529b1900cd10881d857b1045e48383c75dc96d818`.
Only four ELF header bytes concerning nonloaded section tables differed after
the earlier inspection rewrite. This establishes unchanged runtime code and
data for the existing planar workload; noisy timing differences are not used
as a performance acceptance threshold. It does not benchmark the new operation
against an earlier implementation, because none existed.

The new-crate README example also compiled and ran with exit zero. Artifact
comparison details were recorded in
`/tmp/martensite416-planar-artifact-comparison.json`. Throwaway benchmark source
and executables are removed after recording these findings.

The first complete PR gate passed source and expanded unsafe inventories, then
stopped at clippy: the new module used Rust floating-point methods where the
repository requires its shared C-math adapter. Those calls now use
`martensite-cmath` with the same formulas and argument order. Private type and
test-layout warnings were also corrected without changing admission limits.
The dependency was already present transitively; no existing lockfile package
or version changed.

### Final local gate

The corrected source passed `cargo xtask pr` on Linux x86_64 with Rust 1.99.0,
two build jobs, incremental compilation disabled, global dev/test opt-level 1
and debug information disabled. Workspace package overrides remained active.
The command exited zero after source and expanded unsafe inventory checks,
formatting, clippy with warnings denied, affected-package builds, all 326
driver tests, four sketch unit tests and 18 sketch integration tests. No
unrelated toolkit suite ran. The log is `/tmp/martensite416-pr-gate.log`.

The final new-crate and lockfile snapshot has fingerprint
`1c42ed28b3ba7711fc951ba6e02407e24a0e03a26e5347861f6e00902a33844d`,
recorded in `/tmp/martensite416-tksketch-cmath-files.json`. The dependency-policy
and vulnerability checks passed again after the C-math adapter correction.
The remaining merge gate is hosted validation, including ThreadSanitizer;
the local startup failure is not counted as a passing sanitizer run.

### Cylindrical integration

[PR #422](https://github.com/Oblikovati/Martensite/pull/422) merged the cylinder
operation into `release/next` at `7e2d83ca17a1c75d28895317d0e7e3ce36b8873a`.
[Hosted run 42374063707](https://github.com/Oblikovati/Martensite/actions/runs/42374063707)
passed all required checks, both package suites and all nine focused sanitizer
controls. The actual surface-sketch parallel replay test passed under TSan in
26.12 seconds. The merge and tested head have identical tree
`88506603eeab42d98b847f22ab8a80e9a49fc12f`; no merge-only source change occurred.
Tasks #415 and #416 are closed. This establishes the first cylindrical
operation, not the broader feature's cone, curve, full-wrap or attachment tasks.

### Cone contract

The [conical sector contract](CONE.md) passed its first design review. Native
ordinary and seam-crossing sectors under both normal senses must pass before
public implementation. Actual cone carriers and their trimmed half-cone,
positive volume with preserved depth identities, and conditioned coordinate
error estimates remain required checks. The exact rational determinant
controls establish the independent numerical examples only; they do not
certify the constructed B-rep.

The native cone control passed on Linux x86_64: one test covered both normal
senses at ordinary start angle 0.2 and seam-crossing start angle -0.3. All four
bodies passed ordered validation, had six faces/twelve edges/eight vertices and
four conical/two planar carriers, and matched positive expected volumes 53.76
or 47.04. Test execution took 0.14 seconds after a three-minute fresh build.
The command `cargo test -p martensite-tksketch --test surface_sketch native_cone`
exited zero with the same two-job, incremental-disabled, global opt-level 1,
debug-disabled settings used for the cylinder gate. The log is
`/tmp/martensite417-native-cone.log`.

Before changing production code, the existing cylinder replay control emitted
its geometry, semantic-binding and metadata signature with
`MARTENSITE_SURFACE_REPLAY_CHILD=1` and exact filter `replay::child_signature`.
It exited zero and is retained locally in
`/tmp/martensite417-cylinder-before.log` for comparison after structural
extraction. Native feasibility does not yet establish public cone admission,
semantic history, ownership or replay; those remain the implementation gate.

The private shared-sweep extraction passed four unit tests and all 19 existing
integration tests, including the native cone control. All-targets package
clippy with warnings denied also passed. The cylinder replay signature matched
all 12,287 baseline bytes exactly, including geometry, bindings and metadata;
its SHA-256 remained
`34830528a57f0c8a4725e059a9e7d58868050a88331fae01fdfabafc30f8e3da`.
Logs are `/tmp/martensite417-structural-tests-final.log`,
`/tmp/martensite417-structural-clippy.log` and
`/tmp/martensite417-cylinder-structural.log`. This checkpoint changes private
construction/publication inputs only; no public cone operation is included.

### Cone integration conditioning

The first public cone run passed six private controls and 31 of 32 public
tests. The remaining breadth test stayed CPU-bound for several minutes at a
physical angular span of pi. The owned run was stopped with exit 143; it was
not a passing gate. Instrumentation placed the delay inside explicit
Gauss-Kronrod volume integration, after construction, semantic checks, binary
round-trip and ordered body validation had completed.

On the same half-turn solid, the existing reference-plane integration API
with axial plane z=-10 completed in the probe's reported 0.00-second test
time. It measured 241.27431579569566 against the independent expression
`53.76*pi/0.7`, with estimated relative error 6.06666e-15 at a requested
1e-9. The log is `/tmp/martensite417-plane-probe.log`.

The numeric-status integration API retains a purely relative stopping rule.
Its source documents the difficulty of symmetric contributions near zero;
#246 added a roundoff floor only to the completed default route. The cone
probe supports reference conditioning as the cause of the observed delay.
[Bug #424](https://github.com/Oblikovati/Martensite/issues/424) tracks the
general convergence limitation separately. A cone-specific reference choice
does not resolve that general bug. Final cone qualification must retain all
angular-span cases and requested tolerances, check independent closed forms
and actual face geometry, and preserve the existing cylinder route.

The cone operation now selects one axial reference plane below the complete
meridional section. Its coordinates participate in admission's rounding
estimate. The existing cylinder operation retains its original integration
route. The corrected cone run passed six private and 32 public tests; the
public suite took 0.46 seconds. The tests retained narrow, half-turn, major and
near-full sectors, both normal senses, seam and oblique frames, a small-scale
case and an offset crossing the original apex plane. Rational closed-form
volumes and face areas remain the expected results; a second native integration
uses a different plane above the section. Requested tolerances are unchanged.

All-targets clippy passed with warnings denied. The original cylinder signature
again matched all 12,287 bytes and the SHA-256 recorded above. Final focused
logs are `/tmp/martensite417-cone-conditioned-tests.log`,
`/tmp/martensite417-cone-final-clippy.log` and
`/tmp/martensite417-cylinder-final.log`. The 42-file toolkit review snapshot
has fingerprint `468f0e093a70453ba63d9b09bc98f2a51ab80c3a5f651ee959371dfa8b6f938a`,
recorded in `/tmp/martensite417-gate5-files.json`. Temporary diagnostics and the
plane probe were removed. Full PR and hosted sanitizer gates remain pending.

The implementation passed the first correctness review and parallel structure
and standards reviews. They found no material blockers. The combined README
examples also compiled and ran, including the second semi-angle pi/6. Its
measured cone volume agreed with the independent expression
`42 + 2.1*sqrt(3)` within 1e-7. This example check supplements the persistent
3/5–4/5 cone controls; it is not a substitute for the PR gate.
