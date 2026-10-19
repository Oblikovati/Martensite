# Intel macOS compiler inventory review (#198)

This is a candidate for review on product source
`534e7d6134c4a43926736d8c13b99c591126a8d9`. It adds the missing
`x86_64-apple-darwin` target baseline from an actual native Intel compilation.
It is not an approval of new unsafe code, a numerical fixture update or a claim
that the release matrix passed. Integration and use of this candidate require
maintainer review. No existing target baseline is replaced.

## Capture qualification

The clean native capture used Rust 1.99.0 (`b940084d7`, LLVM 23.1.1), Cargo 1.99.0,
macOS 15.7.9 x86_64 and Python 3.14.7. Its existing PR command compared exact source
534e against genuine earlier source `cc7df05910db1593b240ae28e4e2eec7f4b02644`.
The seven real changed paths triggered compiler collection before package tests.
Nine inventory policy controls passed, followed by whitespace and formatting.

The compiler ran `cargo check --locked --workspace --all-targets --target
x86_64-apple-darwin --message-format=json --target-dir target/unsafe-inventory/build`
with `RUSTFLAGS=--force-warn unsafe_code`. All 4,516 output records parse and end in
`build-finished: success`. Collection produced all three report/stream files,
then the PR command failed because this target's baseline file did not exist.
The nonzero status is preserved; no package tests or later PR stages ran.

All 6,342 tracked inputs matched the clean source before and after collection.
The report's SHA256 is
`ab3948e4bc966723ed79341503fc52e63eeba3ed8679d6b2b2d091fa048b9799`.
The candidate copies its exact target, compiler and ordered workspace summary;
it does not derive counts from another architecture. The raw report and streams
remain retained review evidence outside the product source package.

The candidate records 415 location groups and 2,573 emitted primary spans. Repeated
compilations remain counted; these are not unique unsafe sites or proof of safety.
The 1,789 dependency diagnostic records are separate observations and are not added
to the workspace baseline gate. Existing lint levels, collection code, source
ratchets, safety exceptions and test assertions remain unchanged.

## Comparison with existing reviewed baselines

The comparison reads the existing target baseline files as committed in 534e.
Their original `source_revision` metadata is preserved in the comparison JSON;
this does not recollect ARM macOS/Linux or relabel their historical evidence.
`intel-compiler-baseline-comparison.json` lists every changed location, old/new
count, signed delta, source hash and relevant source line ranges.

ARM macOS has exactly the same 415 location keys, with 414 counts unchanged. Its
only difference is the file-open boundary: 14→16 emitted unsafe-block spans.
The safe `rustix` open branch is selected on ARM macOS and GNU64 Linux. Intel
macOS selects the existing `libc::open` fallback. That additional source block
appears twice in the emitted diagnostics. No unsafe block was added to source.

Linux has 418 groups/2,582 spans. The complete differing groups are:

| Source area | Linux → Intel spans | Existing platform selection |
|---|---:|---|
| signal declarations |2→0|GNU floating-point environment extern block is Linux-only|
| thread processor query |0→2|Apple uses the existing `sysconf` boundary|
| host name query |6→8|Apple uses the existing `gethostname` fallback|
| process memory |2→4|Apple task/malloc queries replace Linux `mallinfo2`|
| thread CPU time |2→4|Apple Mach task query complements shared process timing|
| signal operations |12→4|GNU trap control and non-macOS initialization are excluded|
| page size |0→2|Apple reads the existing Mach page-size global|
| interrupted-sleep test |4→0|Linux-only signal-delivery fixture|
| shared-library test |1→0|Linux-only function-pointer conversion fixture|
| C formatting test |1→0|Linux GNU x64 errno fixture|
| mapped-page test |2→0|Linux-only residency fixture|
| allocator test |9→8|The live zero-reallocate check is GNU/MSVC-only|

All differences follow existing conditional compilation in the unchanged source.
They do not authorize broader unsafe use or close the remaining safety migration.
The ordinary source ratchet and separate compiler comparison must still run.

## Limits and next validation

The capture qualifies source 534e only. Later source changes require fresh native
collection and explicit review; equal summary counts alone cannot detect
same-group substitutions. The full diagnostics preserve source/expansion chains.
The release's numerical test failures are separate and are not resolved by adding
this compiler baseline. Final integration must retain both macOS architectures
and the existing Linux x64/Windows x64 release checks, without masking failures.
