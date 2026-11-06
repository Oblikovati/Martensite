# STEP exchange

`martensite-tkdestep` reads and writes STEP files, turns their entities into shapes and documents and
back, and connects the format to the data exchange framework of `martensite-tkde`.

| Module | Capabilities |
| --- | --- |
| `step_file`, `step_data` | The parser of a file, its literal content, the model, the writer, the protocols and modules, the select types and members. |
| `header_section`, `rw_header_section`, `api_header_section` | The file header: `FILE_NAME`, `FILE_DESCRIPTION`, `FILE_SCHEMA` and the tool that edits them. |
| `step_basic`, `step_geom`, `step_shape`, `step_repr`, `step_visual`, `step_dim_tol`, `step_kinematics`, `step_element`, `step_fea`, `step_ap203`, `step_ap214`, `step_ap242` | The entity schemas. |
| `rw_step_*` | The reading, writing and sharing of each entity, and the tables that bind keywords to entities. |
| `geom_to_step`, `step_to_geom`, `topo_ds_to_step`, `step_to_topo_ds` | Geometry and topology in both directions. |
| `step_construct`, `step_control`, `step_selections`, `step_edit`, `step_tidy` | Assemblies, units, styles and validation properties, the reader and writer of shapes, selections, editing, and the merging of duplicate entities before writing. |
| `stepcaf_control` | Reading and writing a document: names, colors, layers, materials, dimensions and tolerances, views, external references. |
| `de_step` | The configuration node and provider that register the format, with vendor `Martensite`. |

## What a written file says

The `originating_system`, `preprocessor_version` and `authorisation` of a written `FILE_NAME` say
Martensite, and so do the default author and the configuration keys, `provider.STEP.Martensite.*`. Other
values in a file that is read stay readable. The crate does not read the account, the host name or the time
zone: the default person and organization of AP203 are the constant `Martensite`, and the date is the
system clock in UTC. A caller who wants real names sets them with `set_default_person_and_organization`.

## What reads and writes back

Each value reads as the file has it and writes back the same:

- Shading methods and the actuated directions of kinematic pairs read as their own text. A text that
  is no value of the enumeration adds a failure message and leaves the default value.
- `APPLICATION_DEFINED_ELEMENT_PURPOSE` stays that, and does not become an enumerated purpose.
- `EXTERNALLY_DEFINED_CLASS`, `EXTERNALLY_DEFINED_GENERAL_PROPERTY`, `REP_ITEM_GROUP` and `COMMON_DATUM`
  records keep their fields.
- `COMPLEX_TRIANGULATED_FACE` and `CUBIC_BEZIER_TRIANGULATED_FACE` write with any number of normals.
- A complex record with `ACTUATED_KINEMATIC_PAIR` reads back after writing.

## Ownership and thread safety

An entity is a handle with its fields in one shared state, so setters take `&self`. A reader or writer
owns its work session and is used from one thread. `STEPCAFControlController` and the format defaults
are process-wide.

## Verification

Run `cargo test -p martensite-tkdestep` with light optimization, `CARGO_PROFILE_DEV_OPT_LEVEL=1`. The
`dataset_cases` target runs the exchange cases, each alone in a child process under its CPU budget, on
public files in the test data submodule. The `viewer_cases` target runs on Linux on the
[renderer test setup](../martensite-vulkan/README.md) and ends with a clean validation report. The
other targets test the parser, the entity round trips, the transfers, the document reader and writer and
the provider.
