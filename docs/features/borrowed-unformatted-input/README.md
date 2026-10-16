# Checked borrowed input experiment

Feature [#116](https://github.com/Oblikovati/Martensite/issues/116) tests a minimal
input context over caller-owned buffered sources. The implementation is private
test support, not an exported foundation API. Existing owned streams, public
signatures, defaults and fixtures remain unchanged.

## Proposed seam

`UnformattedInput` is an object-safe consumer interface for byte extraction,
delimited lines, exact block reads, optional position, state and last count.
`InputContext` borrows an `InputBuffer` and owns its flags, exception mask,
last typed device error and extraction count. `BufferedInput` borrows any BufRead;
`PositionedInput` additionally requires Seek. Neither requires write access,
Any, a static lifetime, unsafe code or a process registry. Position is queried
from the actual positioned reader; it is never inferred from a zero origin.

The prototype distinguishes a device failure from a state-mask failure. A device
failure keeps the original io::Error in an Arc, including an embedded typed
Failure. A generated state failure does not replace the retained device cause.
Each failure includes its operation, state, last published count and bytes
extracted in this context. Clearing flags keeps the reader position and original
cause. Updating the exception mask first stores the new mask, then checks the
current state.

Line extraction performs non-consuming lookahead through fill_buf, consumes the
delimiter but stores a NUL instead, and leaves the next byte in the caller's
buffer when capacity is exhausted. Consumed line bytes remain included in the published count when a following
lookahead fails. A bad-mask failure returns immediately;
a generated fail-mask failure occurs after normal terminator publication.
Block failure retains the partial output but does not publish a successful count.
These ordering rules are asserted with owned literal byte arrays.

## Costs and limits

The context does not allocate or copy the entire input. It copies only requested
output bytes from the borrowed buffer. The caller chooses its buffering cost;
the position-aware wrapper uses the reader's logical position. Error retention
allocates an Arc on failure. There are no new locks, global mutable state or
production fallback paths. Source buffer implementations must honor BufRead's
fill_buf/consume contract.

The object-safe source wrappers are thread-confined because their erased reader
need not be Send. A context over a concrete Send source is automatically Send;
owned failure values are Send and Sync. Compile-time assertions check both
contracts. Sharing a source is not supported and no concurrent-sink or sanitizer
certification is claimed by this experiment.

## Promotion and deletion gate

First integrate the checked seam into one bounded tag-header operation using the
existing parser implementation. Preserve the owned stream fixtures unchanged;
compare exact bytes, partial output, counts, position, masks and original causes.
Propagate checked failures through parser control flow immediately. Do not store
an error and continue parsing, copy the entire input, or duplicate a decoder.
Only after that operation passes should positioned binary readers adopt it.

Before promotion, extract the shared input machinery used by owned and borrowed
streams in a separate structural change. Preserve behavior fixtures and account
for any observable policy difference explicitly. This prototype must then be
deleted; its tests become the shared engine's retained fixture gate. Before promotion, assign #116 to an existing milestone. The prototype
has a one-milestone limit: promote with the complete operation or delete it by
the end of that assigned milestone. No production replacement is kept alongside
an accepted shared engine.

Run the owned cases with:

```sh
cargo test -p martensite-tkernel --test toolkit borrowed_input_primitives
```

The full candidate gate remains `cargo xtask pr`. This experiment adds no public
API or user-visible behavior, so it requires no version or changelog entry.
