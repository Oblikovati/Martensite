# Binary64 reference rounding proof — issue #201

## Result and scope

All **95** literal binary64 references at product revision
`2b1f0e601f05eb14147171b0487bd4dcfe1e4425` have the claimed correctly rounded
nearest-even word. Their parsed identities, input words, order and expected words
are identical at the clean proof base
`2e204184644a697823e5e1b8ac6ad2c608ce5e66`. Both exact source files are retained;
formatting and documentation bytes differ. No production code or fixture changes
are part of this proof.

There are 81 unary and 14 binary records, using 23 function names. All 109 input
words and all 95 output words are finite, nonzero normal binary64 values. Every
case passes with 256-bit directed bounds; no precision escalation was necessary.
`enclosures.json` stores exact rational inputs, endpoints, rounding-cell edges
and positive separation margins, independently of decimal comments in the source.

This is a proof for these individual exact input/output pairs, conditional on the
MPFR implementation's documented arithmetic contract. It is not a whole-domain
correctness proof for the dependency, a test that the candidate implementation
actually returns these words, or approval of changed higher-level expectations.
The existing four binary32 power references, signed zeros, NaNs and their payloads,
infinities, domain errors, underflow/overflow, integer powers, remainder and
combined operations are outside these 95 records. No special-case behavior is
inferred from their absence. The proof generator rejects nonnormal inputs or
outputs; its rational representation intentionally makes no signed-zero claim.

## Method

For each source word, `exact_word` constructs the exact dyadic rational from its
sign, exponent and significand. Conversion to MPFR at precision at least 256 is
exact: every input has at most 53 significant bits. Each function is evaluated
once with rounding toward negative infinity and once toward positive infinity.
The two correctly directed results enclose the exact mathematical result. This
uses each MPFR function directly, including `atan2(y,x)`, `hypot(x,y)` and the
MPFR power operation on two MPFR operands. The negative-base power case has the
exact integral exponent −19 and a well-defined real result.

The [MPFR 4.2.2 rounding contract](https://www.mpfr.org/mpfr-4.2.2/mpfr.html#Rounding)
specifies these directed modes and correctly rounded mathematical-function
results. Bounds use a wide explicit exponent range, not binary64 arithmetic.
The dependency is separate from the candidate's pinned pure Rust implementation;
no candidate function, host libm result or mpmath result supplies an endpoint.
The earlier mpmath generation comments are archived as historical source bytes,
not used as a proof premise.

The certificate check uses Python integer fractions only. It computes the two
midpoints between the expected word and its adjacent finite words, correctly
handling negative ordering and unequal gaps at powers of two. It requires
`left_midpoint < lower <= upper < right_midpoint`. Strict containment avoids any
assumption about a tie: all enclosed values round to the claimed word. Neither a
midpoint contact nor a merely repeated high-precision approximation is accepted.
The verifier checks inclusion and identity; rerunning the generator checks the
function-enclosure premise. Neither is a formal verification of MPFR itself.

## Reproduce

The recorded run used Python 3.12.3 on Linux x86_64, gmpy2 2.3.2, MPFR 4.2.2 and
GMP 6.3.0. `environment.json` records the exact downloaded wheel hash, actual
loaded extension/library paths and hashes, platform and product revisions.
`requirements.txt` pins the tested CPython 3.12 Linux x86_64 wheel by hash; it is
not an assertion that other wheel builds are byte-identical. No MPFR binary is
shipped with the product or copied into this directory.

From this directory, using a separate dependency installation:

```sh
python3 -m pip install --require-hashes --target /tmp/martensite-reference-proof-python -r requirements.txt
PYTHONPATH=/tmp/martensite-reference-proof-python python3 generate.py > /tmp/reference-recomputed.json
cmp enclosures.json /tmp/reference-recomputed.json
PYTHONPATH=/tmp/martensite-reference-proof-python python3 -m unittest discover -s . -v
python3 verify.py
```

The archived output is byte-identical across two separate generator processes.
Seven tests pass, covering exact input decoding, multiline source parsing,
positive/negative and asymmetric cells, midpoint and adjacent-word rejection,
tampered certificate rejection, an exactly representable MPFR control `exp(0)`,
scoped special-value rejection and full regeneration. `tests.log` and
`verify.log` retain the executed results. No Cargo or whole-workspace tests were
run for this evidence-only change. `MANIFEST.json` binds all packet files other
than itself; it is an integrity record, not an external signature.
