# Exact orientation evidence

`oracle.csv` stores six binary64 bit patterns and an exact sign per row. Python
`Fraction` computes the homogeneous 3x3 determinant by cyclic expansion, without
the production integer-unit conversion or translated 2x2 expression. The 391
cases include 256 rounded near-collinear constructions, 128 random finite bit
patterns, exact zeros, signed zeros, repeated points, minimum subnormals,
maximum finite values, cancellation, and a `1e-200` underflow case.
Inputs are stored as bits so fixture parsing introduces no decimal conversion.

Regenerate with `python3 crates/martensite-tkmath/tests/exact_orientation/generate.py`
and compare its output byte-for-byte with `oracle.csv`. Rust tests check all six
point permutations and all 2047 finite exponent fields. They also require the
corpus to contain more than 50 cases where a rounded determinant is wrong or
nonfinite. Every input position is tested with NaN and both infinities; error
identity, display text, and first-error order are checked.

For the cancellation triangle `(0,0), (1+2^-52,1), (1,1-2^-52)`, Jacobian's
`matrix.determinant.compute` independently returned the exact determinant
`-1/20282409603651670423947251286016`. The underflow case has a positive sign,
which follows from positive axis lengths; a rounded determinant returns zero
there. No Python or Jacobian process is required to run the Rust regression
corpus.

The primitive uses integer shifts of at most 2045 bits and no platform-sized
coordinate casts, map iteration or floating determinant.
