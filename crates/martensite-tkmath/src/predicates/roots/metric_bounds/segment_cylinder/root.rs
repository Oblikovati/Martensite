// SPDX-License-Identifier: GPL-2.0-only
//! Exact root identities and separately bounded rounding on a directed segment.
use super::{CylinderSegmentError as Error, Ratio, SegmentCylinderCut, poll};
use crate::predicates::roots::{
    certificate_arithmetic::rounding::{nearest_binary64, outward_binary64, outward_sqrt},
    metric_bounds::{placed_plane::ExactVector, segment_plane::enclose_cut},
};
use num_bigint::BigInt;
use std::cmp::Ordering;

/// The root remains an exact algebraic value while all topology is decided.
pub(super) struct Root {
    offset: Ratio,
    factor: Ratio,
    square: Ratio,
}

impl Root {
    pub(super) fn ordered(a: &Ratio, b: &Ratio, discriminant: &Ratio) -> [Self; 2] {
        let inverse = Ratio::from_finite(0.5).divide(a);
        let offset = b.negated().multiply(&inverse);
        [inverse.negated(), inverse].map(|factor| Self {
            offset: offset.clone(),
            factor,
            square: discriminant.clone(),
        })
    }

    pub(super) fn affine_sign(&self, constant: &Ratio, rate: &Ratio) -> Ordering {
        let a = constant.add(&rate.multiply(&self.offset));
        let b = rate.multiply(&self.factor);
        if self.square.sign() == Ordering::Equal || b.sign() == Ordering::Equal {
            return a.sign();
        }
        if a.sign() == Ordering::Equal || a.sign() == b.sign() {
            return b.sign();
        }
        match a.multiply(&a).cmp(&b.multiply(&b).multiply(&self.square)) {
            Ordering::Greater => a.sign(),
            Ordering::Equal => Ordering::Equal,
            Ordering::Less => b.sign(),
        }
    }

    pub(super) fn materialize(
        &self,
        source: [&ExactVector; 2],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<SegmentCylinderCut, Error> {
        poll(cancelled)?;
        if let Some(radical) = exact_square_root(&self.square) {
            let parameter = self.offset.add(&self.factor.multiply(&radical));
            let xyz = point(source, &parameter);
            return enclose_cut(&xyz, &parameter).map_err(|_| Error::PositionEnclosure);
        }
        for bits in [64, 128, 256, 512, 1024, 2048, 4096, 8192] {
            poll(cancelled)?;
            let bounds = self.parameter_bounds(bits);
            let points = bounds.each_ref().map(|t| point(source, t));
            if let Some(cut) = enclose(&bounds, &points) {
                return Ok(cut);
            }
        }
        Err(Error::PositionEnclosure)
    }

    fn parameter_bounds(&self, bits: usize) -> [Ratio; 2] {
        let denominator = BigInt::from(1) << bits;
        let integer = (self.square.numerator() << (2 * bits)) / self.square.denominator();
        let lower = integer.sqrt();
        let mut bounds = [lower.clone(), lower + 1].map(|numerator| {
            let radical = Ratio::new(numerator, denominator.clone());
            self.offset.add(&self.factor.multiply(&radical))
        });
        if self.factor.sign() == Ordering::Less {
            bounds.swap(0, 1);
        }
        bounds
    }
}

fn exact_square_root(value: &Ratio) -> Option<Ratio> {
    let numerator = value.numerator().sqrt();
    let denominator = value.denominator().sqrt();
    if &numerator * &numerator != *value.numerator()
        || &denominator * &denominator != *value.denominator()
    {
        return None;
    }
    Some(Ratio::new(numerator, denominator))
}

fn point([a, b]: [&ExactVector; 2], t: &Ratio) -> ExactVector {
    std::array::from_fn(|i| a[i].add(&b[i].subtract(&a[i]).multiply(t)))
}

fn enclose(parameter: &[Ratio; 2], points: &[ExactVector; 2]) -> Option<SegmentCylinderCut> {
    let parameter = interval([&parameter[0], &parameter[1]])?;
    let mut coordinates = [[0.; 2]; 3];
    let mut position = [0.; 3];
    let mut error = Ratio::from_finite(0.);
    for i in 0..3 {
        let [a, b] = [&points[0][i], &points[1][i]];
        let ordered = [a.min(b), a.max(b)];
        coordinates[i] = interval(ordered)?;
        let (nominal, magnitude) = unique_rounding(ordered)?;
        position[i] = nominal;
        error = error.add(&magnitude.multiply(&magnitude));
    }
    let error_upper = outward_sqrt(&error)?;
    Some(SegmentCylinderCut {
        parameter,
        coordinates,
        position,
        error_upper,
    })
}

fn interval([a, b]: [&Ratio; 2]) -> Option<[f64; 2]> {
    Some([outward_binary64(a)?[0], outward_binary64(b)?[1]])
}

fn unique_rounding([a, b]: [&Ratio; 2]) -> Option<(f64, Ratio)> {
    let lower = nearest_binary64(a)?;
    let upper = nearest_binary64(b)?;
    if lower != upper {
        return None;
    }
    let rounded = Ratio::from_finite(lower);
    let distance = rounded.subtract(a).max(b.subtract(&rounded));
    Some((lower, distance))
}
