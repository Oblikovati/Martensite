# Boolean regression cases

`martensite-boolean-cases` holds two things:

- A library with the check contract of the Boolean regression scripts: the inherited group hooks,
  the commands the scripts call, a session that draws on a memory raster without a display, and
  whole-stream grading. `martensite-tkbool`, `martensite-tkfillet` and `martensite-vulkan` list
  their own cases on it.
- `tests/dataset_cases`: the Boolean scripts themselves, one test per script.

## Why a separate crate

The contract calls the Boolean operations, so it depends on `martensite-tkbo`. A
development dependency from `martensite-tkbo` back to this crate would be a cycle, and `cargo
package` cannot resolve such a cycle between unreleased crates. So the cases that use the contract
live here rather than under `martensite-tkbo/tests`.

## How a case runs

Each listed case runs in its own child process under its group's CPU budget. The parent reads
everything the child wrote and grades it with the group's rules. A family is wired in two files:
`cases/<grid>.rs` holds one body per script and `<grid>.rs` lists the bodies. See the module
documentation of `support`.

Markers, all checked by the parent:

| Macro | Meaning |
| --- | --- |
| `dataset_cases!` | The stream must earn a passing verdict. |
| `optimized_dataset_cases!` | The same, with no other case of the executable running, for a script whose CPU budget or time limit assumes an optimized Linux build on an idle machine. Any other build reports it ignored with the reason; `build.rs` reads the opt-level and target. CI builds these tests with `CARGO_PROFILE_DEV_OPT_LEVEL=1`. |
| `unstable_dataset_cases!` | Left out of the default run, because the verdict changes from run to run on the reference build too or the limit is a recorded exception. Build with `RUSTFLAGS='--cfg martensite_unstable_cases'` to run them. |
| `native_improvement_case!` | The declared problem does not appear on the reference run either, and the test requires exactly that. |
| `rust_improvement_case!` | The library does better than the reference run by a recorded decision, and the test requires exactly that. |

## Verification

Run `cargo test -p martensite-boolean-cases`. Inputs come from `external/martensite-test-data`,
so run `git submodule update --init` first.
