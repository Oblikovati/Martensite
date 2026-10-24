# Publication history and first source assets

Status: implemented for [Bug #283](https://github.com/Oblikovati/Martensite/issues/283).
The candidate SDK adapter and final release gates run after integration.

## Evidence and current scope

This fresh lane starts from `main` commit
`5a2d9377e5021fbf1f358f18eea20528a4bb209a`. The existing merged SDK lane is
unchanged. The real candidate is `2563dc8b5a44798465c7a2583c3aa5e3e9bd4535`.

Published GitHub history currently contains only release `v0.3.6`, ID 422844727.
Its complete asset inventory contains 12 crate/checksum pairs and a verified
catalog/checksum pair. The catalog marks nine 0.3.5 packages unshipped. The
failed 0.3.5 tag supplies source metadata, but supplies no published archives.

The existing source classifier conflates release tags with published releases.
The packaging selector also conflates source changes with asset publication.
The fix separates those decisions while retaining the package-version policy.

## Authoritative history boundary

The build driver uses an owned read-only history module to the build driver. It retrieves release
metadata with `gh api --paginate --slurp` against the repository releases endpoint.
Retrieve every asset page for each published release through its numeric ID.
Avoid `gh release list` limits and per-Git-tag lookups. A Git tag without an
entry in a successfully completed release enumeration is not a publication.
Source/test planning needs only this release index. Detailed asset enumeration,
catalog validation and archive downloads belong to primary source packaging.

Accept only complete successful JSON page collections. Reject malformed pages,
duplicate release/asset IDs, inconsistent asset identities and non-uploaded
assets. Include published prereleases in the package identity inventory; drafts
do not establish publication. Source release baselines still use final releases
below the requested repository version.

Use archive names and their normalized manifests to establish package
name/version. A repository tag never supplies a package version. Fetch matching
checksum companions and verify payload bytes before using them. When catalogs
exist, verify their checksums and reconcile their shipped claims with the actual
asset inventory. A claimed shipment with absent assets is incomplete publication,
not a never-published package.

If the same package identity appears in multiple releases, verify all occurrences
and require identical bytes. A mismatch is an immutability failure. Persist the
validated history identity/hash summary in the new catalog so later releases can
detect disappearing or changed previously recorded assets. The loader cannot
turn an API or download failure into an empty inventory. Recheck the release/asset
identity summary before admitting first-publication decisions; a changing history
snapshot fails and must be retried.

Do not invent an audit of deleted GitHub data that has no surviving receipt. The
available evidence is the complete successful published API inventory plus its
verified catalogs and retained publication provenance. Known missing records or
receipt inconsistencies fail closed.

## Separate selectors

Keep `ReleaseScope.packages` as changed-source packages and retain its affected
test selection. The fix replaces the Git-tag universe used for release baselines and
historical source/version comparisons with the tags of actually published
releases. The committed manifest snapshots at those tags still describe source
and version policy; a package appearing in a source snapshot is not thereby a
published artifact. Explicit PR Git comparisons remain local and unchanged.

The driver uses a publication plan keyed by exact package name/version. For each
current workspace package, excluding the unpublished build driver:

1. If a verified matching historical archive exists, reuse it. A source-change
   classification requiring the same published identity is an error, never an
   instruction to replace its bytes.
2. If complete validated history contains no matching identity, select the
   current package output for shipment. Record `changed_source` when the source
   selector chose the package, otherwise `first_publication`.
3. Record reused identities as `historical_reuse`, including release and asset
   IDs, archive name and digest. Source-change and first-publication records also
   retain the history snapshot that justified their selection.

Package all workspace members through the existing verified Cargo overlay when
any new asset is selected. Copy only the publication plan's selected outputs to
the release directory. Preserve all existing historical assets; do not rename,
rewrite or reship them as new bytes under an existing identity.

## Expected real candidate plan

| Reason | Packages | Count |
| --- | --- | ---: |
| Changed source | process-policy 0.3.7, tkernel 0.4.0, tkgeomalgo 0.3.7, tktopalgo 0.4.0 | 4 |
| First publication | cmath, tkexpress, tkg2d, tkg3d, tkgeombase, tkhelix, tklcaf, tkmath, tkservice, all 0.3.5 | 9 |
| Historical reuse | test-support, tkbrep, tkcdf, tkhlr, tkmesh, tkprim, tkshhealing, tktobj, all 0.3.6 | 8 |

The `martensite-` prefix applies to every package in this table. All 21 package
versions remain exactly as selected by the coordinator. The nine first
publications may carry the current verified internal requirements because no
earlier archive identity exists. The eight reused archives retain their original
requirements; individual archive compatibility remains a separate truth check.

## Main-first integration

The common repair is viable on `main` without adding the SDK feature there.
Change the history/source baseline, publication plan, source-asset selection and
existing package catalog. Add an Unreleased Fixed entry and focused driver tests.
No SDK module is imported into the patch line.

Release planning and release test jobs need authenticated read-only history
access. Supply `GH_TOKEN` only for release-mode trusted jobs, preserving
`contents: read`; PR planning does not need history or credentials. The existing
SDK's isolated acceptance children must continue to receive no token or inherited
Cargo configuration when the fix is forward-merged.

After the main fix is merged, forward-merge it into the candidate. Adapt the SDK
archive graph to consume this one verified publication plan rather than its own
Git-tag search. The graph reads generated bytes for the 13 selected shipments
and exact historical bytes for the eight reuses. Its honest compatibility status
and the complete SDK consumer gate remain unchanged. The coordinator owns this
serialization and the final release metadata.

## Verification budget

Named fake history and asset providers cover all pagination and failure paths.
Use a fixture with a failed `v0.3.5` Git tag, one published `v0.3.6` release, and
a catalog listing an unshipped 0.3.5 package. Require first publication at its
unchanged version. Publish the resulting fixture receipt, rerun planning, and
require exact historical reuse even when a regenerated local manifest has
different bookkeeping requirements.

Add separate controls for multiple release pages and asset pages, prerelease
publication of a package identity, duplicate identities with identical versus
different bytes, a checksum without its payload, a catalog claiming absent
assets, corrupt downloads, malformed pages, and nonzero HTTP/auth failures.
Each error must be causal; no failed lookup may select first publication.

Retain existing source-change, dependency-impact, removed/reintroduced-package
and immutable-version regressions. Verify that first publication does not widen
behavioral test selection or bump a package version. Required local PR gates and
strict platform-target driver checks apply before pushing.

A read-only plan was run against the real complete
history and verified exactly 4 changed + 9 first + 8 reused identities.
The retained [receipt](evidence/candidate-2563-publication-plan.json) records the
history hashes and archive provenance. It does not verify generated outputs or
SDK consumption. The coordinator runs
the real candidate SDK/release gates. No full release is launched by this plan.

## Job pin and credential boundary

The CI planner emits a hash of the published release index. Every release check
and native test shard receives that hash and refuses a changed index before
computing its source or test scope. Source packaging uses the same index and
rechecks the complete release and asset listings after archive validation.

Only release-mode driver steps receive the read-only GitHub token. The explicit
GitHub API commands retain authentication. Every spawned Cargo command removes
`GH_TOKEN`, `GITHUB_TOKEN`, and the driver-only history pin before running kernel
builds, tests, packaging, or compiler inventory. A real child build-script canary
checks this boundary without changing the parent environment.

Schema-2 catalogs bind every package row to its publication reason, exact archive
digest, and historical origins. Reused rows remain authoritative when no payload
ships in that release. Later planning rejects lost original assets and inconsistent
row digests or origins. Legacy schema-1 catalogs remain readable.
