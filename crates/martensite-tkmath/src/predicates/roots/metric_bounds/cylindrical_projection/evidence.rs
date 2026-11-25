// SPDX-License-Identifier: GPL-2.0-only
//! Bind native Section edges to exact signed and trimmed source intervals.
use super::{
    AxialProjectionWitness, Checked, CylinderProjectionBranch as Branch,
    CylinderProjectionError as Failure, NativeProjectionEdge, ProjectionEdgeCertificate,
    ProjectionSweep, ProjectionTrimEndpoint, SelectedProjectionInterval,
    affine::{Ratio, Vector, bounds, sub},
    native_net::NativeSample,
    poll,
    roots::RadialRoot,
    trim::{SourceInterval, angle},
    verify::{Limits, Readback, abs, sqrt_bound},
};
use crate::predicates::roots::metric_bounds::placed_plane::dot;

struct Expected<'a> {
    branch_index: usize,
    root: &'a RadialRoot,
    retained: &'a SourceInterval,
}

pub(super) fn certify(
    witness: &AxialProjectionWitness,
    sweep: ProjectionSweep,
    candidates: &[NativeProjectionEdge],
    limits: &Limits,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Vec<(usize, ProjectionEdgeCertificate)>> {
    let expected = expected_intervals(witness);
    if expected.len() != candidates.len() {
        return Err(Failure::Coverage);
    }
    let mut covered = vec![None; expected.len()];
    for (index, candidate) in candidates.iter().enumerate() {
        poll(cancelled)?;
        let readback = Readback::capture(candidate, limits, cancelled)?;
        let claim = bind(witness, sweep, &readback, limits, &expected, cancelled)?;
        if covered[claim.slot]
            .replace((index, claim.certificate))
            .is_some()
        {
            return Err(Failure::Coverage);
        }
    }
    poll(cancelled)?;
    covered
        .into_iter()
        .collect::<Option<_>>()
        .ok_or(Failure::Coverage)
}

fn expected_intervals(witness: &AxialProjectionWitness) -> Vec<Expected<'_>> {
    witness
        .roots
        .iter()
        .enumerate()
        .flat_map(|(branch_index, root)| {
            root.retained.iter().map(move |retained| Expected {
                branch_index,
                root,
                retained,
            })
        })
        .collect()
}

struct Claim {
    slot: usize,
    certificate: ProjectionEdgeCertificate,
}

fn bind(
    witness: &AxialProjectionWitness,
    sweep: ProjectionSweep,
    readback: &Readback,
    limits: &Limits,
    expected: &[Expected<'_>],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Claim> {
    let mut found = None;
    for (slot, expected) in expected.iter().enumerate() {
        poll(cancelled)?;
        let Some(certificate) = check(witness, sweep, readback, limits, expected, cancelled)?
        else {
            continue;
        };
        if found.replace(Claim { slot, certificate }).is_some() {
            return Err(Failure::Coverage);
        }
    }
    found.ok_or(Failure::Coverage)
}

fn check(
    witness: &AxialProjectionWitness,
    sweep: ProjectionSweep,
    readback: &Readback,
    limits: &Limits,
    expected: &Expected<'_>,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<ProjectionEdgeCertificate>> {
    let map = EndpointMap::from(readback, expected.retained)?;
    let source_error = map.error();
    if source_error > limits.source {
        return Ok(None);
    }
    let Some(metric) = transport(witness, sweep, readback, &map, expected.root, cancelled)? else {
        return Ok(None);
    };
    if Ratio::from_finite(metric.spatial) > limits.spatial || metric.angle > limits.angle {
        return Ok(None);
    }
    let selected = selection(witness, expected, readback, &map)?;
    Ok(Some(ProjectionEdgeCertificate {
        branch: expected.root.branch,
        signed_travel: bounds(&expected.root.travel)?,
        retained: expected.retained.enclose()?,
        origins: [
            endpoint_origin(witness, &expected.retained.ends[0])?,
            endpoint_origin(witness, &expected.retained.ends[1])?,
        ],
        selected,
        spatial_error: metric.spatial,
        source_parameter_error: bounds(&source_error)?[1],
        angle_error: bounds(&metric.angle)?[1],
    }))
}

fn endpoint_origin(witness: &AxialProjectionWitness, t: &Ratio) -> Checked<ProjectionTrimEndpoint> {
    if *t == witness.source.range[0] {
        return Ok(ProjectionTrimEndpoint::SourceFirst);
    }
    if *t == witness.source.range[1] {
        return Ok(ProjectionTrimEndpoint::SourceLast);
    }
    let hole = witness.chart.hole.ok_or(Failure::Coverage)?;
    let height = witness.source.height_at(t);
    if height == Ratio::from_finite(hole.axial[0]) {
        return Ok(ProjectionTrimEndpoint::HoleLower);
    }
    if height == Ratio::from_finite(hole.axial[1]) {
        return Ok(ProjectionTrimEndpoint::HoleUpper);
    }
    Err(Failure::Coverage)
}

struct EndpointMap<'a> {
    actual: [&'a Ratio; 2],
    required: [&'a Ratio; 2],
}

impl<'a> EndpointMap<'a> {
    fn from(readback: &'a Readback, retained: &'a SourceInterval) -> Checked<Self> {
        let mut a = &readback.samples.first().ok_or(Failure::Candidate)?.prism[0];
        let mut b = &readback.samples.last().ok_or(Failure::Candidate)?.prism[0];
        if !readback.forward {
            std::mem::swap(&mut a, &mut b);
        }
        Ok(Self {
            actual: [a, b],
            required: retained.ends.each_ref(),
        })
    }

    fn error(&self) -> Ratio {
        abs(self.actual[0].subtract(self.required[0]))
            .max(abs(self.actual[1].subtract(self.required[1])))
    }

    fn source(&self, actual: &Ratio) -> Ratio {
        remap(actual, self.actual, self.required)
    }

    fn measured_source(&self, source: &Ratio) -> Ratio {
        remap(source, self.required, self.actual)
    }
}

fn remap(value: &Ratio, [a, b]: [&Ratio; 2], [c, d]: [&Ratio; 2]) -> Ratio {
    c.add(
        &value
            .subtract(a)
            .divide(&b.subtract(a))
            .multiply(&d.subtract(c)),
    )
}

struct Achieved {
    spatial: f64,
    angle: Ratio,
}

fn transport(
    witness: &AxialProjectionWitness,
    sweep: ProjectionSweep,
    readback: &Readback,
    map: &EndpointMap<'_>,
    root: &RadialRoot,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<Option<Achieved>> {
    let first = readback.samples.first().ok_or(Failure::Candidate)?;
    let Some(angle_chart) = resolve_charts(first, root.branch) else {
        return Ok(None);
    };
    let origin_travel = Ratio::from_finite(sweep.start);
    let mut worst = Worst::new();
    for sample in &readback.samples {
        poll(cancelled)?;
        measure(
            witness,
            root,
            &origin_travel,
            sample,
            map,
            &angle_chart,
            &mut worst,
        );
    }
    worst.finish(&readback.vertex_error).map(Some)
}

struct Worst {
    spatial_squared: Ratio,
    prism_squared: Ratio,
    target_model: Ratio,
    angle: Ratio,
}

impl Worst {
    fn new() -> Self {
        let zero = Ratio::from_finite(0.);
        Self {
            spatial_squared: zero.clone(),
            prism_squared: zero.clone(),
            target_model: zero.clone(),
            angle: zero,
        }
    }

    fn finish(self, vertex_error: &Ratio) -> Checked<Achieved> {
        let spatial_root = sqrt_bound(&self.spatial_squared)?;
        let target = Ratio::from_finite(spatial_root).add(&self.target_model);
        let combined = bounds(&target)?[1]
            .max(sqrt_bound(&self.prism_squared)?)
            .max(sqrt_bound(vertex_error)?);
        Ok(Achieved {
            spatial: combined,
            angle: self.angle,
        })
    }
}

fn measure(
    witness: &AxialProjectionWitness,
    root: &RadialRoot,
    origin_travel: &Ratio,
    sample: &NativeSample,
    map: &EndpointMap<'_>,
    angular: &AngularCharts,
    worst: &mut Worst,
) {
    let [raw_source, raw_sweep] = &sample.prism;
    let source = map.source(raw_source);
    let actual_travel = origin_travel.subtract(raw_sweep);
    add_max(
        &mut worst.spatial_squared,
        squared(&sample.position, &witness.position(&source, &root.travel)),
    );
    add_max(
        &mut worst.prism_squared,
        squared(
            &sample.position,
            &witness.position(raw_source, &actual_travel),
        ),
    );
    let height = witness.source.height_at(&source);
    measure_target(
        &sample.target,
        &angular.first,
        &height,
        &witness.chart.radius,
        worst,
    );
    if let (Some(target), Some(chart)) = (&sample.second_target, &angular.second) {
        measure_target(target, chart, &height, &witness.chart.radius, worst);
    }
}

struct AngularCharts {
    first: [Ratio; 2],
    second: Option<[Ratio; 2]>,
}

fn resolve_charts(first: &NativeSample, branch: Branch) -> Option<AngularCharts> {
    let a = chart_angle(&first.target[0], branch);
    let b = first
        .second_target
        .as_ref()
        .map(|uv| chart_angle(&uv[0], branch));
    if b.as_ref()
        .is_some_and(|b| branch != Branch::PositiveU || *b == a)
    {
        return None;
    }
    Some(AngularCharts {
        first: a,
        second: b,
    })
}

fn chart_angle(observed: &Ratio, branch: Branch) -> [Ratio; 2] {
    let pi = std::f64::consts::PI;
    if branch == Branch::PositiveU && *observed > Ratio::from_finite(pi) {
        return [pi, pi.next_up()]
            .map(|value| Ratio::from_finite(value).multiply(&Ratio::from_finite(2.)));
    }
    angle(branch)
}

fn measure_target(
    [theta, axial]: &[Ratio; 2],
    angle: &[Ratio; 2],
    height: &Ratio,
    radius: &Ratio,
    worst: &mut Worst,
) {
    let angular_error = abs(theta.subtract(&angle[0])).max(abs(theta.subtract(&angle[1])));
    let combined = radius
        .multiply(&angular_error)
        .add(&abs(axial.subtract(height)));
    add_max(&mut worst.target_model, combined);
    add_max(&mut worst.angle, angular_error);
}

fn squared(actual: &Vector, expected: &Vector) -> Ratio {
    let d = sub(actual, expected);
    dot(&d, &d)
}

fn add_max(worst: &mut Ratio, observed: Ratio) {
    if *worst < observed {
        *worst = observed;
    }
}

fn selection(
    witness: &AxialProjectionWitness,
    expected: &Expected<'_>,
    readback: &Readback,
    map: &EndpointMap<'_>,
) -> Checked<Vec<SelectedProjectionInterval>> {
    witness
        .selected
        .iter()
        .filter(|span| span.index == expected.branch_index)
        .filter(|span| {
            span.interval.ends[0] >= expected.retained.ends[0]
                && span.interval.ends[1] <= expected.retained.ends[1]
        })
        .map(|span| selected_range(&span.interval, readback, map))
        .collect()
}

fn selected_range(
    interval: &SourceInterval,
    readback: &Readback,
    map: &EndpointMap<'_>,
) -> Checked<SelectedProjectionInterval> {
    let [a, b] = interval.ends.each_ref().map(|source| {
        let measured = map.measured_source(source);
        parameter_at(readback, &measured).and_then(|value| bounds(&value))
    });
    Ok(SelectedProjectionInterval {
        source: interval.enclose()?,
        section_parameters: [a?, b?],
    })
}

fn parameter_at(readback: &Readback, source: &Ratio) -> Checked<Ratio> {
    for pair in readback.samples.windows(2) {
        let [a, b] = [&pair[0], &pair[1]];
        let [u, v] = [&a.prism[0], &b.prism[0]];
        if (u <= source && source <= v) || (v <= source && source <= u) {
            return Ok(remap(source, [u, v], [&a.parameter, &b.parameter]));
        }
    }
    Err(Failure::Coverage)
}
