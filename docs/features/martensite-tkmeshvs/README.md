# Mesh visualization

`martensite-tkmeshvs` shows a mesh that an application lends through a data source in the 3D viewer of
`martensite-tkv3d`, and selects its nodes and elements.

| Module | Capabilities |
| --- | --- |
| `data_source`, `data_source_3d`, `deformed_data_source` | The abstract mesh, its volume topologies, and another mesh with every node moved. |
| `mesh` | The interactive object: presentation builders, hidden and selectable entities, highlight and selection. |
| `mesh_prs_builder`, `elemental_color_prs_builder`, `nodal_color_prs_builder`, `text_prs_builder`, `vector_prs_builder` | The builders: nodes, links, faces and volumes, colors by element and by node, texts and vectors. |
| `mesh_owner`, `mesh_entity_owner`, `sensitive_*`, `common_sensitive_entity` | The owners and sensitive entities of selection. |
| `drawer`, `tool` | Typed appearance values and the aspects built from them. |
| `flags`, `selection_policy`, `volume_topology`, `node_pairs`, `two_colors`, `buffer` | Display and selection masks, prism and pyramid connectivity, undirected links, color pairs and a scratch buffer. |

## Ownership and thread safety

A mesh object holds its data source and builders by handle and is used from one thread. A builder keeps
its parent mesh weakly, so it never keeps the mesh alive.

## Verification

Run `cargo test -p martensite-tkmeshvs` with light optimization, `CARGO_PROFILE_DEV_OPT_LEVEL=1`. The
`toolkit` target has the value, drawer, builder and selection tests with their recorded transcripts. The
`mesh_view_cases` target runs on Linux, each case alone in a child process on the
[renderer test setup](../martensite-vulkan/README.md), and ends with a clean validation report. Set
`MARTENSITE_MESH_VIEW_DIR` to keep the images as PNG files.
