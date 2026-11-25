// SPDX-License-Identifier: GPL-2.0-only
//! Intersect each developed cone contour with one exact lift of the physical zero meridian.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    certify_developed_cone_ray, enclose_developed_cone_angle,
};
use martensite_tkmath::predicates::{OrientationSign, orient2d};

impl AperturePlan {
    pub(in crate::surface_sketch::full_turn::hole::curved) fn admit_cone(
        band: &FullTurnBand,
        request: &WrappedConeNormalRequest,
        options: NormalExtrusionOptions,
        scope: Scope,
        source: SourceModel,
    ) -> Checked<Self> {
        let stock = band_admission::admit(*band, request.depths, options)?;
        let FullTurnSupport::Cone { support, slant } = band.support else {
            return Err(incomplete(
                "full conical stock for a curved cone aperture",
                format!("{:?}", band.support),
            ));
        };
        require_support(support, request)?;
        let contours = request
            .selections
            .iter()
            .map(|selection| {
                cone_contour(&request.profile, selection, support, slant, options, source)
            })
            .collect::<Checked<Vec<_>>>()?;
        Self::compose(*band, stock, contours, scope, options)
    }
}

fn require_support(support: ConeSupport, request: &WrappedConeNormalRequest) -> Checked<()> {
    if request.support == support
        && support.apex == [0.; 3]
        && support.axis == [0., 0., 1.]
        && support.radial == [1., 0., 0.]
        && support.chart_origin == 0.
        && request.profile.plane == ProfilePlane::world_xy()
    {
        return Ok(());
    }
    Err(incomplete(
        "matching canonical conical chart at the zero ray",
        format!("{:?}", request.support),
    ))
}

fn cone_contour(
    profile: &SketchProfile,
    selection: &WrappedRegionSelection,
    support: ConeSupport,
    slant: [f64; 2],
    options: NormalExtrusionOptions,
    source: SourceModel,
) -> Checked<Aperture> {
    let curves = contour_curves(profile, selection, source)?;
    let meridian = certify_cone_hull(&curves, slant, options)?;
    let lattice = crossed::CutLattice::new(FullTurnSupport::Cone { support, slant }, options)?;
    let developed = extreme_angles(&curves)?;
    let Some(cut) = lattice.cut_in_hull(developed)? else {
        return aperture(selection, curves, meridian, Vec::new());
    };
    let ray = certify_developed_cone_ray(cut.coordinate)
        .map_err(|cause| incomplete("certified developed cut ray", format!("{cause:?}")))?;
    let angular = effective_limits(options).angular_clearance;
    let mut roots = Vec::new();
    for curve in &curves {
        // Along the entire hull, the physical angle is at least transverse distance
        // divided by the upper slant, since |sin(delta)| <= |delta| and sin(alpha) <= 1.
        roots.extend(curve.crossing_on_cone(cut.turn, &ray, meridian[1], angular)?);
    }
    aperture(selection, curves, meridian, roots)
}

fn certify_cone_hull(
    curves: &[DirectedCurve],
    slant: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<[f64; 2]> {
    let mut bounds = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
    for curve in curves {
        extend_bounds(&mut bounds, &curve.points);
    }
    let [[min_x, max_x], [y0, y1]] = bounds;
    let y = y0.abs().max(y1.abs());
    cylinder_admission::require(
        min_x > y,
        "cone aperture positive-X wedge",
        min_x - y,
        "positive developed first coordinate exceeding every transverse coordinate",
    )?;
    let l1 = (max_x + y).next_up();
    require_rim_clearance(slant, [min_x, l1], options)?;
    Ok([min_x, l1])
}

fn extreme_angles(curves: &[DirectedCurve]) -> Checked<[f64; 2]> {
    let mut points = curves.iter().flat_map(|curve| curve.points.iter().copied());
    let first = points
        .next()
        .ok_or_else(|| incomplete("developed source contour", "no controls"))?;
    let extremes = points.try_fold([first; 2], extend_angle_extremes)?;
    let [a, b] = extremes.map(|point| {
        enclose_developed_cone_angle(point).map_err(|cause| {
            incomplete(
                "exact source contour developed angles",
                format!("{cause:?}"),
            )
        })
    });
    Ok([a?[0], b?[1]])
}

fn extend_angle_extremes([lo, hi]: [[f64; 2]; 2], point: [f64; 2]) -> Checked<[[f64; 2]; 2]> {
    let orient = |edge| {
        orient2d([0.; 2], edge, point)
            .map_err(|cause| incomplete("strict positive-X angular hull", format!("{cause:?}")))
    };
    let a = if orient(lo)? == OrientationSign::Negative {
        point
    } else {
        lo
    };
    let b = if orient(hi)? == OrientationSign::Positive {
        point
    } else {
        hi
    };
    Ok([a, b])
}
