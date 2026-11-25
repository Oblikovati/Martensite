// SPDX-License-Identifier: GPL-2.0-only
//! Bind the residual proof to the literal decoded analytic face datum.
use super::*;
use martensite_tkmath::{
    gp::Axis3,
    predicates::roots::metric_bounds::{
        ConicalTrimCarrier, ConicalTrimQuery, CylindricalTrimQuery, certify_conical_trim_ranges,
        certify_cylindrical_trim_ranges,
    },
};

pub(in crate::surface_sketch::full_turn::hole::curved::proof) enum CapFrame {
    Cylinder {
        radius: f64,
        axial: [f64; 2],
    },
    Cone {
        carrier: ConicalTrimCarrier,
        meridian_sign: f64,
        spatial_sign: f64,
    },
}

impl CapFrame {
    pub(in crate::surface_sketch::full_turn::hole::curved::proof) fn capture(
        surface: &BRepAdaptorSurface,
        cylinder: bool,
    ) -> Checked<Self> {
        if cylinder {
            return cylindrical(surface);
        }
        conical(surface)
    }

    pub(super) fn certify(
        &self,
        curve: &SpaceCurve,
        uv: &[[f64; 2]],
        chart: [f64; 2],
        limit: f64,
    ) -> Checked<f64> {
        let ranges = [curve.range, chart];
        match *self {
            Self::Cylinder { radius, axial } => {
                certify_cylinder(curve, uv, ranges, limit, radius, axial)
            }
            Self::Cone {
                carrier,
                meridian_sign,
                spatial_sign,
            } => {
                let uv = uv
                    .iter()
                    .map(|&[u, v]| [u, meridian_sign * v])
                    .collect::<Vec<_>>();
                let poles = curve
                    .poles
                    .iter()
                    .map(|&[x, y, z]| [x, y, spatial_sign * z])
                    .collect::<Vec<_>>();
                certify_cone(
                    MetricBezier {
                        poles: &poles,
                        weights: Some(&curve.weights),
                    },
                    &uv,
                    ranges,
                    limit,
                    carrier,
                )
            }
        }
    }
}

fn cylindrical(surface: &BRepAdaptorSurface) -> Checked<CapFrame> {
    let cylinder = surface
        .cylinder()
        .map_err(|e| native("curved cylindrical cap carrier", e))?;
    let radius = cylinder.radius();
    positive_radius(radius)?;
    let axial = canonical_axial(cylinder.position())?;
    Ok(CapFrame::Cylinder { radius, axial })
}

fn conical(surface: &BRepAdaptorSurface) -> Checked<CapFrame> {
    let cone = surface
        .cone()
        .map_err(|e| native("curved conical cap carrier", e))?;
    let radius = cone.ref_radius();
    positive_radius(radius)?;
    let [origin, axis] = canonical_axial(cone.position())?;
    let semi_angle = cone.semi_angle();
    let meridian_sign = semi_angle.signum();
    let spatial_sign = axis * meridian_sign;
    let carrier = ConicalTrimCarrier {
        radius,
        axial_origin: spatial_sign * origin,
        semi_angle: semi_angle.abs(),
    };
    Ok(CapFrame::Cone {
        carrier,
        meridian_sign,
        spatial_sign,
    })
}

fn canonical_axial(pos: Axis3) -> Checked<[f64; 2]> {
    let [x, y, z] = *pos.location().coordinates();
    let axis = *pos.direction().coordinates();
    if x == 0.
        && y == 0.
        && z.is_finite()
        && *pos.x_direction().coordinates() == [1., 0., 0.]
        && *pos.y_direction().coordinates() == [0., 1., 0.]
        && (axis == [0., 0., 1.] || axis == [0., 0., -1.])
    {
        return Ok([z, axis[2]]);
    }
    Err(incomplete(
        "canonical decoded cap angular frame and signed axial datum",
        format!("{pos:?}"),
    ))
}

fn positive_radius(radius: f64) -> Checked<()> {
    cylinder_admission::require(
        radius.is_finite() && radius > 0.,
        "actual curved cap radius",
        radius,
        "positive finite reference radius for the decoded carrier",
    )
}

fn certify_cylinder(
    curve: &SpaceCurve,
    uv: &[[f64; 2]],
    ranges: [[f64; 2]; 2],
    limit: f64,
    radius: f64,
    [origin, sign]: [f64; 2],
) -> Checked<f64> {
    let mut controls = uv.to_vec();
    let pad = normalize_cylindrical_height(&mut controls, origin, sign);
    let query = CylindricalTrimQuery {
        curve: MetricBezier {
            poles: &curve.poles,
            weights: Some(&curve.weights),
        },
        pcurve: &controls,
        radius,
        spatial_limit: (limit - pad).next_down(),
        options: Default::default(),
    };
    let result = certify_cylindrical_trim_ranges(query, ranges, || false).map_err(|cause| {
        incomplete(
            "certified active decoded curved trim",
            format!("{cause:?} with ranges {ranges:?}"),
        )
    })?;
    Ok((result.spatial_residual_bound() + pad).next_up())
}

fn normalize_cylindrical_height(points: &mut [[f64; 2]], origin: f64, sign: f64) -> f64 {
    let mut pad: f64 = 0.;
    for point in points {
        let z = origin + sign * point[1];
        pad = pad.max(z.next_up() - z).max(z - z.next_down());
        point[1] = z;
    }
    pad
}

fn certify_cone(
    curve: MetricBezier<'_, 3>,
    uv: &[[f64; 2]],
    ranges: [[f64; 2]; 2],
    limit: f64,
    carrier: ConicalTrimCarrier,
) -> Checked<f64> {
    let query = ConicalTrimQuery {
        curve,
        pcurve: uv,
        carrier,
        spatial_limit: limit,
        options: Default::default(),
    };
    let proof = certify_conical_trim_ranges(query, ranges, || false).map_err(|cause| {
        incomplete(
            "certified active decoded conical trim",
            format!("{cause:?} with ranges {ranges:?}"),
        )
    })?;
    Ok(proof.residual_bounds()[2])
}
