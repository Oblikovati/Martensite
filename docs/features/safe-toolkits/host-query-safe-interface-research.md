# Safe host-query interfaces

Research for Feature #80 and task #89, against Martensite
`26b2fa84c7d386694a3f06360443bc1089d309eb`. This is a proposal, not a completed
migration or an approved system exception. No production code or dependencies
changed. The humanizer skill is unavailable; its prose pass remains for review.

## Current contracts

`crates/martensite-tkernel/src/osd/process/host/native.rs` has three separate
contracts:

- `host_name` calls libc with length 64 into a zeroed 65-byte array. A return of
  minus one becomes the raw OS error number. Otherwise it returns bytes before
  the first NUL, without UTF-8 conversion.
- `system_names` starts with a zeroed `utsname`, ignores `uname` status, and
  extracts `sysname` and `release`, including any partial output.
- `resolve_ipv4` rejects embedded NUL, calls `gethostbyname`, and copies the first
  address into four bytes. Its safety comment excludes another resolver call
  only on the calling thread.

## Host name: compatible adaptation is plausible

The existing pinned nix 0.31.3 safe `gethostname` allocates a vector with capacity
256, passes its capacity to libc, and returns an `OsString`. It does not expose a
caller-selected buffer length. A direct substitution therefore changes the
64-byte contract. The source inspected is
`nix-0.31.3/src/unistd.rs:1331` in the Cargo registry; the
[versioned source](https://docs.rs/nix/0.31.3/src/nix/unistd.rs.html) provides the
same API. Convert with Unix `OsStringExt::into_vec`, never a lossy string.

Two platform adapters follow from the native implementations:

| Target | Existing 64-byte call | Safe full-name adaptation to test |
| --- | --- | --- |
| GNU/Linux x86_64 and arm64 | Names of 64 bytes or longer fail with `ENAMETOOLONG`; up to 63 bytes succeed. | Reject a successful full name of length at least 64, set `ENAMETOOLONG`, and return that error. |
| macOS arm64 | The small-buffer path copies and terminates at byte 63, succeeding with a truncated name. | Keep at most the first 63 raw bytes of a successful full name. |

The Linux result follows from glibc's `strlen(nodename) + 1` comparison with the
caller length and its explicit errno assignment in
[gethostname.c, revision 04e750e75b73957cf1c791535a3f4319534a52fc](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/sysdeps/posix/gethostname.c).
This is not a claim about musl or every Unix libc.

Apple's small-buffer path obtains a full name with `sysctl`, uses `strncpy`, and
writes a NUL at `namelen - 1`. Both paths translate `ENOMEM` to
`ENAMETOOLONG`. See
[gethostname.c, revision 71bbe350ab79eef58113991d817ccc6165061a64](https://github.com/apple-oss-distributions/Libc/blob/71bbe350ab79eef58113991d817ccc6165061a64/gen/FreeBSD/gethostname.c).
Before shipping, verify the relevant hostname limit and actual libc revision
on native CI. In particular, confirm that nix's larger capacity does not change
the reachable error cases. Its capacity is the value passed to libc, not a
public guarantee of exactly 256.

Preserve raw errno on native failure rather than assuming every OS error fits
nix's enum. Successful calls and synthesized length errors need errno fixtures
too. Allocation occurs before the native call in nix; error preservation must
be tested, not inferred merely from the matching return type. No equivalence is
claimed for an interposed libc whose behavior depends on the exact supplied
length, caller errno, or unusual partial writes.

Other safe interfaces were considered:

- rustix 1.1.5 has no public `gethostname` function; its `src/system.rs:29`
  recommends `uname().nodename()`, while the name in `src/not_implemented.rs`
  is not an implementation. That changes the underlying query and error
  interface; it is not preferable for this contract.
- [gethostname 1.1.0](https://docs.rs/gethostname/1.1.0/gethostname/fn.gethostname.html)
  returns `OsString` directly and uses rustix uname on Unix. It cannot directly
  express the existing fallible libc result.
- [hostname 0.4.1](https://docs.rs/hostname/0.4.1/hostname/fn.get.html) offers a
  fallible `OsString`, but no caller buffer argument. No compatibility benefit
  over the already pinned nix interface was demonstrated.
- The inspected [standard-library hostname source](https://doc.rust-lang.org/src/std/net/hostname.rs.html)
  marks its interface unstable (`gethostname`, issue 135142). It is not a
  demonstrated stable replacement, nor does it expose the buffer length.

Review of the subsequent GNU/Linux experiment found an allocation-order
conflict: nix `gethostname` allocates before querying, but the original rejects
an overlong hostname without allocating. The candidate in
[PR #151](https://github.com/Oblikovati/Martensite/pull/151) instead uses the
existing nix `uname` wrapper, which owns its record on the stack and borrows
the nodename bytes. On GNU/Linux this matches libc's underlying query; the
adapter checks length before allocating the successful output. No dependency
feature change remains. This does not resolve the macOS query contract.

At candidate `9333e5d69eaa92b6a65fbd79b7e5c31d01d25b90`, the full local PR
gate passed, as did the original isolated 63/64-byte hostname fixtures against
the revised implementation. Standards and spec re-reviews reported no remaining
implementation findings. Native replacement validation is still pending in
[run 38985381587](https://github.com/Oblikovati/Martensite/actions/runs/38985381587).
These results do not establish a system exception on any other target.

## System names: failure output matters

nix 0.31.3 `src/sys/utsname.rs:47` zero-initializes a record, invokes libc uname,
and returns `Err` without exposing the record when the call fails. Its successful
accessors preserve raw bytes. Consequently, mapping every error to empty names
is equivalent only if failed calls cannot leave relevant output.

On macOS this is a real source-level distinction: Apple's
[uname.c at 71bbe350ab79eef58113991d817ccc6165061a64](https://github.com/apple-oss-distributions/Libc/blob/71bbe350ab79eef58113991d817ccc6165061a64/gen/uname.c)
queries five fields separately, keeps going after individual sysctl failures,
and returns failure if any query failed. A later failure can coexist with an
already populated system name or release. This research has not demonstrated
such a failure on a supported native runner; it also has not proved the path
unreachable. Do not silently erase that distinction.

rustix 1.1.5's libc backend (`src/backend/libc/system/syscalls.rs:26`) calls
`ret_infallible` before exposing its record. Its public infallible interface
does not solve the partial-output requirement. A Linux-only replacement using
nix may be simpler, but still needs native failure analysis and fixtures.
For macOS, investigate established safe sysctl interfaces and the precise
sequence/error behavior before proposing either equivalence or an API conflict.

## Resolver: existing thread-safety concern

glibc's
[gethstbynm.c at 04e750e75b73957cf1c791535a3f4319534a52fc](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/nss/gethstbynm.c)
instantiates the shared
[getXXbyYY.c implementation](https://github.com/bminor/glibc/blob/04e750e75b73957cf1c791535a3f4319534a52fc/nss/getXXbyYY.c).
That implementation has static result/buffer storage and unlocks before
returning its pointer. Another thread can therefore reuse that storage while
Martensite reads it. The current same-thread safety justification is
insufficient. A Martensite-only mutex would not synchronize external libc
callers. The coordinator independently verified this source and filed
[Bug #150](https://github.com/Oblikovati/Martensite/issues/150). No race was
reproduced in this research. macOS's implementation was not audited here.

Safe `getaddrinfo` interfaces are candidates for a migration, not demonstrated
drop-in replacements for first-address `gethostbyname` behavior. For example,
[dns-lookup 4.0.2 lookup_host](https://docs.rs/dns-lookup/4.0.2/dns_lookup/fn.lookup_host.html)
accepts `&str` and reports getaddrinfo errors. The current API accepts arbitrary
non-NUL bytes. Resolver ordering, numeric-name forms, NSS configuration, raw
input and error side effects need explicit comparison. Investigate a safe owned
reentrant legacy interface first; otherwise record the exact conflict under
#88 and request migration approval. No custom unsafe relocation is proposed.

## Verification and remaining work

1. Pin original native hostname and system-name behavior before implementation,
   including raw bytes and incoming errno values zero, a known error and an
   unknown integer. Record platform/libc revisions.
2. Test the hostname adapter with empty input, 63/64-byte boundaries, long names,
   non-UTF-8 bytes, and native failure errno. These are adapter tests, not proof
   that a runner exercised native boundary behavior.
3. Exercise actual GNU/Linux hostname boundaries only in an isolated UTS
   namespace when available. Do not mutate the contributor's host name. On
   macOS use an isolated disposable test machine or report the native boundary
   coverage exclusion explicitly.
4. Preserve existing public APIs and Windows behavior. Run local PR checks and
   native Linux x86_64/arm64, macOS arm64 and Windows validation, checking exact
   revisions and full unsafe inventories. Review dependency feature changes and
   run dependency policy checks if any are introduced.
5. Address Bug #150 separately; do not hide a behavioral resolver change in
   hostname or uname removal. Run concurrency verification for its repair.
6. Delete each replaced first-party call only after its gate passes. Other Unix
   targets, partial uname failures, and resolver compatibility remain open work;
   none is declared an unavoidable system exception by this note.
