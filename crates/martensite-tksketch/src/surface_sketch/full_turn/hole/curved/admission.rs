// SPDX-License-Identifier: GPL-2.0-only
//! Prove the first curved cut from the exact authored polynomial chart controls.
use super::*;
use martensite_tkbo::profile_extrusion::{
    ProfileCurve, ProfilePlane, ProfileVertexId, SketchProfile,
};
use martensite_tkmath::predicates::roots::{
    RootIsolationOptions, isolate_bernstein_roots,
    metric_bounds::{DevelopedScale, certify_developed_period},
};
use martensite_tkmath::predicates::{OrientationSign, orient2d};

mod cone;

pub(super) struct AperturePlan {
    pub stock: band_admission::BandSection,
    pub band: FullTurnBand,
    pub contours: Vec<Aperture>,
    pub rails: Vec<[PeriodicSeamEnd; 2]>,
}

pub(super) struct Aperture {
    pub region: SurfaceRegionId,
    pub curves: Vec<SurfaceCurveId>,
    pub crossings: Option<[Crossing; 2]>,
    pub meridian: [f64; 2],
    curved: bool,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum Scope {
    OneCrossed,
    Batch,
}

#[derive(Clone, Copy)]
pub(super) struct Crossing {
    pub report: PeriodicCrossing,
    pub ends: [SurfaceVertexId; 2],
    pub angular_speed: f64,
    pub height_hull: [f64; 2],
}

impl AperturePlan {
    pub(super) fn admit(
        band: &FullTurnBand,
        request: &WrappedNormalRequest,
        options: NormalExtrusionOptions,
        scope: Scope,
    ) -> Checked<Self> {
        let stock = band_admission::admit(*band, request.depths, options)?;
        let (support, axial) = cylinder(band.support)?;
        check_support(support, request)?;
        let contours = request
            .selections
            .iter()
            .map(|selection| cylinder_contour(request, selection, support, axial, options))
            .collect::<Checked<Vec<_>>>()?;
        Self::compose(*band, stock, contours, scope, options)
    }

    fn compose(
        band: FullTurnBand,
        stock: band_admission::BandSection,
        mut contours: Vec<Aperture>,
        scope: Scope,
        options: NormalExtrusionOptions,
    ) -> Checked<Self> {
        if !contours.iter().any(|hole| hole.curved) {
            return Err(incomplete(
                "a genuinely curved aperture span",
                "no nonlinear polynomial",
            ));
        }
        if scope == Scope::OneCrossed
            && !matches!(contours.as_slice(), [hole] if hole.crossings.is_some())
        {
            return Err(incomplete(
                "one curved aperture with two transverse seam crossings",
                format!("{} apertures", contours.len()),
            ));
        }
        contours.sort_by_key(|hole| hole.region);
        if contours
            .windows(2)
            .any(|pair| pair[0].region == pair[1].region)
        {
            return Err(incomplete(
                "unique authored aperture regions",
                "duplicate region",
            ));
        }
        let rails = seam_intervals(band.rims, &contours, options)?;
        Ok(Self {
            band,
            stock,
            contours,
            rails,
        })
    }

    pub(super) fn crossing_count(&self) -> usize {
        self.contours
            .iter()
            .filter(|hole| hole.crossings.is_some())
            .count()
            * 2
    }

    pub(super) fn crossings(&self) -> impl Iterator<Item = Crossing> + '_ {
        self.contours
            .iter()
            .filter_map(|hole| hole.crossings)
            .flatten()
    }

    pub(super) fn clear_contours(&self) -> impl Iterator<Item = &Aperture> {
        self.contours.iter().filter(|hole| hole.crossings.is_none())
    }

    pub(super) fn minimum_radius(&self) -> f64 {
        let min = self
            .stock
            .meridian
            .iter()
            .map(|point| point[0])
            .fold(f64::INFINITY, f64::min);
        (min - self.stock.sweep.coordinate_error).next_down()
    }
}

fn seam_intervals(
    rims: [SurfaceCurveId; 2],
    contours: &[Aperture],
    options: NormalExtrusionOptions,
) -> Checked<Vec<[PeriodicSeamEnd; 2]>> {
    let mut along = contours.iter().collect::<Vec<_>>();
    along.sort_by(|a, b| a.meridian[0].total_cmp(&b.meridian[0]));
    verify_separation(&along, options)?;
    let mut rails = Vec::with_capacity(along.len() + 1);
    let mut front = PeriodicSeamEnd::Rim(rims[0]);
    for hole in along {
        let Some([start, end]) = hole.crossings else {
            continue;
        };
        rails.push([front, PeriodicSeamEnd::Crossing(start.report.id)]);
        front = PeriodicSeamEnd::Crossing(end.report.id);
    }
    rails.push([front, PeriodicSeamEnd::Rim(rims[1])]);
    Ok(rails)
}

fn verify_separation(contours: &[&Aperture], options: NormalExtrusionOptions) -> Checked<()> {
    let floor = effective_limits(options).linear_separation / 2.;
    for pair in contours.windows(2) {
        let gap = (pair[1].meridian[0] - pair[0].meridian[1]).next_down();
        cylinder_admission::separated("curved aperture meridian separation", gap, floor)?;
    }
    Ok(())
}

fn check_support(support: CylinderSupport, request: &WrappedNormalRequest) -> Checked<()> {
    if request.support == support
        && support.origin == [0.; 3]
        && support.axis == [0., 0., 1.]
        && support.radial == [1., 0., 0.]
        && support.chart_origin == 0.
        && request.profile.plane == ProfilePlane::world_xy()
    {
        return Ok(());
    }
    Err(incomplete(
        "matching canonical cylindrical support at the zero seam",
        format!("{:?}", request.support),
    ))
}

fn cylinder_contour(
    request: &WrappedNormalRequest,
    selection: &WrappedRegionSelection,
    support: CylinderSupport,
    axial: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<Aperture> {
    let mut bounds = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
    let curves = contour_curves(&request.profile, selection)?;
    for curve in &curves {
        extend_bounds(&mut bounds, &curve.points);
    }
    certify_cylinder_hull(bounds, support.radius, axial, options)?;
    let margin = effective_limits(options).angular_clearance;
    let roots = curves
        .iter()
        .map(|curve| curve.crossing(support.radius, margin))
        .collect::<Checked<Vec<_>>>()?;
    aperture(
        selection,
        curves,
        bounds[1],
        roots.into_iter().flatten().collect(),
    )
}

fn aperture(
    selection: &WrappedRegionSelection,
    curves: Vec<DirectedCurve>,
    meridian: [f64; 2],
    roots: Vec<Crossing>,
) -> Checked<Aperture> {
    let crossings = if roots.is_empty() {
        None
    } else {
        Some(ordered_crossings(roots)?)
    };
    let curved = curves.iter().any(DirectedCurve::is_curved);
    let curves = curves.iter().map(|curve| curve.id).collect();
    Ok(Aperture {
        region: selection.region,
        curves,
        crossings,
        meridian,
        curved,
    })
}

fn contour_curves(
    profile: &SketchProfile,
    selection: &WrappedRegionSelection,
) -> Checked<Vec<DirectedCurve>> {
    if !selection.holes.is_empty() {
        return Err(incomplete(
            "one boundary per curved aperture",
            "nested aperture loops",
        ));
    }
    selection
        .outer
        .uses
        .iter()
        .map(|edge| {
            let curve = profile
                .curves
                .iter()
                .find(|curve| curve.id() == edge.curve)
                .ok_or_else(|| {
                    incomplete("authored aperture curve", format!("{:?}", edge.curve))
                })?;
            DirectedCurve::new(profile, curve, edge.reversed)
        })
        .collect()
}

fn extend_bounds(bounds: &mut [[f64; 2]; 2], points: &[[f64; 2]]) {
    for point in points {
        for (bounds, coordinate) in bounds.iter_mut().zip(point) {
            bounds[0] = bounds[0].min(*coordinate);
            bounds[1] = bounds[1].max(*coordinate);
        }
    }
}

fn certify_cylinder_hull(
    [chart, height]: [[f64; 2]; 2],
    radius: f64,
    axial: [f64; 2],
    options: NormalExtrusionOptions,
) -> Checked<()> {
    require_rim_clearance(axial, height, options)?;
    let period = certify_developed_period(DevelopedScale::CylinderRadius(radius))
        .map_err(|cause| incomplete("curved aperture exact period", format!("{cause:?}")))?;
    cylinder_admission::require(
        (chart[1] - chart[0]).next_up() < period.period[0],
        "periodic aperture overlap",
        chart[1] - chart[0],
        "profile hull narrower than a certified period",
    )?;
    let largest = chart[0].abs().max(chart[1].abs());
    cylinder_admission::require(
        largest < period.period[0] / 2.,
        "curved aperture angular lift",
        largest,
        "strictly inside the two sheets adjoining the zero cut",
    )
}

fn ordered_crossings(mut roots: Vec<Crossing>) -> Checked<[Crossing; 2]> {
    roots.sort_by_key(|crossing| crossing.report.id.transition);
    let count = roots.len();
    let [a, b]: [Crossing; 2] = roots
        .try_into()
        .map_err(|_| incomplete("two transverse curve crossings", format!("{count} roots")))?;
    if a.report.id.transition == b.report.id.transition {
        return Err(incomplete(
            "opposite seam crossings",
            format!("{:?}", [a.report.id, b.report.id]),
        ));
    }
    if a.height_hull[1] < b.height_hull[0] {
        return Ok([a, b]);
    }
    if b.height_hull[1] < a.height_hull[0] {
        return Ok([b, a]);
    }
    Err(incomplete(
        "separated axial seam crossings",
        format!("{:?} and {:?}", a.height_hull, b.height_hull),
    ))
}

struct DirectedCurve {
    id: SurfaceCurveId,
    ends: [SurfaceVertexId; 2],
    points: Vec<[f64; 2]>,
}

impl DirectedCurve {
    fn new(profile: &SketchProfile, curve: &ProfileCurve, reversed: bool) -> Checked<Self> {
        if reversed {
            return Err(incomplete(
                "forward-authored aperture edges",
                format!("{:?}", curve.id()),
            ));
        }
        let [start, end] = curve.ends().ok_or_else(|| {
            incomplete(
                "explicit curved aperture endpoints",
                format!("{:?}", curve.id()),
            )
        })?;
        let [first, last] = [vertex(profile, start)?, vertex(profile, end)?];
        let points = match curve {
            ProfileCurve::Line { .. } => vec![first, last],
            ProfileCurve::Spline {
                degree: 2,
                poles,
                weights: None,
                knots,
                ..
            } if poles.len() == 3
                && knots == &[0., 0., 0., 1., 1., 1.]
                && poles[0] == first
                && poles[2] == last =>
            {
                poles.clone()
            }
            _ => {
                return Err(incomplete(
                    "line or one clamped polynomial quadratic",
                    format!("{curve:?}"),
                ));
            }
        };
        points
            .iter()
            .try_for_each(|p| cylinder_admission::finite("curved aperture control", p))?;
        Ok(Self {
            id: SurfaceCurveId(curve.id().0),
            ends: [start, end].map(|v| SurfaceVertexId(v.0)),
            points,
        })
    }

    fn is_curved(&self) -> bool {
        let [a, b, c] = self.points.as_slice() else {
            return false;
        };
        matches!(
            orient2d(*a, *b, *c),
            Ok(OrientationSign::Positive | OrientationSign::Negative)
        )
    }

    fn crossing(&self, radius: f64, angular: f64) -> Checked<Option<Crossing>> {
        let values = self.points.iter().map(|p| p[0]).collect::<Vec<_>>();
        let first = values[0];
        let last = values[values.len() - 1];
        let margin = (radius * angular).next_up();
        if values.iter().all(|&v| v > margin) || values.iter().all(|&v| v < -margin) {
            return Ok(None);
        }
        if first.abs().min(last.abs()) <= margin || first.signum() == last.signum() {
            return Err(incomplete(
                "transverse crossing clear of authored corners",
                format!("{:?}: {values:?}", self.id),
            ));
        }
        let (transition, speed) = monotone(&values, margin, radius)?;
        let normalized_parameter = isolate(&values)?;
        let id = PeriodicCrossingId {
            curve: self.id,
            turn: 0,
            transition,
        };
        Ok(Some(Crossing {
            report: PeriodicCrossing {
                id,
                normalized_parameter,
            },
            ends: self.ends,
            angular_speed: speed,
            height_hull: [
                self.points
                    .iter()
                    .map(|p| p[1])
                    .fold(f64::INFINITY, f64::min),
                self.points
                    .iter()
                    .map(|p| p[1])
                    .fold(f64::NEG_INFINITY, f64::max),
            ],
        }))
    }
}

fn monotone(values: &[f64], margin: f64, radius: f64) -> Checked<(SheetTransition, f64)> {
    let sign = values[0].signum();
    let min = values
        .windows(2)
        .map(|pair| ((pair[0] - pair[1]) * sign).next_down())
        .fold(f64::INFINITY, f64::min);
    cylinder_admission::require(
        min > margin,
        "curve cut monotonicity",
        min,
        "positive quantitative derivative distance from tangency",
    )?;
    let transition = if sign < 0. {
        SheetTransition::Increasing
    } else {
        SheetTransition::Decreasing
    };
    let speed = ((min * (values.len() - 1) as f64).next_down() / radius).next_down();
    Ok((transition, speed))
}

fn isolate(values: &[f64]) -> Checked<[f64; 2]> {
    let roots = isolate_bernstein_roots(
        values,
        [0., 1.],
        RootIsolationOptions::new(1. / (1u64 << 48) as f64),
        || false,
    )
    .map_err(|cause| incomplete("exact curved cut root isolation", format!("{cause:?}")))?;
    let [root] = roots.roots() else {
        return Err(incomplete(
            "one distinct transverse source root",
            format!("{:?}", roots.roots()),
        ));
    };
    let (lo, hi) = root.bounds();
    let pair = lo
        .binary64_bounds()
        .zip(hi.binary64_bounds())
        .map(|(a, b)| [a[0], b[1]]);
    pair.filter(|[a, b]| root.multiplicity() == 1 && 0. < *a && *b < 1.)
        .ok_or_else(|| incomplete("interior simple source root bounds", format!("{root:?}")))
}

fn vertex(profile: &SketchProfile, id: ProfileVertexId) -> Checked<[f64; 2]> {
    profile
        .vertices
        .iter()
        .find(|vertex| vertex.id == id)
        .map(|vertex| vertex.point)
        .ok_or_else(|| incomplete("authored aperture vertex", format!("{id:?}")))
}
