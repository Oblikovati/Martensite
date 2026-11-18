//! Construct candidate cone fits; bounded certification remains mandatory.
use super::ConeFit;
use martensite_cmath::{atan2, cos, hypot, sin, tan};
use std::f64::consts::{FRAC_PI_2, TAU};

pub(in super::super) struct FitInput<'a> {
    pub source: &'a [[f64; 2]],
    pub semi_angle: f64,
    pub origin: f64,
    pub center: f64,
    pub developed_turns: [i32; 2],
    pub physical_turns: [i32; 2],
    pub outward: bool,
}
#[derive(Debug)]
pub(in super::super) enum FitFailure {
    Count(usize),
    Nonfinite {
        role: &'static str,
        index: usize,
        value: f64,
    },
    Support(f64),
    Apex {
        endpoint: usize,
    },
    Chart {
        endpoint: usize,
        half_angle: f64,
    },
}
impl std::fmt::Display for FitFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Count(count) => write!(f, "source count={count}; expected 2..=4"),
            Self::Nonfinite { role, index, value } => {
                write!(f, "{role}[{index}]={value}; expected finite")
            }
            Self::Support(value) => write!(f, "semi-angle={value}; expected finite in (0, pi/2)"),
            Self::Apex { endpoint } => write!(
                f,
                "endpoint={endpoint} is at the apex; expected positive slant"
            ),
            Self::Chart {
                endpoint,
                half_angle,
            } => chart_failure(f, *endpoint, *half_angle),
        }
    }
}
fn chart_failure(
    f: &mut std::fmt::Formatter<'_>,
    endpoint: usize,
    half_angle: f64,
) -> std::fmt::Result {
    write!(
        f,
        "endpoint={endpoint}, half-angle={half_angle}; expected inside (-pi/2, pi/2)"
    )
}
#[derive(Clone, Copy)]
pub(in super::super) struct OwnedFit {
    tangent: [f64; 4],
    slant: [f64; 4],
    semi_angle: f64,
    center: f64,
    outward: bool,
}
impl OwnedFit {
    pub(in super::super) fn view(&self) -> ConeFit<'_> {
        ConeFit {
            tangent: &self.tangent,
            slant_numerator: &self.slant,
            slant_weights: &[1.; 4],
            semi_angle: self.semi_angle,
            chart_center: self.center,
            outward: self.outward,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in super::super) struct IdealEndpoint {
    pub tangent: f64,
    pub slant: f64,
}
struct Endpoint {
    slant: f64,
    slant_rate: f64,
    tangent: f64,
    tangent_rate: f64,
}
pub(in super::super) fn fit_span(
    input: &FitInput<'_>,
    shared: &[IdealEndpoint; 2],
) -> Result<OwnedFit, FitFailure> {
    admit(input)?;
    let sine = sin(input.semi_angle);
    let first = endpoint(input, 0, sine, shared[0])?;
    let last = endpoint(input, 1, sine, shared[1])?;
    let (tangent, slant) = interpolate(&first, &last);
    finite("tangent coefficients", &tangent)?;
    finite("slant coefficients", &slant)?;
    Ok(OwnedFit {
        tangent,
        slant,
        semi_angle: input.semi_angle,
        center: input.center,
        outward: input.outward,
    })
}
fn interpolate(first: &Endpoint, last: &Endpoint) -> ([f64; 4], [f64; 4]) {
    let tangent = hermite(
        first.tangent,
        first.tangent_rate,
        last.tangent,
        last.tangent_rate,
    );
    let slant = hermite(first.slant, first.slant_rate, last.slant, last.slant_rate);
    (tangent, slant)
}

fn admit(input: &FitInput<'_>) -> Result<(), FitFailure> {
    if !(2..=4).contains(&input.source.len()) {
        return Err(FitFailure::Count(input.source.len()));
    }
    if !input.semi_angle.is_finite() || input.semi_angle <= 0. || input.semi_angle >= FRAC_PI_2 {
        return Err(FitFailure::Support(input.semi_angle));
    }
    finite("chart", &[input.origin, input.center])?;
    for point in input.source {
        finite("source", point)?;
    }
    Ok(())
}
fn finite(role: &'static str, values: &[f64]) -> Result<(), FitFailure> {
    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(FitFailure::Nonfinite { role, index, value });
        }
    }
    Ok(())
}
fn endpoint(
    input: &FitInput<'_>,
    endpoint: usize,
    sine: f64,
    shared: IdealEndpoint,
) -> Result<Endpoint, FitFailure> {
    let n = input.source.len() - 1;
    let (point, neighbor, direction) = if endpoint == 0 {
        (input.source[0], input.source[1], 1.)
    } else {
        (input.source[n], input.source[n - 1], -1.)
    };
    let delta = [
        (neighbor[0] - point[0]) * direction * n as f64,
        (neighbor[1] - point[1]) * direction * n as f64,
    ];
    finite("endpoint derivative", &delta)?;
    endpoint_values(point, delta, sine, shared)
}
fn endpoint_values(
    p: [f64; 2],
    d: [f64; 2],
    sine: f64,
    shared: IdealEndpoint,
) -> Result<Endpoint, FitFailure> {
    let (slant_rate, tangent_rate) = endpoint_rates(p, d, shared.slant, shared.tangent, sine)?;
    Ok(Endpoint {
        slant: shared.slant,
        slant_rate,
        tangent: shared.tangent,
        tangent_rate,
    })
}
pub(in super::super) fn ideal_endpoint(
    input: &FitInput<'_>,
    endpoint: usize,
) -> Result<IdealEndpoint, FitFailure> {
    admit(input)?;
    let p = input.source[if endpoint == 0 {
        0
    } else {
        input.source.len() - 1
    }];
    let slant = hypot(p[0], p[1]);
    if slant == 0. {
        return Err(FitFailure::Apex { endpoint });
    }
    finite("slant", &[slant])?;
    let tangent = tan(half_angle(input, endpoint, p, sin(input.semi_angle))?);
    finite("tangent", &[tangent])?;
    Ok(IdealEndpoint { tangent, slant })
}

fn endpoint_rates(
    p: [f64; 2],
    d: [f64; 2],
    slant: f64,
    tangent: f64,
    sine: f64,
) -> Result<(f64, f64), FitFailure> {
    let [x, y] = [p[0] / slant, p[1] / slant];
    let rate = (x * d[1] - y * d[0]) / slant / sine;
    let tangent_rate = rate * (1. + tangent * tangent) / 2.;
    let slant_rate = x * d[0] + y * d[1];
    finite(
        "endpoint values",
        &[rate, tangent, tangent_rate, slant_rate],
    )?;
    Ok((slant_rate, tangent_rate))
}
fn half_angle(
    input: &FitInput<'_>,
    endpoint: usize,
    p: [f64; 2],
    sine: f64,
) -> Result<f64, FitFailure> {
    // The certificate uses positive pi on the negative axis for either signed zero.
    let y = if p[0] < 0. && p[1] == 0. { 0. } else { p[1] };
    let developed = atan2(y, p[0]) + TAU * f64::from(input.developed_turns[endpoint]);
    let angle = input.origin + developed / sine + TAU * f64::from(input.physical_turns[endpoint]);
    let half = (angle - input.center) / 2.;
    if !half.is_finite() || half.abs() >= FRAC_PI_2 || cos(half) <= 0. {
        return Err(FitFailure::Chart {
            endpoint,
            half_angle: half,
        });
    }
    Ok(half)
}
fn hermite(start: f64, start_rate: f64, end: f64, end_rate: f64) -> [f64; 4] {
    [start, start + start_rate / 3., end - end_rate / 3., end]
}
