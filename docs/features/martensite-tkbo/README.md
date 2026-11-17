# Boolean operations

`martensite-tkbo` computes Boolean operations between shapes and the algorithms built on the same
intersection pipeline: general fuse, section, cells, volume making, splitting, connection,
periodicity and defeaturing.

| Module | Capabilities |
| --- | --- |
| `int_tools` | Edge, face and curve intersection tools, classifiers and the shared context. |
| `bop_ds` | The data structure of a run: shapes, pave blocks, common blocks, interferences and iterators. |
| `bop_tools` | Topology and geometry helpers of the builders. |
| `bop_algo` | The pave filler, builders, the self-interference checker, cells, volume maker, section, connection, periodicity and defeaturing. Each has progress reporting and cancellation. |
| `brep_algo_api` | The operation-level entry points: fuse, common, cut, section, splitter, defeaturing and check. |
| `body_check` | The ordered check of a result, with its stage reports. |
| `profile_extrusion`, `measured` | Region extrusion from planar profiles, with error-bounded area and volume measures. |

## Validation admission

`body_check::validate_body` admits raw stored vertex, edge and face tolerances before
running native checkers. A NaN, infinity or negative value produces an exact failed
tolerance report; topology, geometry and self-intersection remain `NotRun`. Admission
uses the existing indexed shape records and leaves the input unchanged. Valid values
retain the effective `precision::confusion()` floor and the requested cap policy.

This admission precedes the ordered native stages because self-intersection checking
can overwrite an invalid stored vertex tolerance with its effective minimum (#490).
See [the diagnosis and controls](490-raw-tolerance-admission.md).

## Ownership and thread safety

Operations read their arguments and build new shapes. A shape's unchanged edges are shared with
the result, so a new p-curve on a shared edge changes caller-visible state: copy the input with the
copy builder when it must not change.

## Unsafe code

The crate has no unsafe code. It follows the workspace `forbid`.

## Verification

Run `cargo test -p martensite-tkbo`. Unit tests in `src` replay frozen progress and cancellation
sequences under `src/**/*_fixture` and `tests/fixtures`. Those are recorded baselines, and they
make up most of this crate's size. A change to one needs a reviewed update that states its reason
and behaviour version. The [history oracle version 2 correction](../boolean-solid-history/456-assembly-lineage.md#history-oracle-version-2)
retains final Fuse solid ancestry; geometry, numerical and progress/event oracles keep their existing versions (#456).

The Boolean script cases are in [`martensite-boolean-cases`](../martensite-boolean-cases/README.md),
because the check contract they use depends on this crate.

The workspace builds this crate at `opt-level = 1` in the development profile, because a few script
cases have CPU budgets that assume optimization. Debug assertions stay on.

No recorded baseline of this crate moved on correctly rounded math; see
[the attribution](../martensite-tkbool/corpus-attribution/README.md).
