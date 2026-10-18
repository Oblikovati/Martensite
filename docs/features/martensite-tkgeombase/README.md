# Geometric construction and analysis

`martensite-tkgeombase` provides elementary construction, curve and surface
conversion, approximation, extrema, parameter sampling, analytic intersections,
bounds, local differential properties and mass properties.

## Evaluation and ownership boundaries

These are low-level numerical and geometry tools. Completion flags and numerical
error estimates retain their individual method contracts; they do not certify a
validated modelling operation or an achieved fit bound. Enclosing operations must
attach entity identities, validation coverage and tolerance policy.

Geometry handles can share mutable objects. Cloning a handle does not promise
input isolation. Several evaluators retain caller-owned callbacks or borrowed
adaptors. Their lifetimes and thread restrictions remain part of the interface;
no blanket concurrent evaluation or immutable publication guarantee is made.

Finite-element reference caches and polynomial precision settings retain
process-wide state. Run stateful work in an explicitly isolated process when
independent settings are required. Their tests use dedicated executable targets
and restore settings where the interface permits. Reading an unrelated
geometrical result does not establish isolation of these numerical contexts.

Diagnostic category names, descriptor labels and structured text identities are
stable behavior-version-1 interfaces. Their spelling is retained for consumer
diagnostics and replay. Deprecated construction aliases remain available with
guidance to their current Rust names.

## Verification

Run `cargo test -p martensite-tkgeombase`. The ordinary tests exercise numerical
construction, analytic formulas, transformations, derivatives, approximation,
exceptional inputs and reviewed request/result baselines. The standalone targets
preserve stateful precision, reference-cache and finite-element ordering checks.
The finite-element corpus target starts the same compiled executable for each
shard: 64 shards in two passes, with 1,230 retained rows. Its child markers and
exact-test selection are part of the test transport contract.

Expected results are compiled into the test programs. No external geometry
engine or interpreter supplies them at runtime. Frozen numerical baselines are
distinct from independent formulas and invariant checks; updates require a
reviewed reason and behavior version. Existing ignored documentation fragments
remain explicitly ignored and are not counted as executed checks.

Viewer, process-policy and higher-level integration tests remain with the
integration packages that provide their real dependencies. They do not add
upward dependencies to this crate. Package acceptance, supported platforms and
release checks are recorded separately; fixture retention adds no new numerical
accuracy, concurrency or performance guarantee.
