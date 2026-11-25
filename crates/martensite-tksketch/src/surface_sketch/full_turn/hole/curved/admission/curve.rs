// SPDX-License-Identifier: GPL-2.0-only
//! Prove crossing signs on literal polynomial or positive-rational Bernstein controls.
use super::*;
use martensite_tkbo::profile_extrusion::{ProfileCurve, ProfileVertexId};
use martensite_tkmath::predicates::roots::{RootIsolationOptions, isolate_bernstein_roots};
use martensite_tkmath::predicates::{OrientationSign, orient2d};

pub(super) struct DirectedCurve {
    pub(super) id: SurfaceCurveId,
    ends: [SurfaceVertexId; 2],
    pub(super) points: Vec<[f64; 2]>,
    weights: Option<[f64; 3]>,
    reversed: bool,
}

impl DirectedCurve {
    pub(super) fn new(
        profile: &SketchProfile,
        curve: &ProfileCurve,
        reversed: bool,
        model: SourceModel,
    ) -> Checked<Self> {
        let [start, end] = curve.ends().ok_or_else(|| {
            incomplete(
                "explicit curved aperture endpoints",
                format!("{:?}", curve.id()),
            )
        })?;
        let [first, last] = [vertex(profile, start)?, vertex(profile, end)?];
        let (points, weights) = match curve {
            ProfileCurve::Line { .. } => (vec![first, last], None),
            ProfileCurve::Spline {
                degree: 2,
                poles,
                weights,
                knots,
                ..
            } if poles.len() == 3
                && knots == &[0., 0., 0., 1., 1., 1.]
                && poles[0] == first
                && poles[2] == last =>
            {
                (
                    poles.clone(),
                    source_weights(curve.id(), weights.as_deref(), model)?,
                )
            }
            _ => {
                return Err(incomplete(
                    "line or one clamped quadratic span",
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
            weights,
            reversed,
        })
    }

    pub(super) fn is_curved(&self) -> bool {
        let [a, b, c] = self.points.as_slice() else {
            return false;
        };
        matches!(
            orient2d(*a, *b, *c),
            Ok(OrientationSign::Positive | OrientationSign::Negative)
        )
    }

    pub(super) fn crossing(&self, scale: f64, angular: f64) -> Checked<Option<Crossing>> {
        self.crossing_at(
            crossed::ScalarCut {
                turn: 0,
                coordinate: [0.; 2],
            },
            scale,
            angular,
        )
    }

    pub(super) fn crossing_at(
        &self,
        cut: crossed::ScalarCut,
        scale: f64,
        angular: f64,
    ) -> Checked<Option<Crossing>> {
        let signed = self.points.iter().map(|p| cut.signed(p[0]));
        let (bounds, denominator) = self.homogeneous(signed);
        let scale = (scale * denominator).next_up();
        let margin = (scale * angular).next_up();
        let Some(direction) = crossing_direction(self.id, &bounds, margin)? else {
            return Ok(None);
        };
        let speed = monotone(&bounds, direction, margin, scale)?;
        let normalized_parameter = isolate_between(&bounds)?;
        let id = PeriodicCrossingId {
            curve: self.id,
            turn: cut.turn,
            transition: direction,
        };
        let height_hull = self.height_hull();
        Ok(Some(Crossing {
            report: PeriodicCrossing {
                id,
                normalized_parameter,
            },
            ends: self.ends,
            angular_speed: speed,
            height_hull,
            contour_transition: direction.oriented(self.reversed),
        }))
    }

    fn homogeneous(&self, values: impl Iterator<Item = [f64; 2]>) -> (Vec<[f64; 2]>, f64) {
        let Some(weights) = self.weights else {
            return (values.collect(), 1.);
        };
        let max = weights.into_iter().fold(0., f64::max);
        let bounds = values
            .zip(weights)
            .map(|(pair, weight)| scale_bounds(pair, weight));
        (bounds.collect(), max)
    }

    fn height_hull(&self) -> [f64; 2] {
        self.points
            .iter()
            .map(|point| point[1])
            .fold([f64::INFINITY, f64::NEG_INFINITY], |[lo, hi], v| {
                [lo.min(v), hi.max(v)]
            })
    }
}

fn source_weights(
    id: martensite_tkbo::profile_extrusion::ProfileCurveId,
    weights: Option<&[f64]>,
    model: SourceModel,
) -> Checked<Option<[f64; 3]>> {
    match (weights, model) {
        (None, _) => Ok(None),
        (Some([a, b, c]), SourceModel::Rational)
            if [*a, *b, *c].into_iter().all(|w| w.is_finite() && w > 0.) =>
        {
            Ok(Some([*a, *b, *c]))
        }
        _ => Err(incomplete(
            "three positive rational quadratic weights",
            format!("{id:?}: {weights:?}"),
        )),
    }
}

fn scale_bounds([a, b]: [f64; 2], weight: f64) -> [f64; 2] {
    if a == 0. && b == 0. {
        return [0.; 2];
    }
    [(a * weight).next_down(), (b * weight).next_up()]
}

fn crossing_direction(
    curve: SurfaceCurveId,
    bounds: &[[f64; 2]],
    margin: f64,
) -> Checked<Option<SheetTransition>> {
    let side = |[a, b]: [f64; 2]| {
        if a > margin {
            1
        } else if b < -margin {
            -1
        } else {
            0
        }
    };
    let first = side(bounds[0]);
    let last = side(bounds[bounds.len() - 1]);
    if first != 0 && bounds.iter().all(|&bound| side(bound) == first) {
        return Ok(None);
    }
    match [first, last] {
        [-1, 1] => Ok(Some(SheetTransition::Increasing)),
        [1, -1] => Ok(Some(SheetTransition::Decreasing)),
        _ => Err(incomplete(
            "transverse crossing clear of authored corners",
            format!("{curve:?}: {bounds:?}"),
        )),
    }
}

fn monotone(
    values: &[[f64; 2]],
    transition: SheetTransition,
    margin: f64,
    scale: f64,
) -> Checked<f64> {
    let min = values
        .windows(2)
        .map(|pair| match transition {
            SheetTransition::Increasing => (pair[1][0] - pair[0][1]).next_down(),
            SheetTransition::Decreasing => (pair[0][0] - pair[1][1]).next_down(),
        })
        .fold(f64::INFINITY, f64::min);
    cylinder_admission::require(
        min > margin,
        "curve cut monotonicity",
        min,
        "positive quantitative derivative distance from tangency",
    )?;
    Ok(((min * (values.len() - 1) as f64).next_down() / scale).next_down())
}

fn isolate_between(controls: &[[f64; 2]]) -> Checked<[f64; 2]> {
    let lower = controls
        .iter()
        .map(|interval| interval[0])
        .collect::<Vec<_>>();
    let upper = controls
        .iter()
        .map(|interval| interval[1])
        .collect::<Vec<_>>();
    let first = isolate(&lower)?;
    if lower == upper {
        return Ok(first);
    }
    let second = isolate(&upper)?;
    Ok([first[0].min(second[0]), first[1].max(second[1])])
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
