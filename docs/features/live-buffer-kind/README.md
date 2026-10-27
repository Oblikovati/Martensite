# Live buffer-kind capture

`CaptureBufferKindRef` owns one buffer-kind field. Its clones share that field;
constructing another capability from `read()` creates an independent value.
Dropping the original owner leaves retained clones usable. The last clone releases
the storage. Reads and writes release their storage guard before returning.

```rust
use martensite_tkservice::graphic3d::{CaptureBufferKindRef, Graphic3dBufferType};
let field = CaptureBufferKindRef::new(Graphic3dBufferType::Rgb);
let retained = field.clone();
field.set(Graphic3dBufferType::Rgba);
assert_eq!(retained.read(), Graphic3dBufferType::Rgba);
assert!(field.same_slot(&retained));
```

A backend opts into `Graphic3dCView::buffer_dump_referenced` and reads this
capability at each required read site. It may retain a clone across callbacks.
The default method returns `CaptureBufferKindError` with the requested kind,
operation and typed unsupported cause. It never forwards a sampled value to the
immutable method. Existing implementations and `buffer_dump` remain unchanged.
A supported backend's actual false return is `Ok(false)`.

This capability is mutable operation input, not an immutable published result.
It contains only the existing buffer-kind enum; it introduces no generic state
container or callback registry. Each access reuses a brief `SharedState` lease,
and neither a guard nor a closure under that guard crosses the public interface.
Sharing the slot permits separate synchronized reads and writes; it does not make
a whole sequence of callback reads atomic. Callers must order competing writes
when they require repeatable capture choices.

The focused tests exercise virtual dispatch, same-field rereads after writes,
independent copied values, retained ownership, a nested virtual call on the same
field, separate unwind recovery and legacy refusal.
The contention control runs two writers and readers through 512 write/read rounds per
worker before original-owner retirement and 4,096 afterwards. Barriers separate
retirement and the final Depth handoff; intermediate reads must observe an active
writer value, and an independent copied slot must remain unchanged. All eight focused controls passed in two processes, and the earlier existing
driver-model tests and selected strict checks passed. The retained sanitizer
startup control failed before entering its main function on this host. No
instrumented current-package test or sanitizer qualification is claimed; that
check requires a usable runner. The complete affected-package PR gate remains
required before pushing. Renderer adoption and the view-options holder
are separate work under Feature #323; this interface alone does not complete them.

## Required CI checks

`cargo xtask ci thread-sanitizer` uses the pinned nightly toolchain and rebuilt
standard library. Its existing required Linux job first executes the startup
control, then the geometry ownership controls, then all eight reference-aware
capture controls through the actual toolkit test executable. A startup or geometry
failure prevents capture execution; a capture failure fails the same required job.
No workflow permissions, runtime suppressions or host security settings change.
The new driver routing still needs execution and does not turn the local startup
failure into a sanitizer pass.
