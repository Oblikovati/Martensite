# Native wrapped boss/pocket integration controls (#420)

The public wrapped-profile constructor and existing native Boolean APIs complete
boss and pocket operations on a captured cylindrical body. The owned rectangle
fixture passes through the general authored-profile and rational-geometry pipeline;
there is no replacement modelling engine or shape-specific fallback in these tests.
Production entry points are unchanged by this checkpoint.

## Workloads and expected results

The support body has radius 10 and height 4. The developed profile has area 6,
width 2 and height 3, with axial interval `[0.5,3.5]`. Its tool spans physical
normal depths `[-1,2]`, hence radii 9 through 12. Three placements each exercise
Fuse and Cut:

| Placement | Developed arc-length interval | Frame |
| --- | --- | --- |
| Interior chart | `[0.5,2.5]` | Canonical |
| Across native seam | `[-1,1]` | Canonical |
| Oblique | `[0.5,2.5]` | Origin `(1.25,-2.5,3.75)`, axis `(1,2,3)`, radial `(3,-1,2)` |

The independent ideal volumes are `400π + 13.2` for the boss and
`400π - 5.7` for the pocket. The increments come from integrating the cylindrical
Jacobian over the selected area: `6*(12²-10²)/20` and `6*(10²-9²)/20`.
A `0.01` absolute regression threshold checks those ideal values. It is a
regression threshold, not a certified error budget for a published Boolean result.
Native adaptive integration must also complete with a finite nonnegative reported
relative estimate no larger than `1e-8`.

Ordered native validation must pass and report one solid with one shell. Its
coverage remains non-exact. Five points away from boundaries check expected
membership in the retained core, the affected inner and outer tool portions,
and outside the body/profile. Oblique points use the independent closed-form
basis `(5,-4,1)/√42`, `(1,1,-1)/√3`, `(1,2,3)/√14`.

## Ownership, history and replay

Every Boolean runs with non-destructive mode explicitly enabled. The exported
tool's entire native graph and authored semantic references must encode identically
before and after the operation. Reattaching the selected cylinder through actual
Boolean history first checks the source's complete frozen encoding. Independent
exports also have distinct native owners.

Every authored tool entity must have modified/generated images belonging to the
result, be deleted, or survive by exact native identity. The tests retain all
images returned for each parent. This checks native correspondence; it does not
publish the final application history model.

The replay campaign combines an oblique frame with a seam-crossing profile for
both operations. Separate processes compare result graph bytes, selected-face
references, validation data, authored tool history, image references and deletion
flags. Concurrent operations compare those same values against sequential results.
Native image ordinals in these test signatures measure ordering; applications
must not use them as persistent entity identities.

## Qualification limits

The six placement/operation controls passed, followed by process and concurrent
replay. The combined campaign passed eight active tests with one subprocess worker
ignored during the normal run; the parent explicitly executes that worker.
Strict TKSketch all-target Clippy passed. The existing surface replay test also
passed after its native encoding helper was shared with these controls.

Logs are under `target/qualification/`: `p420-wrapped-membership.log`,
`p420-wrapped-boolean-final.log`, `p420-wrapped-boolean-replay-final.log`,
`p420-wrapped-boolean-clippy.log`, and `p420-shared-encoding-regression.log`.

These controls qualify the rectangle integration cases above. The later
[public trim-band feature](FACE-ATTACHMENT.md#applying-a-wrapped-boss-or-pocket)
adds immutable results and composed history for supported rectangular cylinder
bands. Native body validation retains its sampled and native-search coverage.
General support trims, conical profiles, performance qualification, sanitizer
execution and P6 cleanup remain open for the wider feature.
