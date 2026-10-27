# Runtime descriptors through shared handles

Use `Handle::dynamic_type_handle_ref()` to borrow the canonical descriptor of
an owned object through a concrete, base or interface handle. The method returns
`None` for a null handle. Its process-lived reference remains valid after the
object is destroyed and changes neither the object's nor the descriptor's
reference count (#317).

```rust
use martensite_tkernel::standard::{Handle, StandardTransient};

let owner = Handle::new(StandardTransient::default());
let descriptor = owner.dynamic_type_handle_ref().unwrap();
assert_eq!(descriptor.name(), owner.dynamic_type().unwrap().name());
```

`dynamic_type()` borrows the runtime descriptor value, while
`dynamic_type_handle()` returns another owning descriptor handle. Both dispatch
through the original object, as does `dynamic_type_handle_ref()`.

`Transient::dynamic_type_ref()` remains a query on its receiver. Calling it on an
embedded base subobject, including through handle autoderef, selects that
subobject's implementation. Use the owner-aware handle method when the original
object's most-derived descriptor is required.
