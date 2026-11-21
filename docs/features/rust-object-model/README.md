# Rust-native object model

Status: foundation migration in progress. Decided by the maintainer on 2026-11-21.
Tracking issue: #592.

Martensite's object model copies C++: classes inherit by embedding their parent struct
(`parent: StandardTransient`), abstract classes are traits, and `Handle<T>` stores a
type-erased `Arc<dyn Transient>` with OCCT's intrusive reference count beside it. Every
access must find the `&T` again at run time (a `TypeId` walk through the embedded parents,
or a search in a table of interface casters), and every copy updates two counts. This
feature replaces that with Rust's own model: plain structs in `Arc<T>`, traits in
`Arc<dyn Trait>`, compile-time upcasts and `Any` downcasts.

It starts in `martensite-tkernel` and then moves through every toolkit in dependency order.
The migration stays on `feature/rust-object-model` until every toolkit builds
and passes. Submit one integration PR to `release/next` for the complete
migration. A release and its version require separate maintainer approval.

## Why

The parity work of #583 measured `tkernel` against OCCT 8.0 `TKernel`
([inventory](../tkernel-occt-parity/README.md)). Handles were 3-5 times slower than OCCT.
Two designs were measured on the same operations, in one process, on CPUs 8-15, median of 7
runs, Linux x86_64, release:

- Option 1 keeps the current API and stores the projected pointer in each handle (branch
  `perf/585-handle-count`, parked).
- Option 3 is the Rust-native model described here.

| Operation | Option 1 (ns) | Option 3 (ns) |
| --- | ---: | ---: |
| Create a concrete object (and drop it) | 42.9 | 14.2 |
| Clone and drop an abstract handle | 21.9 | 9.9 |
| Upcast to an abstract handle | 24.4 | 9.6 |
| Down-cast to a new handle | 16.5 | 10.6 |
| Down-cast by reference | 3.8 | 2.6 |
| Virtual call through an abstract handle | 1.09 | 2.61 |
| Kind check | 1.33 | 2.62 |
| `dynamic_type` | 1.11 | 2.57 |
| `get` on a concrete handle | 0.22 | 0.21 |
| Mixed workload, per curve | 124 | 55 |

The mixed workload builds a million curves of two classes, evaluates each, copies the set
and counts one class by down-casting. Handle sizes: option 1 40 bytes for a `dyn` handle,
option 3 16 bytes. Option 3 is slower on access through `Arc<dyn Trait>` because `std::sync::Arc`
reads the object's alignment from the vtable to find the data; a custom `Arc` with a fixed
data offset removes that.

A census counted the handle operations of two real workloads (temporary counters on the
handle API, debug build, CI settings):

| Operation | Cone-text attachment (#419), 57 s | Complex Boolean fuse, 115 cases, 9 s |
| --- | ---: | ---: |
| `get`, concrete handle | 211.0 M | 1.56 M |
| clone | 95.3 M | 0.83 M |
| drop (not the last) | 87.1 M | 0.87 M |
| `get`, `dyn` handle | 85.3 M | 1.02 M |
| cast | 15.9 M | 0.17 M |
| observe | 9.2 M | 0.12 M |
| `is_instance` and `is_kind` | 6.5 M | 0.07 M |
| create (with its last drop) | 2.6 M | 0.05 M |
| `dynamic_type` | 1.4 M | 0.02 M |

Weighted by the costs above, handle work in the cone-text run takes 2,696 ms in option 1
and 1,137 ms in option 3. Option 3 saves about 1,700 ms on clone, cast, create and observe and
loses about 140 ms on `dyn` access and kind checks. The Boolean fuse cases give the same ratio
(26.0 ms against 11.3 ms). By count, the operations where option 3 is slower are 22-29% of
all handle operations; by time they are about a tenth of what option 3 saves.

## Target model in `tkernel`

These are the design decisions. Names are proposals; keep each public name distinct and
greppable.

- `Handle<T>`: a nullable shared pointer, a newtype over `Option<Arc<T>>`, 8 bytes for a
  concrete `T` and 16 for `dyn Trait`. Equality, ordering and hashing compare the object
  address (`Arc::ptr_eq`), because OCCT maps key handles by identity. Clone and drop are one
  atomic update each.
- Upcast is a compile-time coercion: `Handle<Line>` to `Handle<dyn Curve>` through the
  `Arc` coercion, wrapped as `handle.upcast::<dyn Curve>()` (a small per-trait impl, or a
  closure helper such as `handle.map_arc(|arc| arc)`), because `CoerceUnsized` is unstable.
- Down-cast to a concrete class goes through `Any`: every transient trait has `Any` as a
  supertrait, so `Arc<dyn Trait>` upcasts to `Arc<dyn Any + Send + Sync>` and
  `Arc::downcast::<C>()` checks one `TypeId`. A borrowed form (`downcast_ref`) costs no count.
- Cross-cast to another abstract trait (for example `dyn GeomCurve` to
  `dyn GeomBoundedCurve`) is the only case that needs a table: each class declares the traits
  it implements, and the cast looks them up once per cast, never per access. Prefer trait
  upcasting (sub-trait to super-trait is a compile-time coercion) where the hierarchy allows.
- `Transient`: `Any + Send + Sync` plus `dynamic_type() -> &'static RuntimeType` and the
  provided `is_kind`, `is_kind_name`, `is_instance`. No embedded `StandardTransient`, no
  `project`, no `transient_base`, no `delete` hook, no intrusive count.
- `RuntimeType`: a static descriptor per class (name, parent, size, `TypeId`), returned as
  `&'static RuntimeType` and cached per class (one atomic load). Keep the name registry,
  because persistence (`storage`, `fsd`) and `is_kind_name` look classes up by name. Descriptor
  handles (`Handle<RuntimeType>`) become `&'static RuntimeType`, so `Handle::from_static` goes.
- `WeakHandle<T>`: a newtype over `std::sync::Weak<T>`. `HandleObserver` and the other
  non-counting views become borrows (`&Handle<T>` or `&T`).
- `ref_count` becomes `Arc::strong_count`. OCCT's `GetRefCount` excludes const-reference
  upcasts and raw pointers (`Standard_Handle.hxx`); check each caller of `ref_count` against
  the OCCT code it ports and keep its intent (most are copy-on-write tests).
- Concrete classes with subclasses (`StandardTransient` as the universal root,
  `AllocatorObject`, the message alerts) become a trait plus concrete types, or composition.
  `Handle<StandardTransient>` used as "any object" becomes `Handle<dyn Transient>`.
- Class declaration: one short declarative macro or a manual impl per class gives the
  descriptor and the declared traits. The maintainer allows removing macros where they limit
  the design.
- Optional: a project-owned `Arc` with a fixed data offset (as `triomphe` does) to bring
  `dyn` access to the cost of a concrete one. Measure first; it needs `unsafe` in `tkernel`
  under the crate's exception.

The accepted API changes of #583 still apply and are best done in the same pass: public items
that no other toolkit calls become crate-private (#591, list in
[consumed-public-api.tsv](../tkernel-occt-parity/consumed-public-api.tsv)),
[`AsciiString` loses its buffer identity](ASCII-STRING-MIGRATION.md) (#588),
and measurements guide the raw `Sequence` and `List` node changes (#586).

## Migration order

Each step migrates one crate, builds and tests it together with the crates already migrated,
and does not run the tests of crates not migrated yet (maintainer instruction). Order by
dependency level, from `cargo metadata`:

1. `martensite-tkernel` (`martensite-cmath` has no object model)
2. `tkmath`, `process-policy`, `tkcdf`, `tkexpress`
3. `tkg2d`, `tkservice`, `tklcaf`
4. `raster-core`, `tkg3d`, `tktobj`, `vulkan`
5. `tkgeombase`
6. `tkbrep`, `test-support`
7. `tkgeomalgo`, `tkde`
8. `tktopalgo`
9. `tkhlr`, `tkshhealing`, `tkprim`, `tkhelix`
10. `tkmesh`, `tkbo`, `topology-test-support`, `tkxsbase`
11. `geometry-raster`, `tksketch`, `tkbool`, `tkcaf`, `tkv3d`, `tkxmesh`
12. `boolean-cases`, `tkvcaf`, `tkfeat`, `tkfillet`, `tkmeshvs`
13. `tkxcaf`, `tkoffset`
14. `tkdeiges`, `tkrwmesh`, `tkdestep`, `tkdestl`
15. `tkdeobj`, `tkdeply`

`martensite-build` (the xtask driver) has no object model. The release candidate
added `tksketch` after this migration was planned. Its normal dependencies
include `tkbo` and `geometry-raster`. `tkv3d` uses it in tests.

## Procedure per crate

1. Convert the class declarations, casts, observers and weak references; replace
   `Handle<StandardTransient>` with `Handle<dyn Transient>`.
2. Keep observable behavior: output, traversal order (`tests/map_traversal_order.rs`), error
   types and messages. When a ported behavior looks like a bug, check the OCCT source before
   changing it (maintainer instruction).
3. Build and run Clippy on the crate. Run affected migrated test suites once
   their workspace dev-dependencies have also migrated. Cargo resolves those
   dependencies for unit tests as well as integration tests. At the later
   dependency's step, run any newly eligible suite. Run `occt-parity` after the
   `tkernel` step and after any step that touches `tkernel`.
4. Commit each crate locally and integrate it into `feature/rust-object-model`
   in dependency order.
5. Report the measured effect to the maintainer: the parity table for `tkernel` steps, and
   for geometry toolkits a release timing of a representative workload when one exists.

Run crate-scoped checks during the migration. Before opening the integration PR,
run `cargo xtask pr --base origin/release/next` for the full workspace gate.
The [TKernel migration guide](TKERNEL-MIGRATION.md) shows how to convert class
declarations, handles, and persistent objects.

## Risks and open points

- Persistence reads and writes class names; the name registry must stay complete and
  deterministic.
- Recursive types (for example `units::Quantity` and `units::Unit`) need `Send`/`Sync` to come
  from auto traits; an explicit `unsafe impl` with bounds on `T` overflows the trait solver.
- `Arc::strong_count` counts temporary clones that OCCT's count does not; review callers.
- Removing the `delete` hook: only a `tkernel` test overrides it today.
- Peak memory: objects lose the 8-byte native counter and handles shrink, so memory should
  fall; record it per step.
- The branch lives long. Forward-merge `release/next` into it regularly.
