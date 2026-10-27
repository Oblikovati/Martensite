# Stable group-node verification (#324)

Each group collection entry needs its own identity, even when several entries retain the same group object. A callback that removes an earlier entry must leave the current node and its successor distinguishable. The storage change uses the existing foundation sequence and detached cursor.

The first checkpoint adds owned tests for the current public contract: ordered snapshots with duplicate payloads, exact intrusive ownership, independent snapshot edits, first-match driver removal, clear retention and copied fields. These tests are source-only until a build lease is granted. Their unchanged assertions must pass on the current storage and after the structural conversion.

The storage conversion is a separate commit. `Graphic3dCStructureFields::groups` changes from a vector to a sequence. Existing `groups()` snapshot methods keep their return types and ownership behavior. Check custom-driver construction, clone behavior, removal and clear ordering. Values retired by virtual callbacks must be released outside the enclosing fields guard.

The selected-node correction is tracked by #325. Its regression clears B while B removes earlier A; the operation must remove B and retain C. The test-source checkpoint precedes the correction so its actual RED can be retained before the corrected run. Neither run has executed yet.

The stable-node callback cases cover: `[A, B, B, C]`, removal of earlier A during the first B callback, removal of the next node and append during traversal. Compare exact visit order and owner counts in two independent processes. Actual cursor node IDs must distinguish the two B entries. Retain typed invalidation errors for a removed current node; a missing node must not look like successful exhaustion.

Reuse the existing driver-model, shadow, bounds, dump, group and retirement suites. A focused package run is the first execution gate; `cargo xtask pr` remains the required merge gate for all changed and affected packages, together with the compiler API gate. Release-platform checks are separate. No build, test, performance or thread-safety result has been recorded at this source checkpoint.

Measure time, peak memory, allocations and locks on the same accepted workloads before and after the change. Ordinary cursor traversal must avoid a collection scan or snapshot per callback. A release decision also needs the applicable concurrency and sanitizer gates. Keep unexecuted gates open.

The clear policy and its destructor/panic tests are specified separately in [clear-retirement.md](clear-retirement.md). These add a behavior decision beyond the storage conversion and selected-node correction.

Changing the public writable field type breaks callers that use vector construction, zero-based indexing or vector-specific mutation. The feature needs migration examples and changelog notes before integration. Package versions and release tags remain unchanged pending explicit approval.
