# rustix and xattr dependency update (#445)

Main `919b8e31` pinned rustix 1.1.5. Its dependency graph includes the
[public RustSec advisory RUSTSEC-2026-0486](https://rustsec.org/advisories/RUSTSEC-2026-0486.html),
which concerns zero-length extended-attribute buffers. The issue was reproduced
as a dependency-audit failure. No exploit in a Martensite application was
established.

## Coherent update

The two direct rustix pins move to 1.1.6, retaining their features, including
`martensite-tkernel`'s `use-libc` backend. xattr moves from 1.6.1 to 1.7.0.
Only those two registry package versions and checksums change in Cargo.lock;
package versions for Martensite are unchanged. The tar and tempfile dependency
paths share the updated rustix, and tar shares the updated xattr.

The xattr update is necessary. Its 1.6.1 allocation path asks for a value or name
list's size through an empty buffer after the initial 4096-byte read is too
small. rustix 1.1.6 returns an empty result for zero-length buffers and exposes
explicit size-query functions. xattr 1.7.0 uses those functions. Keeping xattr
1.6.1 with rustix 1.1.6 silently returned empty oversized values and name lists
in the owned regression fixtures. The candidate updates both dependencies.

Source inspection used the crates.io archives and the
[upstream rustix fix](https://github.com/bytecodealliance/rustix/commit/8e6aaa6713d2bb64b1fdd8c472e3a32a10a76d70).
The xattr runtime-source changes replace all six old size queries with explicit
query functions. Broader rustix patch changes include platform padding and
syscall corrections; existing native file, clock and resource-limit consumers
therefore retain their regression suites. Established dependencies still own
unsafe system interfaces; this change does not count their unsafe code as
removed from the trust boundary.

## Evidence and reproduction

Two default Linux tests use owned temporary files and small attributes. They
exercise zero-capacity uninitialized value and name buffers through rustix and
require an empty initialized result. Both panicked with rustix 1.1.5 and pass
with 1.1.6. The ordinary xattr reads still return the expected value and name.

Two additional filesystem-capability tests are ignored by default and must be
invoked explicitly with a supplied temporary-file root. On this host the normal
temporary filesystem refused an 8192-byte attribute with ENOSPC. An owned file
under `/dev/shm` accepted it. This is a filesystem capacity distinction; no
production filesystem requirement was added.

The explicit campaign verified all 8192 payload bytes and the exact 200-name
set (6000 encoded bytes), each through path, dereferenced-path and descriptor
APIs. Both cases failed with the intermediate rustix-only update and passed
with the coherent update. The tests do not silently skip unavailable assertions.

```sh
MARTENSITE_LARGE_XATTR_ROOT=/dev/shm cargo test -p martensite-tkernel --lib dependency_xattr -- --ignored --nocapture
```

The root must accept attributes and attribute-name lists larger than 4096 bytes.
Each test creates and removes its own file through tempfile. Existing document
access-policy and ACL controls run in the kernel library suite; no public
Martensite API for arbitrary user-attribute copying was introduced.

The locked change affects rustix's direct native-interface consumers and their
transitive dependents, plus tempfile/xattr/tar consumers. Focused Linux evidence
covers the kernel library, process-policy tests, archive tooling and both
dependency audits. Full affected-package qualification belongs to integration;
macOS runtime and release-matrix checks were not performed in this lane.

## Focused qualification

On Linux x86_64 with the pinned Rust 1.99.0 toolchain, two build jobs, disabled
incremental compilation and stripped development/test debug information:

| Check | Result |
| --- | --- |
| `cargo deny --all-features check` | Advisories, bans, licenses and sources passed |
| `cargo audit` | Passed without the previous unsoundness warning |
| `cargo test -p martensite-tkernel --lib` | 424 passed, 8 ignored by default |
| Explicit large-xattr campaign above | 2 passed, no ignored cases in that invocation |
| `cargo test -p martensite-process-policy --all-features` | 55 unit tests, 8 integration tests and 6 doc tests passed; 1 worker ignored by default |
| `cargo test -p martensite-build archive` | 36 archive-related tests passed |
| `cargo clippy -p martensite-tkernel -p martensite-process-policy --all-targets --all-features -- -D warnings` | Passed |
| `cargo fmt --check -p martensite-tkernel -p martensite-process-policy` | Passed |

Two of the kernel's default exclusions are the explicit filesystem-capability
cases executed separately above. Existing subprocess workers and stress controls
retain their original default dispositions. The broad PR gate is reserved for
the combined integration candidate with #447's local/CI profile correction.
