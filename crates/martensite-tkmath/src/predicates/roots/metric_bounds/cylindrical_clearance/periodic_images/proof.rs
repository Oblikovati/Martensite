// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use planning::PeriodicPlan;
use types::{PeriodicEnclosure, PeriodicWork};

pub(super) fn execute(
    source: &CertifiedCylindricalClearance,
    maximum: u32,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalPeriodicClearance> {
    let plan = planning::plan(source, maximum, cancelled)?;
    let mut remaining = source.options;
    remaining.max_operations -= plan.operations;
    let images = certify_images(source, &plan.images, &mut remaining, cancelled)?;
    assemble(source, plan, images, remaining)
}

fn certify_images(
    source: &CertifiedCylindricalClearance,
    shifts: &[CylindricalPeriodicTranslation],
    remaining: &mut CylindricalClearanceOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<Vec<CertifiedCylindricalImage>> {
    if shifts.is_empty() {
        return Ok(Vec::new());
    }
    let poles = capture_sources(&source.sources);
    let inputs = bind_sources(source, &poles);
    let mut images = Vec::new();
    for shift in shifts {
        images.push(certify_and_charge(
            source, &inputs, shift, remaining, cancelled,
        )?);
    }
    Ok(images)
}

fn bind_sources<'a>(
    source: &CertifiedCylindricalClearance,
    poles: &'a [Vec<[f64; 2]>],
) -> Vec<CylindricalClearanceSource<'a>> {
    poles
        .iter()
        .zip(&source.intervals)
        .map(|(authored, interval)| CylindricalClearanceSource {
            authored,
            interval: *interval,
        })
        .collect()
}

fn certify_and_charge(
    source: &CertifiedCylindricalClearance,
    inputs: &[CylindricalClearanceSource<'_>],
    shift: &CylindricalPeriodicTranslation,
    remaining: &mut CylindricalClearanceOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalImage> {
    let certificate = certify_image(source, inputs, shift, *remaining, cancelled)?;
    remaining.max_operations -= certificate.operations();
    remaining.max_domains -= certificate.domain_visits();
    remaining.max_curve_nodes -= certificate.curve_nodes();
    Ok(CertifiedCylindricalImage {
        translation: shift.clone(),
        certificate,
    })
}

fn capture_sources(sources: &[Vec<u64>]) -> Vec<Vec<[f64; 2]>> {
    sources
        .iter()
        .map(|source| {
            source[3..]
                .as_chunks::<2>()
                .0
                .iter()
                .map(|point| point.map(f64::from_bits))
                .collect()
        })
        .collect()
}

fn certify_image(
    source: &CertifiedCylindricalClearance,
    inputs: &[CylindricalClearanceSource<'_>],
    shift: &CylindricalPeriodicTranslation,
    options: CylindricalClearanceOptions,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CertifiedCylindricalClearance> {
    let query = image_query(source, inputs, shift, options);
    certify_cylindrical_clearance(query, cancelled).map_err(|cause| {
        CylindricalClearanceFailure::PeriodicImage {
            turn: shift.turn,
            offset: shift.offset,
            cause: Box::new(cause),
        }
    })
}

fn image_query<'a>(
    source: &CertifiedCylindricalClearance,
    inputs: &'a [CylindricalClearanceSource<'a>],
    shift: &CylindricalPeriodicTranslation,
    options: CylindricalClearanceOptions,
) -> CylindricalClearanceQuery<'a> {
    CylindricalClearanceQuery {
        sources: inputs,
        support_radius: source.radius,
        angular_origin: source.origin,
        domain: CylindricalClearanceDomain::Translated {
            offset: [shift.offset, 0.],
        },
        requested_clearance: shift.comparison,
        options,
    }
}

fn assemble(
    source: &CertifiedCylindricalClearance,
    plan: PeriodicPlan,
    images: Vec<CertifiedCylindricalImage>,
    remaining: CylindricalClearanceOptions,
) -> Checked<CertifiedCylindricalPeriodicClearance> {
    let work = work_receipt(source, remaining, plan.maximum);
    let enclosure = enclosure(source, plan, images.is_empty())?;
    Ok(CertifiedCylindricalPeriodicClearance {
        source: source.clone(),
        images,
        enclosure,
        work,
    })
}

fn work_receipt(
    source: &CertifiedCylindricalClearance,
    remaining: CylindricalClearanceOptions,
    image_limit: u32,
) -> PeriodicWork {
    PeriodicWork {
        image_limit,
        operations: source.options.max_operations - remaining.max_operations,
        domains: source.options.max_domains - remaining.max_domains,
        nodes: source.options.max_curve_nodes - remaining.max_curve_nodes,
    }
}

fn enclosure(
    source: &CertifiedCylindricalClearance,
    plan: PeriodicPlan,
    empty: bool,
) -> Checked<PeriodicEnclosure> {
    let bits = source.options.fractional_bits;
    let tail = bound(plan.tail.lower, bits);
    let margin = margin(source.clearance, &tail, empty)?;
    Ok(PeriodicEnclosure {
        width: bound(plan.width.upper, bits),
        period: [
            bound(plan.period.lower, bits),
            bound(plan.period.upper, bits),
        ],
        tail,
        margin,
    })
}

fn margin(clearance: f64, tail: &DyadicBound, empty: bool) -> Checked<DyadicBound> {
    if empty {
        return Ok(tail.clone());
    }
    DyadicBound::from_f64(clearance).map_err(|_| CylindricalClearanceFailure::Value {
        field: "periodic physical clearance",
        index: 0,
        value: clearance,
    })
}
