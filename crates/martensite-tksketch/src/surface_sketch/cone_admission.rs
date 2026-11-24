//! Conditioned cone admission keeps the whole meridional section away from its axis.
use super::{admission::*, cone_math, cone_request::*, failure::*, request::*, sweep::Sweep};
use martensite_cmath::{cos, sin};
use std::f64::consts::{FRAC_PI_2, TAU};

pub(super) struct AdmittedCone {
    pub sector: ConeSector,
    pub depths: [f64; 2],
    pub sine: f64,
    pub cosine: f64,
    pub start: f64,
    pub sweep: Sweep,
}
pub(super) fn admit(
    sector: &ConeSector,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<AdmittedCone> {
    input_fields(sector, depths, options)?;
    let mut admitted = derive(*sector, depths, options);
    dimension_checks(&admitted)?;
    admitted.sweep.reference_plane = Some(super::cone_plane::reference(&admitted)?);
    resolution_checks(&mut admitted)?;
    admitted.sweep.points = cone_math::section_points(&admitted);
    for point in admitted.sweep.points {
        finite("section point", &point)?;
    }
    Ok(admitted)
}
fn input_fields(
    sector: &ConeSector,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<()> {
    identities(sector.corners, sector.boundaries)?;
    let support = sector.support;
    frame_inputs(support.apex, support.axis, support.radial)?;
    finite("slant", &sector.slant)?;
    finite("developed angle", &sector.developed_angle)?;
    finite("depth", &depths)?;
    finite("cone support", &[support.semi_angle, support.chart_origin])?;
    option_fields(options)?;
    support_angle(support, options)
}
fn support_angle(support: ConeSupport, options: NormalExtrusionOptions) -> Checked<()> {
    let angular = effective_limits(options).angular_clearance;
    require(
        support.semi_angle > angular && FRAC_PI_2 - support.semi_angle > angular,
        "cone semi-angle",
        support.semi_angle,
        "resolvable angle strictly between zero and pi/2",
    )
}
fn derive(sector: ConeSector, depths: [f64; 2], options: NormalExtrusionOptions) -> AdmittedCone {
    let sine = sin(sector.support.semi_angle);
    let cosine = cos(sector.support.semi_angle);
    let start = sector.support.chart_origin + sector.developed_angle[0] / sine;
    let angle = (sector.developed_angle[1] - sector.developed_angle[0]) / sine;
    let mut admitted = AdmittedCone {
        sector,
        depths,
        sine,
        cosine,
        start,
        sweep: initial_sweep(sector, options, angle),
    };
    let [volume, area] = cone_math::scalars(&admitted);
    admitted.sweep.expected_volume = volume;
    admitted.sweep.surface_area = area;
    admitted
}
fn initial_sweep(sector: ConeSector, options: NormalExtrusionOptions, angle: f64) -> Sweep {
    Sweep {
        region: sector.region,
        corners: sector.corners,
        boundaries: sector.boundaries,
        options,
        origin: sector.support.apex,
        axis: sector.support.axis,
        angle,
        points: [[0.; 3]; 4],
        first_depth: usize::from(sector.support.normal == NormalSense::Inward),
        expected_volume: 0.,
        surface_area: 0.,
        coordinate_error: 0.,
        reference_plane: None,
    }
}
fn radii(admitted: &AdmittedCone) -> [f64; 4] {
    [(0, 0), (0, 1), (1, 0), (1, 1)].map(|(l, t)| {
        cone_math::meridian(admitted, admitted.sector.slant[l], admitted.depths[t])[0]
    })
}
fn dimension_checks(admitted: &AdmittedCone) -> Checked<()> {
    let minimum = effective_limits(admitted.sweep.options).linear_separation / 2.;
    separated("slant origin", admitted.sector.slant[0], minimum)?;
    separated(
        "slant interval",
        admitted.sector.slant[1] - admitted.sector.slant[0],
        minimum,
    )?;
    separated(
        "depth interval",
        admitted.depths[1] - admitted.depths[0],
        minimum,
    )?;
    for radius in radii(admitted) {
        separated("offset radius", radius, minimum)?;
    }
    angular_checks(admitted, minimum)?;
    measure_checks(admitted)
}
fn measure_checks(admitted: &AdmittedCone) -> Checked<()> {
    for (field, value) in [
        ("volume", admitted.sweep.expected_volume),
        ("area", admitted.sweep.surface_area),
    ] {
        require(
            value.is_finite() && value > 0.,
            field,
            value,
            "finite positive analytic measure",
        )?;
    }
    Ok(())
}
fn angular_checks(admitted: &AdmittedCone, minimum: f64) -> Checked<()> {
    let angular = effective_limits(admitted.sweep.options).angular_clearance;
    let angle = admitted.sweep.angle;
    let gap = TAU - angle;
    require(
        admitted.start.is_finite() && angle > angular && gap > angular,
        "physical angular span",
        angle,
        "resolvable positive span with full-turn clearance",
    )?;
    let radius = radii(admitted).into_iter().fold(f64::INFINITY, f64::min);
    separated("angular width", radius * angle, minimum)?;
    separated(
        "end-face separation",
        2. * radius * sin(gap / 2.).abs(),
        minimum,
    )
}
fn coordinate_scale(admitted: &AdmittedCone) -> f64 {
    admitted
        .sector
        .support
        .apex
        .into_iter()
        .chain(admitted.sector.slant)
        .chain(admitted.depths)
        .chain(radii(admitted))
        .chain(cone_math::depth_apex(admitted, 0))
        .chain(cone_math::depth_apex(admitted, 1))
        .chain(cone_math::slant_apex(admitted, 0))
        .chain(cone_math::slant_apex(admitted, 1))
        .chain(super::cone_plane::reference_coordinates(admitted))
        .map(f64::abs)
        .fold(0., f64::max)
}
fn angular_error(admitted: &AdmittedCone) -> f64 {
    let support = admitted.sector.support;
    let scale = support.chart_origin.abs()
        + admitted
            .sector
            .developed_angle
            .into_iter()
            .map(f64::abs)
            .sum::<f64>()
            / admitted.sine
        + admitted.sweep.angle;
    64. * f64::EPSILON * scale * (1. + 1. / admitted.sine)
}
fn resolution_checks(admitted: &mut AdmittedCone) -> Checked<()> {
    let angular_error = angular_error(admitted);
    require(
        angular_error.is_finite() && angular_error <= admitted.sweep.options.resolution.angular,
        "angular rounding estimate",
        angular_error,
        "within requested angular tolerance",
    )?;
    let radius = radii(admitted).into_iter().fold(0., f64::max);
    let amplification = 1. + 1. / admitted.sine + 1. / admitted.cosine;
    let error =
        64. * f64::EPSILON * coordinate_scale(admitted) * amplification + radius * angular_error;
    admitted.sweep.coordinate_error = error;
    coordinate_resolution(admitted, error)
}
fn coordinate_resolution(admitted: &AdmittedCone, error: f64) -> Checked<()> {
    let smallest = (admitted.sector.slant[1] - admitted.sector.slant[0])
        .min(admitted.depths[1] - admitted.depths[0]);
    require(
        error.is_finite()
            && error <= admitted.sweep.options.resolution.weld
            && smallest > 2. * error,
        "coordinate rounding estimate",
        error,
        "within spatial tolerance and half every section dimension",
    )
}
