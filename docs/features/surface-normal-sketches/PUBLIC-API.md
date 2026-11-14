# Wrapped-profile API checkpoint

The P4 working tree exposes `normal_extrude_profile` and its cancellable form
from `martensite_tksketch::surface_sketch` (#418). This checkpoint remains
uncommitted and unqualified for integration. The existing cylinder-rectangle
and conical-sector APIs retain their signatures and behavior.

## Input and result

`WrappedNormalRequest` owns a solved `SketchProfile`, explicit region selections,
a cylindrical support, ordered signed normal depths and accuracy options. Each
selection names an outer loop and optional holes. Curve directions are resolved
once; history translates dyadic spans and split vertices back to the original
authored parameter direction using integer arithmetic.

Current admission supports lines and polynomial Bezier spans of degree one
through three in the world-XY developed plane. Physical support placement is
origin zero, +Z axis, +X radial and zero chart origin. Unsupported placements
refuse. Inward normals reverse and negate physical depth endpoints and preserve
the caller's first/last-depth history roles. General conics, rational and
multi-span authored splines, oblique frames and general conical profiles remain
open.

`WrappedNormalResult` owns frozen geometry bytes, authored history, per-region
reports and the family budget. Reports distinguish certified ideal normal
volume, retained boundary-patch flux and the native quadrature estimate. They
also retain the sampled geometry and native-search self-intersection labels,
incidence coverage, material-classification receipts and effective work limits.
Cap-trace approximation, coefficient rounding and filled-cap realization bounds
have separate fields. Spatial evidence is grouped under
`result.reports()[&region].spatial`; certified and estimated measures are under
`result.reports()[&region].measures`. The groups retain their distinct evidence
types and units.

`export_native` decodes fresh native owners and maps their references to authored
keys. Separate exports share no native topology owners. Mutating source input
after construction leaves the frozen result unchanged. Published metadata has
no mutation interface; request, result and failure types have compile-time
`Send + Sync` checks.

Errors retain operation, stage, failure class, known authored entities and an
owned cause. Cancellation at the final handoff returns an error after discarding
the constructed value. `Refuse(allowance)` rejects unmet aggregate volume bounds;
`Report` preserves the achieved bounds. Neither mode changes spatial allocations.

## Verification

On Linux x86_64, the complete library suite passed 199 tests in 30.59 seconds,
excluding compilation. Two tests are excluded by default: the subprocess worker
is explicitly run by its parent; the separate stress campaign has its own gate.
The public integration controls passed seven tests in 4.22 seconds. Their ignored
worker was explicitly executed twice by the process-replay parent.

The consumer controls check the closed-form volume of a developed rectangle,
source-input isolation, independent native ownership, typed early and final
handoff cancellation, refusal of an insufficient volume allowance, two-process
replay and four concurrent operations followed by sequential replay. Replay
compares full frozen bytes and public reports, history, budgets and material
receipts. These controls cover the canonical cylindrical rectangle. The library
API controls separately cover mixed line/quadratic/cubic profiles, a hole and
disconnected material.

The runs used two build jobs, no incremental compilation and CI's unoptimized,
stripped-debug profile. Strict all-target clippy passed before the subsequent
history-helper extraction; it must be rerun before committing. The full
repository affected-package gate has not passed for this working tree.

## Publication qualification remains open

The approved contract describes an exact normal extrusion of an epsilon-close
profile with the same topology, subject to certified storage bounds. The
[cap-trace accounting review](PUBLICATION-BUDGET.md#cap-trace-accounting-still-needs-publication-qualification)
records the original gap relating authored-angle cap pcurves to fitted rational
space edges. The [cap-realization composition](CAP-REALIZATION.md#filled-cap-distance-composition)
now binds decoded controls, angular lifts, closure, homotopy cones, clearance
and a physical filled-cap bound. That implementation still needs independent
review and the remaining qualification gates before integration.

Sanitizer qualification also remains open. Host startup failed before the test
body; the host loader preload is a diagnostic lead, with no demonstrated cause
or successful sanitizer campaign. Bug #447 and the interrupted full gate remain
separate integration work. Glyph scale, complete application operations,
oblique frames and replacement deletion remain P5/P6 work. No package version
or release tag changed.

The [P4 review](P4-REVIEW-2026-11-14.md) records the independent standards and
spec findings and their follow-up checks. Bug #448 has a tested error-projection
patch, and the endpoint-control and identified new-code size findings have been
addressed. Broader foundation metrics and integration qualification remain open.
