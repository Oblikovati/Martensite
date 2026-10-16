# Storage output seeking

Feature [#136](https://github.com/Oblikovati/Martensite/issues/136) adds
`IoStream::seekp(position)` for rewriting fixed-width headers after a payload has
been written. The method returns the same mutable stream for chaining.

```rust
use martensite_tkernel::storage::IoStream;

let mut stream = IoStream::from_bytes(Vec::new());
stream.write(&[0; 8]).write(b"payload");
let end = stream.tellp();
stream.seekp(0).write(&(end as u64).to_le_bytes());
stream.seekp(end);
assert_eq!(stream.tellp(), 15);
```

## State and device contract

A valid output seek requires an open device enabled for writing. Negative
positions, output-disabled streams and device refusal set `FAIL`. The device
decides which nonnegative positions it supports. Seeking beyond current contents
can therefore be accepted by an extensible device and refused by a bounded one.

`seekp` preserves EOF. An EOF-only stream can move its output position; `FAIL`
or `BAD` prevents the seek, and `BAD` also gains `FAIL`. It preserves the last
unformatted input count, precision and field width. Read-ahead is synchronized
before absolute device positioning, so subsequent writes use the requested
logical offset. Device failures use the existing stream state interface; this
extension does not add an exception-mask API.

Existing `seekg` continues to clear EOF before an input seek. Existing read,
write and tell operations retain their behavior.

## Ownership and verification

The stream owns its injected device. `IoStream` remains thread confined because
`StreamDevice` does not require `Send` or `Sync`; this extension introduces no
new sharing or synchronization requirement.

The tests use a named recording byte device with a configurable refused seek.
They check independently specified backpatch bytes, EOF/FAIL/BAD state,
read-only and closed streams, negative/refused positions, read-ahead and preserved
input/formatting fields. The existing storage and file-format suites pass
unchanged. The project PR gate runs these checks with the affected packages.
