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
| Concurrency and replay are deterministic | Compile-time `Send + Sync`; same inputs in separate processes and parallel versus sequential execution yield identical source/history metadata | Implementation owner |
| Existing planar behaviour and cost are preserved | Existing planar fixtures unchanged; compare identical planar workloads against the baseline, recording time and peak memory | Coordinator |
| Repository checks pass | Focused tests, then `cargo xtask pr` for changed and affected packages; review against the pinned merge-base with main | Coordinator |

The generic body checker supplies sampled geometry and native-search
self-intersection evidence. Tests must not relabel those checks as exact. A
closed-form volume computed from the request alone does not validate the built
solid. Display tessellation is excluded from all geometry acceptance checks.

## Execution record

No implementation tests have run yet. Geometry contract review passed on its
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
