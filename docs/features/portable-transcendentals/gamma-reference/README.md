# Independent approximate gamma values

The approximate gamma methods retain their existing direction names. At input
`0.5f32`, the forward method uses exponent `2.2f32`, word `400ccccd`.
The first record in the sibling binary32 proof certifies result `3e5edc66`.

The inverse method evaluates `1.0f32 / 2.2f32` before power. The exact divisor is
`9227469/4194304`; its exact reciprocal lies strictly inside the binary32 rounding
cell for **`3ee8ba2e`**. This differs from `3ee8ba2f`, the explicit exponent in the
second sibling reference. That sibling reference cannot certify this method.

`gamma_certificate.py` proves the division cell with exact rational arithmetic.
It obtains a proposed power result independently with MPFR at 24-bit precision,
then uses the existing 256-bit directed evaluator to enclose the exact real power
and certify direct binary32 rounding. Both endpoints lie strictly inside the cell
for **`3f3ad03f`**. No kernel function supplies an expected result. The certificate
retains exact endpoints and rounding-cell margins. It reuses MPFR 4.2.2 and the
pinned dependency provenance of the sibling reference proofs.

The new ordinary test checks scalar and vector methods against fixed words;
existing adapter-consistency tests remain unchanged. The scope is these two
input/exponent pairs, not the full gamma curve or full-domain power correctness.

Review caught an initial operand mismatch: using the sibling second reference
for the inverse caused a one-bit test failure. Exact division analysis identified
the different exponent; the new independently computed certificate establishes
the correction. Neither tolerances nor existing expectations changed.

Run the evidence controls from the repository root with the sibling proof's
hash-pinned Python dependencies installed:

```sh
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/tmp/martensite-reference-proof-python python3 -m unittest discover -s docs/features/portable-transcendentals/gamma-reference -v
cargo test -p martensite-tkernel approximate_gamma_matches_independent_binary32_references
```

Four proof controls cover source identities, the exact division, full certificate
regeneration, and rejection of the adjacent expected result. The product source and reused
proof helpers are bound by `source-bindings.json`; the generator's import name
is distinct from the sibling generator to avoid Python module-name collisions.
