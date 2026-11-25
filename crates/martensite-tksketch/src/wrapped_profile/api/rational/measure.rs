//! Transfer exact authored area and bounded boundary displacements to signed sweep volume.
use super::*;
use crate::surface_sketch::{CylinderSupport, NormalSense};
use martensite_tkmath::predicates::roots::DyadicBound;

type Range = WrappedCertifiedRange;

pub(super) fn cylinder(area: Range, support: CylinderSupport, depths: [f64; 2]) -> Checked<Range> {
    let (delta, sum) = intervals(depths);
    let midpoint_over_radius = divide(multiply(sum, exact(0.5))?, support.radius);
    let scale = match support.normal {
        NormalSense::Outward => add(exact(1.), midpoint_over_radius),
        NormalSense::Inward => subtract(exact(1.), midpoint_over_radius),
    };
    positive(multiply(multiply(area, delta)?, scale)?)
}

pub(super) fn cone(
    area: Range,
    retained: &WrappedConeNormalResult,
    region: SurfaceRegionId,
    inverse_error: f64,
) -> Checked<Range> {
    let depths = retained.depths();
    let (delta, sum) = intervals(depths);
    let limit = retained.proof_options().authored_volume.enclosure_width;
    let volume = if depths[0] == -depths[1] {
        multiply(area, delta)?
    } else {
        let proxy = proxy_volume_area(retained, region)?;
        let coefficient = relative_coefficient(retained.support().semi_angle, delta, sum);
        transfer(area, proxy, delta, (coefficient * inverse_error).next_up())?
    };
    enforce_width(positive(volume)?, limit)
}

fn enforce_width(volume: Range, limit: f64) -> Checked<Range> {
    let width = (volume.upper - volume.lower).next_up();
    if !limit.is_finite() || width > limit {
        return Err(invalid(
            format!("authored volume width={width}, requested={limit}"),
            "authored volume uncertainty within the requested cone enclosure width",
        ));
    }
    Ok(volume)
}

fn proxy_volume_area(
    retained: &WrappedConeNormalResult,
    region: SurfaceRegionId,
) -> Checked<[Range; 2]> {
    let proof = retained.authored_receipt(region).ok_or_else(|| {
        invalid(
            format!("retained cone region={region:?}"),
            "certified reference material area and volume",
        )
    })?;
    Ok([outward(proof.volume())?, outward(proof.area())?])
}

fn outward([low, high]: [DyadicBound; 2]) -> Checked<Range> {
    let bounds = low.binary64_bounds().zip(high.binary64_bounds());
    let Some(([lower, _], [_, upper])) = bounds else {
        return Err(invalid(
            format!("dyadic enclosure={low}..{high}"),
            "finite binary64 bounds",
        ));
    };
    Ok(Range { lower, upper })
}

fn relative_coefficient(angle: f64, delta: Range, sum: Range) -> f64 {
    // The exact coefficient is |delta * sum * cot(angle) / 2|.
    // On the acute interval cos(angle) <= 1 and sin(angle) >= angle/2.
    let max_sum = sum.lower.abs().max(sum.upper.abs());
    ((delta.upper * max_sum).next_up() / angle).next_up()
}

fn transfer(
    area: Range,
    [volume, area_proxy]: [Range; 2],
    delta: Range,
    error: f64,
) -> Checked<Range> {
    let corrected = add(volume, multiply(subtract(area, area_proxy), delta)?);
    Ok(add(
        corrected,
        Range {
            lower: -error,
            upper: error,
        },
    ))
}

pub(super) fn inverse_slant_error(slanted_radius: f64, spans: &[RationalReferenceSpan]) -> f64 {
    // Stokes gives the difference of the two boundary integrals over the
    // straight homotopy. The two-form is dx ∧ dy / |p|, with |p| at least
    // slanted_radius throughout. The unchanged edges contribute zero.
    spans
        .iter()
        .map(|span| {
            let speed = (maximum_speed(&span.poles) + span.bounds.derivative).next_up();
            ((speed * span.bounds.position).next_up() / slanted_radius).next_up()
        })
        .fold(0., |total, term| (total + term).next_up())
}

fn maximum_speed(poles: &[[f64; 2]; 4]) -> f64 {
    let upper = poles
        .windows(2)
        .map(|window| {
            let [a, b] = [window[0], window[1]];
            let [dx, dy] = std::array::from_fn(|axis| (b[axis] - a[axis]).abs().next_up());
            (dx + dy).next_up()
        })
        .fold(0., f64::max);
    (upper * 3.).next_up()
}

fn intervals([first, last]: [f64; 2]) -> (Range, Range) {
    (
        subtract(exact(last), exact(first)),
        add(exact(last), exact(first)),
    )
}

fn positive(volume: Range) -> Checked<Range> {
    if volume.lower > 0. && volume.upper.is_finite() && volume.upper >= volume.lower {
        return Ok(volume);
    }
    Err(invalid(
        format!("authored normal volume={volume:?}"),
        "positive finite ordered bounds",
    ))
}

fn exact(value: f64) -> Range {
    Range {
        lower: value,
        upper: value,
    }
}

fn add(a: Range, b: Range) -> Range {
    Range {
        lower: (a.lower + b.lower).next_down(),
        upper: (a.upper + b.upper).next_up(),
    }
}

fn subtract(a: Range, b: Range) -> Range {
    add(
        a,
        Range {
            lower: -b.upper,
            upper: -b.lower,
        },
    )
}

fn divide(a: Range, divisor: f64) -> Range {
    Range {
        lower: (a.lower / divisor).next_down(),
        upper: (a.upper / divisor).next_up(),
    }
}

fn multiply(a: Range, b: Range) -> Checked<Range> {
    let endpoints = [
        a.lower * b.lower,
        a.lower * b.upper,
        a.upper * b.lower,
        a.upper * b.upper,
    ];
    if !endpoints.into_iter().all(f64::is_finite) {
        return Err(invalid(
            format!("volume factors={a:?} and {b:?}"),
            "finite interval products",
        ));
    }
    let lower = endpoints
        .into_iter()
        .fold(f64::INFINITY, f64::min)
        .next_down();
    let upper = endpoints
        .into_iter()
        .fold(f64::NEG_INFINITY, f64::max)
        .next_up();
    Ok(Range { lower, upper })
}
