# Four binary32 power references — issue #201

All four literal records in
`single_precision_pow_returns_the_correctly_rounded_reference` at
`2b1f0e601f05eb14147171b0487bd4dcfe1e4425` have their claimed correctly rounded
binary32 output. The exact records and order are unchanged at proof base
`2e204184644a697823e5e1b8ac6ad2c608ce5e66`. Both complete source files are
archived here. No fixture or production change is included.

The eight inputs and four expected outputs are finite nonzero normal binary32
words. Each input embeds exactly into binary64 by shifting its significand and
rebiasing its exponent. The [existing MPFR evaluator](../reference-proof/generate.py)
then imports those exact dyadics and evaluates real power at 256 bits, once toward
negative infinity and once toward positive infinity. This reuses the independent
MPFR 4.2.2 calculation and library provenance from the sibling proof at commit
`746a61f96adaa43c03be00a9845583607eeef141`; no candidate code is called.

Each enclosure lies strictly within the midpoints to the expected **binary32**
word's binary32 neighbors. This proves direct nearest-even binary32 rounding of
the exact real power. No high-precision result is first rounded to binary64, and
no double-rounding equivalence is assumed. All four pass at 256 bits. Every exact
endpoint, cell and positive separation margin is retained in `enclosures.json`.
The [MPFR directed-rounding contract](https://www.mpfr.org/mpfr-4.2.2/mpfr.html#Rounding)
is the mathematical-function enclosure premise; the integer-only verifier checks
the rounding-cell arithmetic independently of the MPFR execution.

Six tests pass: exact embedding and special rejection, asymmetric/negative cells,
complete regeneration and both source identities, adjacent-word rejection,
tampered endpoint/identity rejection, and source cardinality rejection. Two
separate generator processes produce byte-identical certificates. The
`borrowed-source.json` record binds the two reused sibling modules and the
previous library provenance file; those bytes remain unchanged. No additional
library installation, production dependency or Cargo run is involved.

From this directory, with the sibling proof's hash-pinned dependency installed:

```sh
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/tmp/martensite-reference-proof-python python3 prove_binary32.py > /tmp/binary32-recomputed.json
cmp enclosures.json /tmp/binary32-recomputed.json
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=/tmp/martensite-reference-proof-python python3 -m unittest discover -s . -v
PYTHONDONTWRITEBYTECODE=1 python3 verify_binary32.py
```

The standalone verifier needs only Python's standard library. Reproduction of
function bounds and the tests needs gmpy2 as pinned in
`../reference-proof/requirements.txt`. This packet proves these four exact input
pairs only. It does not establish whole-domain power correctness, candidate
execution, special-value behavior, or changed higher-operation expectations.
Zero, subnormal, infinity and NaN inputs/outputs are explicitly rejected; output
cells touching nonnormal neighbors are unsupported and rejected too.
`MANIFEST.json` binds this packet's files except itself and is not an external
signature. The earlier 95-record packet and its recorded review commit are
unchanged.
