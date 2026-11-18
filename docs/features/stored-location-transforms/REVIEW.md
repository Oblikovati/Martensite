# Local candidate review for #451

Reviewed author commit `a18a789ecfbe9a69222da5b313a2b0fc64c9501c`
against pinned main merge-base `fb1217550ed9277f697f14a96a53cb19b8823770`.
The feature incorporated it as `aebd7c26`; the only conflict was the additive
changelog entry, retained alongside the existing feature entries.

The independent Standards review found no documented violations or actionable
heuristic smells. The independent Spec review found no missing requirements,
scope creep or confirmed implementation defects. It verified legacy defaults,
explicit policy propagation, sign factoring and the limited elementary-datum
preservation contract.

The Spec review noted one coverage limitation: malformed matrices are tested
at scalar and location-table boundaries, while complete grouped/reference
shape tests cover valid policy propagation. Malformed complete shape files
would add confidence; no propagation defect was identified.

The author reported 35 passing BinTools behavior tests, including 11 new cases,
16 passing location-wire controls and strict all-target clippy for TKMath and
TKBRep. The parent inspected the code and reused those focused results. Both
reviews were read-only and ran no builds. No version, format or legacy-default
change was approved or made. Candidate composition checks and eventual hosted
integration remain separate gates; this record does not close #451.

## Wrapped codec composition

The wrapped frozen encoder and detached decoder now select both stored-surface
and stored-rigid-location policies explicitly. Existing surface-only and legacy
codec entry points retain their previous location policy. The composition
passed all 242 sketch library tests, with two existing ignored tests, and
strict all-target math/sketch clippy. Two new controls check actual encoded
oblique root/binding coefficients across replay, and rejection of a scaled
complete grouped shape while the legacy path still accepts it. This adds one
complete-shape refusal control to the boundary coverage noted above.

One initial local composition check used incompatible cached TKMath metadata
after worktree switching and reported the new constructor missing. Rebuilding
only TKMath's cached artifacts resolved it without a source or contract change;
the strict composition check then passed. The original failure log is retained.
Physical oblique modelling and its publication proof remain separate work.
