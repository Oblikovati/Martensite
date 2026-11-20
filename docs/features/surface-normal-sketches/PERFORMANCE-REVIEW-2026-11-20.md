# Performance review of the attached cone text path, 2026-11-20

Reviewed tree: `feature/cone-chart-atlas` at `5161ffc6`, `main` at `28bbdd7b`.
Fixture: the zero-depth 239-curve V5 outward boss from
`crates/martensite-tkv3d/tests/wrapped_text_regions/conical_text/attachment.rs`.
Its decoded archive was saved once and the final `validate_body` call was timed
alone. Unless stated otherwise, times are from Linux x86_64 runs pinned to eight
physical cores, and every run produced the same validation report.

## Where the time goes

The complete call took 111.6 s in debug in this review. The matched V5 stage
table in CONE-REGION-ATLAS.md gives 18.7 s for the tool, 15.2 s for the Fuse and
65.3 s for the final validation of a 99.6 s call. The final check alone took
65.3 s in debug and 15.2 s in release.

One instrumented serial release run of the final check split as follows.

| Stage | Seconds |
| --- | ---: |
| Topology | 0.46 |
| Geometry (sampled) | 0.82 |
| Edge–face intersection | 5.82 |
| Face–face intersection | 7.17 |
| Pave blocks | 0.81 |
| Other self-intersection work | 0.98 |
| Total | 16.06 |

Edge–face and face–face tasks are 81% of the check, and about 70% of those tasks
pair two tool faces (CONE-REGION-ATLAS.md, candidate census).

## Findings

### 1. The parallel self-check of #503 never reached `main`

PR #503 was merged into its stacked base `fix/496-line-on-2s-storage` twelve
seconds after that base had merged into `main`. Its two commits are on no
integration branch. PR #508 later scoped the parallel path out without a
recorded decision. Before merging a stacked PR, retarget it to the integration
branch; after merging, check that the head is an ancestor of `main`.

### 2. Workers rebuilt every cached service

Each context-task worker built its own `IntTools` context. Eight workers on the
final check used 813 MB against 243 MB serial and 2.5 times the CPU. Sharing the
caller's context gave 267 MB with the same report. The fix, with an
operation-scoped worker bound, is on branch `perf/496-restore-parallel-self-check`
and its note `docs/features/self-intersection-search/shared-context.md`. With
four workers the check took 7.3 s in release and 32.0 s in debug.

### 3. A global worker cap slows other workloads

#503's global cap of eight made the parallel pave filler of
`bugs_modalg_5::bug24639` 39% slower on 32 logical CPUs. The earlier evidence for
that cap used `bfuse_complex`, which runs the serial one-step Boolean, so it never
exercised the cap. A worker bound belongs to the operation that needs it.

### 4. The feature's Fuse runs serially

`apply_material` builds the Boolean with the default options, so the Fuse is
serial. With parallel mode, the shared context and the parallel check, the
complete call produced an archive identical byte for byte to the serial one.
That run overlapped test suites, so its time is not usable, and its peak RSS was
465 MB against 318 MB serial. Measure the parallel Fuse alone on a quiet host
before enabling it; the memory tradeoff limit applies.

### 5. The debug profile leaves the geometry crates unoptimized

Only `martensite-tkmath` (opt-level 3) and `martensite-tkbo` (1) are optimized in
the dev profile. In the debug profile of the final check, 35% of self time was in
`core` iterator and option code that is not inlined, 9% in libc memory routines,
7% in `metallic` and 5.5% in `alloc`. Release was four times faster. The 5.4 s
gate is a debug wall time, so this profile choice decides part of that gate and of
CI test time. Rust does not reorder floating-point arithmetic at higher
optimization levels, so raising `opt-level` for the geometry crates and
`metallic` should leave results unchanged; measure compile time and test time,
and confirm identical archives, before proposing it.

### 6. The intersection code allocates very often

With `MALLOC_ARENA_MAX=1`, 32 workers spent 316 s of system time in allocator
locks during a 22 s check. In the serial debug profile, dropping and freeing took
about 10%. Fewer allocations in the intersection kernels would help serial and
parallel runs alike.

### 7. The member work allowance is sequential

`operation/cone/storage/work.rs` threads one decreasing allowance through every
member, so members are proved one after another. Per-member grants fixed before
the members start would allow parallel proofs, but they change the replay
identity and need a new proof version.

### 8. Arithmetic and fitting waste

The briefing of 2026-11-19 records reference-volume running sums of about 7,100
bits from inputs of about 220 bits, and 136 refused parent fits that used 37% of
the fit work. Accumulating over a common dyadic denominator, and choosing the fit
level before fitting from a derivative bound, would remove most of both.

### 9. A hidden per-call clamp

`Work::remaining()` caps every call at 1,000,000 operations whatever the
request. A proof that needs more in one call refuses while the request still has
allowance, and reports do not show the clamp. Report it as an effective setting
or remove it.

### 10. Measurement hygiene

Several runs in this review overlapped Cargo test suites; one Boolean comparison
showed a false 2.3 times slowdown. Check the host load before timing, and keep
builds and suites off the measured cores.

## Next step for the feature

Transfer certified tool–tool coverage into the final check. The tool proofs
already certify those pairs. The transfer needs exact identity of the decoded
carriers and filled trims, the attachment seam and any split images, and native
checks for every pair outside the certificate. On this fixture it removes up to
70% of the edge–face and face–face tasks.
