# Shape healing behavior baselines

These frozen expected values are the existing frozen regression baselines used
by this crate's unit and integration tests. Keeping them inside the crate lets
packaging retain every fixture without depending on the repository's tools tree.
`SHA256.json` pins every payload. This move changes paths only; no expected byte,
assertion, numerical tolerance, or production computation changes.

These historical differential baselines are not independent mathematical proofs.
The migration readiness evidence records their original paths and hashes. A
product migration must review and describe their owned behavior, preserve their
values, and remove upstream-specific provenance from product-facing names and
text. Do not regenerate their expected values merely to make a test pass.
