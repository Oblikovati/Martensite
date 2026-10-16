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
