# Portable transcendental functions — review inventory (#201)

This is a review inventory, not an accepted baseline or release qualification.
The proposed replacement routes transcendental operations through one owned
adapter. The captured issue records the numerical-policy decision; it does not
prove that this implementation or its changed expected values meet acceptance.

## Fixed scope

`inventory.json` pins the original and candidate commits. `issue-201.json`
contains the GitHub issue fields fetched during this review. `changed-paths.json`
lists all 437 changed paths between those commits.

The fixture inventory covers all 73 changed `.txt` / `.txt.gz` test fixtures:
975 changed decoded rows and 167,741 positional token differences. Expression
wire output accounts for 118 rows. Other fixture families are kept distinct in
`fixture-files-*.json`. Counts describe textual records, not distinct tests,
algorithmic errors, floating-point results, or semantically aligned fields.

`changed-rows-*.json` records every changed row's old/new byte hash and each
changed whitespace-delimited token. Row numbers are one-based; token indices
are zero-based. Gzip files are decompressed before comparison, while both
compressed and decoded hashes are retained. Positional comparison does not
establish request identity; insertions can shift subsequent token positions.
Whitespace-only changes are explicitly marked. The immutable Git blobs retain
complete old and new bytes, including line endings.

`changed-test-source-*.json` retains zero-context patches for all 159 changed
Rust test source files. This includes assertions, input builders and new tests;
these are not silently counted as unchanged tests or mapped semantic cases.

## Remaining admission work

Every changed row is marked `independent_validation: pending`. A green replay
against a regenerated expected result cannot establish correctness. Before
admission, reviewers must map each row to its consumer schema and original
request, distinguish scalar elementary-function changes from propagated
whole-operation differences, and attach independent expected-result evidence.
High-precision elementary-function spot checks alone do not validate changed
geometry, topology, iteration decisions, text formatting, or compound outputs.
Request, assertion and tolerance preservation also requires source review.

Behavior version 2 is a **proposal**, not an approved migration. The replacement
changes already published foundation/math behavior as well as the new toolkit
batch. A reviewed decision must establish versioning, earlier-behavior replay
or a recorded migration, and changelog guidance. Existing version-1 labels must
not describe changed baselines as if their contract were unchanged.

Dependency trust and policy/audit results, correctly-roundedness scope,
exceptional-value behavior, and the four supported release targets still need
recorded verification. No production code, fixtures or version declarations are
modified by this inventory.
