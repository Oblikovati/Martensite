// SPDX-License-Identifier: GPL-2.0-only
//! A complete turn has no angular endpoints to distinguish or admit.
use super::{
    super::{
        admission::*,
        failure::*,
        frame,
        sweep::{Sweep, SweepNames},
        *,
    },
    FullTurnBand, FullTurnSupport,
};
use martensite_cmath::{cos, sin};
use martensite_tkmath::gp::{Direction3, Plane3, Point3};
use std::f64::consts::{FRAC_PI_2, TAU};

pub(super) struct BandSection {
    pub sweep: Sweep,
    pub meridian: [[f64; 2]; 4],
    pub cylinder: bool,
}

struct MeridianMap {
    origin: [f64; 3],
    axis: [f64; 3],
    radial: [f64; 3],
    seam: f64,
    normal: NormalSense,
    radius_origin: f64,
    radius_rate: f64,
    axis_rate: f64,
    along: [f64; 2],
    cylinder: bool,
}

pub(super) fn admit(
    band: FullTurnBand,
    depths: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<BandSection> {
    option_fields(options)?;
    finite("depth", &depths)?;
    if band.rims[0] == band.rims[1] {
        return Err(SurfaceFailureCause::DuplicateIdentity {
            kind: "rim",
            value: band.rims[0].0,
        });
    }
    let map = MeridianMap::admit(band.support, options)?;
    map.section(band, depths, options)
}

impl MeridianMap {
    fn admit(input: FullTurnSupport, options: NormalExtrusionOptions) -> Checked<Self> {
        let map = match input {
            FullTurnSupport::Cylinder { support, axial } => Self::cylinder(support, axial)?,
            FullTurnSupport::Cone { support, slant } => Self::cone(support, slant, options)?,
        };
        frame_inputs(map.origin, map.axis, map.radial)?;
        finite("seam angle", &[map.seam])?;
        finite("meridian bounds", &map.along)?;
        Ok(map)
    }

    fn cylinder(support: CylinderSupport, axial: [f64; 2]) -> Checked<Self> {
        require(
            support.radius.is_finite() && support.radius > 0.,
            "radius",
            support.radius,
            "positive finite cylinder radius",
        )?;
        Ok(Self {
            origin: support.origin,
            axis: support.axis,
            radial: support.radial,
            seam: support.chart_origin,
            normal: support.normal,
            radius_origin: support.radius,
            radius_rate: 0.,
            axis_rate: 1.,
            along: axial,
            cylinder: true,
        })
    }

    fn cone(
        support: ConeSupport,
        slant: [f64; 2],
        options: NormalExtrusionOptions,
    ) -> Checked<Self> {
        let limits = effective_limits(options);
        require(
            support.semi_angle > limits.angular_clearance
                && FRAC_PI_2 - support.semi_angle > limits.angular_clearance,
            "cone semi-angle",
            support.semi_angle,
            "resolvable angle strictly between zero and pi/2",
        )?;
        separated("slant origin", slant[0], limits.linear_separation / 2.)?;
        Ok(Self {
            origin: support.apex,
            axis: support.axis,
            radial: support.radial,
            seam: support.chart_origin,
            normal: support.normal,
            radius_origin: 0.,
            radius_rate: sin(support.semi_angle),
            axis_rate: cos(support.semi_angle),
            along: slant,
            cylinder: false,
        })
    }

    fn section(
        self,
        band: FullTurnBand,
        depths: [f64; 2],
        options: NormalExtrusionOptions,
    ) -> Checked<BandSection> {
        let first_depth = usize::from(self.normal == NormalSense::Inward);
        let meridian = self.vertices(depths, first_depth);
        let spans = [self.along[1] - self.along[0], depths[1] - depths[0]];
        let accuracy = self.admit_section(meridian, spans, options)?;
        let sweep = self.sweep(band, meridian, spans, options, accuracy)?;
        Ok(BandSection {
            sweep,
            meridian,
            cylinder: self.cylinder,
        })
    }

    fn vertices(&self, depths: [f64; 2], first: usize) -> [[f64; 2]; 4] {
        let sign = if first == 0 { 1. } else { -1. };
        [(0, first), (0, 1 - first), (1, 1 - first), (1, first)].map(|(along, depth)| {
            let along = self.along[along];
            let depth = sign * depths[depth];
            [
                self.radius_origin + self.radius_rate * along + self.axis_rate * depth,
                self.axis_rate * along - self.radius_rate * depth,
            ]
        })
    }

    fn admit_section(
        &self,
        meridian: [[f64; 2]; 4],
        spans: [f64; 2],
        options: NormalExtrusionOptions,
    ) -> Checked<f64> {
        let minimum = effective_limits(options).linear_separation / 2.;
        separated("meridian width", spans[0], minimum)?;
        separated("depth interval", spans[1], minimum)?;
        for [radius, axial] in meridian {
            separated("offset radius", radius, minimum)?;
            finite("axial coordinate", &[axial])?;
        }
        self.rounding(meridian, spans, options)
    }

    fn rounding(
        &self,
        meridian: [[f64; 2]; 4],
        spans: [f64; 2],
        options: NormalExtrusionOptions,
    ) -> Checked<f64> {
        let condition = if self.cylinder {
            1.
        } else {
            1. + 1. / self.radius_rate + 1. / self.axis_rate
        };
        let angular = 64. * f64::EPSILON * (self.seam.abs() + TAU) * condition;
        require(
            angular <= options.resolution.angular,
            "angular rounding estimate",
            angular,
            "within requested angular tolerance",
        )?;
        let scale = self
            .origin
            .into_iter()
            .chain(meridian.into_iter().flatten())
            .map(f64::abs)
            .fold(0., f64::max);
        let error = scale * (64. * f64::EPSILON * condition + angular);
        require(
            error <= options.resolution.weld && spans.into_iter().all(|v| v > 2. * error),
            "coordinate rounding estimate",
            error,
            "within spatial tolerance and half both spans",
        )?;
        Ok(error)
    }

    fn sweep(
        &self,
        band: FullTurnBand,
        meridian: [[f64; 2]; 4],
        spans: [f64; 2],
        options: NormalExtrusionOptions,
        coordinate_error: f64,
    ) -> Checked<Sweep> {
        let points = self.section_points(meridian);
        for point in points {
            finite("section point", &point)?;
        }
        let [expected_volume, surface_area] = measures(meridian, spans)?;
        Ok(Sweep {
            region: band.region,
            names: SweepNames::FullTurn { rims: band.rims },
            options,
            origin: self.origin,
            axis: self.axis,
            angle: TAU,
            points,
            first_depth: usize::from(self.normal == NormalSense::Inward),
            expected_volume,
            surface_area,
            coordinate_error,
            reference_plane: self.integration_reference(meridian)?,
        })
    }

    fn section_points(&self, meridian: [[f64; 2]; 4]) -> [[f64; 3]; 4] {
        let tangent = frame::cross(self.axis, self.radial);
        let [sine, cosine] = [sin(self.seam), cos(self.seam)];
        meridian.map(|[radius, axial]| {
            std::array::from_fn(|i| {
                self.origin[i]
                    + axial * self.axis[i]
                    + radius * (cosine * self.radial[i] + sine * tangent[i])
            })
        })
    }

    fn integration_reference(&self, meridian: [[f64; 2]; 4]) -> Checked<Option<Plane3>> {
        // Follow the same conditioning as the existing analytic sector paths.
        if self.cylinder {
            return Ok(None);
        }
        let margin = meridian.into_iter().map(|p| p[0]).fold(0., f64::max);
        let axial = meridian
            .into_iter()
            .map(|p| p[1])
            .fold(f64::INFINITY, f64::min)
            - margin;
        let [x, y, z] = std::array::from_fn(|i| self.origin[i] + axial * self.axis[i]);
        finite("integration reference plane", &[x, y, z])?;
        let [a, b, c] = self.axis;
        let normal =
            Direction3::new(a, b, c).map_err(|e| native("integration reference normal", e))?;
        Plane3::new(Point3::new(x, y, z), normal)
            .map(Some)
            .map_err(|e| native("integration reference plane", e))
    }
}

fn measures(meridian: [[f64; 2]; 4], [span, depth]: [f64; 2]) -> Checked<[f64; 2]> {
    // The meridian coordinates form an orthonormal affine map. Opposite edges
    // cancel in the radius integral, leaving the mean of the four corners.
    let radius = meridian.into_iter().map(|p| p[0] * 0.25).sum::<f64>();
    let volume = TAU * span * depth * radius;
    let area = 2. * TAU * radius * (span + depth);
    for (field, value) in [("volume", volume), ("area", area)] {
        require(
            value.is_finite() && value > 0.,
            field,
            value,
            "finite positive analytic measure",
        )?;
    }
    Ok([volume, area])
}
