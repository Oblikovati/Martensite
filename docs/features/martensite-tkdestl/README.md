# STL exchange

`martensite-tkdestl` reads and writes STL files and connects them to shapes and to the data exchange
framework of `martensite-tkde`.

| Module | Capabilities |
| --- | --- |
| `rw_stl` | Text and binary STL on paths and caller-owned streams: triangulations, one per domain on request, node merging by angle, and the callback interface of the readers. |
| `stl_api` | Shape transfers: a shape from a file or stream, and the triangulations of a shape's faces to a file or stream. Nothing is meshed here. |
| `de_stl` | The configuration node and provider that register the format, with vendor `Martensite`. |

Binary files carry the header `STL Exported by Martensite`.

## Ownership and thread safety

Readers and writers borrow their streams and return owned triangulations and shapes. A provider keeps
its configuration node. A node and its provider are used from one thread.

## Verification

Run `cargo test -p martensite-tkdestl` with light optimization, `CARGO_PROFILE_DEV_OPT_LEVEL=1`. The
`toolkit` target has the reader, writer, shape transfer and provider tests. The `stl_cases` target runs on
Linux: 27 cases on real files. The 22 that draw run each alone in a child process on the
[renderer test setup](../martensite-vulkan/README.md) and end with a clean validation report. Their inputs are
public files in the test data submodule. Set `MARTENSITE_STL_IMAGE_DIR` to keep the images as PNG files.
