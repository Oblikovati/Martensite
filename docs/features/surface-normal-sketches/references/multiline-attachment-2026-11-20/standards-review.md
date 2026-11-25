# Standards review — B1 multiline attachment controls

ACCEPT at source level: zero documented-standard or actionable heuristic findings. Reviewed only the two-path test delta against `da08ca1a46c622c4a63fceb696ba1a06f807bdc8`, with trusted main merge-base `b61a031d0bde0e21d283e0f94975f79b9ae15574`.

Both hashes in `/tmp/martensite-b1-current-cylinder-text-20261120/multiline-attachment-source.json` matched before and after inspection (manifest SHA-256 `2ca08114fb53baccd6886b51a97cbf0e63ec485f1411c3afb8bdfda0a5a910a1`). The child has 103 physical lines; its existing parent remains 182 lines.

## Documented standards

- `AGENTS.md:148–156` requires shared logic and scoped changes. The new tests reuse the owned-text request and existing Boolean setup, input serialization/isolation, full body validation, solid-count and entity-history helpers. New helpers cover only multiline support placement, selected-cap survival and attachment direction. No production code or public test framework is introduced.
- The fixture explicitly requires 239 curves and 18 selections. Both operations retain the existing explicit 8192 pair-bisection allowance used by the multiline fixture; no default, numerical version, tolerance, cap, dependency, unsafe authority or package version changes occur.
- Outward/inward authored-tool volume expectations are independent fixed rational values, with the inward Jacobian relationship explained. Native attached-volume integration is used only for the explicitly named direction estimate. Successful attachment additionally requires ordered body checks, one solid, complete existing history checks and an output image for every selected region's semantic last-depth face. This avoids treating volume alone as acceptance, consistent with `AGENTS.md` verification rules.
- Input request equality, encoded result bytes and existing source-shape snapshots check isolation after composition. The tests cover both Fuse and Cut without weakening existing controls. All new functions have focused bodies within the repository's meaningful-body convention, and files remain below 500 lines.
- This is test-only churn, exempt from the user-visible CHANGELOG requirement. The short explanatory comments state intent and numerical provenance; no prose rewrite is needed.

## Heuristics and remaining gates

No actionable Fowler smell. The shared attachment driver has two concrete callers; the cap helper expresses a stricter survival assertion than the existing general history helper and is not speculative abstraction.

Reviewer ran no builds/tests. The tests were still executing at delegation; no passing-runtime or performance result is inferred. Functional and capability acceptance remain separate from this source review.

## Corrected face-survival guard

Final source verdict: ACCEPT, zero Standards findings. Refreshed two-path manifest SHA-256 `9131610a7461d192b01cf1b4fe565182e1616ccc088beda607bc24881e796a78` matched before/after review. The source must be a Face, and the shared `retained_face_in_output` predicate requires both Face kind and output membership for retained/modified/generated images. This focused helper removes the edge/vertex acceptance gap while preserving complete history checks. The earlier weaker candidate's two passes do not qualify this correction; its rerun remains pending. Reviewer ran no tests.

## Final Result-type correction

Final source verdict remains ACCEPT for manifest `5b0fe8f18066e30d4e04574067c6de75ffd0acd8c1a160db2c0bae99247c1605`, verified before/after inspection. This supersedes the earlier Option-based face guard, which did not compile; that earlier source acceptance missed the return type. The source assertion now unwraps the Result in test code, while image admission uses `matches!(shape_type(), Ok(Face)) && membership`, so errors and nonfaces cannot pass. No assertion or ownership requirement is relaxed. The coordinator reports two corrected controls passed in 207.16 s (`attempt4.log`); reviewer ran no builds/tests and infers no performance acceptance.
