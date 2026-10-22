# Shape healing behavior baselines

These frozen expected values are the existing frozen regression baselines used
by this crate's unit and integration tests. Keeping them inside the crate lets
packaging retain every fixture without depending on the repository's tools tree.
`SHA256.json` pins every payload. This move changes paths only; no expected byte,
assertion, numerical tolerance, or production computation changes.

These baselines complement the crate's independent checks. Preserve their values
until a reviewed behavior change updates the associated assertions. Do not
regenerate expected values merely to make a test pass.
