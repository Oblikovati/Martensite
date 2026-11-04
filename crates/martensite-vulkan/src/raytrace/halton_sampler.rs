//! GlHaltonSampler: the three-dimensional Halton sequence with Faure
//! scrambling (bases 2, 3 and 5) that the adaptive tile sampler draws its tiles with. Base 2 is
//! the bit-reversed index written into a float mantissa; bases 3 and 5 combine four
//! precomputed permuted digit groups, evaluated in the same 32-bit integer and `f32` steps.

/// Digit groups of base 3 (3^5) and base 5 (5^3).
const BASE3_GROUP: u32 = 243;
const BASE5_GROUP: u32 = 125;

/// The scrambled Halton sequence; `RaytraceHaltonSampler::new().sample(0, 1) == 0.5`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RaytraceHaltonSampler {
    perm3: [u16; BASE3_GROUP as usize],
    perm5: [u16; BASE5_GROUP as usize],
}

impl Default for RaytraceHaltonSampler {
    fn default() -> Self {
        Self::new()
    }
}

impl RaytraceHaltonSampler {
    /// The number of dimensions `sample` serves (GlHaltonSampler::get_num_dimensions).
    pub const DIMENSIONS: u32 = 3;

    /// The sampler with Faure permutations (GlHaltonSampler::initFaure).
    pub fn new() -> Self {
        let permutations = faure_permutations();
        let perm3 = std::array::from_fn(|i| invert(3, 5, i as u16, &permutations[3]));
        let perm5 = std::array::from_fn(|i| invert(5, 3, i as u16, &permutations[5]));
        Self { perm3, perm5 }
    }

    /// Sample `index` in `dimension` (0: base 2, 1: base 3, 2: base 5), in [0, 1); other
    /// dimensions return 0 as natively. Example: `sampler.sample(1, 7)`.
    pub fn sample(&self, dimension: u32, index: u32) -> f32 {
        match dimension {
            0 => halton2(index),
            1 => self.halton3(index),
            2 => self.halton5(index),
            _ => 0.,
        }
    }

    fn halton3(&self, index: u32) -> f32 {
        let digit = |group: u32| u32::from(self.perm3[(group % BASE3_GROUP) as usize]);
        let sum = digit(index) * 14_348_907
            + digit(index / 243) * 59_049
            + digit(index / 59_049) * 243
            + digit(index / 14_348_907);
        sum as f32 * (0.999_999_999_999_999_f64 / 3_486_784_401.) as f32
    }

    fn halton5(&self, index: u32) -> f32 {
        let digit = |group: u32| u32::from(self.perm5[(group % BASE5_GROUP) as usize]);
        let sum = digit(index) * 1_953_125
            + digit(index / 125) * 15_625
            + digit(index / 15_625) * 125
            + digit(index / 1_953_125);
        sum as f32 * (0.999_999_999_999_999_f64 / 244_140_625.) as f32
    }
}

/// The bit-reversed index as the mantissa of a float in [1, 2), minus one.
fn halton2(index: u32) -> f32 {
    f32::from_bits(0x3f80_0000 | (index.reverse_bits() >> 9)) - 1.
}

/// `digits` base-`base` digits of `index`, permuted and reversed.
fn invert(base: u16, digits: u16, mut index: u16, permutation: &[u16]) -> u16 {
    let mut result = 0;
    for _ in 0..digits {
        result = result * base + permutation[(index % base) as usize];
        index /= base;
    }
    result
}

/// Faure's permutations of bases 1 to 5: identity up to 3, then built from the smaller ones.
fn faure_permutations() -> Vec<Vec<u16>> {
    let mut permutations: Vec<Vec<u16>> = (0..=3u16).map(|base| (0..base).collect()).collect();
    for base in 4..=5u16 {
        let half = base / 2;
        let mut next = vec![0u16; base as usize];
        if base % 2 == 1 {
            for (i, &value) in permutations[base as usize - 1].iter().enumerate() {
                let i = i as u16;
                next[(i + u16::from(i >= half)) as usize] = value + u16::from(value >= half);
            }
            next[half as usize] = half;
        } else {
            for (i, &value) in permutations[half as usize].iter().enumerate() {
                next[i] = 2 * value;
                next[half as usize + i] = 2 * value + 1;
            }
        }
        permutations.push(next);
    }
    permutations
}

#[cfg(test)]
#[path = "halton_sampler_tests.rs"]
mod tests;
