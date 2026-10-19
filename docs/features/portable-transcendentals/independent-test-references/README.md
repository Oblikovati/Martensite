# Independent composed test references — #201

This addition addresses five expected-side dependencies on the same elementary
functions used by the implementation. Every existing input expression, assertion,
tolerance and test remains present. `original-check-preservation.json` verifies
that all original lines in the five files survive in order from base
`7fb63ee2f79464abd087e5e793c99b84ee18ce7b`. No production, dependency, version or
existing numeric fixture changed. The earlier gamma-conversion correction is not
duplicated.

## Exact requests and independent expectations

| Existing test file | Added independent evidence |
| --- | --- |
| `tkg2d/tests/analytic_curve_closed_forms.rs` | 33 exact dyadics `i/8`, `i=-16..16`, each with independently enclosed sine, cosine, sinh and cosh words. Supplemental checks evaluate all 792 circle/ellipse/hyperbola jets in both frames at all four original scales, using the original lower evaluators and `32*EPSILON*max(abs(expected),1)` criterion. The original 1320-jet test remains. |
| `tkmath/tests/elclib.rs` | Three new tests use the original identity frame, circle radius 2, ellipse radii 3/2, and hyperbola radii 3/2. Circle and ellipse use the actual stored `PI/4` word `3fe921fb54442d18`, not the real number π/4; hyperbola uses exact 0.5. Fixed point/derivative jets come from directed real scaled-function bounds. Existing `confusion()` checks are reused. |
| `tkmath/tests/mathsys_newton_behavior.rs` | Both stored vector entries are checked against word `6974e718d7d7625a`. For this exact dyadic x, the norm is `abs(x)*sqrt(2)`, independently bounded without calling a hypot oracle. The result word is `697d8f9811335b57`. The existing exact comparison with cmath hypot and finite-result check remain. |
| `tkmath/tests/math_integration.rs` | The real integral of exp(x) over exact [0,1] is expm1(1). Its independently enclosed nearest word is `3ffb7e151628aed3`. The new check retains the original `1e-8` tolerance. |
| `tkmath/tests/mathinteg/tests_6.rs` | The same independent integral word is checked with the original `1e-11` acceptance tolerance. Adaptive configuration and callback are unchanged. |

The last two tests retain their original rounded `exp(1)-1` comparison as a wiring
check. It differs from the nearest real expm1(1) word: this addition does not
silently equate a rounded composition to its real closed form. Runtime guards
bind actual planar parameters, the stored quarter-pi parameter, and both norm
operands to the certified words, avoiding an accidental binary32 operand.

The planar expected jet uses the fixed independently rounded primitive words,
then the existing sign/scale/frame arithmetic. It does not call cmath on the
expected side. Its scales are powers of two and its translations are the original
3 and −5. This is an independent reference check with the existing tolerance,
not a formal interval certificate for the full geometric operation. The spatial
jets instead enclose each real radius-times-function product directly before
rounding; neither the scalar word nor a candidate-generated jet is an oracle.
Zero z coordinates and repeated hyperbolic derivative orders follow the analytic
closed forms. The added spatial checks cover value, third-order jets and orders
one through three through the generic derivative interface; all other original
checks remain too.

## Reproducible rounding evidence

`generate_references.py` reuses the independent MPFR evaluator and exact rational
cell check from `../reference-proof`, without importing product code. MPFR 4.2.2
at 256 bits evaluates each exact dyadic input toward negative and positive
infinity; integer rational scaling cannot narrow the interval. Expected words
are discovered from a bound, then admitted only when the entire enclosure lies
strictly within that word's nearest-rounding cell. Failed or inconclusive
inclusion raises an error. The [MPFR rounding contract](https://www.mpfr.org/mpfr-4.2.2/mpfr.html#Rounding)
is the function-enclosure premise.

`certificates.json` contains **154 records**: 132 planar primitives, 20 scaled
spatial products and two scalar references. There are 152 strict-cell proofs and
two exact-zero proofs (sin(0), sinh(0)); no NaN, signed-zero or whole-domain claim
is inferred. The norm uses directed sqrt(2) followed by exact rational scaling.
The real exponential integral uses directed expm1(1), avoiding cancellation in
its independent expected calculation.

Using the sibling proof's hash-pinned Python dependency:

```sh
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/tmp/martensite-reference-proof-python python3 -m unittest discover -s docs/features/portable-transcendentals/independent-test-references -v
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/tmp/martensite-reference-proof-python python3 docs/features/portable-transcendentals/independent-test-references/generate_references.py
```

Seven controls pass, including full certificate regeneration, source-table
identity, actual operand words, derivative cycles and fail-closed inconclusive
bounds. Separate generator processes produced byte-identical certificates and
both Rust reference modules. The existing loaded-library hashes still match the
sibling proof's recorded environment; no high-precision dependency is added to
Cargo or used by ordinary Rust tests.

## Executed scope and retained attempts

At final source, both worker counts **1 and 4** pass the same **96 selected tests**:
15 spatial elementary-curve, 3 Newton, 16 integration, 60 adaptive-integration,
and 2 planar-curve tests. There are zero failures and ignores. Four new Rust tests
are included; the other additions supplement existing assertions. Scoped strict
Clippy for the two toolkit test targets and scoped formatting checks pass. No
whole-workspace, cross-platform or changed-fixture admission is claimed.

The first operand-identity guard mistakenly called a crate-private accessor.
Four queued Cargo attempts failed compilation on that same error; Cargo's cache
lock serialized their builds. Two subsequent index-type compile attempts are
also retained. The final guard uses the public ordinal `at(0)` / `at(1)` interface.
These were test-authoring errors, not numerical failures. Final gates ran
sequentially against frozen source. Initial passing pre-final runs are explicitly
historical and not used as final-source qualification.

`receipt.json` binds source, raw compressed logs, compiler/environment, exact
commands, certificate records and the original-check preservation result. The
new private cache was scoped to these two crates (about 286 MB); no shared cache
was used. Completion of these five bounded test-reference gaps does not establish
correctness of unrelated changed expectations or an entire operation pipeline.
