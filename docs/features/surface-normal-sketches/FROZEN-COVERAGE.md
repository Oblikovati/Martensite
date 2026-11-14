# Frozen cylindrical coverage (#418)

The private rational pipeline checks the decoded body's incidence graph before
its storage, trim, flux and native validation stages. The check receives the
prepared profile after admission and planar qualification. It binds the decoded
root to the semantic solid binding and accounts for every direct child down to
the vertices. It does not infer coverage from the number of stored span proofs.

## Incidence checks

The solid has one forward, identity-located shell. Its face-use multiset contains
exactly the first cap reversed, the last cap forward, and each declared side
forward. Every binding has its declared kind, forward orientation and identity
location. Duplicate keys, missing or extra bindings and aliases between distinct
semantic keys are refused. Native shape-map indices are temporary lookup
references; authored region, span and corner keys retain identity.

Each cap wire matches one whole declared directed contour. Each side has one
wire with its first-depth edge forward, end connector forward, last-depth edge
reversed and start connector reversed. Wire and edge-use multiplicities matter.
Matching is independent of wire order and edge traversal order. Every wire and
the shell must retain their closed flags, and their actual incidence is checked
separately. Each edge has exactly two distinct owning faces with opposite
composed orientations. Its two endpoint uses match the declared corner bindings
and orientations. Vertices have no children.

Every retained edge has exactly three curve representations: one identity-located
3D curve and one single pcurve on each of its two actual owning faces. Matching
uses native support identity and location. Duplicate pcurves, extra
representations, null representations, closed-surface dual pcurves and unsupported
representation kinds are refused. Later storage stages still check curve classes,
coefficients, domains, side nets, cap supports, trim residuals and tolerances.

Failures retain the semantic owner, failure class and offending values. Native
failures propagate. The report counts checked faces, wires, unique edges,
vertices and edge uses. Its Euler characteristic includes the cap holes through
`V - E + sum(2 - face_wire_count)`. Counts describe the checked graph; they do not
establish geometric injectivity or body validity.

## Controls

The mixed line/quadratic/cubic profile with one hole has 117 semantic bindings,
21 faces, 23 wires, 57 unique edges, 38 vertices and 114 edge uses. Its boundary
Euler characteristic is zero. The pipeline retains these counts alongside the
conditional flux enclosure and its separate native validation report.

Sixteen focused controls cover complete coverage, missing and duplicate keys,
a root from an independent decode, reversed root or face uses, aliased face
bindings, duplicated side boundaries, added vertex and face uses, false wire
closed flags, extra representations and duplicate face pcurves with an unchanged
representation count. Binding order, cap-wire order and side-edge order changes
preserve the report. The evidence type has compile-time `Send` and `Sync`
assertions; this does not establish concurrent-operation safety.

The sketch package passed 71 library tests and 32 integration tests on Linux
x86_64. The final focused controls also passed after refactoring the coverage
checker. Strict all-target clippy passed. The approximate source scan found no
inspected file over 500 lines, function over 20 code lines or nesting over two.
The isolated debug P1 fixture took 3.78 seconds with peak RSS 51,536 KiB,
excluding compilation. It remains within the unchanged five-second limit; this
single timing does not establish a performance improvement or regression.
No dependency, public primitive API, version or release tag changed. The full
affected-package PR gate remains required before a push.

## Remaining gates

The result remains `Unqualified`. Incidence and representation coverage support
the conditional flux interpretation. The later [measure comparison](MEASURE-COMPARISON.md)
now supplies its numerical ideal-volume discrepancy. Geometric injectivity,
disconnected material selection, history, isolation and replay/concurrency checks
remain open. General cones, authored arcs and
rational splines, oblique frames and glyph-scale acceptance remain outside this
checkpoint. Superseded production paths cannot be removed until their replacement
gates pass.
