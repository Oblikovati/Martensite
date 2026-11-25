// SPDX-License-Identifier: GPL-2.0-only
//! Prove the first curved cut from the exact authored polynomial chart controls.
use super::*;
use martensite_tkbo::profile_extrusion::{ProfilePlane, SketchProfile};

mod cone;
mod curve;
use curve::DirectedCurve;

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum SourceModel {
    Polynomial,
    Rational,
}

#[derive(Clone, Copy)]
pub(super) struct Crossing {
    pub report: PeriodicCrossing,
    pub ends: [SurfaceVertexId; 2],
    pub angular_speed: f64,
    pub height_hull: [f64; 2],
    contour_transition: SheetTransition,
}

impl AperturePlan {
    pub(super) fn admit(
        band: &FullTurnBand,
        request: &WrappedNormalRequest,
        options: NormalExtrusionOptions,
        scope: Scope,
        source: SourceModel,
    ) -> Checked<Self> {
        let stock = band_admission::admit(*band, request.depths, options)?;
        let (support, axial) = cylinder(band.support)?;
        check_support(support, request)?;
        let contours = request
            .selections
            .iter()
            .map(|selection| cylinder_contour(request, selection, support, axial, options, source))
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
    source: SourceModel,
) -> Checked<Aperture> {
    let mut bounds = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
    let curves = contour_curves(&request.profile, selection, source)?;
    for curve in &curves {
        extend_bounds(&mut bounds, &curve.points);
    }
    require_rim_clearance(axial, bounds[1], options)?;
    let lattice = crossed::CutLattice::new(FullTurnSupport::Cylinder { support, axial }, options)?;
    let margin = effective_limits(options).angular_clearance;
    let mut roots = Vec::new();
    if let Some(cut) = lattice.cut_in_hull(bounds[0])? {
        for curve in &curves {
            roots.extend(curve.crossing_at(cut, support.radius, margin)?);
        }
    }
    aperture(selection, curves, bounds[1], roots)
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
    model: SourceModel,
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
            DirectedCurve::new(profile, curve, edge.reversed, model)
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

fn ordered_crossings(roots: Vec<Crossing>) -> Checked<[Crossing; 2]> {
    let count = roots.len();
    let [a, b]: [Crossing; 2] = roots
        .try_into()
        .map_err(|_| incomplete("two transverse curve crossings", format!("{count} roots")))?;
    if a.contour_transition == b.contour_transition {
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
