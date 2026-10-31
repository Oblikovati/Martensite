# Borrowing an existing input stream

`storage::with_borrowed_input_stream` runs an `IoStream` decoder on the caller's existing `osd::IStream` buffer. The caller grants exclusive use for the synchronous callback. The bridge preserves the initial state and last-read count, then returns unread lookahead and commits the decoder's final observations.

The callback chooses its read, clear and seek operations. Its return value is preserved exactly, including a returned error or a value borrowing other caller-owned state. The outer `Result` reports whether the stream was rejoined; callers still check their decoder's result and stream state before using decoded contents.

```rust
use martensite_tkernel::osd::IStream;
use martensite_tkernel::storage::{StreamRejoinFailure, with_borrowed_input_stream};

fn read_prefix(input: &mut dyn IStream) -> Result<([u8; 8], i64, u8), StreamRejoinFailure<([u8; 8], i64, u8)>> {
    with_borrowed_input_stream(input, |stream| {
        let mut bytes = [0; 8];
        stream.read(&mut bytes);
        (bytes, stream.gcount(), stream.rdstate())
    })
}
```

`StreamRejoinFailure<R>` retains a returned `R`, the typed cleanup cause and known unread bytes. Its observation distinguishes the last input count from position coherence. A device interruption can leave movement unknown even when subsequent buffered bytes are retained. Replacing or closing the temporary stream is unsupported; the failure reports that loss of identity and does not promise rollback.

A callback panic resumes with its original payload after successful cleanup. If cleanup also fails, `BorrowedInputUnwind` retains both causes. A caught device interruption remains recorded for the whole borrowing session, including across a clear/retry sequence.

The callback runs without a buffer lock spanning it. Individual I/O calls use the existing shared-buffer synchronization. The temporary `IoStream` remains thread confined; panic payloads are `Send` and can be non-`Sync`. The returned value needs no `Send`, `Sync` or `'static` bound.

`IoStream::with_shared_buffer` constructs an ordinary stream with independent flags and counts. The archive file helpers use that constructor and retain their existing behavior. The scoped borrowing operation serves an already-open caller whose observations must be reconciled.

See [#319](https://github.com/Oblikovati/Martensite/issues/319).
