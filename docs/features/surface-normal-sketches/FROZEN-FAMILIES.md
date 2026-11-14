# Frozen disconnected families

## Construction and evidence ownership

The private rational implementation now constructs disconnected cylindrical
material as one frozen family (#418). It qualifies the original selection once,
prepares one global boundary relation, and checks periodic material containment
before native construction. It builds each member solid with the existing
rational extrusion stages, then writes all solids and semantic references into
one shape table. The compound root is a transport container with no invented
persistent identity. Each member keeps its authored region identity.

Construction orders members by authored region ID and semantic references by
binding key. Decoding uses `PreserveStoredV1`. The decoded compound must have
exactly the declared forward solid children, with identity placement. A global
binding registry refuses duplicate keys, aliases across members, missing or
extra bindings, and incorrect shape kinds. Individual member checks then verify
the full face, trim, wire, edge and vertex coverage already required for a single
solid.

Every member's retained O2 sources are gathered in the original selection order.
Their complete source list must match both the global within-boundary clearance
certificate and its periodic certificate. The original boundary sources and
exact dyadic partition are rechecked before this composition. Missing member
proofs and substituted profile certificates refuse the operation. Clearance is
not recomputed independently per member.

After this binding, the existing member stages certify cones, compare retained
flux with ideal normal measure, and run the ordered native checks and quadrature
estimate. Member reports retain separate measures and work receipts. The result
owns encoded bytes, authored keys, member evidence and the shared selection
evidence; it exposes no native geometry handles. `UnqualifiedFamily` has a
compile-time `Send` and `Sync` check.

## Controls

The mixed line/quadratic/cubic fixture with a hole and a separate unit square
freezes and re-encodes byte for byte. Independent authored-volume targets are
11.10375 and 3.15. These targets are checked against authored-volume enclosures;
the retained-flux comparison uses its existing explicit error policy. Native
quadrature is reported separately and does not prove the approximation error.

Other controls cover reversed selection order, missing and reversed children,
duplicate and unbound solid uses, a noncompound root, cross-member aliases,
duplicate authored owners, missing retained members, substituted O2 profiles,
and mutation of a detached export. Full construction also covers an island in
an unselected hole and disconnected material across one cylindrical turn.

## Remaining scope

The result remains unqualified. The later [authored-history checkpoint](AUTHORED-HISTORY.md)
binds complete private extrusion relations to its frozen keys. Cancellation and
aggregate operation policy, replay in separate processes, concurrent-operation
stress checks, and the final publication contract remain open. The detached
export mutation control establishes that case only; it does not replace the
required isolation checks after success, failure, cancellation and source
mutation. General conical profiles, additional authored curve classes, the glyph
workload, oblique frames and deletion of superseded production paths remain in
the approved plan. No package version, public primitive API, dependency or
release tag changed.

## Verification

On Linux x86_64, `cargo test -p martensite-tksketch --quiet` passed 117 library
and 32 integration tests; the suites took 15.04 and 1.91 seconds respectively,
excluding compilation. Nine library tests exercise the family path.
`cargo clippy -p martensite-tksketch --all-targets -- -D warnings` passed.
Formatting and whitespace checks passed. The approximate metrics scan found no
new family function over 20 lines, file over 500 lines, or nesting above two.
These suite timings are observations, not an approved family performance budget.
The full affected-package PR gate remains required before pushing the branch.
