# Group storage migration (#324)

`Graphic3dCStructureFields::groups` uses `Sequence<Handle<dyn Graphic3dGroup>>`. Each entry has a stable node identity, so two entries can retain the same object without becoming indistinguishable during callback mutation. This changes a public writable field type and requires source changes in custom drivers.

The `Graphic3dCStructureMembers::groups()` and `Graphic3dStructureMembers::groups()` methods still return ordered `Vec` snapshots. Snapshot edits remain independent of the live collection. Each snapshot entry retains its payload, including repeated handles.

For direct field access, replace vector `push(group)` with sequence `append(group)`. Initialize the field with `Sequence::default()`. Sequence `value(1)` reads the first entry; `at(0)` is the explicit zero-based accessor. Both return checked results. Sequence `remove(1)` removes the first entry, while `clear(None)` keeps the sequence's allocator. Collecting values into `Sequence<_>` preserves their order and duplicates.

Use a detached `SequenceNativeCursor` when virtual callbacks can change the live collection. Resolve the current value under a short field guard and take a `HandleObserver`. Release that guard before the callback, then advance the same node cursor against the actual collection. The cursor identifies the entry independently of the group object's address. Replacing the collection with a new snapshot invalidates those node identities.

Removal and clearing must preserve their callback and destructor order. The sequence's phased retirement APIs can detach a selected node under the field guard and release it afterward. A removed current node produces a typed invalidation error when advancement is attempted; it must not be treated as successful end-of-iteration.

The first structural checkpoint changes storage and index spelling while keeping the existing null driver's first-match removal algorithm. Its five owned snapshot/owner tests have the same assertions as the preceding vector checkpoint. The separate #325 callback checkpoint captures the selected node before virtual clear and detaches that same node afterward, releasing retirement outside the field guard. These checkpoints are source-only until their execution gates pass.

This is a breaking package API change. Versions and release tags remain unchanged until explicitly approved.
