# Integer-power sequence regression evidence

The integer-power adapter uses repeated squaring, rounding each multiplication to
binary64, followed by one reciprocal for negative exponents. This is a sequence
contract, not a claim that the complete mathematical power is correctly rounded
(#201). Changing the multiplication order or taking the reciprocal first can
change results and needs a separate numerical decision.

The ordinary Rust regression `integer_power_matches_independent_operation_sequence`
checks 700 independently derived input/exponent/result records. Expected finite
values were calculated with exact rational arithmetic, not with the adapter or a
floating-point power library. Every multiplication and final reciprocal was
rounded separately as follows:

1. Decode the input sign, integer significand and power of two exactly.
2. Compute each product or quotient as a rational number.
3. Locate its binary exponent with integer bit lengths and an exact comparison.
4. Divide by the representable spacing, using spacing `2^-1074` for subnormals.
5. Round the integer quotient using the exact remainder: below half rounds down,
   above half rounds up, and an exact half selects the even significand.
6. Encode the sign, exponent and significand, including overflow to infinity.

The operation schedule visits absolute-exponent bit positions from least to most
significant, multiplies the accumulator for each set bit, and squares the base
between positions. The absolute value is evaluated without signed-i32 overflow,
so `i32::MIN` is covered. The accumulator starts at positive one. Only after this
sequence does a negative exponent apply its reciprocal.

## Cases and exceptional values

The first 400 records are a Cartesian product of 20 input words and 20 exponents.
They include both zeros, minimum subnormals, the normal/subnormal boundary,
neighbors of one, negative one, powers of two, largest finite values, infinities,
and quiet/signaling NaNs. Exponents cover both i32 endpoints, negative and positive
powers around overflow/underflow, zero, and odd/even powers.

The remaining 300 inputs have exponent field `0x3fe` and mantissas from a fixed
64-bit linear congruential sequence: seed `0x243f6a8885a308d3`, multiplier
`6364136223846793005`, increment `1442695040888963407`, modulo `2^64`. Their
exponents are `index % 31 - 15`. Each text record stores input bits, exponent and
expected bits, or the explicit class `nan`. There are 76 NaN-class records.

All non-NaN results are compared bit for bit, including signed zeros and infinities.
NaN payload, sign, and signaling exception flags are not part of this test's
contract. Exponent zero yields positive one even for NaN input. Negative odd powers
of negative zero yield negative infinity.

For example, `powi(2, -1074)` yields positive zero: the intermediate positive power
overflows before inversion. The exact full power would instead be the smallest
positive subnormal. `powi(2, -1024)` likewise yields zero under this sequence.
The test pins these intentional distinctions without using compiler-folded powers
as expected results.

Run the regression with `cargo test -p martensite-cmath powi_sequence`.
