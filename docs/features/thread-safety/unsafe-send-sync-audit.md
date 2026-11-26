# Unsafe `Send` and `Sync` Audit (#63)

**Date:** 2026-11-26  
**Scope:** All 64 `unsafe impl Send` / `unsafe impl Sync` implementations across the workspace (`51` in `martensite-tkernel`, `13` in `martensite-tkservice`, `0` in all other crates).

## Summary

- Every `unsafe impl Send` and `unsafe impl Sync` in the workspace carries an explicit `// SAFETY:` comment enforced by `clippy::undocumented_unsafe_blocks`.
- No unsound `Sync` implementation was found: `SequenceTopology` serializes inspection cache writes with `Mutex<()>` and uses an atomic `SequenceLookupCache` for indexed reads (#586); `AccEngine` and `IncEngine` are `Send`-only and guarded by a `Mutex`; `SparseArrayBase` uses `Rc<UnsafeCell<_>>` and is intentionally `!Send` and `!Sync`.
- Collection and allocator `Send`/`Sync` contracts are verified under Miri (`cargo +nightly-2026-09-25 miri test -p martensite-tkernel --test unsafe_send_sync_audit`).
- Removal of the remaining `ncollection` raw-pointer `unsafe impl`s is tracked under `#83`–`#90` (Wave 3) and `#65` (Wave 1).

## Audited Implementations

### `martensite-tkernel` (51 implementations)

| File | Type | Traits | Invariant | Verification |
| --- | --- | --- | --- | --- |
| `src/message/printer_system_log/system.rs` | `EventSource` (`#[cfg(windows)]`) | `Send`, `Sync` | Windows event-log handle is read-only after registration; `ReportEventW` is thread-safe. | OS boundary (`#[cfg(windows)]`) |
| `src/ncollection/allocator/accumulating.rs` | `AccEngine` | `Send` | Exclusively owns block allocations; accessed only behind `Mutex`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/allocator/incremental.rs` | `IncEngine` | `Send` | Exclusively owns block payloads; accessed only behind `Mutex`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/allocator/owned.rs` | `AllocatedBytes` | `Send`, `Sync` | Exclusively owns plain bytes; allocator handle is `Send + Sync`; shared reads only. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/list.rs` | `List<T>` | `Send` (`T: Send`), `Sync` (`T: Sync`) | Base links are inert ownership metadata; shared access never mutates links. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/list.rs` | `ListCursor` | `Send`, `Sync` | Inert identity/revision metadata; dereferencing requires checked `List` borrow. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/list/iteration.rs` | `ListIter<'_, T>` | `Send` (`T: Sync`), `Sync` (`T: Sync`) | Shared borrow of `List`; yields `&T` only. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/list/iteration.rs` | `ListIterMut<'_, T>` | `Send` (`T: Send`), `Sync` (`T: Sync`) | Exclusive borrow of `List`; yields each `&mut T` once; `&ListIterMut` cannot yield `&mut T`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/list/owner.rs` | `ListNodeOwner<T>` | `Send` (`T: Send`), `Sync` (`T: Sync`) | Mediated by `List` exclusive/shared borrows, identical to `Box<T>`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/packed_map/iteration.rs` | `PackedMapIter<'_, T>` | `Send`, `Sync` | Shared read-only borrow of `PackedNodeTable`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/packed_map/storage.rs` | `PackedNodeTable<T>` | `Send`, `Sync` | Owned packed node table; mutation requires `&mut self`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sequence.rs` | `SequenceCursor` | `Send`, `Sync` | Inert node identity/revision; dereference requires checked `Sequence` borrow. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sequence/base_storage/node_liveness.rs` | `SequenceNodeLiveness` | `Send`, `Sync` | Synchronized liveness tracking for active sequence nodes. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sequence/iteration.rs` | `SequenceIter<'_, T>` | `Send` (`T: Sync`), `Sync` (`T: Sync`) | Shared borrow of `Sequence`; yields `&T` only. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sequence/iteration.rs` | `SequenceIterMut<'_, T>` | `Send` (`T: Send`), `Sync` (`T: Sync`) | Exclusive borrow of `Sequence`; yields each `&mut T` once. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sequence/lookup_cache.rs` | `ClearCachePair` | `Send` | Moved only with owning lookup cache. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sequence/nodes.rs` | `SeqNode` | `Send`, `Sync` | Inert link header owned by `Sequence`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sequence/owner.rs` | `SequenceNodeOwner<T>` | `Send` (`T: Send`), `Sync` (`T: Sync`) | Mediated by `Sequence` borrows, identical to `Box<T>`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sequence/owner_registry.rs` | `SequenceOwnerRegistry<T>` | `Send` (`T: Send`), `Sync` (`T: Sync`) | Owned by `Sequence`; shared access reads only. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sequence/topology.rs` | `SequenceTopology` | `Send`, `Sync` | Edits require `&mut self`; indexed reads use atomic cache; inspection writes serialized by `Mutex<()>`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sparse_array/iteration.rs` | `SparseEntriesMut<'_, T>` | `Send` (`T: Send`), `Sync` (`T: Sync`) | Exclusive borrow of `TypedSparseStorage`; disjoint mutable slots. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/sparse_array/storage.rs` | `TypedSparseStorage<T>` | `Send` (`T: Send`), `Sync` (`T: Sync`) | Exclusively owned sparse blocks; mutation requires `&mut self`. | Miri (`unsafe_send_sync_audit`) |
| `src/ncollection/ubtree/storage.rs` | `TreeArena<N>` | `Send` (`N: Send`), `Sync` (`N: Sync`) | Owned arena storage; shared access reads only. | Miri (`unsafe_send_sync_audit`) |
| `src/osd/file/native_unix/directory_cursor.rs` | `UnixDirectoryCursor` | `Send` | Owned `DIR*` handle; exclusive `&mut` iteration and single `closedir` on drop. | Unix OS boundary (`osd_files`) |
| `src/osd/parallel/job.rs` | `JobRef` | `Send` | Borrowed job pointer synchronized by scoped pool completion barrier. | TSan (`ToolkitBorrowedJobs`, `#110`) |
| `src/standard/c_locale_sentry/locale_posix.rs` | `SharedCLocale` | `Send`, `Sync` | Immutable process-lifetime POSIX `locale_t` handle. | POSIX OS boundary |
| `src/standard/memory/storage.rs` | `StandardByteStorage` | `Send`, `Sync` | Owned initialized bytes with no allocator thread affinity; mutation requires `&mut self`. | Miri (`unsafe_send_sync_audit`) |
| `src/standard/mmgr_opt/mapped_region.rs` | `MappedRegion` | `Send`, `Sync` | Owned `mmap` region; shared access reads bytes only; unmapped once on drop. | Unix OS boundary |

### `martensite-tkservice` (13 implementations)

| File | Type | Traits | Invariant | Verification |
| --- | --- | --- | --- | --- |
| `src/wayland_client/mod.rs` | `WlInterface`, `WlMessage` | `Sync` | Immutable process-lifetime Wayland interface descriptor tables. | Static C FFI table (`tkservice`) |
| `src/wayland_client/mod.rs` | `CoreInterfaces`, `WaylandApi` | `Send`, `Sync` | Process-lifetime read-only function pointers and symbols in `libwayland-client`. | Static C FFI table (`tkservice`) |
| `src/wayland_client/protocol.rs` | `XdgProtocol` | `Send`, `Sync` | Process-lifetime leaked immutable protocol table. | Static C FFI table (`tkservice`) |
| `src/wayland_client/xkb.rs` | `XkbSymbols` | `Send`, `Sync` | Process-lifetime read-only function pointers. | Static C FFI table (`tkservice`) |
| `src/wayland_client/xkb.rs` | `XkbKeymapState` | `Send` | Serialized behind `Mutex` in `WlWindow`. | Mutex-confined FFI (`tkservice`) |
| `src/wl/state.rs` | `WlCallbackTarget` | `Send`, `Sync` | Raw Wayland proxy pointers guarded by `Mutex<WlInner>`. | Mutex-confined FFI (`tkservice`) |
