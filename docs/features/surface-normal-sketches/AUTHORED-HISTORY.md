# Authored history for rational families

## Contract

The private family result retains one history record for every semantic key in
its frozen encoding (#418). Records use the existing surface-sketch authored
source IDs and geometric roles. They add an explicit owner, relation and sorted
set of parents. The planar extrusion's `OperationHistory` uses one source and
indexed result entities; it cannot express these multi-parent records without
losing their authored span/corner identity. Existing public history APIs remain
unchanged.

Region solids and depth caps are derived from their authored region and every
boundary curve and vertex, including holes. A span's depth edges are split
images when its authored curve was subdivided, and derived images otherwise.
Its swept side face is derived from that span. Span records include the region,
curve and any authored vertices at that span's endpoints.

An authored corner's depth vertices and normal edge are derived from the region,
that vertex and both incident curves. A subdivision corner's corresponding
entities are created from the region and its curve. Both incident pieces
contribute parents before a single shared-corner record is emitted. Extrusion
removes no input source, and each authored source has images; no deletion or
merge is inferred.

## Identity and binding

The history gate rechecks the original planar snapshots and global exact
parameter partition. Every prepared endpoint must have the identity determined
by its authored curve and exact dyadic parameter. Interior dyadic parameters
are reduced to their canonical numerator and level. Equivalent coordinates or
an equivalent but noncanonical dyadic label cannot authorize a new identity.
Authored curve endpoints retain their original vertex IDs.

Every record target must occur exactly once in the frozen key list. Missing,
extra and duplicate targets refuse with typed diagnostics. The preceding native
coverage gate binds those keys to the actual decoded shapes. Neither native
traversal order, coordinate proximity nor proof-work receipts provide identity.
Records and their parent sets have total ordering. The evidence owns values
only, with a compile-time `Send` and `Sync` control.

## Evidence and limits

Seven focused controls cover both incident parents, split-corner creation,
complete region provenance, missing/extra/duplicate targets, changed original
geometry, relabelled prepared corners, noncanonical dyadic aliases and history
ordering. Existing family tests also compare histories after reversing selection
order and require the history census to equal the frozen key census.

The affected sketch package passed 124 library and 32 integration tests on Linux
x86_64, taking 15.40 and 1.99 seconds excluding compilation. Strict all-target
clippy passed after removing a needless lifetime in a test helper. The focused
controls were rerun after that cleanup. The approximate scan found no new
history function over 20 lines, file over 500 lines or nesting above two.

This establishes private extrusion history. Copy, Boolean and healing history
composition, application naming, separate-process replay, concurrent-operation
checks and publication remain separate gates. No public API, dependency, package
version or release tag changed. The full affected-package PR gate remains
required before a push.
