# Topological operations

`martensite-tkbool` holds the topological operation family that sweeps, evolved shapes and older
Boolean paths are built on.

| Module | Capabilities |
| --- | --- |
| `top_ope_brep` | Face, edge and point intersection records and fillers. |
| `top_ope_brep_ds` | The data structure of a run: interferences, curves, points and their iterators. |
| `top_ope_brep_build` | The builders, loops, areas and solid and shell construction. |
| `top_ope_brep_tool` | Curve, surface and classification tools. |
| `brep_algo` | Image bookkeeping, loops, face restriction and the section entry points. |
| `brep_fill` | Sweeps, pipes, filling, draft, evolved shapes and offset wires. |
| `brep_proj` | Projection of wires onto shapes. |

## Ownership and thread safety

The operations build new shapes. The unbound image accessor of
`brep_algo` appends to a process-wide list; the test that observes it has a dedicated executable
(`brep_algo_image_fallback`).

## Verification

Run `cargo test -p martensite-tkbool`. `tests/fixtures/full_numeric` holds frozen requests and
answers, replayed through `tests/replay_operations`. They are recorded baselines. A change to one
needs a reviewed update that states its reason and behaviour version. The script cases in
`tests/dataset_cases` use the contract of
[`martensite-boolean-cases`](../martensite-boolean-cases/README.md). The numeric baseline changes
from correctly rounded math are in [`corpus-attribution`](corpus-attribution/README.md).
