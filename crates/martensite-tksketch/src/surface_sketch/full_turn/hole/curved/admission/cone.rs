// SPDX-License-Identifier: GPL-2.0-only
//! In a developed +X cone chart the physical zero meridian is the positive X ray.
use super::*;

impl AperturePlan {
    pub(in crate::surface_sketch::full_turn::hole::curved) fn admit_cone(
        band: &FullTurnBand,
        request: &WrappedConeNormalRequest,
        options: NormalExtrusionOptions,
    ) -> Checked<Self> {
        let stock = band_admission::admit(*band, request.depths, options)?;
        let FullTurnSupport::Cone { support, slant } = band.support else {
            return Err(incomplete(
                "full conical stock for a curved cone aperture",
                format!("{:?}", band.support),
            ));
        };
        require_support(support, request)?;
        let [selection] = request.selections.as_slice() else {
            return Err(incomplete(
                "one selected conical aperture",
                format!("{} selections", request.selections.len()),
            ));
        };
        if !selection.holes.is_empty() {
            return Err(incomplete(
                "one simply connected conical aperture",
                "nested aperture loops",
            ));
        }
        let crossings = cone_contour(&request.profile, selection, support, slant, options)?;
        Ok(Self {
            stock,
            band: *band,
            region: selection.region,
            crossings,
        })
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
) -> Checked<[Crossing; 2]> {
    let mut curves = contour_curves(profile, selection)?;
    let maximum_x = certify_cone_hull(&curves, support, slant, options)?;
    // atan(y/x) has magnitude at least |y|/(2 max(x)) when |y| <= x.
    // Division by sin(half angle) only increases this bound.
    let denominator = (2. * maximum_x).next_up();
    let angular = effective_limits(options).angular_clearance;
    for curve in &mut curves {
        for point in &mut curve.points {
            point.swap(0, 1);
        }
    }
    let roots = curves
        .iter()
        .map(|curve| curve.crossing(denominator, angular))
        .collect::<Checked<Vec<_>>>()?;
    ordered_crossings(roots.into_iter().flatten().collect())
}

fn certify_cone_hull(
    curves: &[DirectedCurve],
    support: ConeSupport,
    slant: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<f64> {
    if !curves.iter().any(DirectedCurve::is_curved) {
        return Err(incomplete(
            "at least one conical polynomial curve",
            "only lines",
        ));
    }
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
    certify_phase(support, y, min_x)?;
    Ok(max_x)
}

fn certify_phase(support: ConeSupport, y: f64, min_x: f64) -> Checked<()> {
    let developed = certify_developed_period(DevelopedScale::ConeSemiAngle(support.semi_angle))
        .map_err(|cause| incomplete("conical aperture angular scale", format!("{cause:?}")))?;
    let phase = ((y / min_x).next_up() / developed.scale[0]).next_up();
    cylinder_admission::require(
        phase < PI,
        "curved cone periodic overlap",
        phase,
        "entire developed contour within the two sheets adjoining the zero ray",
    )
}
