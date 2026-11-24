# Owned text capture

Feature [#425](https://github.com/Oblikovati/Martensite/issues/425) needs text
occurrences that remain identifiable when their glyphs become sketch profiles.
`brep_font::capture_owned_font_text` captures that input above the modeling
layer. Existing `StdPrsBRepFont::plan_text` and rendering defaults remain
unchanged.

## Content and source contract

The caller supplies immutable font bytes, a logical asset ID and a nonnegative
collection face selector. The private FreeType face opens those exact bytes;
font discovery and Unicode-subset fallback are disabled. Published results own
no native font or library handle. Asset binding compares the logical ID,
selector and every content byte. The ID alone does not establish content
identity, and this API has no global registry for detecting conflicting assets
in separate documents.

Each UTF-8 scalar has a caller-assigned occurrence ID and its exact half-open
byte span. IDs must be unique within the capture; records must cover the text
in source order without gaps or overlap. Repeated characters keep distinct
occurrence IDs while sharing an immutable glyph prototype. Neither a prototype
pool index nor an address is a persistent ID.

Space and LF have explicit successful no-ink outcomes. Other control characters,
including NUL and TAB, are refused. Admission validates the caller binding first;
`UnsupportedScalar` retains its exact occurrence ID, UTF-8 span, scalar and
required policy even after the request has been consumed. This interface maps one scalar per occurrence;
cluster shaping and cluster-to-source lineage are outside its scope. A resolved glyph index of zero is a missing
glyph failure. A nonzero glyph with an empty outline is a distinct successful
empty-glyph outcome. Any required glyph, outline, layout or regularization
failure refuses the complete capture; missing letters cannot become partial
successful text.

## Geometry and replay

Each prototype retains its asset binding, actual primary glyph index, raw
analytic outline and capture settings. Lines, quadratic and cubic Beziers and
contour grouping survive in the raw outline. The published layout retains every
source scalar and its position, including successful no-ink occurrences. The
existing `BRepFontBuilder::build_text(capture.plan())` consumes the frozen plan.
It retains its existing null-shape convention for empty text or build failure;
capture completion is determined by the typed capture result.

Recipe version 1 opens a filled, unsynthesized, unhinted face at 72 points and
4800 dpi and uses the existing formatter. It records width scaling, model size,
model topology tolerance and a separate design-unit regularizer tolerance.
The glyph region is an explicitly frozen processed-source boundary: the
existing regularizer can heal endpoints and omit point-like segments. Capture
does not claim a raw-segment-to-processed-edge lineage receipt. A later modeling
adapter must assign authored segment IDs before consuming the raw outline or
use the processed boundary as its declared source.

Frozen geometry is the replay source. Regenerating it from font bytes is a new
capture. The current font API exposes no FreeType runtime version receipt, so
this interface does not promise byte-identical regeneration across FreeType
versions. It creates no extrusion, Boolean result or validated body.

## Resource and cancellation contract

Requested limits bound font bytes, UTF-8 bytes, scalar records, distinct glyphs,
raw segments, contours, logical control points, processed entities and
cooperative checkpoints. A logical control-point count charges two points for
a line, three for a quadratic and four for a cubic, including repeated endpoints.
Processed entities count vertices, curves, uses, loops, hole references and
material regions. Prototype counts are charged once per distinct glyph.
Placements and source records are bounded by the scalar-record limit.

Input limits are checked before the private font-buffer copy and formatter.
Raw output counts are checked immediately after outline loading and before
regularization. Processed counts are checked after regularization and before
publication. Reported work records charged output counts and completed
checkpoints; failure diagnostics retain the original limits and those counters.
Font and text input byte counts remain available from the owned asset and
request, and refusal diagnostics record their lengths and asset selector. The
private memory-font buffer temporarily duplicates the font bytes and drops with
the private face. Counters are not measurements of allocator peaks.

Cancellation is checked before and after atomic backend, formatter and
regularizer calls and while traversing sources and outline segments. These
existing calls have no interior cancellation interface. Their execution time
and temporary allocations are excluded from the retained-storage and
checkpoint guarantees. Native buffer allocation/layout failures retain their
original typed cause; face and outline rejection remain explicitly opaque
when the backend provides no finer cause.

## Verification scope

The focused controls require the pinned DejaVu 2.37 font in the test-data
submodule and an available native FreeType library. They do not discover an
installed font or skip unavailable fixtures. Executed evidence covers the
supplied `This thing\nworks!` text, independent authored IDs, multiline layout,
quadratic contours, holes and disconnected components, the real text builder,
missing versus valid-empty glyphs, exact content binding, source isolation,
limits, cancellation and compile-time `Send + Sync` for each public type.
Linux x86_64 qualification used `rustc 1.99.0 (b940084d7 2026-09-28)`, two
build jobs, disabled incremental compilation, dev/test optimization 1 and debug
information 0, retaining the existing Math package optimization 3. The final
capture run passed 20 controls in 0.05 seconds. The existing
`legacy_stdprs_brep_font` suite passed 17 controls in 0.36 seconds; all 17
availability records entered their assertions. Strict all-target TKV3D clippy
with `-D warnings`, including the changed build driver, passed in 48.50 seconds.
The full build-driver suite passed 331 tests in 4.08 seconds. These timings describe test
execution/checking, not an operation benchmark. The initial dependency and
capture-test compilation took 7 minutes 53 seconds.

The failed-atomic-call control verifies cancellation before a typed rejection
is propagated. Successful buffer allocation charges an additional checkpoint
before the font bytes are copied; requested caps remain unchanged. No native
call can be interrupted internally. Peak memory and a cross-platform matrix were not measured or executed in this
batch.
Cubic transport uses the existing analytic outline/region types; this fixture
contains quadratic curves and does not establish a cubic-font acceptance case.


## Concurrent ownership and sanitizer gate

Two stress controls each run eight workers for eight iterations. Independent
captures open separate private FreeType libraries from the same immutable font
content and compare their source records, placement bits, raw outlines,
processed geometry and work counters with a sequential capture. The shared
snapshot control concurrently reads one published capture and builds fresh
shapes from its plan. It compares before/after source bits, topology counts,
vertex-coordinate bits and analytic edge values with the sequential result.
The shape comparisons canonicalize value multisets for comparison only; they
never assign persistent IDs from positions, returned order or font-owner
addresses. These are ownership/determinism controls, not body-validation proofs.
Both controls passed under the normal test runner.

The existing CI sanitizer job now has a `FontTextCapture` control owned by
`martensite-tkv3d`, with stable receipt ID `font-text-capture`. Its command filters
`brep_font::text_capture::tests::stress`; affected dependency changes select the
owner through the same package scope as ordinary tests. Driver tests verify
the exact instrumented command, owner/dependency selection and refusal after
startup or control failure. Its existing checkout initializes the dataset
submodule, including the pinned font, before the controls run. A job-specific
contract checks this prerequisite. The pinned startup check and sanitized environment
are unchanged. The runner rebuilds Rust and its standard library with ThreadSanitizer;
it does not rebuild platform FreeType.

The local attempt used `nightly-2026-09-25`, rustc `1.100.0-nightly` commit
`f7575a9da8e4a4fca3b5668d5a2ea7476db44b3f`, LLVM 23.1.1 and the existing
`-Zsanitizer=thread -Cforce-frame-pointers=yes` flags. Startup compiled in
14.71 seconds, then exited 139 with a segmentation fault before the fixture
printed `main_entered`. Its log ended at the startup binary's `Running` line;
there was no race report. Following the existing startup guard, the actual
capture sanitizer controls were not executed.

Read-only diagnosis of our own std-only child located the failure during
sanitizer preinit. `/etc/ld.so.preload` injects AppProtection 26.4.0.5 despite
cleared inherited `LD_PRELOAD`. Its interposed `dlsym` logs through
`__vsnprintf_chk` before TSan's `real_vsnprintf` target is initialized. Register
capture observed RIP zero and a null real formatter pointer; the caller return
addresses and disassembled call sites connect the AppProtection logger and
TSan interceptor. This is a host preload/runtime initialization incompatibility,
not a font-test race report. The host preload and global settings were unchanged.
The one requested per-child ASLR probe also failed; no passing local mode was
established, and sanitizer checks were never suppressed.

Evidence references are `/tmp/martensite425-tsan-startup-diagnosis.md`,
`/tmp/martensite425-tsan-startup-register-stack.json`,
`/tmp/martensite425-tsan-startup-symbols.json` and the loader/syscall logs listed
in that diagnosis. The unchanged startup fixture and exact pinned compiler
already passed on the [existing hosted Ubuntu 24.04 sanitizer job](https://github.com/Oblikovati/Martensite/actions/runs/42379437070/job/128423873118).
That older run establishes runtime availability; it does not cover the new font
controls. ThreadSanitizer qualification remains incomplete until the unchanged
startup and actual capture stress controls pass on the exact candidate source
SHA in a documented compatible execution environment.


### Hosted fixture prerequisite and isolated execution

After the checkout prerequisite change, 43 focused sanitizer/CI-selection driver
controls passed and strict all-target driver clippy passed in 2.47 seconds.
The capture source hashes are unchanged from the 20-control normal test run.

An isolated smoke used the already cached Ubuntu 24.04 image
`sha256:fe81560990fae86dc4743202b2cb4d3df98d0bcf949173cecef15ce1127ef852`
and the unchanged smoke binary
`204940512c1e93d8c0b1ea3317a87abc5116482a496e2b4d312c3b4c54358af0`.
It used no network, a read-only root and binary bind, user 1000:1000, dropped
capabilities, no-new-privileges and Docker's default seccomp policy. No home,
credentials, socket, loader override or host security setting was passed in.
This removed the host preload from the execution environment, but startup
exited 66 before either main marker: ThreadSanitizer reported an incompatible
memory layout and inability to disable ASLR under the sandbox. The suggested
ASLR/sandbox weakening was not applied. No font sanitizer binary was compiled
or executed under this unsuccessful mode. Its exact invocation and failure are
recorded in `/tmp/martensite425-container-qualification/smoke.json`.

The remaining qualification path is the existing hosted sanitizer job with the
new pinned-font checkout prerequisite. Both startup markers and both actual
font stress controls must pass there at the candidate SHA before merge. The
older hosted primitive control remains runtime-existence evidence only.
