// SPDX-License-Identifier: GPL-2.0-only
//! Carriers and rational rows give a boundary correspondence with the exact cap reference.
use super::*;

/// Filled reference measures and certified correspondence to the decoded boundary patches.
#[derive(Debug)]
pub struct ConicalDecodedCapEvidence {
    normal: ConicalCapNormalEvidence,
    caps: [DyadicBound; 2],
    ruled_side: DyadicBound,
    composed: DyadicBound,
    operations: u32,
}
impl ConicalDecodedCapEvidence {
    /// Exact normal-reference measurements. Read them with `let proof = evidence.normal();`.
    pub fn normal(&self) -> &ConicalCapNormalEvidence {
        &self.normal
    }

    /// Model-unit bounds for both decoded cap carriers, in depth order.
    /// Read them with `let [first, last] = evidence.cap_errors();`.
    pub fn cap_errors(&self) -> &[DyadicBound; 2] {
        &self.caps
    }

    /// Model-unit bound over every decoded common-weight side ruling.
    /// Read it with `let error = evidence.ruled_side_error();`.
    pub fn ruled_side_error(&self) -> &DyadicBound {
        &self.ruled_side
    }

    /// Sum of the filled-reference error and the largest decoded boundary bound.
    /// Read it with `let error = evidence.composed_error();`.
    pub fn composed_error(&self) -> &DyadicBound {
        &self.composed
    }

    /// Complete arithmetic, including the normal receipt's operations.
    /// Charge it once with `let spent = evidence.operations();`.
    pub fn operations(&self) -> u32 {
        self.operations
    }
}
impl ConicalCapHomotopyEvidence<'_> {
    /// Measure the exact cap reference and its correspondence to decoded boundary patches.
    /// The caller binds the canonical +Z cap frames, filled material and complete affine side rulings.
    /// Use `let evidence = proof.certify_decoded_reference(epsilon, options, || false)?;`.
    pub fn certify_decoded_reference(
        &self,
        spatial_limit: f64,
        options: ConeClearanceOptions,
        mut cancelled: impl FnMut() -> bool + Send,
    ) -> Result<ConicalDecodedCapEvidence, ConicalCapNormalError> {
        admit_limit(spatial_limit)?;
        let mut arithmetic = volume_arithmetic(options, &mut cancelled)?;
        let measures = self.normal_measures(spatial_limit, &mut arithmetic)?;
        let normal_operations = arithmetic.operations();
        let bounds = calculate(self.cap, &measures, spatial_limit, &mut arithmetic)
            .map_err(|cause| spatial_error(cause, arithmetic.operations()))?;
        Ok(bounds.evidence(measures, spatial_limit, normal_operations, &arithmetic))
    }
}

struct DecodedBounds {
    caps: [Interval; 2],
    ruled_side: Interval,
    composed: Interval,
}
impl DecodedBounds {
    fn evidence(
        self,
        measures: NormalMeasures,
        spatial_limit: f64,
        normal_operations: u32,
        arithmetic: &Arithmetic<'_>,
    ) -> ConicalDecodedCapEvidence {
        let to_bound = |interval: Interval| {
            DyadicBound::new(interval.upper, -i32::from(arithmetic.fractional_bits()))
        };
        let normal = measures.evidence(spatial_limit, arithmetic, normal_operations);
        ConicalDecodedCapEvidence {
            normal,
            caps: self.caps.map(to_bound),
            ruled_side: to_bound(self.ruled_side),
            composed: to_bound(self.composed),
            operations: arithmetic.operations(),
        }
    }
}
fn calculate(
    cap: &CapBinding,
    measures: &NormalMeasures,
    limit: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> SpatialResult<DecodedBounds> {
    let caps = [
        cap_error(cap, 0, &measures.meridian, arithmetic)?,
        cap_error(cap, 1, &measures.meridian, arithmetic)?,
    ];
    let cap_upper = (&caps[0].upper).max(&caps[1].upper).clone();
    let ruled_side = side_error(cap.trace_position_upper, cap_upper, arithmetic)?;
    // The ruled bound includes the maximum cap datum discrepancy.
    let composed = arithmetic.add(&measures.displacement, &ruled_side)?;
    reference::geometry::allocated(
        &composed,
        limit,
        1,
        "composed decoded cap spatial error",
        arithmetic,
    )?;
    Ok(DecodedBounds {
        caps,
        ruled_side,
        composed,
    })
}

fn cap_error(
    cap: &CapBinding,
    end: usize,
    meridian: &Meridian,
    arithmetic: &mut Arithmetic<'_>,
) -> SpatialResult<Interval> {
    let [radius, z, angle] = cap.carriers[end];
    if angle != cap.support[0] {
        return Err(ConicalReferenceFailure::Binding {
            field: "decoded cap semi-angle",
        });
    }
    let depth = f64::from_bits(cap.support[2 + end]);
    let signed_depth = if cap.support[4] == 0 { -depth } else { depth };
    let exact = exact_cap_datum(signed_depth, meridian, arithmetic)?;
    let radial = difference(f64::from_bits(radius), &exact[0], arithmetic)?;
    let axial = difference(f64::from_bits(z), &exact[1], arithmetic)?;
    // The common angle and slant terms cancel throughout the trimmed domain.
    Ok(arithmetic.add(&radial, &axial)?)
}

fn exact_cap_datum(
    signed_depth: f64,
    meridian: &Meridian,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Interval; 2]> {
    let depth = arithmetic.scalar(signed_depth)?;
    let dr = arithmetic.multiply(&depth, &meridian.cosine)?;
    let dz = arithmetic.multiply(&depth, &meridian.sine)?;
    let axial = arithmetic.multiply(&meridian.reference_slant, &meridian.cosine)?;
    Ok([
        arithmetic.add(&meridian.reference_radius, &dr)?,
        arithmetic.subtract(&axial, &dz)?,
    ])
}

fn difference(actual: f64, exact: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let actual = arithmetic.scalar(actual)?;
    let difference = arithmetic.subtract(&actual, exact)?;
    Ok(Interval {
        lower: 0.into(),
        upper: difference.magnitude(),
    })
}
fn side_error(
    trace_limit: f64,
    cap_upper: num_bigint::BigInt,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let trace = arithmetic.scalar(trace_limit)?;
    let cap = Interval {
        lower: 0.into(),
        upper: cap_upper,
    };
    // The paired trace proof already accounts for decoded edge storage. The largest
    // endpoint bound covers each affine interpolation through its common-weight side.
    arithmetic.add(&trace, &cap)
}

#[cfg(test)]
mod tests;
