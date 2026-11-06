# Extended exchange document

`martensite-tkxcaf` extends a document with the data exchange formats carry beside the shapes.

| Module | Capabilities |
| --- | --- |
| `xcaf_doc` | The document's tools and attributes: shapes and assemblies, colors, layers, materials, clipping planes, dimensions and tolerances, notes and views. |
| `xcaf_dim_tol_objects` | The objects that describe dimensions, tolerances and datums. |
| `xcaf_note_objects` | The object that describes a note. |
| `xcaf_view` | The object that describes a saved view. |
| `xcaf_prs` | The presentation of a document: styles and the walk over its shapes. |
| `xcaf_app` | The application of these documents. |

## Ownership and thread safety

Documents and their labels are thread-confined. Some tests change process-wide shape naming or
transfer units, so each of those has its own test binary.

## Verification

Run `cargo test -p martensite-tkxcaf`. The tests that open a viewer need a Vulkan driver and run only
on Linux; see [how to run them on a software driver](../martensite-vulkan/README.md). They are
`demo_samples`, `color_alpha_view`, `empty_compound_bounding`, `empty_compound_shaded`,
`presentation_properties`, `repeated_document_show` and `viewer_document_close`. The `pencil` sample
reads FreeSerif from `external/martensite-test-data`.

Two values of the logo sample moved on correctly rounded math; see
[the attribution](corpus-attribution/README.md).

Documents stored in a native format are not read here, and neither are STEP files, so the cases that
need them run with the toolkits that provide them.
