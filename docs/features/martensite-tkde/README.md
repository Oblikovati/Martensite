# Data exchange framework

`martensite-tkde` is where exchange formats register, so that a file can be read or written by its
extension or content without naming a format.

| Module | Capabilities |
| --- | --- |
| `de` | Configuration contexts and nodes, providers, the format registry (`DeWrapper`), plugin holders, validation helpers and the shape healing parameters shared by the format nodes. |

## Ownership and thread safety

The registry shares one global instance through `DeWrapper::global_wrapper`. Providers and nodes are
reached by handle. Registration is process-wide, so `de_plugin_lifecycle` has its own test binary.

## Verification

Run `cargo test -p martensite-tkde`. The tests replace formats with named fakes (`tests/support`) and
compare the text of a saved configuration with `tests/data`. File checks use fresh temporary folders,
so they behave the same on every platform. No recorded baseline of this crate moved on correctly rounded
math; see [the attribution](../martensite-tkxcaf/corpus-attribution/README.md).

The suites that need every format's real configuration node run with the toolkits that provide them.
The header of a saved configuration is compared byte for byte with `tests/data`, including its wording
about resource files.
