# OBJ exchange

`martensite-tkdeobj` reads and writes Wavefront OBJ files with their MTL material files, and connects the
format to the data exchange framework of `martensite-tkde`.

| Module | Capabilities |
| --- | --- |
| `rw_obj` | The reader of OBJ and MTL text, triangulations and documents from a file, and the writer of a document's triangulated faces. |
| `de_obj` | The configuration node and provider that register the format, with vendor `Martensite`. |

## What a written file says

The first line of an OBJ and an MTL file is `# Exported by Martensite`. The configuration keys are
`provider.OBJ.Martensite.*`.

## Reading and writing

- `read.memory.limit.mib` is a limit in mebibytes.
- The provider reads a shape as one face, or as shapes with `read.create.shapes`.
- A comment keeps a one-letter last line and repeated lines. A line break in a group or material name is
  written as a space, so the rest of the name does not read as a statement.
- A polygon may have any number of nodes.
- A `g` with no name is ignored when reading, and an index out of range ends its face with an error while
  the read still answers true.

## Ownership and thread safety

A reader or writer owns its file and is used from one thread, as is a provider and its node.

## Verification

Run `cargo test -p martensite-tkdeobj` with light optimization, `CARGO_PROFILE_DEV_OPT_LEVEL=1`. The
`toolkit` target has the reader, writer and configuration tests. The `obj_cases` target runs on Linux, each
case alone in a child process on the [renderer test setup](../martensite-vulkan/README.md), on hand-written
inputs and public files in the test data submodule, and ends with a clean validation report.
