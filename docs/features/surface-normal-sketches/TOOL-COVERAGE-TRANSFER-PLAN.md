# Plan: transfer certified tool coverage into the attached-solid check (#419)

Status: plan, not implemented. Measurements: [performance review](PERFORMANCE-REVIEW-2026-11-20.md).

## Claim

For a wrapped boss or pocket attached to a cone, the final `validate_body` may
skip a self-intersection candidate pair only when the tool certificate already
proves that pair separated. With those pairs skipped, the report must have the
same verdict and the same findings as the full native search, and it must state
how each candidate pair was covered.

On the 239-curve boss, edge–face and face–face tasks are 81% of the serial check,
and about 70% of them pair two tool faces, so the gain is up to about half of the
check.

## What the certificate covers today

`CertifiedConicalBody` binds one numeric proof per member to the exact bits of
each member's side and cap carriers, trims and incidences
(`body_check/certified_conical/binding/snapshot.rs`). Pairs inside a member come
from the member proof; pairs across members come from the family clearance
proof. The snapshot holds no tolerances: the proof is about exact geometry.

## Failure modes

1. A result face binds to the wrong certified source, so a pair whose geometry
   changed is skipped.
2. The Fuse changes tool topology. At zero depth the first-depth caps disappear,
   their edges become seam edges shared with the support face, and a support edge
   can split or trim a tool face. A face that is not an exact image must fall back
   to native search.
3. The Fuse raises edge or vertex tolerances. Native search treats overlapping
   tolerance zones as interference, so a pair may be skipped only when its
   certified separation exceeds the two tolerances plus the fuzzy value.
4. A pair outside the certificate is skipped: tool–support, support–support, or
   a pair that involves a split image.
5. Skipping a pair changes the pave filler's later state (pave blocks, splits)
   for pairs that are still searched.
6. The filter adds order dependence.

## Design

- Bind each result face, edge and vertex that has one tool parent to its
  certified source through the existing exact binder. Refuse ambiguity; an
  unbound entity is native.
- A pair is covered only when both entities are bound, the certificate states
  separation for their sources, and the certified lower bound exceeds the actual
  tolerances plus fuzzy. Decide each pair once, in `BOPDSIteratorSI`, next to the
  existing contained-pair and oriented-box filters.
- Every other pair keeps native search.
- The self-intersection stage reports covered and native pair counts per
  interference kind. Its check mode stays `NativeSearch`, because part of the
  stage is a search; the certified part is reported as coverage, not as a mode.

## Evidence path

| Claim | Evidence | Kind |
| --- | --- | --- |
| Same verdict and findings | Full and filtered checks of the boss and pocket fixtures give identical reports | Differential |
| Changed faces are not skipped | Perturb one result side by one ulp in a carrier coefficient: it stays native | Red control |
| Tolerance growth is not skipped | Raise one seam-edge tolerance above the certified margin: its pairs stay native | Red control |
| A planted fault is found | Move one tool face into another after binding: native search reports it | Red control |
| Only certified pairs are skipped | Unit tests of the covered-pair predicate over bound, unbound, split and support entities | Unit |
| No order dependence | Serial and parallel filtered checks give identical reports | Differential |
| The gain is real | Paired timing of the final check, before and after, same archive | Benchmark |

The coverage counts are a durable part of the report, because the AGENTS rules
require explicit validation coverage. Fixtures that perturb geometry are test
code only.

## Order of work

1. Land the shared-context fix on `main`, forward it to `release/next` and merge
   it here, so measurements start from the parallel check.
2. Add the coverage counts to the report and the binding of result entities,
   without skipping anything. Check that the counts match the census in
   CONE-REGION-ATLAS.md.
3. Add the covered-pair predicate and its unit tests.
4. Enable skipping behind the operation's own path, run the evidence table, then
   measure.
