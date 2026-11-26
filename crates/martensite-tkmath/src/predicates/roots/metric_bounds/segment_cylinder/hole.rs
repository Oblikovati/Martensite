// SPDX-License-Identifier: GPL-2.0-only
//! Exact axial signs and certified angular signs on original algebraic roots.
use super::{
    CylinderSegmentError as Error, CylinderWallPlace, LocalSegment, Ratio, poll, root::Root,
};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::bounded_dyadic::{Arithmetic, ArithmeticPolicy, Interval},
    metric_bounds::cylindrical_profile::trigonometric,
};
use std::{cmp::Ordering, f64::consts::TAU};

#[derive(Clone, Debug)]
pub(super) struct Hole {
    pub(super) axial: [Ratio; 2],
    rays: [Ray; 2],
}

impl Hole {
    pub(super) fn capture(
        angular: [f64; 2],
        axial: [f64; 2],
        outer: &[Ratio; 2],
        cancelled: &mut impl FnMut() -> bool,
    ) -> Result<Self, Error> {
        let [u0, u1] = angular;
        let [v0, v1] = axial;
        if !(0. < u0 && u0 < u1 && u1 < TAU && v0.is_finite() && v1.is_finite() && v0 < v1) {
            return Err(Error::HoleBounds { angular, axial });
        }
        let axial_exact = axial.map(Ratio::from_finite);
        if axial_exact[0] <= outer[0] || axial_exact[1] >= outer[1] {
            return Err(Error::HoleBounds { angular, axial });
        }
        Ok(Self {
            axial: axial_exact,
            rays: [Ray::capture(u0, cancelled)?, Ray::capture(u1, cancelled)?],
        })
    }

    pub(super) fn classify(
        &self,
        line: &LocalSegment,
        root: &Root,
    ) -> Result<CylinderWallPlace, Error> {
        use CylinderWallPlace as Place;
        let [first, last] = self
            .axial
            .each_ref()
            .map(|bound| root.affine_sign(&line.start[2].subtract(bound), &line.rate[2]));
        if first == Ordering::Less || last == Ordering::Greater {
            return Ok(Place::Interior);
        }
        let [u0, u1] = self.angular(line, root)?;
        if u0 == Ordering::Less || u1 == Ordering::Greater {
            return Ok(Place::Interior);
        }
        if [first, last, u0, u1].contains(&Ordering::Equal) {
            return Ok(Place::HoleBoundary);
        }
        Ok(Place::HoleInterior)
    }

    pub(super) fn angular(&self, line: &LocalSegment, root: &Root) -> Result<[Ordering; 2], Error> {
        Ok([
            self.rays[0].compare(line, root)?,
            self.rays[1].compare(line, root)?,
        ])
    }
}

#[derive(Clone, Debug)]
struct Ray {
    angle: f64,
    trig: [[Ratio; 2]; 2],
    second_half: bool,
}

impl Ray {
    fn capture(angle: f64, cancelled: &mut impl FnMut() -> bool) -> Result<Self, Error> {
        let trig = trig_bounds(angle, cancelled)?;
        let sine = trig[1].each_ref().map(Ratio::sign);
        let second_half = match sine {
            [Ordering::Less, Ordering::Less] => true,
            [Ordering::Greater, Ordering::Greater] => false,
            _ => return Err(Error::AngularSign(angle)),
        };
        Ok(Self {
            angle,
            trig,
            second_half,
        })
    }

    fn compare(&self, line: &LocalSegment, root: &Root) -> Result<Ordering, Error> {
        let xs = root.affine_sign(&line.start[0], &line.rate[0]);
        let ys = root.affine_sign(&line.start[1], &line.rate[1]);
        let second_half = ys == Ordering::Less || (ys == Ordering::Equal && xs == Ordering::Less);
        let halves = second_half.cmp(&self.second_half);
        if halves != Ordering::Equal {
            return Ok(halves);
        }
        self.cross_sign(line, root)
    }

    fn cross_sign(&self, line: &LocalSegment, root: &Root) -> Result<Ordering, Error> {
        let [cosines, sines] = &self.trig;
        let determinant = |xy: &[Ratio; 3], c: &Ratio, s: &Ratio| {
            c.multiply(&xy[1]).subtract(&s.multiply(&xy[0]))
        };
        let mut signs = cosines
            .iter()
            .flat_map(|c| sines.iter().map(move |s| (c, s)));
        let sign = |(c, s)| {
            root.affine_sign(
                &determinant(&line.start, c, s),
                &determinant(&line.rate, c, s),
            )
        };
        let first = signs
            .next()
            .map(sign)
            .ok_or(Error::AngularSign(self.angle))?;
        if first == Ordering::Equal || !signs.all(|corner| sign(corner) == first) {
            return Err(Error::AngularSign(self.angle));
        }
        Ok(first)
    }
}

pub(super) fn trig_bounds(
    angle: f64,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<[[Ratio; 2]; 2], Error> {
    poll(cancelled)?;
    let policy = ArithmeticPolicy::default();
    let mut bounded_call = || false;
    let mut arithmetic = Arithmetic::new(policy, &mut bounded_call);
    let input = arithmetic.scalar(angle).map_err(Error::from_trig)?;
    let result = trigonometric::scalar(&input, &mut arithmetic).map_err(Error::from_trig)?;
    poll(cancelled)?;
    let exponent = -i32::from(policy.fractional_bits);
    Ok(result.map(|Interval { lower, upper }| {
        [lower, upper].map(|n| Ratio::from_dyadic(&DyadicBound::new(n, exponent)))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unresolved_nonzero_ray_is_typed_incomplete_under_the_fixed_grid() {
        let angle = martensite_cmath::powi(2., -500);
        assert!(
            matches!(Ray::capture(angle, &mut || false), Err(Error::AngularSign(value)) if value == angle)
        );
    }

    #[test]
    fn trigonometric_capture_polls_before_and_after_the_bounded_arithmetic() {
        for cutoff in [1, 2] {
            let mut polls = 0;
            let error = Ray::capture(0.5, &mut || {
                polls += 1;
                polls >= cutoff
            })
            .unwrap_err();
            assert_eq!(error, Error::Cancelled);
        }
    }
}
