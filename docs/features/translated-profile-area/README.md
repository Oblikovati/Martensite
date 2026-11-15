# Translated profile area checks (#446)

On main `77053b4cd`, the public region builder rejected a unit square at
`(1e8, 1e8)` with `certified area 1 vs closed form 0`. A square requested with
side `0.01` failed with certified area `0.00010000010728838937` versus closed
form `2`. The exact certificate was correct. The separate binary64 Green sum
lost the small area while subtracting products near `1e16`.

The represented small side is `671089 / 2^26`, not the ideal decimal `0.01`.
Its exact represented area is `450360445921 / 2^52`. The tests use this dyadic
expectation and compare the translated profile with an origin-centred profile
of the same represented size.

The fix evaluates the same closed-form sum about a loop-local working origin.
Line endpoints are expressed relative to that origin. Conic carrier centres
are translated for their Green term; the endpoint chord stays in its original
coordinates because translation cancels from that difference. This working
copy serves only the scalar cross-check. Source curves, native geometry,
identities and lineage are unchanged.

The exact certified integral remains the area authority. Its work limits,
error limits, evidence tags and stored geometry are unchanged. The existing
rounding-scale calculation and acceptance allowance also stay unchanged.
Empty profiles retain their admission decline, and an empty internal loop
retains the former zero closed-form sum.

Controls cover origin and positive/negative large offsets, both polygon
orientations, holes, small represented widths, thin positive profiles and exact
degeneracy. Circle and ellipse carriers and reversed conic chords are checked
at nonzero origins within the existing integral work capacity. The source-to-edge
mapping is checked alongside the area.

## Separate conic work exhaustion (#458)

A full circle of radius 1 centred at `(1e8, 1e8)` and an upper semicircle closed
by its diameter fail earlier in the exact integral, on both unmodified main and
the centred-cross-check patch. With the issue's resolution and unchanged
subinterval maximum 4096, the full circle reports certified parameter length
`0.0418701171875`; the semicircle reports `0.058502197265625`, then WorkLimit.
Origin-centred versions complete. This is tracked separately by #458; this fix
neither raises its work limit nor claims completion for those far conics.

The full-circle reproduction uses `ProfilePlane::world_xy()`, no authored
vertices, and `ProfileCurve::Circle` with id 5, centre `[1e8, 1e8]` and radius 1.
The semicircle uses vertices `[1e8 + 1, 1e8]` and `[1e8 - 1, 1e8]`, a
counterclockwise circular arc between them, and a line back to its start.
Both use `Resolution::new(1e-12, 1e-11, 1e-9, DisplayTolerance { chordal: 1e-3,
angular: 0.1 }, ToleranceCap::Refuse(1e-5))`. The failed runs stop first on these
positive-offset circles; they do not establish a far-ellipse or negative-offset
outcome.

## Verification and performance evidence

All thirteen profile library tests passed on the final centred implementation,
including ten new regression controls. Unmodified
main with the same controls failed only the translated polygon and hole checks;
the nonzero-origin conic controls completed within the existing integral budget.
All 45 existing public profile integration tests also passed. Strict
library-and-test Clippy and the formatting check passed.

The retained performance records compare the same sixteen accepted origin-centred
profile builds in two executables with identical 2,176-test listings. The test
profile was unoptimized, with debug information and incremental compilation off.
The separate #456 canonical gate was active, so these are preliminary measurements,
not a quiet performance qualification.

Six alternating pairs had median user CPU time of 0.48 s before and 0.485 s after.
Warm peak RSS differed by executable-backed page residency. Three sampled pairs
had anonymous memory of 3,936–3,940 KiB before and 3,932–3,936 KiB after. Four
alternating pairs that discarded only each owned executable's cached file pages
had overlapping peak RSS: 41,968–42,200 KiB before and 41,604–42,884 KiB after.
These observations identify the memory source but do not establish performance
parity. The reserved-window comparison below supplies the final paired timing evidence.

Raw timing, RSS, process-map and file-cache controls are retained under
`references/`, including both executable hashes. The final paired rows are stored
losslessly in `446-reserved-window-paired-performance.json.gz`. No machine-wide cache
was discarded, and the conditioning path introduces no allocation.

The final reserved window stopped other primary build/test work while the existing
#456 renderer gate continued. Both executables were pinned to CPU 3. Across 32
balanced pairs after three warmup pairs, mean user CPU time was 0.556890 s before
and 0.545767 s after; mean elapsed time was 0.572782 s and 0.571628 s. The mean
paired user-time difference was −0.011123 s, with a bootstrap 95% interval of
[−0.023567, +0.000135] s. These runs show no observed slowdown; they do not prove
exact equality or describe an isolated production-profile benchmark.

Four final paired file-cache controls gave GNU peak RSS of 43,548–44,020 KiB
before and 43,712–44,632 KiB after. At sampled RSS peaks, anonymous memory was
3,924–3,944 KiB before and 3,836–3,944 KiB after. The residual total-RSS difference
is file-backed; the warmer runs had a larger executable-residency difference.
The records retain these totals rather than subtracting or normalizing them.
Exact peak-memory parity remains unproven.

The temporary benchmark executables were removed after checking for active
executable and file-descriptor users. Their hashes and all measurements remain
in the reference records. Normal build artifacts and unrelated worktrees were
preserved.
