# Historical package restoration

Publication checks normally verify each retained archive against its original release asset. If that asset is retired, the release owner can restore its exact bytes under a separate current asset anchor (#358). This does not make the retired asset live again.

The supported container is `martensite-historical-package-restoration-1.tar.gz`, with its SHA-256 companion. It contains `restoration.json` and regular `archives/name-version.crate` members. The manifest uses schema 1, pins the same-release published package catalog with `catalog_sha256`, and lists its complete retained history as `entries[].historical_origin`. Existing published catalogs remain unchanged.

The loader checks the actual container asset size, digest and checksum, the pinned catalog bytes, exact closed historical membership, and each archive's digest, normalized manifest digest and package identity. It refuses extra, duplicate, missing, nonregular and foreign members. Container parsing permits at most 257 members and 128 MiB of payload bytes, with a 129 MiB decompressed-stream cap including headers and padding. Inner archive parsing permits at most 65,536 raw headers, a 1 MiB normalized manifest and a 1 GiB decompressed stream. GNU long-name metadata is limited to 4 KiB, requires recognized GNU or ustar magic and a canonical package-rooted UTF-8 name with one terminal NUL, and must describe exactly the following regular member (#360). Repeated, orphaned or manifest-alias long names are refused. Links, sparse records and other extension headers remain unsupported. The outer container still accepts only regular members. Exhausting any limit refuses the operation.

A restored origin retains its historical release and asset identifiers, with a separate `restoration` record naming the actual current release, asset, container digest and catalog digest. Existing origins omit that field, preserving their record format. Future catalogs carry both the historical identity and its verified recovery anchor. The release owner remains responsible for authorizing the restoration publication; a manifest is not an independent signature of past publication.

For example, a schema-1 entry preserves the original origin descriptor:

```json
{
  "schema": 1,
  "catalog_sha256": "<sha256 of the current published catalog>",
  "entries": [{"historical_origin": {
    "release_id": 1, "release_tag": "v0.1.0", "asset_id": 2,
    "archive": "martensite-example-0.1.0.crate",
    "sha256": "<archive sha256>",
    "manifest_sha256": "<normalized manifest sha256>"
  }}]
}
```

The example identifiers and digests are placeholders, not publication evidence. All entries must match the pinned catalog and verified payload bytes before reuse.
