# Document presentation attributes

`martensite-tkvcaf` shows the contents of a document in the 3D viewer of `martensite-tkv3d`.

| Module | Capabilities |
| --- | --- |
| `tprs_std` | The viewer and presentation attributes of a document, the drivers that build the interactive object of a label, and the presentations of constraints. |

## Ownership and thread safety

Documents and their labels are thread-confined. A presentation attribute reads its shape by handle
and keeps its settings through transactions.

## Verification

Run `cargo test -p martensite-tkvcaf`. `caf_cases` replays the presentation and transaction scripts
on the contract of [`martensite-boolean-cases`](../martensite-boolean-cases/README.md), each in a child
process on the real viewer, and the `document_*_view` targets check what a document shows. These run
by default on Linux and need a Vulkan driver; see
[how to run them on a software driver](../martensite-vulkan/README.md). No recorded baseline of this
crate moved on correctly rounded math; see
[the attribution](../martensite-tkxcaf/corpus-attribution/README.md).

`heap_cases::bug23489` runs with the allocator's thread cache off, because the heap counter includes
that cache.

The cases that show a named shape built by naming history are in `martensite-tkcaf`.
