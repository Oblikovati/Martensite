# Binary persistence property checks

The translated 100 × 200 × 300 box has volume 6000000 and surface area
2 × (100 × 200 + 100 × 300 + 200 × 300) = 220000. Counting each edge occurrence
on its two adjacent faces gives total length 8 × (100 + 200 + 300) = 4800.
These integers are exactly representable in binary64.

Issue #247 corrects the persistence regression to permit four representable steps between each
computed property and its closed form. It uses the shared sign-aware ULP
assertion, with tests accepting four steps and rejecting five on both sides
of each expectation, and rejecting nonfinite computed results. A relative
bound of four times machine epsilon does not express this contract: its
number of representable steps varies with the expectation's significand.

This is a fixture acceptance contract, not a certified integration error bound.
The source/restored property comparison retains its absolute tolerance of
1e-7. Bounds, validity and topology assertions also remain unchanged.
