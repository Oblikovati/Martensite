# Memory statistics boundary research

Feature #80 / task #89, based on
`b49a56f2302fe5a800d1c6d2034017491d8c8928` and
`crates/martensite-tkernel/src/osd/process/mem_info/native.rs`.
No production changes, dependencies or runtime experiments accompany this
note. The unavailable humanizer prose pass remains for review.

## Preserve the metric, not just its integer type

GNU Linux returns `libc::mallinfo2().uordblks` directly as usize; other targets
return zero from this particular heap query. Apple task counters return
`Some((virtual_size, resident_size, stats.size_in_use))` only after successful
TASK_BASIC_INFO. On failure they return None without calling the allocator
statistics function. The successful path ignores the returned Mach count and
calls `malloc_zone_statistics(NULL, ...)` once with an explicitly zeroed
four-field record. These are the contracts in the
[baseline source](https://github.com/Oblikovati/Martensite/blob/b49a56f2302fe5a800d1c6d2034017491d8c8928/crates/martensite-tkernel/src/osd/process/mem_info/native.rs).

RSS, virtual address space, allocator accounting and live Rust object sizes
are not interchangeable. Switching to process RSS or counting Vec capacities
would change the reported metric rather than remove unsafe compatibly.

## GNU allocator accounting

The GNU manual describes mallinfo2 as allocator statistics with size_t fields;
it is not a query of arbitrary replacement allocators or Rust object liveness.
See [GNU allocator statistics](https://sourceware.org/glibc/manual/latest/html_node/Statistics-of-Malloc.html).
For concrete implementation evidence, glibc **2.39** initializes allocator
state if necessary, iterates arenas with per-arena locking and accumulates
`uordblks += av->system_mem - avail`. Mapped-allocation accounting is held
separately in hblkhd. Therefore do not replace this field with an RSS total,
add hblkhd, or assume this version only visits the main arena. The source is
more precise than stale general descriptions of mallinfo behavior. See
[glibc-2.39 malloc.c, int_mallinfo and __libc_mallinfo2](https://github.com/bminor/glibc/blob/glibc-2.39/malloc/malloc.c#L5261-L5346).

The original ignores errno and returns the field without a failure sentinel.
Do not add error translation, save/restore errno, allocator trimming, XML
malloc_info parsing or allocation instrumentation as a supposedly neutral
replacement. A wrapper's own allocations could perturb the measurement.
Allocator initialization/locks mean the call is not literally free of process
state effects. This source inspection does not establish an errno guarantee
for every deployed glibc or interposed allocator.

## Apple task flavor, layout and failure

Pinned mach2 **0.7.0** defines TASK_BASIC_INFO as 5 on x86_64 and 18 on aarch64.
Its task_basic_info is repr(C, packed(4)), with virtual/resident vm_size_t fields;
the count is structure size divided by natural_t size. This is not the
MACH_TASK_BASIC_INFO flavor 20 and its different record. See
[mach2 0.7.0 task_info source](https://docs.rs/mach2/0.7.0/src/mach2/task_info.rs.html).
The current code supplies that exact initial count, zero-default record and
current-task port; it checks the Mach return code, not errno, and ignores
the updated count. A stricter returned-count check changes the wrapper's
behavior for synthetic short-success responses and requires a decision.

Apple XNU revision **f6217f891ac0bb64f3d375211650a4c1ff8ca1ea** has separate
TASK_BASIC_INFO_64, ARM TASK_BASIC_INFO_64_2 and MACH_TASK_BASIC_INFO branches.
They validate minimum input count, populate flavor-specific layouts, and
write output count. In this revision the relevant successful branches derive
virtual size from vm_map_adjusted_size and resident size from the phys_mem
ledger; MACH_TASK_BASIC_INFO additionally reads peak resident information.
That similarity is evidence to investigate, not proof that changing flavor
preserves every supported kernel's errors, count, layout or partial outputs.
See [XNU task_info implementation](https://github.com/apple-oss-distributions/xnu/blob/f6217f891ac0bb64f3d375211650a4c1ff8ca1ea/osfmk/kern/task.c#L5456-L5530).
No native kernel-version equivalence or failure injection was run here.

## Apple allocator zones

Apple libmalloc revision **c49dafa25f1efe8607701ae6014a663ad2ee437f** explicitly
handles NULL by zeroing the result, visiting registered zones, invoking each
zone's statistics callback and summing four fields. A non-null argument invokes
only that zone's callback. Thus default-zone statistics alone are not the
existing all-zone query. The returned size_in_use is allocator-zone accounting,
not task resident memory. See
[malloc_zone_statistics implementation](https://github.com/apple-oss-distributions/libmalloc/blob/c49dafa25f1efe8607701ae6014a663ad2ee437f/src/malloc.c#L3576-L3593).
The same source's [mstats function](https://github.com/apple-oss-distributions/libmalloc/blob/c49dafa25f1efe8607701ae6014a663ad2ee437f/src/malloc.c#L3757-L3770) derives bytes_used from all-zone size_in_use,
but invoking another unsafe C entry point would not remove first-party unsafe.
This inspection does not guarantee atomic snapshots across zones or errno
preservation by arbitrary registered statistics callbacks.

## Safe-interface search and concrete limits

Existing dependencies were checked first: libc **0.2.190** and mach2 **0.7.0**
expose the raw calls; searches of nix **0.31.3** and rustix **1.1.5** source
found no mallinfo, malloc_zone_statistics or task_info wrapper. Standard
allocation interfaces do not provide these particular allocator/task counters;
using a different allocator or object tracker would change scope. This is a
bounded capability search, not an ecosystem impossibility claim.

The established `memory-stats` candidate was inspected at upstream revision
**431e74654c27ec38a6f7d19c186bec42c2a938b8**. Its safe Darwin query uses
MACH_TASK_BASIC_INFO, an uninitialized output record, the corresponding count,
and None on non-success. It exposes resident/virtual values but no all-zone
allocator total. Its Linux implementation reads smaps/statm, not mallinfo2.
Consequently it is not a drop-in replacement for either complete query here.
Sources: [Darwin implementation](https://github.com/Arc-blroth/memory-stats/blob/431e74654c27ec38a6f7d19c186bec42c2a938b8/src/darwin.rs)
and [Linux implementation](https://github.com/Arc-blroth/memory-stats/blob/431e74654c27ec38a6f7d19c186bec42c2a938b8/src/linux.rs).
It is a candidate for further task-field equivalence research only, not an
approved dependency addition or replacement. No evaluated safe interface
preserves the GNU field and Apple all-zone query exactly. More established
wrappers may exist; no permanent exception follows from this search.

## Next evidence and decisions

1. Pin existing pure behavior with named fake queries: exact selected fields,
   usize boundaries, failure returning None before the allocator call, success
   dispatch order, and the current policy of ignoring returned count. Retain
   the existing four-field Apple initialization test. Do not add unsafe fake
   callbacks merely to obtain coverage.
2. Any task wrapper candidate must first demonstrate matching flavor/layout,
   initial count, current-task identity and failure policy on each supported
   Apple architecture. Compare successful fields without expecting separate
   live samples to be equal. A fixture with fixed counters proves mapping,
   not native equivalence across kernel versions.
3. For allocator candidates, capture immediate errno observations around
   original/candidate calls with saved/restored named sentinels and no logging
   allocations in the observation interval. Use isolated processes and record
   allocator/runtime versions. Do not assert an allocation increases these
   counters by exactly its requested bytes or that freed memory returns to RSS.
4. Continue a focused search for established safe mallinfo2 and all-zone
   wrappers. If only different metrics/flavors are available, request an
   explicit behavior-policy decision rather than silently relabeling them.
   These private boundaries do not currently demonstrate a public API that
   inherently requires unsafe; no #88 API conflict is proved by this note.

Keep #80 open. Custom unsafe moved into another crate is not removal, and
the absence of a matching wrapper in this bounded search is not proof that
safe implementation is impossible.
