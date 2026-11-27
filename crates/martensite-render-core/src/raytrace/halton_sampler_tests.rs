use super::*;

/// Radical inverse of `index` in `base` over `digits` digits with digit map `permutation`,
/// in exact rational arithmetic (independent of the table and float steps above).
fn radical_inverse(base: u64, digits: u32, index: u64, permutation: &[u64]) -> f64 {
    let (mut index, mut value, mut scale) = (index, 0u64, 1u64);
    for _ in 0..digits {
        value = value * base + permutation[(index % base) as usize];
        index /= base;
        scale *= base;
    }
    value as f64 / scale as f64
}

#[test]
fn base_two_is_the_van_der_corput_sequence() {
    let sampler = RaytraceHaltonSampler::new();
    let expected = [0., 0.5, 0.25, 0.75, 0.125, 0.625, 0.375, 0.875];
    for (index, value) in expected.iter().enumerate() {
        assert_eq!(
            sampler.sample(0, index as u32),
            *value as f32,
            "index {index}"
        );
    }
    // 23 mantissa bits: the 24th reversed bit is dropped.
    assert_eq!(sampler.sample(0, 1 << 23), 0.);
}

#[test]
fn faure_permutations_of_four_and_five_follow_the_construction() {
    let permutations = faure_permutations();
    assert_eq!(permutations[3], [0, 1, 2]);
    assert_eq!(permutations[4], [0, 2, 1, 3]);
    assert_eq!(permutations[5], [0, 3, 2, 1, 4]);
}

#[test]
fn bases_three_and_five_are_scrambled_radical_inverses_within_float_rounding() {
    let sampler = RaytraceHaltonSampler::new();
    let identity3 = [0, 1, 2];
    let faure5 = [0, 3, 2, 1, 4];
    for index in [0u32, 1, 2, 3, 4, 5, 26, 242, 243, 1000, 59_049, 123_456_789] {
        // Each group of 5 (base 3) or 3 (base 5) digits is reversed separately; the four groups
        // span 20 base-3 and 12 base-5 digits, the full radical inverse of a 32-bit index.
        let three = radical_inverse(3, 20, u64::from(index), &identity3);
        let five = radical_inverse(5, 12, u64::from(index), &faure5);
        assert!(
            (f64::from(sampler.sample(1, index)) - three).abs() < 1e-6,
            "{index}"
        );
        assert!(
            (f64::from(sampler.sample(2, index)) - five).abs() < 1e-6,
            "{index}"
        );
        assert!(sampler.sample(1, index) < 1. && sampler.sample(2, index) < 1.);
    }
    assert_eq!(sampler.sample(3, 7), 0.);
    assert_eq!(RaytraceHaltonSampler::DIMENSIONS, 3);
}
