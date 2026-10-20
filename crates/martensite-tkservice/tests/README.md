# Visualization service tests

## Prerequisites

The ordinary suite requires FreeType, the bundled fallback font, and an
installed conventional system font. A missing font or failed glyph load is a
test failure. Font tests keep their point sizes, DPI, characters, inputs and
assertions; missing prerequisites cannot produce a pass by returning early.

The `x11-integration-tests` feature enables three additional Linux display tests.
Run the complete suite against a live X11 server:

```sh
cargo test -p martensite-tkservice --all-features --locked
```

Set `DISPLAY` to that server before running the command. The display tests fail
if the connection cannot be opened. Without this feature, the platform-neutral
and pure event-layout assertions still run, but real X11 dispatch, pointer
failure propagation and delivered destruction are not exercised. These are
separate configurations, not equivalent acceptance runs.

The video recorder is built without a codec backend. Three recorder scenarios
need that backend for their bodies; they are not part of this suite.
`image_video_recorder_disabled_backend.rs` pins the disabled behavior: open
accepts even invalid parameters without allocating frames, frame pushes fail,
and frame count remains zero. Those tests keep the enabled scenario's
dimensions, rates, file names and codec parameter strings as inputs, but do not
claim its enabled frame allocation or encoding results.

The Cocoa (macOS) and WNT (Windows) windows, the non-Linux Xw window and the
non-Linux display connection are interfaces without a native backend yet; their
constructors raise `Standard_NotImplemented` on every platform, and the tests
pin that typed error. Tests of platform-specific behavior are gated to their
platform.

## Fixtures

The files under `fixtures/` are reviewed baselines. The tests compare the
crate's output with them exactly (text and IEEE-754 bit patterns; only NaN sign
and payload are normalized where a test says so). No test regenerates them.

- `service_corpus/`: 3,142 requests (`inputs-NN.txt`) and their results
  (`native-NN.txt`), one result line per request, replayed in process by
  `service_corpus_replay.rs`. The test also locks the bytes of both sets with a
  64-bit FNV-1a digest.
- `shader_manager_corpus/`: shader-manager requests and, per request, the
  64-bit FNV-1a digest and byte length of the generated program text.
- `light_set_live/`: an event stream (`events.txt`) and six snapshot files, one
  per iteration filter (`native-0.txt` ... `native-14.txt`), 96 snapshots each.
  Each row holds the filter, the action and its arguments, the action result,
  the state of a primary and a copied cursor, the set revision and extent, the
  raw and enabled light counts, the ambient color bit patterns, the shader keys
  and the ordered source ordinals. Exceptions are coded `-11` (NoSuchObject),
  `-12` (OutOfRange), `-13` (ProgramError) and `-14` (NotImplemented).
- `array-buffer-reference/native.txt`: the buffers of a primitive array.
- `*-native.txt`: expected rows of the named scenario (clip-plane sequences,
  allocators and iterators, view camera and clip slots, exception ancestry,
  texture runtime types, XR actions and reference arrays).
- `frame-stats-*.txt`: frame statistics after assignment and after reset.

These are Linux x86-64 CPU recordings; no GPU context is involved.

## Changing a baseline

A baseline changes only through a reviewed update that states the reason and
the behavior version (AGENTS.md, "Verification and completion"). Do not
regenerate a file to make a test pass.

Behavior version 2 (correctly rounded transcendental functions, #201) changed one
value: the second scale/bias pair of request 901 (`sv_pbr_lut 8.0 2.0 64.0`) in
`service_corpus/native-03.txt` moved by one single-precision unit in the last
place, because the importance-sampling angles of the split-sum lookup table are
now computed by the portable library instead of the platform's single-precision
functions. The FNV-1a lock of the result set was updated with it.
