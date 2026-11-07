//! Refuse unresolved, axis-crossing or ambiguously closed domains before native construction.
use super::{failure::*, frame, request::*};
use martensite_cmath::sin;
use std::{collections::BTreeSet, f64::consts::TAU};

pub(super) struct Admitted {
    pub rectangle: CylinderRectangle,
    pub depths: [f64; 2],
    pub options: NormalExtrusionOptions,
    pub radii: [f64; 2],
    pub angle: f64,
    pub start: f64,
    pub expected_volume: f64,
    pub coordinate_error: f64,
}

fn invalid(field: &'static str, value: f64, expected: &'static str) -> SurfaceFailureCause {
    SurfaceFailureCause::InvalidInput {
        field,
        value,
        expected,
    }
}
fn finite(field: &'static str, values: &[f64]) -> Checked<()> {
    match values.iter().copied().find(|value| !value.is_finite()) {
        Some(value) => Err(invalid(field, value, "finite coordinates")),
        None => Ok(()),
    }
}
fn identities(rectangle: &CylinderRectangle) -> Checked<()> {
    for (kind, ids) in [
        ("corner", rectangle.corners.map(|id| id.0)),
        ("boundary", rectangle.boundaries.map(|id| id.0)),
    ] {
        let mut seen = BTreeSet::new();
        for value in ids {
            if !seen.insert(value) {
                return Err(SurfaceFailureCause::DuplicateIdentity { kind, value });
            }
        }
    }
    Ok(())
}
fn frame_inputs(rectangle: &CylinderRectangle) -> Checked<()> {
    let support = &rectangle.support;
    finite("origin", &support.origin)?;
    finite("axis", &support.axis)?;
    finite("radial", &support.radial)?;
    let error = (frame::dot(support.axis, support.axis) - 1.)
        .abs()
        .max((frame::dot(support.radial, support.radial) - 1.).abs())
        .max(frame::dot(support.axis, support.radial).abs());
    if error > 64. * f64::EPSILON {
        return Err(invalid(
            "frame",
            error,
            "orthonormal axis/radial vectors within 64 epsilon",
        ));
    }
    Ok(())
}
fn input_fields(
    rectangle: &CylinderRectangle,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<()> {
    identities(rectangle)?;
    frame_inputs(rectangle)?;
    finite("s", &rectangle.s)?;
    finite("z", &rectangle.z)?;
    finite("depth", &depths)?;
    support_fields(rectangle.support)?;
    option_fields(options)
}
fn option_fields(options: NormalExtrusionOptions) -> Checked<()> {
    options
        .resolution
        .admit()
        .map_err(|(field, value)| invalid(field, value, "finite positive tolerance"))?;
    let relative = options.volume_relative_error;
    require(
        (64. * f64::EPSILON..=0.001).contains(&relative),
        "volume_relative_error",
        relative,
        "64 epsilon through 0.001",
    )
}
fn require(
    condition: bool,
    field: &'static str,
    value: f64,
    expected: &'static str,
) -> Checked<()> {
    if !condition {
        return Err(invalid(field, value, expected));
    }
    Ok(())
}
pub(super) fn admit(
    rectangle: &CylinderRectangle,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<Admitted> {
    input_fields(rectangle, depths, options)?;
    let mut admitted = derive(*rectangle, depths, options);
    let dimensions = [
        rectangle.s[1] - rectangle.s[0],
        rectangle.z[1] - rectangle.z[0],
        depths[1] - depths[0],
    ];
    dimension_checks(&admitted, dimensions)?;
    resolution_checks(&mut admitted, dimensions)?;
    Ok(admitted)
}
fn derive(
    rectangle: CylinderRectangle,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Admitted {
    let radii = offset_radii(&rectangle, depths);
    let angle = (rectangle.s[1] - rectangle.s[0]) / rectangle.support.radius;
    let start = rectangle.support.chart_origin + rectangle.s[0] / rectangle.support.radius;
    let expected_volume = sector_volume(&rectangle, depths, radii, angle);
    Admitted {
        rectangle,
        depths,
        options,
        radii,
        angle,
        start,
        expected_volume,
        coordinate_error: 0.,
    }
}
fn separated(field: &'static str, value: f64, minimum: f64) -> Checked<()> {
    require(
        value.is_finite() && value > 2. * minimum,
        field,
        value,
        "finite positive separation greater than twice effective spatial tolerance",
    )
}
fn dimension_checks(admitted: &Admitted, dimensions: [f64; 3]) -> Checked<()> {
    let minimum = effective_limits(admitted.options).linear_separation / 2.;
    for (field, value) in ["width", "height", "depth"].into_iter().zip(dimensions) {
        separated(field, value, minimum)?;
    }
    for radius in admitted.radii {
        separated("offset radius", radius, minimum)?;
    }
    angular_checks(admitted, minimum)?;
    require(
        admitted.expected_volume.is_finite() && admitted.expected_volume > 0.,
        "volume",
        admitted.expected_volume,
        "finite positive volume without arithmetic overflow",
    )
}
fn angular_checks(admitted: &Admitted, minimum: f64) -> Checked<()> {
    let angular = effective_limits(admitted.options).angular_clearance;
    let gap = TAU - admitted.angle;
    require(
        admitted.start.is_finite() && admitted.angle > angular && gap > angular,
        "angular span",
        admitted.angle,
        "resolvable positive span with full-turn clearance",
    )?;
    let separation = 2. * admitted.radii[0].min(admitted.radii[1]) * sin(gap / 2.).abs();
    separated("end-face separation", separation, minimum)
}
fn coordinate_scale(admitted: &Admitted) -> f64 {
    let request = admitted.rectangle;
    request
        .support
        .origin
        .into_iter()
        .chain(request.z)
        .chain(request.s)
        .chain(admitted.depths)
        .chain(admitted.radii)
        .map(f64::abs)
        .fold(request.support.radius, f64::max)
}
fn resolution_checks(admitted: &mut Admitted, dimensions: [f64; 3]) -> Checked<()> {
    let angular_scale = admitted.start.abs() * admitted.radii[0].max(admitted.radii[1]);
    admitted.coordinate_error = 64. * f64::EPSILON * (coordinate_scale(admitted) + angular_scale);
    let error = admitted.coordinate_error;
    let resolved = error.is_finite()
        && error <= admitted.options.resolution.weld
        && dimensions.into_iter().all(|size| size > 2. * error);
    require(
        resolved,
        "coordinate rounding estimate",
        error,
        "below spatial tolerance and half every dimension",
    )
}

fn support_fields(support: CylinderSupport) -> Checked<()> {
    finite("support", &[support.radius, support.chart_origin])?;
    require(
        support.radius > 0.,
        "radius",
        support.radius,
        "positive finite radius",
    )
}
fn offset_radii(rectangle: &CylinderRectangle, depths: [f64; 2]) -> [f64; 2] {
    let sign = match rectangle.support.normal {
        NormalSense::Outward => 1.,
        NormalSense::Inward => -1.,
    };
    depths.map(|depth| rectangle.support.radius + sign * depth)
}
fn sector_volume(
    rectangle: &CylinderRectangle,
    depths: [f64; 2],
    radii: [f64; 2],
    angle: f64,
) -> f64 {
    let height = rectangle.z[1] - rectangle.z[0];
    angle * height * (depths[1] - depths[0]) * ((radii[0] + radii[1]) / 2.)
}
