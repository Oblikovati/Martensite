# Final Fuse solid ancestry (#456)

Two overlapping owned box fixtures reproduce the defect on main `77053b4cd`:
Fuse completes with one solid, but the first original solid has no Modified
relation to that final solid. The geometry operation succeeds; the ancestry is lost.

General Fuse records source-solid images as split cells. Fuse then assembles new
solids from the selected cells' boundary faces. Generic history preparation keeps
only images whose native identities occur in the final result. The intermediate
cells fail that filter because the final assembled solid has a different native
identity.

The fix composes source-to-cell provenance with the actual solid-builder areas
before generic history preparation. Each area's boundary faces select their
recorded cell owners. Traversal across the faces canceled between selected cells
includes interior cells whose faces disappear from the final boundary. The
resulting relations refer to the actual builder area handles. Native face
incidences carry ancestry; they do not define output identity. No coordinates,
proximity, output rank, or parent-set matching participate.

All area relations are collected before source images are replaced, and the
reverse Origins bindings are retained with every actual source parent. The
shared retention path covers ranged and ordinary Fuse. Disabling history skips
the new traversal. Face selection, geometry construction, limits and Boolean
operation defaults stay unchanged. Untouched solids retain exact identity;
Cut retains its existing selected-cell ancestry.

The regression plan checks overlapping, nested, face-touching and disconnected
inputs, multiple source parents, split and empty Cut results, all supported
vertex/edge/face/solid result ancestry, and identical geometry with ranged,
ordinary and disabled-history execution. Open-solid reconstruction is outside
this closed-solid Fuse correction.

## Nested cells and redundant operands

The nested-box control exposed a second ancestry case. Its inner original solid
is also a selected cell of the outer argument. The cell's Origins list records
the outer argument, while the unchanged inner source has no separate image-map
entry. Treating exact self-source as a fallback only when Origins is absent loses
the inner parent. The fix retains the union of both proven origins.

This differs from attaching every operand to every result. The test explicitly
checks the selected cell's native identity. Another control places an interior
source across the interface of two adjacent outer boxes: all its boundary faces
disappear from the union, but its selected-cell ancestry must remain. A merged
component beside a disconnected input checks that unrelated parents stay absent.

## History oracle version 2

The raw Boolean census has no separate runtime history-version selector. Its
source-history rows were captured alongside geometry, diagnostics and callback
traces; the toolkit verification guide requires a reviewed reason and behaviour
version when a recorded baseline changes.

The kernel decision for #456 is to correct the final-solid history oracle to
version 2. Version 1 is retained in Git at `77053b4cd`, including the original
`bop/progress_fixture/states*.expected` files. It records intermediate cells as
images and may mark contributing solids deleted after final Fuse assembly.
Version 2 requires the actual final solid and every proven original parent.
This is a migration of incomplete history, not a new public option for producing
that incomplete history. The geometry, numerical policy and progress/event
oracles retain their existing behaviour versions. Package versions are unchanged.

The independent expectations come from the owned two-box union and the eleven
lineage controls. The two overlapping original boxes must both parent their
single assembled solid. The selected nested original must retain its own
identity as a source alongside another source's Origins binding. Disconnected
inputs must not become parents of the rebuilt component. Cut and empty results
retain their existing actual ancestry and deletions.

The coordinator approved the exact case-keyed history-only expected-file
correction after independently auditing all 39 delta patterns. A bounded diagnostic run covers profiles 21, 37, 49, 57 and 61. It compares
all geometry, core diagnostic flags, poll counts, result-container fields and
ordered callback traces to the original files, while recording differences in
source-history rows. That diagnostic mode is removed before acceptance testing;
it cannot serve as the PR gate.

The lineage walk runs inside the existing assembly/retention interval between
cancellation polls. It adds no poll and claims no bounded cancellation latency.
The existing ranged driver keeps observed cancellation sticky and classifies the
operation at its next established boundary. The original poll/event oracle is
retained for this correction.

## Bounded oracle comparison

The diagnostic comparison executed 4,034 cases in 111.77 seconds. It found 519
source-history rows to correct: 174 in profile 21, 174 in profile 37, 150 in
profile 49 and 21 in profile 57. Profile 61 stayed unchanged. These rows change
only Modified counts, Removed flags and Images. Generated counts, image-binding
presence and exact source survival stay unchanged.

Across 99 cases, 129 aggregate history flag values differ: HasDeleted changes
in 99 and HasModified in 30; HasGenerated stays unchanged. Every other core field,
geometry count, result-container field and poll count matched the original
oracle. Existing ordered callback assertions passed; as before, the census
checks ordered callbacks only for sequential cases and checks core/poll fields
for both execution modes.

The approved correction touches seven state files and has 39 distinct
profile/history-row delta patterns, including interrupted states. No event or
frame dictionary changed. The temporary diagnostic instrumentation
was removed byte for byte before strict Clippy; the ordinary 19,598-case census
remains the acceptance check. The unmodified full matrix passed with the
approved expected files.

## Operation-level API reference correction

The canonical gate completed with one failing test: the 300-row Boolean API
reference expected `HasDeleted = 1` for a box fused with a through-cylinder.
The corrected history reported zero. The other 2,644 toolkit tests passed;
the complete 2,177-test library suite, including the Boolean cancellation
census, also passed. The failed gate remains recorded as exit 1.

A bounded probe replayed every recorded operation and captured all supported
input dispositions for the fuse. Its baseline linked the native Boolean
implementation from the #446 lane: its native Boolean source is byte-identical to
main `77053b4cd`; the lane's separate planar-area cross-check is not called by
these native primitives or Boolean operations. This is a scoped native-path
comparison, not a claim that the whole baseline checkout equals main.
Both variants used the same unoptimized dependency profile.

The baseline matched all 300 original reference rows. The corrected result
differed only at row 49, changing `deleted 1` to `deleted 0`. Of the 36 supported
input vertices, edges, faces and solids, only the two original solids changed
history: each acquired the same real final-solid image and ceased being marked
removed. The other 34 dispositions were unchanged, and no input remained removed
or without a trace. Both solids contribute material independently: the
fixture here is a side-10 box and radius-2, height-20 cylinder crossing its
interior over height 10, giving union volume `1000 + 40π`.

The reviewed BooleanSolidHistoryV2 correction therefore changes that single
aggregate history field. All geometry rows, completion flags, warnings, section
indices and other history flags remain byte-for-byte unchanged. A direct API
regression also requires both concrete source solids to map to the one final
solid and to remain nondeleted. This correction changes no production code,
operation settings or package version, so it adds no production execution or
allocation cost; the extra test performs one fuse.

The raw before/after snapshots and disposition rows are retained compressed in
`references/456-api-probe-*`, with library hashes and the bounded comparison.
The temporary probe implementation and executables were removed after capture.
The approved one-field patch had SHA-256
`62057552161e16e42debb21802cb64ce9cc7dd5c1d4deecd1f813c00a098e2bf`.

The two focused API tests passed in 0.09 s with the canonical optimized test
profile. Strict library-and-test Clippy passed in 4.76 s, and the formatter check
passed. The corrected reference and regression require a new canonical gate;
the earlier exit-1 run is not counted as a pass.

[Matched enabled-history performance measurements](456-performance.md) retain the
raw timing/memory observations, #461 qualification and temporary-probe cleanup.
