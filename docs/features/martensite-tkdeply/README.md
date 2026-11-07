# PLY exchange

`martensite-tkdeply` writes documents and shapes into PLY files and connects the format to the data
exchange framework of `martensite-tkde`.

| Module | Capabilities |
| --- | --- |
| `rw_ply` | The text writer of positions, normals, colors, texture coordinates and faces, and the writer of a document's triangulated faces. |
| `de_ply` | The configuration node and provider that register the format, with vendor `Martensite`. |

## What a written file says

The first comment of a file is `comment Exported by Martensite`. The configuration keys are
`provider.PLY.Martensite.*`.

## Coordinates and precision

The provider's `WriteFile` applies the settings of the configuration: `system.cs` and `file.cs` are the
coordinate systems, and the document's length unit converts to the global length unit, which is the unit of
the file. The default systems are Z up and Y up, so a default document holds `(x, z, -y)` in the file. A
shape with no unit of its own is taken in the global system unit, with a warning. `RwPlyCafWriter` with no
conversion set writes coordinates as they are.

Single-precision values keep 9 significant digits, so a 32-bit float reads back exactly, and double-precision
positions keep 17. Colors and indices are integers.

`close` answers false for a file with fewer records than its header states, unless the export was aborted.
A record past the header's count is a failure, after it is written.

## Ownership and thread safety

A writer owns its file and is used from one thread, as is a provider and its node.

## Verification

Run `cargo test -p martensite-tkdeply`. The `toolkit` target has the writer, conversion, document and
provider tests.
