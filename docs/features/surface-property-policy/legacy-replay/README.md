# Historical surface-property replay

The producer ran against the two revisions recorded in [receipt.json](receipt.json). All 30 rows were byte-equal; [properties.txt](properties.txt) is their shared payload. Each row records all 16 stored property fields as binary64 bits. The three explicit epsilon modes also record the returned error bits. This establishes preservation for these six owned fixtures and five modes, without claiming full-corpus or numerical accuracy coverage.

[producer.rs.txt](producer.rs.txt) is the exact producer source used for both runs. It calls only the existing methods. Its fixture and property-serialization modules came unchanged from the feature revision's `crates/martensite-tktopalgo/tests/surface_property_policy/{fixtures,support}.rs`; the receipt records both hashes.

To reproduce, create an external binary crate for each recorded revision. Copy the producer as `main.rs`, and copy the two recorded modules beside it. Add path dependencies on `martensite-tktopalgo`, `martensite-tkbrep`, `martensite-tkernel`, `martensite-tkg3d`, `martensite-tkmath` and `martensite-tkgeombase` from that revision. With the recorded Cargo environment, run `cargo run --manifest-path <external-consumer>/<base-or-head>/Cargo.toml --bin legacy-replay`, capturing stdout separately from compiler output. Compare the two stdout files byte for byte. The recorded dependency resolutions were identical.

The pristine base run is separate from the feature tests that compare old and new APIs within one revision. The narrow-parameter square is included in this historical comparison: both existing methods retain their earlier outputs even though the new query refuses its incomplete adaptive measurement.
