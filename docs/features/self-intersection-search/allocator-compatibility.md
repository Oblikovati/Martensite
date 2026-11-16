# Walking-line allocator compatibility (#496)

`IntSurfLineOn2S::new(None)` uses contiguous points. An explicit allocator keeps
the original `Sequence` storage, including allocator object ownership, virtual
policy dispatch, node release, reset exclusion and split-tail handling. Box
rebuilds dispatch once per traversal and use slice iteration on the default path.
The public methods and index errors are unchanged.

The original PR #502 head, `28292f63fb16ed25aae45fabab01cb403d36119d`, discarded
an explicit allocator. A live probe observed no allocation in a supplied
incremental arena and allowed its reset while a line point remained alive.
The original allocator-backed sequence allocated one arena block and refused
that reset with `Standard_ProgramError`.

Five regression controls cover allocator handle ownership, incremental allocation
and reset exclusion, default split tails, accumulating allocation/release counts,
and edit/order/box parity on both storage paths. Three ownership/allocation
controls failed against the unpatched PR's existing release artifacts. The edit
parity control passed there. The accumulating control was added afterward.

A disposable Linux x86_64 release microbenchmark compared the unpatched PR's
existing release library with the corrected line module compiled at optimization
level 3 against the same dependencies. Each sample constructed 20 default lines,
appended 20,000 points per line, then read every point by rank. After one warm-up,
nine pairs alternated execution order. Median time was 8,570 microseconds for
the unpatched vector and 8,063 microseconds for the compatibility implementation.
The source and binaries were removed after measurement. These shared-machine
measurements cover point storage only; they do not qualify whole-body time or
peak memory. The conical body-check benchmark remains an integration gate.

Focused verification uses:

```sh
cargo test --locked -p martensite-tkgeomalgo --test toolkit int_surf_behavior
cargo test --locked -p martensite-tkgeomalgo --test toolkit
cargo clippy --locked -p martensite-tkgeomalgo --lib --test toolkit -- -D warnings
```
