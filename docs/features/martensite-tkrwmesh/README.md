# Mesh exchange tools

`martensite-tkrwmesh` holds what the mesh file formats share, so that OBJ and PLY read and write
documents the same way.

| Item | Capabilities |
| --- | --- |
| `RwMeshCoordinateSystemConverter`, `RwMeshCoordinateSystem` | Axis and unit conversion of positions and normals between a document and a file. |
| `RwMeshCafReader`, `RwMeshTriangulationReader`, `RwMeshTriangulationSource` | Filling an exchange document from a mesh reader, with deferred triangulation data. |
| `RwMeshFaceIterator`, `RwMeshEdgeIterator`, `RwMeshVertexIterator` | The triangulated faces, polygons and vertices of a shape for a writer, with their styles. |
| `RwMeshMaterialMap` | Unique material names and the texture files of a written document. |
| `RwMesh`, `RwMeshNameFormat` | Node names of a document. |
| `split_text_lines` | The lines of a comment for a writer. |
| `single_precision_text`, `double_precision_text` | Numbers as text with 9 and 17 significant digits, so each reads back bit-identical. |

## Ownership and thread safety

The iterators borrow the shape and the document they walk. A converter is a plain value. A reader and a
material map belong to one export or import and are used from one thread.

## Verification

Run `cargo test -p martensite-tkrwmesh`. The `toolkit` target tests the converter, the document reader,
the iterators, the material map, the names and the triangulation source.
