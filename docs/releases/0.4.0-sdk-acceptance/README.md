# Local SDK acceptance for 0.4.0

The Linux release packaging retry passed at
`a137876b62543601525287322c60fa988ee738c3`. The driver verified the extracted
source SDK, built its production libraries offline with all features, ran an
external consumer, wrote the package catalog and passed the expanded unsafe
inventory. This records that local run. The four-platform matrix on the final
`main` revision is pending, and this proof does not publish a release.

The owner ran the standard CI entry point:

```sh
cargo xtask ci run --target x86_64-unknown-linux-gnu --checks-only
```

[invocation.json](invocation.json) records the exact arguments and explicit
environment overrides from the execution call. It does not claim a complete
capture of the inherited environment. The successful retry reused the earlier
affected-package test results as described below; it did not rerun those kernel
suites. The retained [packaging log](packaging-retry.log.gz) is compressed without
editing its raw bytes.

## Artifacts and SDK integrity

All 30 files in the local distribution were hashed and their 15 payload/checksum
pairs matched. The [artifact inventory](artifact-inventory.json) records each
filename, byte count and digest. The [checksum files](checksums/) retain the
original companion bytes. Large SDK and crate archives remain outside Git.

The [catalog](package-catalog.json.gz) records 21 package identities:

| Publication decision | Packages |
| --- | ---: |
| Changed source, generated archive | 4 |
| First publication of an unchanged version, generated archive | 9 |
| Reused historical archive | 8 |

Each of the 13 generated crate archives matches its catalog digest. Each reused
identity matches the catalog's verified history entry, digest and release origin;
those eight archives are not regenerated into this distribution. Their origin
records remain in the catalog, together with the history index and asset digests.

The SDK record identifies the producer revision above, archive digest
`b4cf2138e423e9a6368a884ab38e76ec3d951b3fda76e7f4f8e8580ab1f1c7d9`
and sealed manifest digest
`60ad3c72e53edcfbf4cbfaf625d02bba57289cf3a84307fc20b7e53ac65633f4`.
A separate read of the actual archive verified all 16,676 payload paths, hashes
and executable flags against that manifest, with no duplicate or unsafe member
types. The [manifest](source-workspace-manifest.json.gz) preserves its exact
bytes; [the verification record](sdk-payload-verification.json) records the result.
The catalog's SDK acceptance record is emitted only after verified extraction,
the offline library build and the external consumer succeed.

Individual historical archives keep their original requirements. The catalog
reports 12 incompatible requirement edges for direct individual-archive
consumption; it does not claim that those archives form a coherent graph or
have passed a separate archive consumer build. The complete source SDK is the
verified consumption graph for this run.

## Tests reused and executed

The current build-driver suite passed all 217 tests with zero failures or
ignored tests. Strict all-target Clippy also passed. The [driver test log](driver-tests.log.gz)
and [Clippy log](driver-clippy.log.gz) retain those results.

The earlier rehearsal at `bbe57e2bb7f0d44ac9eaa54cf7d4112521a517eb` passed its
affected-package test stage, optimized build and Cargo package verification.
That whole rehearsal exited 1 later at the archive-directory lookup fixed by
[#285](https://github.com/Oblikovati/Martensite/issues/285). Its retained summary
has 137 successful result summaries, 41,485 reported passes and 78 pre-existing
ignored results. Those totals sum reported results; they are not unique test
identities or a coverage claim. [kernel-test-reuse.json](kernel-test-reuse.json)
binds the original full log by hash and records the stage limits.

Comparing that revision with this SDK producer found no runtime crate changes,
root manifest, lockfile, toolchain or Cargo configuration changes. The only crate
diff adds positive `Send + Sync` assertions to an existing observation test.
[kernel-reuse-scope.patch.gz](kernel-reuse-scope.patch.gz) records the exact diff; the
[targeted observation test](thread-observation-test.log.gz) passed afterward.
The driver changes have the fresh 217-test validation above. This is the bounded
basis for reusing the earlier kernel stage.

Linux x86_64, macOS x86_64, macOS arm64 and Windows x64 still require the release
matrix on the final exact `main` revision before publication. This local proof
records one producer revision and supplies no result for that pending matrix.
[receipt.json](receipt.json) records the complete scope and retained log hashes.
