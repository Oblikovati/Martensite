//! Quantized scalar and component hashes with explicit integer conversion rules.
//!
//! Stable quantization matters because hash values affect container iteration.
use martensite_tkernel::standard::Failure;
use martensite_tkernel::standard::hashing::hash_bytes;

// 2^63: the first double that no longer fits in an int64_t.
const TWO_POW_63: f64 = 9_223_372_036_854_775_808.0;

/// Round to a signed integer, returning `i64::MIN` for NaN or an out-of-range value.
///
/// Explicit range checks preserve the sentinel rather than saturating;
/// `native_rounded_i64(f64::NAN) == i64::MIN`.
pub fn native_rounded_i64(value: f64) -> i64 {
    let rounded: f64 = value.round();
    if rounded.is_nan() || !(-TWO_POW_63..TWO_POW_63).contains(&rounded) {
        return i64::MIN;
    }
    rounded as i64
}

/// Hash the rounded scaled value; `rounded_hash(0.5, 1e7)`.
pub fn rounded_hash(value: f64, factor: f64) -> usize {
    int64_hash(native_rounded_i64(value * factor))
}

/// Hash four native-endian integer bytes; `int_hash(3)`.
pub fn int_hash(value: i32) -> usize {
    hash_bytes(&value.to_ne_bytes())
}

/// Hash eight native-endian integer bytes; `int64_hash(7)`.
pub fn int64_hash(value: i64) -> usize {
    hash_bytes(&value.to_ne_bytes())
}

/// Combine the native-endian words of component hashes;
/// `combined_hash(&[1, 2])`.
pub fn combined_hash(hashes: &[usize]) -> usize {
    let bytes: Vec<u8> = hashes.iter().flat_map(|hash| hash.to_ne_bytes()).collect();
    hash_bytes(&bytes)
}

/// `[leading..., round(value_i / HashTolerance)...]` combined: the shape of every
/// hash that follows a placement or basis hash with scalar parameters;
/// `leading_and_rounded(&[axis_hash], &[radius], 1e-7)`.
pub fn leading_and_rounded(leading: &[usize], values: &[f64], hash_tolerance: f64) -> usize {
    let factor: f64 = 1.0 / hash_tolerance;
    let mut hashes: Vec<usize> = leading.to_vec();
    hashes.extend(values.iter().map(|&value| rounded_hash(value, factor)));
    combined_hash(&hashes)
}

/// Every `(first, second)` pair within `comp_tolerance`, checked in order
/// (`std::abs(a - b) <= CompTolerance && ...`); `values_within(1e-7, &[(1.0, 1.0)])`.
pub fn values_within(comp_tolerance: f64, values: &[(f64, f64)]) -> bool {
    values
        .iter()
        .all(|&(first, second)| (first - second).abs() <= comp_tolerance)
}

/// The pole and weight walk that ends every Bezier and B-spline comparison: the poles
/// at `indices` compared by `equal_poles` in order, then, for rational geometry, the
/// weights within `comp_tolerance`.
///
/// The defined loops read `Pole(i)`/`Weight(i)` of the second object with the first
/// object's count and would raise (inside a `noexcept` operator, terminating the
/// process) when it has fewer poles; the failure is returned so the caller can report
/// the pair as different;
/// `same_poles_and_weights(&[1, 2], false, 1e-12, |a, b| a == b, poles, weights)?`.
pub fn same_poles_and_weights<I: Copy, P>(
    indices: &[I],
    rational: bool,
    comp_tolerance: f64,
    equal_poles: impl Fn(P, P) -> bool,
    pole: [&dyn Fn(I) -> Result<P, Failure>; 2],
    weight: [&dyn Fn(I) -> Result<f64, Failure>; 2],
) -> Result<bool, Failure> {
    for &index in indices {
        if !equal_poles(pole[0](index)?, pole[1](index)?) {
            return Ok(false);
        }
    }
    if !rational {
        return Ok(true);
    }
    for &index in indices {
        if (weight[0](index)? - weight[1](index)?).abs() > comp_tolerance {
            return Ok(false);
        }
    }
    Ok(true)
}

/// The knot value and multiplicity accessors of one knot sequence (`Knot(i)`,
/// `Multiplicity(i)`, or their U/V forms).
///
/// This borrowed callback pair is thread-confined: its erased callbacks do not
/// require `Sync`, so callers must invoke it on the owning thread. Copying the
/// references does not change that contract.
///
/// ```compile_fail,E0277
/// use martensite_tkg2d::geom2d_hash::hash_key::KnotAccess;
/// fn requires_send_sync<T: Send + Sync>() {}
/// requires_send_sync::<KnotAccess<'static>>();
/// ```
pub type KnotAccess<'a> = (
    &'a dyn Fn(i32) -> Result<f64, Failure>,
    &'a dyn Fn(i32) -> Result<i32, Failure>,
);

/// Knots 1..=`count` within `comp_tolerance` with equal multiplicities, knot by knot,
/// as every B-spline comparison checks them; `same_knots(1e-12, 2, [first, second])?`.
pub fn same_knots(
    comp_tolerance: f64,
    count: i32,
    knots: [KnotAccess<'_>; 2],
) -> Result<bool, Failure> {
    let [(knot1, mult1), (knot2, mult2)] = knots;
    for i in 1..=count {
        let knot_gap: f64 = (knot1(i)? - knot2(i)?).abs();
        if knot_gap > comp_tolerance || mult1(i)? != mult2(i)? {
            return Ok(false);
        }
    }
    Ok(true)
}
