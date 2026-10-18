# Checked borrowed input

Feature [#142](https://github.com/Oblikovati/Martensite/issues/142) promotes the
input experiment from #116 into `storage::checked_input`. Callers borrow a buffered
source without write access or a static lifetime. The context owns stream flags,
exception masks, published counts and retained typed device causes.

```rust
use martensite_tkernel::storage::checked_input::{BufferedInput, InputContext, UnformattedInput};

let mut bytes = b"tag>body".as_slice();
let mut source = BufferedInput(&mut bytes);
let mut input = InputContext::new(&mut source);
let mut tag = [0; 8];
input.getline(&mut tag, b'>')?;
assert_eq!(&tag[..4], b"tag\0");
# Ok::<(), martensite_tkernel::storage::checked_input::InputFailure>(())
```

## State, counts and causes

`UnformattedInput` is object safe. It offers get, fixed-buffer getline, block read,
actual optional position, state/count inspection and mask control. An `Ok` return
means no enabled exception was raised; it does not promise a complete extraction.
Check state and count when the mask is zero. A mask change stores the new mask
before checking existing state. Clearing flags leaves position, count and the
retained original cause intact.

Line extraction consumes its delimiter and stores a NUL. A full line leaves its
next byte in the caller's buffer. The count includes the delimiter. A device
failure under a bad-state mask returns immediately, before normal terminator
publication; generated fail-state errors occur after terminator publication.
Block failure retains partial bytes but publishes no successful count. Device
failures retain the original `io::Error` and any typed payload through an `Arc`;
generated state failures have a distinct classification.

`BufferedInput` requires only `BufRead`; unknown position is `None`.
`PositionedInput` additionally uses the actual `Seek` capability, including its
nonzero origin and position errors. Position is never inferred from consumption.
The context makes no whole-input copy; it copies only requested output bytes.
The caller chooses its buffer allocation. Retaining a device failure allocates
an `Arc`. There is no new lock or global input state.

## Binary-header consumer

`BinaryFile::read_header_checked` reads the thirteen host-order fields through
this interface. It shares integer-byte publication and header traversal with the
existing owned reader. A partial field is published before an error returns;
later fields remain untouched. Reading needs no write or seek capability and
leaves the body in the caller's source.

`CheckedReadError` distinguishes a masked input failure from an insufficient
field count. The latter describes the actual count and expected four bytes.
When bad state has a retained device cause, the format error owns the same cause;
a later EOF after clearing bad state does not attach stale device history.
Existing owned diagnostics remain unchanged. This header entry alone does not decode container sections.

## Container metadata consumer

`BinaryFile::read_complete_info_checked` shares the existing owned reader's
primitive, metadata and section decoding. It consumes the caller's actual buffer
and leaves remaining caller bytes unread. Section dispatch needs an actual position capability;
the operation never seeks or assumes an origin of zero. It does not copy the
complete input into an owned buffer; decoded metadata fields allocate normally.

A complete header precedes dataset allocation. Info fields are collected before
header metadata publication. A short header leaves a null dataset unchanged; a
short info record leaves the allocated dataset or existing header intact.
`Domain` retains typed storage failures separately from incomplete fields.
`PositionUnavailable` retains state, mask, count and any original device cause
when section dispatch cannot obtain an actual offset. Masked query failures
remain typed input failures. Existing owned diagnostics stay unchanged.

Independent literal containers cover nonempty text and UTF-16 metadata, user
info, comments, shifted origins, untouched bodies and publication/failure order.
Persistent-object construction and document parsing remain separate concerns;
this operation reads container metadata through the existing storage contracts.
The success fixtures declare an empty persistent-data section; their unread
trailing bytes lie beyond its end. A declared nonempty persistent-data section
retains the existing decoder's typed no-progress refusal; it is not decoded by
this capability.

## Owned replay and verification

The existing `IoStream` methods remain the owned behavior-version-1 replay path.
Their signatures, flags and counts remain unchanged. Owned refill refusal still
appears as end-of-input, and partial block reads publish their consumed count.
Checked input has explicit retained causes and mask failures. These policies
share byte, line and block scanning without silently replacing owned behavior.

The erased borrowed adapters are thread confined. A context over a concrete Send
source is automatically Send; failures are Send and Sync. Compile assertions
check those bounded claims. No shared-reader or sanitizer certification is claimed.

Twenty independent tests retain literal bytes, positions, state transitions,
counts and cause identity. A separate reviewed behavior-version-1 fixture retains
3,864 state/count/position/output rows, exercised directly through the public
interface. It is a semantic baseline, not independently derived expected values.
Each stored row ends in a `|` marker outside its payload so meaningful trailing
spaces survive text checks. The test removes only that marker; it does not trim
whitespace. Baseline updates require review stating the reason and behavior version.

The test-only experiment algorithm was deleted. Its fixtures exercise the shared
public implementation. Q3 is the one-milestone promotion deadline for #116/#142;
any unfinished replacement must be removed by the end of that milestone. The
first consumer gate is a complete binary-header read using shared field decoding.
Run `cargo test -p martensite-tkernel`; the full local gate is `cargo xtask pr`.
