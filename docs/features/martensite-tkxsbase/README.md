# Data exchange foundation

`martensite-tkxsbase` is what an exchange format builds on: a model of the file's entities, and the
transfer between those entities and shapes.

| Module | Capabilities |
| --- | --- |
| `interface` | Entity models and protocols, generic libraries, checks, graphs, copies, file reading tools and static parameters. |
| `if_graph` | Graph evaluations over the entities of a model. |
| `if_select` | Selections, modifiers, dispatches, work sessions and their commands. |
| `lib_ctl` | Libraries keyed by protocol. |
| `moni_tool` | Typed parameter values and the time counters. |
| `transfer`, `transfer_brep` | Transfer processes with their binders and actors, and transfers to and from shapes. |
| `xs_algo`, `xs_control` | Shape processing around a transfer, and the reader, writer and session on top. |
| `xs_message` | The message texts, embedded byte for byte. |

## Configuration

`MARTENSITE_XS_MESSAGE` names a folder of message files to read instead of the embedded English text.

## Ownership and thread safety

Models and sessions are thread-confined. Static parameters and message catalogs are process-wide, so
the tests that change them have their own test binaries.

## Verification

Run `cargo test -p martensite-tkxsbase`. The tests use `martensite-tkde` as a development dependency,
and `bug33318_brep_text` reads and writes a shape file through `martensite-tkbrep`. No recorded
baseline of this crate moved on correctly rounded math; see
[the attribution](../martensite-tkxcaf/corpus-attribution/README.md).

The cases that read or write a STEP file run with the toolkit that provides it.
