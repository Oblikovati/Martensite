# Stable nodes for List and Sequence

Option A keeps one address-stable allocation per item. The list or sequence owns its chain. An explicitly allocated node carries its lease, so moving the chain transfers ownership without rebuilding a table. Default lists allocate with `Box`. Default sequences use the existing Standard raw heap layout because `SequenceNodeRetirement::retire_using` gives the selected compatible allocator the original raw address. [#586](https://github.com/Oblikovati/Martensite/issues/586)

A detached cursor can reuse its pointers when they belong to the same chain revision. Across an edit that may invalidate its position, it resolves the address from the owning chain and uses the provenance found there. Same-resource splits and splices leave surviving node addresses intact. The usual borrowed iterators follow the links directly.

A sequence's lookup cache publishes a pointer to a live node. The node contains its index, which stays fixed during shared reads. An exclusive edit imports the current cache pair before changing the base and publishes the resulting pair afterward. Explicit `base_access()` guards keep the borrowed base and its allocator identity and serialize inspection of the raw cache fields. Indexed reads use the separate cache and structural fields. Detached observers hold synchronized copies of the metadata and can read it during destruction.

Controlled sequence clear (#77) leaves historical endpoints and size visible while nodes are destroyed. This path builds a temporary liveness index. If an append reuses a retired address, the index resolves the new allocation's pointer. The mutable iterator tracks yielded addresses during this phase so a reused address cannot produce two exclusive references. Once the clear traversal retires a node, splitting or splicing the historical topology returns a typed failure before sharing ownership with another chain. Ordinary destruction follows the owned links and advances its cleanup guard before calling each destructor.

## Paired measurements

`cargo xtask occt-parity` compared `ebd221a1` with OCCT 8.0 on Linux x86_64, CPU 23. Each row is in nanoseconds per operation, from two rounds of seven runs with one million items or edit batches per run. Every checksum matched. The configured 5% tolerance passed one list case and zero sequence cases.

| Case | OCCT | Martensite |
| --- | ---: | ---: |
| List append and iterate | 10.448 | 9.648 |
| List append, explicit common resource | 8.787 | 466.779 |
| List cursor walk | 1.224 | 57.747 |
| List edit batch | 46.274 | 110.494 |
| Sequence append | 8.787 | 17.292 |
| Sequence append, explicit common resource | 8.642 | 451.223 |
| Sequence cursor walk | 1.218 | 41.947 |
| Sequence edit batch | 61.132 | 314.536 |
| Sequence iterate | 1.212 | 3.438 |
| Sequence indexed value | 3.335 | 15.642 |

The full command runs several cases in one process. The explicit common cases change the heap layout before the later cases. In separate processes on the same CPU, the exact candidate's list cursor walk took 1.266 ns against OCCT's 1.247 ns. Sequence cursor walk took 7.447 ns against 1.212 ns, and indexed value took 8.507 ns against 3.287 ns. The isolated list cursor process used 36,652 KiB peak RSS against OCCT's 37,148 KiB. The isolated sequence cursor process used 67,852 KiB against 37,112 KiB.

Two isolated diagnostic rounds during implementation used `5fb67dbc` as the Rust control. Default list append and iteration fell from 695.45 to 9.81 ns. Default sequence append fell from 712.23 to 15.89 ns, borrowed iteration from 83.33 to 3.58 ns, and indexed value from 15.44 to 6.76 ns. Those Rust measurements predate the final cross-owner clear guard and the shared serial issuer.

The collection parity bench also checks two edit batches. The sequence batch appends, prepends, inserts, exchanges, reverses, splits, rejoins and removes a range while holding the steady-state size at four. The list batch exercises append, prepend, a positioned insert, removal through that cursor, reverse and head removal. Both benches consume the resulting endpoints. Their checksums match OCCT.

## Remaining parity work

The explicit common-allocator cases still take the virtual typed allocation path. The parked allocator change from #584 was measured as a temporary control on top of these changes. In two isolated rounds, it reduced explicit common list append from 482.23 to 20.33 ns and sequence append from 436.89 to 20.37 ns. The OCCT controls were 8.94 ns and 8.64 ns. The candidate branch retains its original allocator implementation.

Default sequence nodes also carry the raw heap allocation header and cache index. The one-node-per-item design still trails OCCT on the measured `Sequence` cases. Measure the proposed index-linked arena from #586 on the cone-text attachment and complex Boolean workloads once those toolkits build on the feature branch. That comparison also needs to account for the stable-address adapters that `tkservice` and `tkv3d` currently use.


## Verification

All 5,306 `tkernel` tests passed with eight existing skips, and all-target Clippy passed with warnings denied. Miri covered controlled clear, inspection during a concurrent read, addressed node retirement and mixed-allocator splicing. The ThreadSanitizer startup fixture exited with SIGSEGV before running its controls.
