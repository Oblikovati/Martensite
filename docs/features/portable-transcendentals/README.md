# Portable transcendental functions (#201)

All transcendental operations go through one owned adapter, `martensite-cmath`,
which is backed by a correctly rounded library. This gives bit-identical results
on every release target. The maintainer decided this numerical policy on
2026-10-19 and approved it as numerical behavior version 2, with a recorded
migration instead of version-1 replay. The changed baselines described below were
admitted through the [attribution proof](attribution/README.md). Release
qualification still depends on the full platform matrix.

## Fixed scope

`inventory.json` pins the original and candidate commits. `issue-201.json`
contains the GitHub issue fields fetched during this review. `changed-paths.json`
lists all 437 changed paths between those commits.

The fixture inventory covers all 73 changed `.txt` / `.txt.gz` test fixtures:
975 changed decoded rows and 167,741 positional token differences. Expression
wire output accounts for 118 rows. Other fixture families are kept distinct in
`fixture-files-*.json`. Counts describe textual records, not distinct tests,
algorithmic errors, floating-point results, or semantically aligned fields.

`changed-rows-*.json` records every changed row's old/new byte hash and each
changed whitespace-delimited token. Row numbers are one-based; token indices
are zero-based. Gzip files are decompressed before comparison, while both
compressed and decoded hashes are retained. Positional comparison does not
establish request identity; insertions can shift subsequent token positions.
Whitespace-only changes are explicitly marked. The immutable Git blobs retain
complete old and new bytes, including line endings.

`changed-test-source-*.json` retains zero-context patches for all 159 changed
Rust test source files. This includes assertions, input builders and new tests;
these are not silently counted as unchanged tests or mapped semantic cases.

## Admission

Each changed row now carries an `independent_validation` status and
its attribution: the test that replays it, the pass status of that test in both
replays, and the differing elementary-function calls in that test.
[attribution/README.md](attribution/README.md) explains the method and the
results: 971 of 975 rows are attributed to rounding. The other 4 had changed only
the NaN sign of an `acosh`/`atanh` domain error; #203 restores their original
bytes. Every one of the 1,371,767 finite
differing calls was confirmed against MPFR. The proof also reviews every changed
test source; no input, comparison or tolerance was weakened.

Behavior version 2 and its migration notes are recorded in `CHANGELOG.md` and
`behavior-migration.json`. Version 1 is not selectable inside the release, and
applications that need its exact results stay on the 0.3.2 crates.

The dependency review is complete: license compatibility, exact pins, and
`cargo deny` and `cargo audit` results are in `dependency-review/review.json`.
CPUs without FMA are not qualified, and the four supported release targets are
qualified by the release matrix.
