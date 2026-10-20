# Boundary representation

`martensite-tkbrep` provides the boundary representation data structure: topological
shapes, their exploration, the geometric shapes with their builder and queries, and the
graph form of a shape (#208).

| Module | Capabilities |
| --- | --- |
| `topods` | Shapes, their orientation and location, and the builder of compounds, solids, shells, faces, wires, edges and vertices. |
| `topexp` | Exploration of sub-shapes by type, and vertex and edge queries. |
| `toptools` | Shape map hashers, location sets and the text shape set. |
| `brep` | Geometric shapes: curves and surfaces on edges and faces, polygons, triangulations, tolerances, the builder and `BRepTool` queries. |
| `brep_tools` | Shape tools: modifiers, reshaping, history, the NURBS conversion, wire exploration, mesh queries and the text shape stream. |
| `brep_adaptor` | Curve and surface adaptors of edges and faces, composite curves. |
| `brep_lprop` | Local continuity of two curves at a vertex. |
| `brep_graph`, `brep_graph_inc` | The graph form of a shape: typed node and reference ids, layers, history, editing, copying, compaction, deduplication and transformation. |
| `bintools` | The binary shape stream and its format versions. |

```rust
use martensite_tkbrep::topods::Shape;

let shape = Shape::default();
assert!(shape.is_null());
```

## Ownership and thread safety

Shapes share their underlying topology and geometry through handles. Cloning a shape
or a handle keeps the shared identity; copy shapes with the copy and modification
tools when later edits must be isolated. The builder mutates shared topology in
place. These low-level types make no immutable-publication promise, and the queries
do not validate a modelling result: an enclosing operation attaches validation,
history and tolerance policy.

The node and reference ids of the shape graph are graph-local; they are not
persistent entity identities.

## Verification

Run `cargo test -p martensite-tkbrep`. The ordinary tests cover the graph ids,
layers, batch operations and sparse models, the transformation modifiers, curve and
surface copies and their parameter contracts, the dump callbacks and the binary
stream.

The `dataset_cases` target runs 88 regression cases that restore their inputs from
the public test dataset (release 7.9.0) in the `external/martensite-test-data`
submodule, or from `$MARTENSITE_TEST_DATA_DIR`. Each member is pinned by size and
SHA-256; a missing or changed member fails the case with the command that fetches the
submodule:

```sh
git submodule update --init external/martensite-test-data
```

Each case runs alone in a fresh child process of its test executable, under the
elapsed-time budget of its group. View steps of the case scripts are skipped; every
kernel check still runs.
