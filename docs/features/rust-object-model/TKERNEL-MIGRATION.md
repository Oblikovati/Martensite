# TKernel handle migration

`Handle<T>` owns an optional `Arc<T>`. Moving a handle transfers that owner.
Cloning it adds one strong reference. `get()` borrows the stored `T` directly,
including when `T` is a trait object. At the final strong release, Rust runs
the concrete object's `Drop`.

## Register a class and its interfaces

Remove the embedded `StandardTransient` field. The parent in a class
declaration describes the class hierarchy used by `is_kind` and
`is_kind_name`. Share ordinary state through composition. For a base with
virtual methods, make those methods part of a trait that extends `Transient`.

```rust
use martensite_tkernel::{impl_transient, impl_transient_interface};
use martensite_tkernel::standard::{Handle, Transient};

trait Curve: Transient {
    fn degree(&self) -> u8;
}
impl_transient_interface!(dyn Curve, "Demo_Curve", dyn Transient);

struct Line;
impl_transient!(Line, "Demo_Line", dyn Curve, interfaces: [dyn Curve]);

impl Curve for Line {
    fn degree(&self) -> u8 { 1 }
}

let curve = Handle::new(Line).into_upcast::<dyn Curve>();
assert_eq!(curve.degree(), 1);
assert!(curve.downcast_ref::<Line>().is_some());
```

List the interfaces that the concrete class supports in `impl_transient!`.
When an interface extends another Rust trait, list the supported conversions
in its `upcasts: [dyn Parent]` clause. The compiler checks the `Arc`
coercions. Use `impl_transient_interface_target!` with a manual
`RuntimeTyped` implementation when an interface shares a concrete base's
native class name and descriptor, as `Persistent` does.

| Previous usage | Replacement |
| --- | --- |
| `Handle<StandardTransient>` as an erased owner | `Handle<dyn Transient>` |
| `source.cast::<dyn Base>()` when the class implements `Base` | `source.upcast::<dyn Base>()` |
| `source.into_cast::<dyn Base>()` when the class implements `Base` | `source.into_upcast::<dyn Base>()` |
| Cast to a concrete class | `downcast::<Class>()`, `into_downcast::<Class>()` or `downcast_ref::<Class>()` |
| Cast across abstract interfaces when the concrete class is unknown | `cast::<dyn Interface>()` consults the class's registered conversions |
| `as_ptr()` on an erased handle for identity | `object_address()` |
| `T::static_type_handle()`, `T::static_type_ref()` or `dynamic_type_handle*()` | `T::static_type()` or `handle.dynamic_type()` |
| `This()` or the intrusive reference counter | Use a real handle for ownership and read its `ref_count()` when needed |

Handle equality, ordering and hashing use allocation identity across views.
A null or unsuccessful cast returns a null handle. A static descriptor is a
process-lived borrow and can be kept after the object goes away. Keep persisted
class names and ancestry in the declarations. The registry resolves a unique
registered name. Generic collections share their native class name and select
their concrete descriptor by Rust type, so lookup by their name returns
`None`. Persistence callbacks continue to use the names registered with the
schema. Implement `Persistent` on new persistent classes and lend their
`StandardPersistent` fields through `persistent_state()`.

An algorithm class can own an `AlgorithmState` and implement `AlgorithmSource`
to lend that state for reporting. Initialize the state with the concrete
class's descriptor to retain its message names. Register `dyn AlgorithmSource`
as both its parent and a supported interface.

## References and callbacks

Use `&Handle<T>` for a borrowed slot and `&T` for an object the caller already
keeps alive. `observe()` on an immutable handle borrows its slot. `view()` and
`downcast_ref()` borrow the object without changing its strong count. A
`WeakHandle<T>` keeps pointer identity and exposes the current strong count.
Call `upgrade()` to acquire a strong owner. The upgrade returns null after
the last owner releases the object.

A read that must outlive a field lock takes a snapshot under that lock, then
releases the lock. The snapshot is an actual strong reference. Keep it only
for the read and drop it before operations that may retire the receiver.
Aliases of fields and retained slots still follow each replacement. An
owning snapshot keeps the value it captured. Transfer a snapshot with
`HandleObserver::into_handle()` to avoid copying the owner again.

Replace virtual `delete` with `Drop`. Code that needs to inspect a retiring
object passes the actual destructor borrow to the slot's
`with_retiring_pointee()`. The weak pointer cannot upgrade during destruction.
Retirement releases displaced Rust owners after the slot state is updated and
its write guard is gone.

Custom allocators declare `dyn AllocatorSource` in their implementation.
`AllocatorInvocation` holds a weak receiver. Its typed capture acquires a
short-lived owner while inspecting actual fields and releases it before the
virtual body continues. Use `AllocatorInvocation::ref_count()` at a callback
boundary when measuring owners independently of that capture.

## Access the named-color palette

Look up a palette entry with `NamedColor::from_name("GRAY68")` or
`NamedColor::from_index(200)` when a constant has been removed. The palette
keeps its names, aliases, and integer indices for persisted data.
