# Pinned text workload for P5

The bundled DejaVu Latin font supplies a workload close to the approved plan's
roughly 200 pieces. Loading the supplied text `This thing` / `works!` through
`FontFTFont` produced 239 curve pieces across 20 contours and 15 non-whitespace
glyph instances. The font uses 2048 units per EM. Every loaded outline reported
`NonZero` fill and no possible overlaps. The
[census](references/p5-pinned-glyph-census.tsv) records each authored character
position, including distinct occurrences of repeated letters.

The asset is `crates/martensite-tkservice/src/font/dejavu_sans_latin.woff`,
SHA-256 `1e450312a215e1ad660e7ccfd4619ca54f0020dfcdaf72569fd0e2b7331b6cea`.
The census used `FontFontMgr::embed_fallback_font`, face zero and font parameters
72 points at 4800 DPI. It ran on Linux x86_64 through the repository's existing
font interface. The throwaway source and its example binaries were removed
after collecting the result. These are outline counts, not geometry acceptance
or timing evidence.

## Reuse and ownership

`StdPrsBRepFont::plan_text` already returns positioned items over reusable
`BRepFontPlanarRegion` objects. Those regions expose curves, oriented uses,
outer loops and holes. The adapter belongs above `tksketch`; the modelling
crate accepts owned solved profiles and must remain independent of viewer and
font discovery dependencies. A caller can also provide externally shaped
outlines and placements.

The [owned contour fixture](references/p5-owned-glyphs.tsv) now retains all
239 pieces as exact f64 bit patterns in font units, with explicit text-instance,
line, column, Unicode, glyph, contour and segment references. Its SHA-256 is
`c6061510a1f4df6b5aadf1ebed7aa9a8df43ddbb2409841300d06348f4c91dd8`.
A separate rational-arithmetic Green integral over the polynomial controls
produced the [signed contour areas](references/p5-owned-glyph-areas.tsv).
All 20 contours close exactly in the captured coordinates. These checks prove
fixture integrity and area integrals; they do not establish nesting, selected
material, successful wrapping or performance. The temporary capture example
and its binaries were removed.

The existing planar regularizer accepted all 15 captured glyph instances:
239 curves, 20 loops, 18 disconnected material regions and two holes. Every
returned curve use retained its whole `[0, 1]` parameter range. A focused
source-retention check also proved one-to-one equality of every retained curve
with its captured source segment, including all control points; the fixture
adapter therefore verifies its source-index mapping rather than assuming that
regularizer output order is a persistent identity. The owned-fixture
regression is `wrapped_text_regions::pinned_text_preserves_counters_and_disconnected_material`
in the `martensite-tkv3d` toolkit suite; its focused run passed on Linux x86_64.
This establishes material classification for this fixture, not wrapped-body
acceptance. The next step is to map these selections into owned solved profiles,
preserving the source references and explicit instance IDs. Do not use capture-local region order as
persistent identity. Repeated letters, dots, punctuation and counters need
separate checks. Layout bounds remain metadata and must not become a filled
rectangular region.

## Verification still required

The consumer regression `wrapped_text_regions::wrapped::owned_multiline_text_wraps_all_selected_material`
now submits all 239 curves and 18 selected regions in one request to the public
operation. Its fixed layout uses `[x / 1024 + 2 * column, y / 1024 - 3 * line]`,
a radius of 10, depths `[0, 0.25]`, spatial error `1e-5`, fit level 8 and the
explicit report-only volume policy. This is an owned geometry workload, not
the application font-layout contract. The first full execution used clockwise outer selections and was stopped
after a minimized diagnostic proved that adapter error. The corrected
full-text execution is pending. A single captured `T` with normalized loop
direction completed successfully in 2.18 seconds; this is diagnostic timing,
not a qualified performance baseline. Font outer loops are reversed, with
each directed use flipped, when their signed area is negative; holes receive
the opposite orientation. Captured control points and source IDs are unchanged.

Run and qualify the request through the same public operation as the P4
mixed-profile fixture. For the fixed dyadic placement, the independent exact developed material area
is `42000445 / 6291456`. The radial Jacobian integral for radius 10 and depths
`[0, 0.25]` is `81 / 320`, giving exact authored ideal volume
`226802403 / 134217728` (exactly representable as f64). Translation does not
change the contour integrals; clockwise material contours and the two
counterclockwise holes retain their signed contributions before the final
sign reversal. This oracle has not yet been compared with the operation result.

Record authored and prepared span counts, selected
regions, achieved errors, proof work, time and peak RSS. Derive the independent
developed-area oracle from the owned polynomial controls. Keep ideal normal
volume, native boundary flux and native estimates distinct.

Exercise multiline placement, resizing and seam-crossing placement before
comparing oblique frames. The application cases still require both emboss and
engrave, non-destructive body integration, source history and replay after text,
placement, support and depth edits. The [visual workflow](VISUAL-WORKFLOW.md)
retains the supplied requirements and image hashes.

Measure the P4 and glyph workloads with identical compiler/profile settings and
record host load and CPU allocation. Do not use a concurrent bug investigation
as a performance baseline. A P5 time and peak-memory budget has not yet been
proposed or approved; the measurements must precede that decision. No version
or release change is authorized by this workload checkpoint.

## Inward-depth qualification

Use the same owned profile and selections with `NormalSense::Inward` and
positive depths `[0, 0.25]`. The physical interval is `[-0.25, 0]`; the radial
Jacobian integral is `79 / 320`. The independent exact authored ideal volume
is `663607031 / 402653184`. Preserve the same spatial limit and fit-level
limit, and compare per-region coverage, source history and input isolation.
This case has not run yet. It qualifies inward normal tools only; subtraction
from an attached body and replay remain separate required checks.

## Focused wrapped-glyph controls

The normalized outer-contour `T` regression passed in 2.36 seconds. The
captured `g` regression, with one material region and one hole, passed in
7.27 seconds and enclosed the independent authored volume
`118610109 / 671088640`. Both call the public operation with the same fixed
layout, support, depth and accuracy settings as the full text. These are
focused acceptance controls; concurrent investigation load makes their times
unsuitable as release performance baselines. Full-text acceptance, inward
qualification and application body integration remain open.

The focused inward `g` control passed in 6.47 seconds. It used the same captured
outer and hole, `NormalSense::Inward`, and positive depths `[0, 0.25]`. Ordered
body validation passed, the cap bound met `1e-5`, and the operation preserved
its owned input. The authored-volume range enclosed the independent exact
oracle `347044393 / 2013265920`. This is evidence for a standalone inward
normal tool, not a body-subtraction or full-text engraving result.

A two-region family containing the captured `T` and counter-bearing `g`
completed in 12.99 seconds. Both regions were submitted together, and the
aggregate authored-volume range enclosed the independent oracle
`196901307 / 671088640`. This verifies small-family composition with a hole;
it does not establish the cause of full-text cost or its scaling law. The
239-piece request is still required unchanged.

The seam-crossing `T` control passed in 2.72 seconds after a dyadic arc-length
translation of 62.5 at radius 10. Its vertices straddle `2πR`; the authored
volume oracle remains `39145599 / 335544320`, body validation passed and input
ownership was preserved. This uses the general wrapped-profile pipeline.

The `g` resized by a factor of two in both developed coordinates passed in
9.21 seconds at the unchanged `1e-5` spatial error and fit limit 8. Its
independent volume oracle is four times the unscaled volume,
`118610109 / 167772160`. Validation and cap-realization bounds passed, and
inputs were preserved. These focused cases do not replace full-text,
placement/revision replay or body attachment qualification.

## Body-operation control

The captured `T` outward tool was exported and fused with a radius-10,
height-4 cylinder through `BRepAlgoAPIBooleanOperation` in explicit
non-destructive mode. The arc-length placement was shifted by 2 to keep this
control separate from the seam case. The resulting shape passed the ordered
body validator at spatial resolution `1e-5` and contained exactly one solid.
The immutable wrapped publication retained identical bytes. The focused test
passed in 4.91 seconds. This verifies one native boss composition; full input
native-storage isolation, composed history, independent attached-body measure,
pocket composition and persistent body attachment/replay remain required.

The matching inward `T` tool was cut from the same cylinder in explicit
non-destructive mode. The pocket result passed ordered body validation and
contained one solid; the wrapped publication bytes and request were preserved.
Both body controls now also require a positive finite native volume estimate,
a finite Gauss–Kronrod error estimate at most `1e-8`, and the expected direction
of volume change (boss increases; pocket decreases). These checks reject an
unchanged-cylinder result. They do not certify a Boolean volume error bound
or full native input isolation. The focused `glyph_` selection passed all
14 selected tests, including seven new wrapped-glyph controls and existing
font cases, with zero failures or ignored tests in 9.23 seconds.

## Full-workload cost investigation (#449)

The corrected full-text run remained CPU-bound for more than eleven CPU
minutes without a verdict. A second, temporary stage-probe execution reached
planar qualification but did not leave it in more than two minutes. This
places the observed cost before family clearance, native construction and
body validation, but does not identify its exact inner step. Bug #449 tracks
minimization, attribution and an evidence-based fix. The second probe was
stopped after collecting the stage evidence, and all four temporary probe
lines were removed. The original corrected full run was later intentionally stopped after
27 minutes 45 seconds of CPU time once the main-line minimized reproducer
localized the cost to the planar census. Its terminal exit was 101 after
SIGTERM, not a geometric result or refusal. The retained log is incomplete
acceptance evidence; the unchanged full workload must run again after the fix. No
acceptance, performance equivalence or approved P5 budget is claimed.

## Kernel request edit/replay control

A focused glyph request test changed developed placement by 0.5, support
radius from 10 to 12, and outward depth from 0.25 to 0.5. Each revised
publication enclosed its independent authored-volume oracle and produced
different bytes while the prior publication stayed unchanged. Replaying the
original request restored byte-identical geometry and identical authored
history. The test passed in 11.07 seconds. This verifies deterministic owned
kernel-request replay for these edits; persistent document revisions, attached-body replay and composed Boolean
history remain open. The extended test also replaced the owned captured `T`
request with the captured lowercase `t` at its pinned placement. The replacement
enclosed the independent volume oracle `62194689 / 671088640`; prior bytes
stayed unchanged and original replay still matched. This extended run passed
in 17.10 seconds. Persistent text-document replacement remains open.

The boss and pocket controls now retain the Boolean operation and explicitly
request its native history. For every exported wrapped-tool face, edge and
vertex, they require either retained native identity, reported result images,
or explicit deletion. Every reported modified/generated image must belong to
the output. Both controls passed in a focused two-test run (4.90 seconds).
This establishes native leaf-entity accounting for these cases, not a
persistent composed-history implementation or coverage of solid/shell/wire
provenance.

The successful boss/pocket controls now serialize both native Boolean inputs
with the existing BinTools writer before and after the operation, requesting
triangles and normals as well as topology and geometry. Both snapshots remain
byte-identical in both controls. The focused two-test run passed in 10.97
seconds under concurrent investigation load. This checks successful-operation
input preservation for the fixtures; post-result source mutation, failure and
cancellation isolation still need separate evidence.


## First complete full-text qualification after #449

The final live-check-preserving #449 fix was incorporated locally as
`81419e64`. The full 239-piece profile then passed two controls without changing
its geometry, selections, spatial epsilon or fit level:

- The existing default consumed its 4096 shared bisections and returned the
  original typed `SubdivisionLimit` refusal through the wrapped API.
- An explicit 8192-bisection request published all 18 selected material regions.
  Every member passed its retained validation and physical cap-realization bound
  at epsilon `1e-5`. The outward sum enclosed the independent authored-volume
  oracle `226802403 / 134217728`, and the frozen BRep was nonempty. Requested
  and effective work remained 8192; consumed work did not exceed that cap.

Both tests passed, with no ignored tests, in 587.42 seconds of test runtime.
The enclosing Cargo command took 620.56 seconds elapsed, 638.24 seconds user
CPU and 1.74 seconds system CPU. Its peak RSS was 1,287,176 KiB, including
compilation; that number does not establish modelling-process peak memory.
Other validation jobs were active, so this is qualification evidence rather
than a controlled performance comparison. Strict V3d toolkit-test clippy passed.

The retained log is `target/qualification/p5-full-text-work8192.log`, SHA-256
`e048ff2434434d4793e7a9450d06d195d19327e4a3306b97d5f384d45e58e3a9`.
The executed toolkit binary SHA-256 is
`97d327cc14bd5e0e21a94a44b513d02164474c8a67b97e9a855a58a053b73dec`.
The log records both the compiler profile and the exact test selection.

This closes the first full-text modelling witness. It does not approve a P5
performance budget or qualify oblique frames, conical authored profiles,
persistent text/body replay, composed Boolean history or application layout.
Runtime-only peak memory and a controlled P4 comparison remain required. The
remaining fresh global source checks still need investigation before claiming
that the scale cost is acceptable.


## Runtime-only full-text measurement

The same built toolkit binary ran only
`wrapped_text_regions::wrapped::owned_multiline_text_wraps_all_selected_material`
with one test thread and without Cargo or compilation. It passed in 378.98
seconds elapsed, using 378.86 seconds user CPU, 0.08 seconds system CPU and
194,204 KiB peak RSS (about 190 MiB). The binary hash is the one recorded above;
its feature source and fixture correspond to `daaf5a60`. Subsequent documentation
and main forward-merge changes did not change production source.

The runtime-only log is `target/qualification/p5-full-text-runtime-only.log`,
SHA-256 `f0556410be22fcdfa61b244cf24ba9b770ab35d3f39d16caba74072033c0f573`.
This distinguishes modelling memory from the earlier compile-inclusive peak.
Other validation work was still active. It is one successful measurement, not
an approved budget, an isolated benchmark campaign or evidence of performance
parity with an earlier release.

The next cost investigation preserves fresh live-geometry validation and first
checks whether existing bulk APIs can copy Bézier state coherently under one
read guard. A snapshot-based replacement would also need a kernel ownership and
behavior decision; a cached validity flag under a borrowed native handle is
not sufficient. No such replacement has been implemented or approved here.
