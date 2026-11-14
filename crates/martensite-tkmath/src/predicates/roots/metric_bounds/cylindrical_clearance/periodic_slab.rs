// SPDX-License-Identifier: GPL-2.0-only
//! One developed slab separated from every nonzero true cylindrical translate.
use super::*;
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::{bounded_failure::BoundedArithmeticFailure, circular_constants},
};

/// An enclosing source slab separated from every nonzero multiple of the true circumference.
/// Material enclosed by these boundaries lies in this slab as well; arbitrary wider selections
/// require a separate periodic classification. No floating-point period is treated as exact.
#[derive(Clone, Debug)]
pub struct CertifiedCylindricalPeriodicSlab {
    source: CertifiedCylindricalClearance,
    width_upper: DyadicBound,
    circumference: [DyadicBound; 2],
    separation_lower: DyadicBound,
    operations: u32,
}
impl CertifiedCylindricalPeriodicSlab {
    /// Conservative developed source width; `proof.width_upper_bound()`.
    pub fn width_upper_bound(&self) -> &DyadicBound {
        &self.width_upper
    }
    /// Outward true `2πR` enclosure; `proof.circumference_bounds()`.
    pub fn circumference_bounds(&self) -> &[DyadicBound; 2] {
        &self.circumference
    }
    /// Lower separation for all nonzero periodic slab images; `proof.separation_lower_bound()`.
    pub fn separation_lower_bound(&self) -> &DyadicBound {
        &self.separation_lower
    }
    /// Declared physical margin without widening; `proof.requested_clearance()`.
    pub fn requested_clearance(&self) -> f64 {
        self.source.clearance_lower_bound()
    }
    /// Charged operations for this additional proof; `proof.operations()`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
    /// Bind to the same original inputs and retained O2 chart; `proof.matches_profiles(&sources)`.
    pub fn matches_profiles(&self, sources: &[CylindricalProfileSource<'_>]) -> bool {
        self.source.matches_profiles(sources)
    }
}

impl CertifiedCylindricalClearance {
    /// Prove every nonzero periodic source-slab image is separated by the declared margin.
    /// Uses a fresh allowance with the recorded arithmetic policy; wide slabs refuse rather
    /// than claiming collision. Example: `let periodic = clearance.certify_periodic_slab(|| false)?;`.
    pub fn certify_periodic_slab(
        &self,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<CertifiedCylindricalPeriodicSlab, CylindricalClearanceError> {
        periodic(self, &mut cancelled).map_err(|cause| CylindricalClearanceError {
            requested_clearance: self.clearance,
            requested_options: self.options,
            source_count: self.sources.len(),
            cause,
        })
    }
}

fn periodic(
    source: &CertifiedCylindricalClearance,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalPeriodicSlab> {
    let policy = ArithmeticPolicy {
        fractional_bits: source.options.fractional_bits,
        max_operations: source.options.max_operations,
    };
    let mut arithmetic = Arithmetic::new(policy, cancelled);
    let width = source_width(&source.sources, &mut arithmetic)?;
    let period = circumference(source.radius, &mut arithmetic)?;
    let gap = arithmetic.subtract(&period, &width)?;
    let required = arithmetic.scalar(source.clearance)?;
    require_gap(&gap, &required, &arithmetic)?;
    Ok(assemble(source, width, period, gap, &arithmetic))
}

pub(super) fn circumference(radius: f64, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let pi = circular_constants::pi(arithmetic)?;
    let radius = arithmetic.scalar(radius)?;
    let diameter = arithmetic.fraction(&radius, 2, 1)?;
    Ok(arithmetic.multiply(&pi, &diameter)?)
}

pub(super) fn source_width(sources: &[Vec<u64>], arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let mut range: Option<Interval> = None;
    for source in sources {
        let poles = restricted_source(source, arithmetic)?;
        for pole in poles {
            range = Some(include_range(range, pole));
        }
    }
    let range = range.ok_or(CylindricalClearanceFailure::InputCount {
        field: "periodic slab sources",
        actual: 0,
        maximum: 16_384,
    })?;
    let width = arithmetic.subtract(&range, &range)?.upper;
    Ok(Interval {
        lower: width.clone(),
        upper: width,
    })
}

fn include_range(range: Option<Interval>, pole: Interval) -> Interval {
    match range {
        None => pole,
        Some(old) => Interval {
            lower: old.lower.min(pole.lower),
            upper: old.upper.max(pole.upper),
        },
    }
}

fn restricted_source(source: &[u64], arithmetic: &mut Arithmetic<'_>) -> Checked<Vec<Interval>> {
    // Only immutable snapshots assembled after admission enter here: interval, count, XY bits.
    let poles = source[3..]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|point| arithmetic.scalar(f64::from_bits(point[0])))
        .collect::<Result<Vec<_>, BoundedArithmeticFailure>>()?;
    Ok(curve::restrict(
        &poles,
        source[0],
        source[1] as u8,
        arithmetic,
    )?)
}

fn require_gap(gap: &Interval, required: &Interval, arithmetic: &Arithmetic<'_>) -> Checked<()> {
    if gap.lower < required.upper {
        return Err(CylindricalClearanceFailure::PeriodicSlabUnresolved {
            separation_lower: bound(gap.lower.clone(), arithmetic),
        });
    }
    Ok(())
}

fn bound(integer: num_bigint::BigInt, arithmetic: &Arithmetic<'_>) -> DyadicBound {
    DyadicBound::new(integer, -i32::from(arithmetic.fractional_bits()))
}

fn assemble(
    source: &CertifiedCylindricalClearance,
    width: Interval,
    period: Interval,
    gap: Interval,
    arithmetic: &Arithmetic<'_>,
) -> CertifiedCylindricalPeriodicSlab {
    CertifiedCylindricalPeriodicSlab {
        source: source.clone(),
        width_upper: bound(width.upper, arithmetic),
        circumference: [
            bound(period.lower, arithmetic),
            bound(period.upper, arithmetic),
        ],
        separation_lower: bound(gap.lower, arithmetic),
        operations: arithmetic.operations(),
    }
}
