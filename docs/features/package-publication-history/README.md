# Bug #283: first publication of unchanged package versions

## Observed failure

The 0.4.0 candidate selects four changed-source packages for publication. Nine
unchanged packages remain at 0.3.5, as required by package-version policy, but
those identities have never had published crate assets. The release cannot
retrieve them from a historical release and currently stops before the SDK gate.

The earlier assumption that the `v0.3.5` Git tag established publication was
wrong. GitHub has one published release, `v0.3.6` (release ID 422844727), with
26 assets: 12 crates, their checksums, and the package catalog with its checksum.
`gh release view v0.3.5 --repo Oblikovati/Martensite` reports `release not found`.

The verified `martensite-0.3.6-packages.json` has SHA-256
`66d1d98c93fda8dd05bf11681e0b0ad436c5ec75fe6f5500ece8d802ea2707bc`.
It marks these nine package versions as unshipped:

- `martensite-cmath` 0.3.5
- `martensite-tkexpress` 0.3.5
- `martensite-tkg2d` 0.3.5
- `martensite-tkg3d` 0.3.5
- `martensite-tkgeombase` 0.3.5
- `martensite-tkhelix` 0.3.5
- `martensite-tklcaf` 0.3.5
- `martensite-tkmath` 0.3.5
- `martensite-tkservice` 0.3.5

Complete paginated release and asset enumeration finds no earlier publication
of those identities. Their absence is therefore a first-publication case.
A failed fetch, authentication error or incomplete enumeration cannot establish
the same conclusion.

## Cause

`ReleaseScope::load` derives release/source history from Git tags.
`release.rs::package_sources` ships only `scope.packages`, which represents
changed source. It has no separate first-publication selection. The SDK's
historical archive lookup also scans Git tags and stops when a tag has no
GitHub release. Package version and repository release tag are independent.

## Implemented behavior

Use complete published release and asset history to select source assets:

- Changed-source identities that are not already published ship their current
  verified package output.
- Unchanged identities that have never been published ship that same verified
  output once, retaining their existing version.
- Already-published identities retain their exact archive bytes and normalized
  requirements. Their regenerated local outputs do not replace those archives.

For the current candidate this means four changed-source publications, nine
first publications, and eight exact historical reuses. The catalog must record
each reason and its provenance separately from behavioral test scope.

## Acceptance

- Enumerate every page of published releases and every page of their assets;
  include published prereleases when determining package identity immutability.
  Do not infer publication from Git tags or matching repository/package versions.
- Fail on HTTP/authentication failures, malformed or partial history, missing
  checksum companions, corrupt payloads, conflicting repeated identities, or
  publication receipts that claim missing assets.
- Establish first publication only after complete validated history contains no
  publication of the exact package name/version.
- Preserve source-change version guards and select affected tests from source
  and dependency impact. First-publication status alone does not change source
  or justify a package version bump.
- Ship the four changed and nine previously unpublished candidate packages;
  retain the eight historical archives with verified hashes and requirements.
- Add causal regressions for failed Git tags, pagination, first publication,
  subsequent exact reuse, immutable repeated identities, incomplete publication,
  corrupt assets and network/authentication failures.
- Add an Unreleased Fixed entry and update source-asset/catalog documentation.
  Run the required local PR gate before pushing.

Land the common bug fix on `main`, then forward-merge and adapt the SDK's
archive graph to the same publication plan on `release/0.4`. No package-version
changes, old tag changes or historical asset replacements are authorized.
