# Numerical special values — #201

The owned contract is documented in `martensite-cmath/src/lib.rs` and the function
rustdoc. These ordinary tests extend its special-value coverage. They preserve
all existing tests and finite reference fixtures, and reuse the integrated #202
single-precision NaN guard without changing implementation.

## Independent specification

The numerical special cases come from the public
[C committee draft N1570](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf):

| New module | Specification and obligations |
| --- | --- |
| `special_unary` | F.10.1–F.10.4: zero signs, endpoints, infinity limits, logarithm poles and invalid domains. |
| `special_angles` | F.10.1.1–F.10.1.4: inverse endpoints and all zero quadrants; finite/infinite and doubly infinite `atan2` quadrants. |
| `special_powers` | F.10.4.3–F.10.4.4: infinity precedence in `hypot`, power identities before NaN propagation, zero/infinity signs, integer parity, infinite exponents and negative-base domains. |
| `special_remainder` | 7.12.10.2/F.10.7.2: numerator-signed zero, finite numerator with infinite divisor, invalid operands and NaN classification. |

Power zero/infinity/parity tables run in both binary64 and binary32. Their operands
and specified non-NaN results embed exactly in binary32, so the shared expected
words do not depend on another implementation. Existing #202 tests cover binary32
NaN identity precedence; the new binary64 tests supply the corresponding matrix.

Exact expected words distinguish both zero signs and infinity signs. Domain-error
and propagated NaNs are checked only with `is_nan()`, including inputs with both
signs and quiet/signaling encodings. Output NaN sign, payload and quieting are not
asserted. This follows the owned contract and the
[Rust NaN representation rules](https://doc.rust-lang.org/std/primitive.f32.html#nan-bit-patterns).
No errno, exception flag, trap, floating-environment or text-serialization claim
is made. Expression wire formatting remains a separate contract. Integer-power
sequence verification is a separate test lane.

The four positive angle constants and their negatives were independently enclosed
using MPFR 4.2.2 directed 256-bit bounds for π, scaled by exact rational factors
1, 1/2, 1/4 and 3/4. `pi-bounds.json` retains those exact rational bounds and
expected binary64 words. Each interval lies strictly between the neighboring-word
midpoints, using the same integer `require_inside` check as `reference-proof`.
This validates these constants only; it is not a whole-function proof.

The bound calculation can be reproduced with the sibling proof's hash-pinned
Python dependency and exact integer verifier (no product code is imported):

```python
import json, sys, gmpy2
from fractions import Fraction
from pathlib import Path
sys.path.insert(0, "docs/features/portable-transcendentals/reference-proof")
from exact_cells import require_inside
rows = json.loads(Path("docs/features/portable-transcendentals/special-values/pi-bounds.json").read_text())
for row, scale in zip(rows, [Fraction(1), Fraction(1, 2), Fraction(1, 4), Fraction(3, 4)]):
    bounds = []
    for mode in [gmpy2.RoundDown, gmpy2.RoundUp]:
        with gmpy2.context(precision=256, round=mode):
            numerator, denominator = gmpy2.const_pi().as_integer_ratio()
            bounds.append(Fraction(int(numerator), int(denominator)) * scale)
    assert bounds == [Fraction(row["lower"]), Fraction(row["upper"])]
    word = int(row["expected"], 16)
    require_inside(*bounds, word)
    require_inside(-bounds[1], -bounds[0], word | (1 << 63))
```

## Executed scope

At base `148217f91efe6bb8ce0c9f96c6bff7d3b8834305`, 23 new tests bring the scoped
crate to **33 ordinary tests and 28 doctests**, with zero failures or ignores.
The final source passed both one-worker and four-worker runs, scoped strict
Clippy, scoped format checking and `git diff --check`. The initial scoped run
also passed; there was no test failure or altered expected result.

`receipt.json` and its logs remain unchanged historical evidence for
`d0582606d8eebd3bcad0849d9852c9eb7b4fbdae`. Review corrected the misleading
group name `odd_functions_preserve_both_zero_signs` (because `expm1` is not odd)
to `functions_preserving_both_zero_signs`. Only that test identifier changed;
all cases and assertions remain identical. `receipt-renamed.json` supersedes
the current-source/hash qualification with refreshed one/four-worker runs and
strict Clippy logs; the same 33 ordinary and 28 doc tests pass without failures
or ignores. Both receipts record exact commands, compiler, source hashes and
compressed raw log hashes. The cache was a new exclusive, crate-scoped directory; no
workspace build or canonical PR gate was run. This Linux result does not claim
execution on the other supported release targets or universal domain coverage.
The new shared helper is test-only and introduces no public library API,
dependency or version change.
