# Fallible message delivery

Feature [#112](https://github.com/Oblikovati/Martensite/issues/112) adds an explicit
failure channel for operation callers whose diagnostic sink can refuse a message.
Existing `send` methods, buffering, defaults and printer implementations keep their
behavior. No release or version change is included in this candidate.

## Contract

`Printer::try_send` accepts an already filtered ASCII message and returns
`Result<(), Failure>`. Its default invokes `send` once and succeeds. A printer
that can refuse delivery overrides this callback and returns an owned failure
with its type, message bytes and stack. There is no panic interception or implicit
retry. A printer can already have emitted part of its message when it fails;
previous deliveries are not rolled back.

Filtered `try_send_*` methods use the current trace threshold before conversion.
Extended text is converted to UTF-8; byte and string-stream inputs end at the
first NUL. Null objects are successful no-ops. These new methods call the new
callback; they do not invoke custom overrides of the corresponding infallible
overloads. Such printers must explicitly implement their desired fallible policy.
Existing infallible overload dispatch is unchanged.

`Messenger::try_send_*` visits an ordered snapshot and returns the first failure
unchanged. Later printers in that snapshot are not visited. `try_send` uses Warning
gravity. An empty sequence succeeds. Callers choose whether a returned failure
aborts their operation or becomes a diagnostic; the messenger makes no such choice.

```rust
use martensite_tkernel::message::{Gravity, Messenger};
use martensite_tkernel::standard::Failure;

fn report_progress(messenger: &Messenger) -> Result<(), Failure> {
    messenger.try_send_c_str(b"operation started", Gravity::Info)
}
```

## Ownership, reentrancy and costs

The printer-list lock is released before a callback. Editing the list affects
subsequent snapshots, including nested delivery, while the current snapshot keeps
its original ordered printer handles. Callback side effects remain owned by the
printer. Sharing a printer intentionally shares its state; messenger snapshots
do not provide sink isolation or transactional output.

Delivery retains the existing snapshot cost: one vector allocation for a nonempty
sequence and one Arc increment/decrement per selected printer. Conversion has the
same per-printer allocation shape as existing entry points. ASCII delivery adds
no message copy in the framework. Returning a Failure moves it unchanged; any
failure construction or clone is the sink's policy. No global state, new locks,
unsafe code, dependencies or production fallback is added.

The existing Printer Send+Sync bound remains; Messenger and Failure compile-time
assertions, concurrent delivery tests and independent-messenger assertions guard the interface. This
candidate does not certify every sink's thread safety, bound callback execution,
or provide cancellation. The coordinating release gate must run the configured
concurrency/sanitizer suite on its accepted candidate.

## Verification

Owned literal records check byte conversions, gravity, filtered/no-op delivery,
original infallible dispatch, first-failure stopping, failure payload identity,
reentrant snapshot edits, independent messengers and failure-copy isolation.
Existing foundation tests remain unchanged. Run the new cases with:

```sh
cargo test -p martensite-tkernel --test toolkit message_fallible_delivery
```

The full PR gate remains `cargo xtask pr`. This additive candidate is pending
maintainer review and the usual integration and release checks.
