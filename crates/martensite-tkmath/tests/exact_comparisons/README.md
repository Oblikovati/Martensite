# Exact coordinate comparison oracles

`oracle.py` generates 296 cases using Python `Fraction` arithmetic on the exact
supplied binary64 values. The seed is 3563. Rows retain eight hexadecimal bit
patterns followed by the exact signs of:

- `|b-a|² - |c-a|²`, for squared-distance ordering;
- `(b-a) · (c-d)`, for projection ordering along `a` to `b`.

The first eight cases use ordinary representable metric values. The remaining
cases include subnormals, exponent extremes, adjacent floats, signed zeros,
coincident points, perpendicular vectors, and deterministic generated inputs.
The Rust tests also check all 2,047 finite exponent fields, operand reversals,
non-finite argument precedence, and finite false ties from rounded arithmetic.

`recorded-ordinary.csv` is a reviewed baseline of the rounded binary64 metric
signs for the first eight ordinary cases, with the case index and both signs.
Rounded metrics are not an oracle for extreme or near-tie cases; the exact
rational corpus supplies those expected signs. The Rust tests do not need Python
at run time.

```sh
python3 crates/martensite-tkmath/tests/exact_comparisons/oracle.py > /tmp/comparisons.csv
cmp /tmp/comparisons.csv crates/martensite-tkmath/tests/exact_comparisons/oracle.csv
cargo test --locked -p martensite-tkmath --test toolkit exact_comparisons
```

Both predicates order supplied coordinates. They do not establish geometric
construction accuracy or complete intersection roots.
