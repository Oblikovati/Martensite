// SPDX-License-Identifier: GPL-2.0-only
//! Conditioned world-+Z carrier separation of decoded positive-rational ruled faces.
//!
//! A boundary strictly on one side of the analytic cone or rim plane proves
//! separation from every trim of that carrier. This certificate has no authority
//! over native face identity, trim coverage, intersection among tool faces, or sewing.
use super::cylindrical_profile::{CylindricalProfileFailure, trigonometric};
use crate::predicates::roots::certificate_arithmetic::bounded_dyadic::{
    Arithmetic, ArithmeticPolicy,
};
use crate::predicates::roots::{DyadicBound, metric_bounds::BoundedArithmeticFailure};

mod admission;
mod dyadic;
mod interval;
mod ruled;
mod types;
pub use types::{
    CapSeparation, CarrierSeparationQuery, CarrierSeparationReceipt, DecodedCapTrim,
    DecodedCoaxialLocation, DecodedConeFrame, DecodedRimPlane, DecodedRuledSide, Patch,
    SeparationCause, SeparationError, SeparationOptions, SeparationSense, SeparationWork,
    SideSeparation,
};
#[cfg(test)]
mod tests;
use interval::Enclosure;

const MAX_PATCHES: usize = 4096;
const MAX_POLES: usize = 17;
const MAX_ABS_SCALAR: f64 = 4_294_967_296.;
const MIN_WEIGHT: f64 = 1. / MAX_ABS_SCALAR;
const GRID: u16 = 128;

/// Certify the supplied decoded controls, with no evidence on incomplete work.
///
/// The caller must establish the native source identities, the complete admitted
/// face cover, the order of the side rows by depth, and the enclosed cap trims.
/// The interval filter assumes IEEE binary64 with gradual underflow.
/// Example: `let proof = certify_separation(query, || false)?;`.
pub fn certify_separation(
    query: CarrierSeparationQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CarrierSeparationReceipt, SeparationError> {
    let policy = ArithmeticPolicy {
        fractional_bits: GRID,
        max_operations: query.options.max_dyadic_operations,
    };
    let mut checker = Checker {
        query,
        work: SeparationWork::default(),
        arithmetic: Arithmetic::new(policy, &mut cancelled),
    };
    checker.execute()
}

struct Checker<'a, 'c> {
    query: CarrierSeparationQuery<'a>,
    work: SeparationWork,
    arithmetic: Arithmetic<'c>,
}

impl Checker<'_, '_> {
    fn execute(&mut self) -> Result<CarrierSeparationReceipt, SeparationError> {
        self.poll(Patch::Stock)?;
        self.at(Patch::Stock, admission::admit_family(self.query))?;
        let result = Trig::checked(self.query.stock.semi_angle, &mut self.arithmetic);
        let trig = self.at(Patch::Stock, result)?;
        self.at(Patch::Stock, support_radius(self.query, &trig))?;
        let sides = self
            .query
            .sides
            .iter()
            .map(|side| self.side(*side, &trig))
            .collect::<Result<_, _>>()?;
        let caps = self
            .query
            .caps
            .iter()
            .map(|cap| self.cap(*cap, &trig))
            .collect::<Result<_, _>>()?;
        let source = admission::source_bits(self.query);
        self.poll(Patch::Stock)?;
        Ok(CarrierSeparationReceipt {
            source,
            sides,
            caps,
            work: self.work(),
        })
    }

    fn side(
        &mut self,
        side: DecodedRuledSide<'_>,
        trig: &Trig,
    ) -> Result<SideSeparation, SeparationError> {
        let patch = Patch::Side(side.id);
        self.poll(patch)?;
        self.at(patch, admission::admit_side(side))?;
        let poles = side.rows[0].len() as u32;
        self.charge(patch, 3 * poles * poles + 2 * poles)?;
        let result = ruled::side(
            side,
            self.query,
            trig,
            &mut self.arithmetic,
            &mut self.work.exact_sides,
        );
        self.at(patch, result)
    }

    fn cap(&mut self, cap: DecodedCapTrim, trig: &Trig) -> Result<CapSeparation, SeparationError> {
        let patch = Patch::Cap(cap.id);
        self.poll(patch)?;
        self.at(patch, admission::admit_cap(cap, self.query.stock))?;
        self.charge(patch, 2)?;
        self.at(patch, ruled::cap(cap, self.query, trig))
    }

    fn charge(&mut self, patch: Patch, steps: u32) -> Result<(), SeparationError> {
        // The bounded number and size of patches make the addition fit u32.
        let required = self.work.scalar_steps + steps;
        let limit = self.query.options.max_scalar_steps;
        if required > limit {
            return self.at(
                patch,
                Err(SeparationCause::ScalarWorkLimit { required, limit }),
            );
        }
        self.work.scalar_steps = required;
        Ok(())
    }

    fn poll(&mut self, patch: Patch) -> Result<(), SeparationError> {
        let result = self.arithmetic.check_cancelled();
        self.at(patch, result.map_err(Into::into))
    }

    fn at<T>(&self, patch: Patch, error: Result<T, SeparationCause>) -> Result<T, SeparationError> {
        error.map_err(|cause| SeparationError {
            patch,
            cause,
            work: self.work(),
        })
    }

    fn work(&self) -> SeparationWork {
        SeparationWork {
            dyadic_operations: self.arithmetic.operations(),
            ..self.work
        }
    }
}

struct Trig {
    sine: Enclosure,
    cosine: Enclosure,
    exact: [super::super::certificate_arithmetic::bounded_dyadic::Interval; 2],
}

impl Trig {
    fn checked(alpha: f64, arithmetic: &mut Arithmetic<'_>) -> Result<Self, SeparationCause> {
        let angle = arithmetic.scalar(alpha)?;
        let exact = trigonometric::scalar(&angle, arithmetic).map_err(|cause| match cause {
            CylindricalProfileFailure::Arithmetic(cause) => SeparationCause::Arithmetic(*cause),
            cause => SeparationCause::Trigonometric(Box::new(cause)),
        })?;
        let cosine = outward(&exact[0])?;
        let sine = outward(&exact[1])?;
        if cosine.lo <= 0. || sine.lo <= 0. {
            return Err(SeparationCause::InvalidAngle { angle: alpha });
        }
        Ok(Self {
            sine,
            cosine,
            exact,
        })
    }
}

fn outward(
    bounds: &super::super::certificate_arithmetic::bounded_dyadic::Interval,
) -> Result<Enclosure, SeparationCause> {
    let bound = |endpoint: &num_bigint::BigInt, i| {
        let bound = DyadicBound::new(endpoint.clone(), -i32::from(GRID));
        bound
            .binary64_bounds()
            .map(|values| values[i])
            .ok_or(BoundedArithmeticFailure::UnrepresentableBound { bound })
    };
    Ok(Enclosure {
        lo: bound(&bounds.lower, 0)?,
        hi: bound(&bounds.upper, 1)?,
    })
}

fn support_radius(query: CarrierSeparationQuery<'_>, trig: &Trig) -> Result<(), SeparationCause> {
    let delta =
        Enclosure::point(query.rims[0].elevation).subtract(Enclosure::point(query.stock.origin[2]));
    let radius = Enclosure::point(query.stock.radius).multiply(trig.cosine);
    let at_rim = radius.add(delta.multiply(trig.sine));
    if at_rim.lo > 0. {
        Ok(())
    } else {
        Err(SeparationCause::PositiveRadius { lower: at_rim.lo })
    }
}
