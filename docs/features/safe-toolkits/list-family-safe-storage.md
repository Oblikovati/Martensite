# List family: safe storage feasibility

Feature #80 / task #86, List/BaseList family only. Baseline checkout
`f432c27af57bac48cb0f92ffa903d2804dd438f2`. No production code changes.
The humanizer prose pass is left for review.

Scope: `ncollection/list.rs`, `list/`, `list_node.rs`, `base_list.rs`,
`base_list/`, `node_release.rs`.

## Unsafe inventory at baseline

| Kind | Count | Where |
| --- | --- | --- |
| `unsafe` blocks (non-comment) | 72 | 34 in `list/` + `list.rs`, 35 in `base_list/`, 1 `list_node.rs`, 2 `node_release.rs` |
| `pub unsafe fn` | 27 | `BaseListStorage` (13), `BaseListBasicIterator` (11), `BaseListRawIterator::init_after`, `ListValueNode::del_node`, trait method `BasicListNode::from_list_link` |
| internal `unsafe fn` | 7 | `owner::item_ref/item_mut`, `detach_removal`, `transfer_before/after`, `release_node_payload`, `release_typed_node` |
| `unsafe impl` | 13 | 12 manual `Send`/`Sync` (`List`, `ListIter`, `ListIterMut`, `ListNodeOwner`, `ListSharedState`, `NativeCursorState`), 1 `BasicListNode for ListValueNode` |
| `unsafe trait` / unsafe fn type | 1 / 1 | `BasicListNode`, `DelListNode` |

Production callers outside the family use only the safe `List<T>` surface
(`append`, `clear`, `iter`, `Default`, type aliases): `tkmath` `bnd/range`,
`bnd/sort_box`, `poly/package`, `poly/make_loops`,
`poly/coherent/triangulation/{edit,export}`, and `TypedAllocator::list`. Every
raw item below has callers in `crates/martensite-tkernel/tests/` only. That
does not cover external downstream users.

## Classification

**A: a safe implementation is possible with no change (8).** `ListCursor`
(opaque `Option<usize>` identities, Copy, equality on current).
`BaseListIterator<T, M>` (delegates to `ListCursor`; `value_ptr` only creates
a pointer). `ListValueNode` safe constructors and accessors.
`ListValueNode::allocate_with/allocate_ref` (these depend on `AllocatedBox`,
#84). `ListLinkNode` (`MapLinkNode`, stores a pointer without dereferencing
it). The non-dereferencing parts of `BaseListRawIterator`, `BaseListStorage`
and `BaseListBasicIterator` (constructors, `more`, equality, endpoint pointer
getters, `exchange`).

**B: internal migration candidates, conditional on the raw-view decision (6).**
These items cannot migrate independently of C while `base_storage()` must
remain a dereferenceable chain. Preserving signatures is not enough.
1. `List<T>` ownership and links: the `owners` HashMap of raw nodes, all edits
   and `Drop`, and its manual `Send`/`Sync`.
2. `ListIter` / `ListIterMut`: the raw `next` field and manual `Send`/`Sync`.
3. `ListObserver`: `UnsafeCell` metadata and manual `Send`/`Sync`.
4. `ListNativeCursor` / `ListCursorObserver`: raw iterator in a `Mutex` and
   manual `Send`/`Sync`.
5. `node_release.rs`: shared with `sequence/owner.rs` and
   `sequence/current_release.rs`, so it can be deleted only after the
   Sequence family migrates as well.
6. `removal_phase.rs`: internal two-phase removal.

**C: public API conflicts for #88 (6).**
1. `List::base_storage() -> &BaseListStorage` gives the List's own chain as
   dereferenceable `*mut ListLinkNode`. Fixtures drive
   `BaseListBasicIterator<ListValueNode<T>>` over it and require
   `base.first() as usize == cursor.current_node()`. A safe List can still
   return identity-only pointers (built from node addresses, which needs no
   unsafe), so the size, null and address-equality checks in
   `list_topology_contracts/{destruction,transitions}.rs` can stay unchanged.
   Only the dereference through the raw iterator conflicts.
2. `BaseListStorage` unsafe editing, splicing, `clear` and `remove`, plus
   `DelListNode`: a type-erased chain of caller-owned raw nodes.
3. `BaseListRawIterator::init_after`, which dereferences a caller predecessor.
4. `BaseListBasicIterator` traversal, dereference and arrow methods (all
   `unsafe fn`).
5. The `BasicListNode` unsafe trait (and `BasicListNodeMut`): pointer
   projection from an embedded link.
6. `ListValueNode::into_raw` and `unsafe del_node`, and the `repr(C)`
   link-first layout that item 5 relies on.

All six describe raw intrusive ownership. They are not system APIs. Minimal
replacement: the safe `List<T>` already covers owned use. Identity and size
reads already exist on `ListObserver` and `ListCursor` (`first_node`,
`last_node`, `size`, `current_node`). Staged migration: (1) obtain the #88
decision and an approved migration for the dereferenceable `base_storage()`
contract. (2) Only within that approved transition, migrate `List<T>`
internally (B), replace `base_storage()` with identity and size accessors,
and deprecate then remove the raw BaseList engine, `BasicListNode`,
`into_raw` and `del_node`. Pinned rows that need the raw engine
(`base_list_storage_contracts`, `base_list_basic_iterator_contracts`,
`base_iterator_address_policies` list rows, `list_node_contracts` raw
deleter tests, and the raw-iterator test in `list_topology_contracts`) stay
unchanged until that decision. They are not rewritten as if they were
compatible.

## Prototype

Source: `experiments/list_safe_storage.rs`, with fixtures in
`experiments/list_safe_storage/tests.rs`, which is throwaway and
`#![forbid(unsafe_code)]`. Run it with
`rustc --edition=2024 -D warnings --test`. Result: 11 passed, 0 failed
(rustc 1.99.0).

Representation: one `Box<Node<T>>` per element. This gives a stable value
address, and the node address is the opaque identity, read without unsafe.
The boxes are held in a per-list `Vec<Option<Slot>>` whose `next` links are
slot indices. A `HashMap<identity, slot>` resolves detached cursors only.
Traversal, destruction and splicing follow the links and never iterate the
hash. An owned accounting `Reservation` models the allocator; releasing it
consumes it. `Send`, `Sync`, `Unpin` and `UnwindSafe` come from the field
types; there is no manual impl.

Contracts that the prototype preserves, each with its own test:

- One complete insert, traverse, remove and drop lifecycle. Per node, the
  order is reserve, then construct, then (later) drop in place, then
  release. The value is dropped at its node address and dropped exactly
  once.
- Iteration order, `reverse`, and `iter_mut` that yields disjoint `&mut`
  in link order.
- A same-allocator `append_list` moves nodes, so the cursor identity and the
  value address survive in the destination. A foreign-allocator splice
  copies and then clears the source.
- Destruction follows link order after the owner order diverges: `[3,4,2,1]`.
- `clear` with a panic in a destructor: the observer reads `3 0`, `3 0`, then
  `0 1`. The metadata resets before the remaining destructors, each value
  drops once, and every reservation is released.
- A panic in a destructor during removal keeps the cursor and the tail. The
  later `clear` order is `[2,1,3,4]`.
- A panic in a constructor releases the reservation and leaves the chain
  unchanged.
- A stale or foreign cursor reads `None` and `remove` refuses it. A stale
  remembered predecessor is repaired.
- The observer sees the detach phase (`size 1, empty 1` during the drop).
  It stays with its owner across `exchange`, reads empty after the owner is
  dropped, and works from another thread.

Contracts that the prototype does not yet preserve:

- Physical allocation through `BaseAllocator`. Nodes come from the global
  heap. `incremental` arena reuse, `reset` refusal and
  `native_cursor_resolves_fresh_provenance_after_arena_address_reuse` (stale
  identity equal to a reused arena address) depend on #84.
- The `base_storage()` raw view and every C item.
- `ListNativeCursor`, `insert_before/after`, `emplace_*`, `try_retain`,
  `move_assign`, `clear_with_allocator`, cross-allocator prepend and insert
  paths, and the 1090 transition rows and 23 observer rows. These were not
  ported; there is no evidence either way.
- After the first destructor panic in `clear`, the prototype drops the
  remaining nodes in link order and catches further panics. The current code
  drops them in `HashMap` order during unwinding, and a second panic aborts.
  This is a behavior difference that needs review, not a refactor.
- Negative auto-trait checks (for example `List<Rc<_>>: !Send`) need
  compile-fail tests.
- `iter_mut` allocates one pointer table per call (O(slot capacity)).

## Measurement

Harness: `experiments/list_safe_storage_bench.rs`. The original measurements
below used a custom counting `GlobalAlloc` at commit `187f3cc`; that unsafe
instrumentation is not an approved exception and has been replaced locally.
The workload is identical for both lists, release build (`-C opt-level=3`),
9 runs, median, with `u64` items and N = 100 000. Machine: AMD Ryzen AI
MAX+ 395, Linux 7.1.5.

| Phase | Current µs | Current allocations | Prototype µs | Prototype allocations |
| --- | ---: | ---: | ---: | ---: |
| build (append N) | 42 558 | 500 077 | 4 907 | 100 033 |
| traverse ×10 | 3 014 | 0 | 1 338 | 0 |
| `iter_mut` +1 | 145 | 0 | 257 | 1 |
| splice (build N/2 + `append_list`) | 24 707 | 250 019 | 6 643 | 100 033 |
| `retain` half (cursor removals) | 28 179 | 0 | 5 775 | 16 |
| drop | 32 963 | 2 | 1 924 | 0 |
| total | 133 076 | | 20 758 | |
| peak live heap bytes | 47 128 646 | | 18 235 040 | |

The totals were stable over three repetitions: current 130–135 ms,
prototype 20.0–20.8 ms.

The replacement harness forbids first-party unsafe and uses `alloc_tracker`
0.7.13 in an isolated, unpublished experiment package. Run it with
`cargo run --release --manifest-path experiments/list_safe_storage/Cargo.toml`.
The dependency wraps `System`, including forwarding reallocation, and remains
part of the dependency trust inventory, not a claim of zero transitive unsafe.
Thread-local spans cover this single-threaded workload; whole-workload peak
bytes exclude measurement setup and reporting. Instrumented timings are not
uninstrumented production timings.

Local rerun on 2026-10-18 (nine runs, same N and phase order): every phase's
allocation count matched the historical table, as did both peak-byte totals.
Current phase medians were 43,262 / 2,760 / 158 / 28,435 / 40,636 / 42,195 µs;
prototype medians were 4,514 / 1,389 / 280 / 6,480 / 8,882 / 1,813 µs.
Median total times were 161,732 and 23,308 µs respectively. These are fresh
instrumented observations, not evidence of a production performance change.
The isolated package passed all 12 tests, including a measurement regression
that checks six disjoint 800-byte allocations produce six allocations but an
800-byte peak. `cargo deny` with the repository configuration and `cargo audit`
passed for its lockfile. The local PR gate passed its inventory, whitespace,
formatting and expanded-unsafe checks; it found no affected workspace package
and skipped workspace builds/tests. The isolated experiment tests above cover
the harness. Standards and spec re-review confirmed the substantive research
blockers resolved; integration still requires green checks on the pushed commit.

The comparison is not like for like. A separate measurement shows that
`ListValueNode::allocate_with` through `BaseAllocator::common()` alone makes
500 043 global allocations for 100 000 nodes (5 per node). Thus most of the
current cost is in the allocator path (#84), not in the links. The prototype
has no physical allocator routing. `iter_mut` is the one phase where the
prototype is slower (1.8×) because of its pointer table.

## Recommended next step

Resolve the #88 raw-view contract before replacing `BaseListStorage` backing.
An identity-only `base_storage()` is not a compatible intermediate view:
existing raw iterators dereference it. Keep the current production backing
until an approved migration exists or a genuinely compatible safe
representation has been demonstrated. After that decision, prove the owned
slot representation for `append`, `iter`, `remove` and `Drop`. Gate it on the
unchanged `ncollection_list`, `list_owner_registry_contracts`,
`list_topology_contracts/destruction.rs` and `list_observer_contracts`
fixtures, with any intentionally changed raw-view expectations reviewed as
part of the approved migration rather than silently rewritten.
Also wait for #84 to decide how an owned node obtains `BaseAllocator`
physical storage: the allocation-order and arena-reuse fixtures depend on that
decision. These remain production-migration prerequisites; the research harness
now uses the separately accounted instrumentation dependency described above.
