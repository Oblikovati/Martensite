# Meshing factory

`martensite-tkxmesh` registers a meshing factory named `XBRepMesh` in the registry of
`martensite-tkmesh`. The factory makes an incremental mesher that has not been performed.

Nothing runs when the library loads, so an application registers the factory by calling
`XBRepMeshFactory::new`. The registry's default stays first, because this factory does not ask to be
preferred.

## Verification

Run `cargo test -p martensite-tkxmesh`.
