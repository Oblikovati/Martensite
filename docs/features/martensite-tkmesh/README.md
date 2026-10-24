# Mesh construction

`martensite-tkmesh` builds and inspects discrete meshes for boundary shapes (#216).

## Verification

Run `cargo test -p martensite-tkmesh`. The suite covers mesh elements, edge discretization, triangulation, incremental meshing, model construction and dataset-backed regression cases.

The trimmed-torus assembly regression remains unregistered because its Boolean
construction dependency is unavailable in this release (#233). It does not count
as executed coverage. Running the registered suite does not establish complete
mesh regression coverage; the original construction and assertions must be
restored when that dependency is available.
