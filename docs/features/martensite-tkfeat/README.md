# Local operations and form features

`martensite-tkfeat` adds and removes material on a shape: a prism, a revolution or a pipe of a profile
on a face, up to a limit, and the holes, ribs and slots built on them.

| Module | Capabilities |
| --- | --- |
| `brep_feat` | The form features: prisms, drafted prisms, revolutions, pipes, cylindrical holes, linear and revolution forms, glued shapes and face splits. |
| `loc_ope` | The local operations under them: sweeps, gluing, splitting a shape along wires, and the intersection and location queries. |

## Ownership and thread safety

The builders take their shapes by handle and build a new shape. A builder is used from one thread.

## Verification

Run `cargo test -p martensite-tkfeat` with `CARGO_PROFILE_DEV_OPT_LEVEL=1`, because the CPU budgets of the
script cases assume light optimization.

`tests/dataset_cases` holds the form feature script cases. Each runs alone in a child process and is graded
on its whole output. Five of them end on a shaded view, so they run on Linux only and need the
[renderer test setup](../martensite-vulkan/README.md).
