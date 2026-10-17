# Scoped pool alternatives for #87

## Conclusion

None of the interfaces examined establishes a compatible, lower-cost replacement
for the current worker executor. This is an interface assessment, not an
impossibility proof or a benchmark. In particular, accepting borrowed jobs is
not enough: nested completion must make progress on the same reserved physical
workers without coupling unrelated reservations.

## `scoped_threadpool` 0.1.9

The pool persists its threads and accepts borrowed `FnOnce + Send` jobs.
Its scope requires `&mut Pool`, each submission boxes its closure, and scope
completion synchronizes with every pool worker. Pool destruction disconnects
the sender but does not join its stored thread handles.
[Versioned source](https://docs.rs/scoped_threadpool/0.1.9/src/scoped_threadpool/lib.rs.html#75-250).

Inference: a shared pool lacks selective worker dispatch; wrapping a one-thread
pool in a mutex would serialize scopes and deadlock recursive acquisition.
That direct adapter fails nested same-launcher completion. Destruction also
does not establish the required physical shutdown guarantee. Running the caller
inside the scheduling closure is possible, but does not resolve these failures.
Version 0.1.9 was published on 2018-02-20; age alone establishes neither
unsoundness nor active maintenance.
[Release metadata](https://docs.rs/crate/scoped_threadpool/0.1.9).

## `scoped-pool` 1.0.0

`Pool::scoped` takes `&self`; scopes have separate completion sets. The interface
documents explicit shutdown and warns against concurrent shutdown/scoping.
There is no worker-target selection method.
[Pool API](https://docs.rs/scoped-pool/1.0.0/scoped_pool/struct.Pool.html).
Jobs are boxed, scopes allocate wait groups, and joining blocks on a condition
variable rather than executing queued work.
[Version 1.0.0 source](https://docs.rs/scoped-pool/1.0.0/src/scoped_pool/lib.rs.html#275-460).

Inference: a shared pool cannot directly preserve reserved physical identities.
Separate one-thread pools restore routing, but a worker that submits an inner
job to its own pool and waits cannot run that queued job. Separate completion
counters therefore do not establish nested progress. An adapter changing that
scheduling behavior needs its own proof, not an assumption that `recurse`
solves it.

## `scoped-thread-pool` 1.0.4

This separately named package also exposes shared-reference scoping and explicit
shutdown without a worker-selection API.
[Versioned API](https://docs.rs/scoped-thread-pool/1.0.4/scoped_thread_pool/struct.Pool.html).
Its implementation uses boxed jobs, per-scope `Arc<WaitGroup>`, a shared channel,
and condition-variable joins. A worker panic triggers replacement of that
worker. Shutdown tracks worker completion, but does not retain/join OS thread
handles, so complete thread-exit/TLS teardown is not proved by that counter.
[Versioned source](https://docs.rs/scoped-thread-pool/1.0.4/src/scoped_thread_pool/lib.rs.html).

Inference: the same routing and nested-wait mismatches apply. Catching callback
panics before they reach the pool could preserve identity, but cannot repair
the scheduling mismatch. Lower retention, bounded-stack behavior, and overall
soundness have not been established here.

## Other boundary and next decision

`threadpool` 1.8.1 requires `Send + 'static` jobs, a concrete mismatch with
borrowed callbacks.
[Execution signature](https://docs.rs/threadpool/1.8.1/threadpool/struct.ThreadPool.html#method.execute).
For scoped interfaces above, a submitted closure borrowing `&F` can satisfy
`Send` when the existing `F: Sync`; moving `F` itself is unnecessary. That
observation avoids a new callback bound, but is not a complete adapter.

Do not add a dependency from this shortlist yet. A further candidate must first
demonstrate targeted, reentrant execution and actual thread joining; only then
measure allocations, retention, peak memory, and time against identical
workloads. Existing panic/failure isolation and 128-participant small-stack
fixtures remain acceptance requirements. Dependency trust requires a separate
maintenance/advisory and source review; safe API spelling is not certification.
No candidate was compiled or benchmarked for this note.
