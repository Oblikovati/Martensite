# IGES exchange

`martensite-tkdeiges` reads and writes IGES files, turns their entities into shapes and documents and
back, and connects the format to the data exchange framework of `martensite-tkde`.

| Module | Capabilities |
| --- | --- |
| `iges_data`, `iges_file` | The model and its entities, the global section and directory records, the protocols and modules, the parser of a file and the writer. |
| `iges_basic`, `iges_geom`, `iges_solid`, `iges_appli`, `iges_defs`, `iges_dimen`, `iges_draw`, `iges_graph` | The entity packages: each entity, its reading, writing, sharing, copying and dump. |
| `iges_to_brep`, `iges_conv_geom` | Entities to curves, surfaces and shapes. |
| `geom_to_iges`, `geom2d_to_iges`, `brep_to_iges`, `brep_to_iges_brep` | Curves, surfaces and shapes to entities, as faces or as boundary representation. |
| `iges_control` | The reader, writer and controller of shapes. |
| `iges_select` | Selections, signatures and edits of a model, among them the editor of the file header. |
| `igescaf_control` | Reading and writing a document: names, colors and layers. |
| `de_iges` | The configuration node and provider that register the format, with vendor `Martensite`. |

## What a written file says

The sending product and the preprocessor are `Martensite IGES processor` and the version, and the system
is `Martensite` and the version. The author's default is `Martensite`; the company and the receiver stay
empty until a caller sets them. The configuration keys are `provider.IGES.Martensite.*`. Other values in a
file that is read stay readable. The crate does not read the account, the host name or the time zone: the
dates of a written file are the system clock in UTC.

## What reads and writes back

- A B-spline surface reads its bounds in the order of the format, `U(0), U(1), V(0), V(1)`, so a file read
  and written keeps them.
- A copy of a text font, a tabular property or a connect point keeps its values, and a circular array
  lists its base entity among the entities it shares.
- A rebuilt group lists its own members, and a rebuilt drawing is added to the model.
- A macro definition cut before its statements ends its read.
- A copious data record with no point keeps its type and form and writes zeros, and the failure stays on
  the entity's check.

## Ownership and thread safety

An entity is a handle with its fields in one shared state, so setters take `&self`. The format's
transfer actor is one per process: the reader's construction, its reads and the transfers of
`IgesControlReader`, `IgesCafControlReader` and `IgesToBRepReader` take one lock of the format, so reads
on two threads run one at a time and give the same shapes as on one thread. The format defaults are
process-wide.

## Verification

Run `cargo test -p martensite-tkdeiges` with light optimization, `CARGO_PROFILE_DEV_OPT_LEVEL=1`. The
`dataset_cases` and `restored_cases` targets run the exchange cases, each alone in a child process under
its CPU budget, on public files in the test data submodule. The `viewer_cases` target runs on Linux on the
[renderer test setup](../martensite-vulkan/README.md) and ends with a clean validation report. The other
targets test the entity records, the round trips, the settings, the provider and two threads.
`IGES_ROUND_FILE`, `IGES_TRANSFER_FILE`, `IGES_TRANSFER_OUT` and `IGES_SET` point the round trip tests at
another file or settings; without them the tests use the public files.
