# Stream device ownership during unwinding

`storage::IoStream` keeps its device attached when a write, input or output seek, or position query panics. The original panic continues to the caller. Closing a stream releases its device, and `into_device` transfers ownership.

A refill publishes bytes only after the device's read returns. If the read unwinds, the stream discards the incomplete scratch buffer and sets `BAD`. A following read follows the existing failed-stream rules.

The device may have consumed input or changed output before it panicked. Recovery depends on that device's I/O contract; retaining its owner does not promise rollback.

The regression cases in `crates/martensite-tkernel/tests/stream_device_unwind.rs` exercise each positioning entry point and injected read/write panics. See [#318](https://github.com/Oblikovati/Martensite/issues/318).
