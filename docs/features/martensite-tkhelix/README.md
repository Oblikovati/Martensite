# Helix curves

`martensite-tkhelix` builds helix curves: the analytic helix, its B-spline
approximation coil by coil and composite helix wires of several parts (#208).

| Module | Capabilities |
| --- | --- |
| `helix_geom` | The analytic helix curve, the coil and multi-coil builders and the approximation tools. |
| `helix_brep` | Helix wires of one or several parts with given heights, pitches or turn counts, and their smoothing. |

## Ownership and thread safety

Builders own their parameters and give new curves and wires. The crate has no
process-wide state. A builder that cannot reach its tolerance reports the error code
and the reached tolerance.

## Verification

Run `cargo test -p martensite-tkhelix`. The tests cover the helix curve and its
derivatives, the coil and multi-coil builders, the helix wire builder with its failure
contracts and the wire stage of the helix script cases. The `numeric_corpus` tests
replay frozen requests against frozen answers bit for bit; the answers that changed
with the correctly rounded transcendental functions are attributed in the
[hidden-line and helix attribution](../martensite-tkhlr/corpus-attribution/README.md).
