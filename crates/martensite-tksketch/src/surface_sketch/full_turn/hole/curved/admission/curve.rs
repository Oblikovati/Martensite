// SPDX-License-Identifier: GPL-2.0-only
//! Prove crossing signs on literal polynomial or positive-rational Bernstein controls.
use super::*;
use martensite_tkbo::profile_extrusion::{
    ProfileCurve, ProfileVertexId, Resolution, exact_conic_quarter,
};
use martensite_tkmath::predicates::roots::{RootIsolationOptions, isolate_bernstein_roots};
use martensite_tkmath::predicates::{OrientationSign, orient2d};

mod repeated;

#[derive(Clone, Copy)]
pub(super) enum AngularSpeed {
    Whole(f64),
    Restricted {
        derivative: [[f64; 2]; 2],
        scale: f64,
    },
}

impl AngularSpeed {
    pub(super) fn certify(self, span: [f64; 2], transition: SheetTransition) -> Checked<f64> {
        match self {
            Self::Whole(value) => Ok(value),
            Self::Restricted { derivative, scale } => {
                repeated::speed(derivative, span, transition, scale)
            }
        }
    }
}

pub(super) struct DirectedCurve {
    pub(super) id: SurfaceCurveId,
    ends: [SurfaceVertexId; 2],
    pub(super) points: Vec<[f64; 2]>,
    weights: Option<[f64; 3]>,
    conic_map: Option<WrappedConicParameterMap>,
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
        let net = source_net(profile, curve, [first, last], model)?;
        net.points
            .iter()
            .try_for_each(|p| cylinder_admission::finite("curved aperture control", p))?;
        Ok(Self {
            id: SurfaceCurveId(curve.id().0),
            ends: [start, end].map(|v| SurfaceVertexId(v.0)),
            points: net.points,
            weights: net.weights,
            conic_map: net.conic_map,
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

    pub(super) fn crossing(&self, scale: f64, angular: f64) -> Checked<Vec<Crossing>> {
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
    ) -> Checked<Vec<Crossing>> {
        let signed = self.points.iter().map(|p| cut.signed(p[0]));
        let (bounds, denominator) = self.homogeneous(signed);
        let scale = (scale * denominator).next_up();
        let margin = (scale * angular).next_up();
        let Some(sides) = crossing_sides(self.id, &bounds, margin)? else {
            return Ok(Vec::new());
        };
        if sides[0] == sides[1] {
            return self.repeated_crossings(cut.turn, &bounds, sides, scale);
        }
        let direction = direction(sides[0]);
        let speed = monotone(&bounds, direction, margin, scale)?;
        let parameter = isolate_between(&bounds)?;
        let crossing = self.crossing_record(
            PeriodicCrossingId {
                curve: self.id,
                turn: cut.turn,
                ordinal: 0,
                transition: direction,
            },
            parameter,
            AngularSpeed::Whole(speed),
            self.height_hull(),
        );
        Ok(vec![crossing])
    }

    fn repeated_crossings(
        &self,
        turn: i64,
        bounds: &[[f64; 2]],
        sides: [i8; 2],
        scale: f64,
    ) -> Checked<Vec<Crossing>> {
        let controls = bounds.try_into().map_err(|_| {
            incomplete(
                "three signed controls for two transverse quadratic roots",
                format!("{:?}", self.id),
            )
        })?;
        let events = repeated::isolate_two(controls, sides)?;
        let [a, b] = events;
        Ok(vec![
            self.repeated_record(turn, 0, a, scale)?,
            self.repeated_record(turn, 1, b, scale)?,
        ])
    }

    fn repeated_record(
        &self,
        turn: i64,
        ordinal: u8,
        event: repeated::Event,
        scale: f64,
    ) -> Checked<Crossing> {
        let id = PeriodicCrossingId {
            curve: self.id,
            turn,
            ordinal,
            transition: event.transition,
        };
        let meridian = self.height_bound_at(event.parameter)?;
        cylinder_admission::finite("repeated source root meridian enclosure", &meridian)?;
        let speed = AngularSpeed::Restricted {
            derivative: event.derivative,
            scale,
        };
        Ok(self.crossing_record(id, event.parameter, speed, meridian))
    }

    fn crossing_record(
        &self,
        id: PeriodicCrossingId,
        normalized_parameter: [f64; 2],
        speed: AngularSpeed,
        height_hull: [f64; 2],
    ) -> Crossing {
        Crossing {
            report: PeriodicCrossing {
                id,
                normalized_parameter,
                conic_map: self.conic_map,
            },
            ends: self.ends,
            speed,
            height_hull,
            contour_transition: id.transition.oriented(self.reversed),
        }
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

    fn height_bound_at(&self, t: [f64; 2]) -> Checked<[f64; 2]> {
        let [a, b, c] = self.points.as_slice() else {
            return Err(incomplete(
                "quadratic meridian controls",
                format!("{:?}", self.id),
            ));
        };
        let heights = [a[1], b[1], c[1]];
        let Some(weights) = self.weights else {
            return Ok(repeated::bezier_value(heights.map(|h| [h; 2]), t));
        };
        let controls: [[f64; 2]; 3] =
            std::array::from_fn(|i| scale_bounds([heights[i]; 2], weights[i]));
        let numerator = repeated::bezier_value(controls, t);
        let denominator = repeated::bezier_value(weights.map(|w| [w; 2]), t);
        cylinder_admission::require(
            denominator[0] > 0.,
            "meridian root denominator",
            denominator[0],
            "strictly positive enclosure on the original root interval",
        )?;
        let reciprocal = [
            (1. / denominator[1]).next_down(),
            (1. / denominator[0]).next_up(),
        ];
        Ok(repeated::multiply(numerator, reciprocal))
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

struct SourceNet {
    points: Vec<[f64; 2]>,
    weights: Option<[f64; 3]>,
    conic_map: Option<WrappedConicParameterMap>,
}

fn source_net(
    profile: &SketchProfile,
    curve: &ProfileCurve,
    [first, last]: [[f64; 2]; 2],
    model: SourceModel,
) -> Checked<SourceNet> {
    match (curve, model) {
        (ProfileCurve::Line { .. }, _) => Ok(SourceNet {
            points: vec![first, last],
            weights: None,
            conic_map: None,
        }),
        (
            ProfileCurve::Arc { .. } | ProfileCurve::EllipticArc { .. },
            SourceModel::Rational(resolution),
        ) => conic_net(profile, curve, resolution),
        _ => spline_net(curve, [first, last], model),
    }
}

fn spline_net(curve: &ProfileCurve, ends: [[f64; 2]; 2], model: SourceModel) -> Checked<SourceNet> {
    let ProfileCurve::Spline {
        degree: 2,
        poles,
        weights,
        knots,
        ..
    } = curve
    else {
        return Err(incomplete(
            "line, conic quarter or clamped quadratic span",
            format!("{curve:?}"),
        ));
    };
    if poles.len() != 3 || [poles[0], poles[2]] != ends || knots != &[0., 0., 0., 1., 1., 1.] {
        return Err(incomplete(
            "literal clamped quadratic span and its authored endpoints",
            format!("{curve:?}"),
        ));
    }
    Ok(SourceNet {
        points: poles.clone(),
        weights: source_weights(curve.id(), weights.as_deref(), model)?,
        conic_map: None,
    })
}

fn conic_net(
    profile: &SketchProfile,
    curve: &ProfileCurve,
    resolution: Resolution,
) -> Checked<SourceNet> {
    let id = curve.id();
    let quarter = exact_conic_quarter(profile, id, resolution).map_err(|cause| {
        incomplete(
            "admitted exact analytic conic",
            format!("{id:?}: {cause:?}"),
        )
    })?;
    let quarter =
        quarter.ok_or_else(|| incomplete("exact analytic conic quarter", format!("{id:?}")))?;
    Ok(SourceNet {
        points: quarter.poles.into(),
        weights: Some([1., 1., 2.]),
        conic_map: Some(WrappedConicParameterMap {
            first_quarter: quarter.first_quarter,
            increasing: quarter.increasing,
        }),
    })
}

fn source_weights(
    id: martensite_tkbo::profile_extrusion::ProfileCurveId,
    weights: Option<&[f64]>,
    model: SourceModel,
) -> Checked<Option<[f64; 3]>> {
    match (weights, model) {
        (None, _) => Ok(None),
        (Some([a, b, c]), SourceModel::Rational(_))
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

fn crossing_sides(
    curve: SurfaceCurveId,
    bounds: &[[f64; 2]],
    margin: f64,
) -> Checked<Option<[i8; 2]>> {
    let first = sign(bounds[0], margin);
    let last = sign(bounds[bounds.len() - 1], margin);
    if first == 0 || last == 0 {
        return Err(incomplete(
            "transverse crossing clear of authored corners",
            format!("{curve:?}: {bounds:?}"),
        ));
    }
    if bounds.iter().all(|&bound| sign(bound, margin) == first) {
        return Ok(None);
    }
    Ok(Some([first, last]))
}

fn direction(side: i8) -> SheetTransition {
    if side < 0 {
        SheetTransition::Increasing
    } else {
        SheetTransition::Decreasing
    }
}

fn sign([a, b]: [f64; 2], margin: f64) -> i8 {
    if a > margin {
        1
    } else if b < -margin {
        -1
    } else {
        0
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
    let first = isolate(&lower, 1)?[0];
    if lower == upper {
        return Ok(first);
    }
    let second = isolate(&upper, 1)?[0];
    Ok([first[0].min(second[0]), first[1].max(second[1])])
}

fn isolate(values: &[f64], expected: usize) -> Checked<Vec<[f64; 2]>> {
    let partition = isolate_bernstein_roots(
        values,
        [0., 1.],
        RootIsolationOptions::new(1. / (1u64 << 48) as f64),
        || false,
    )
    .map_err(|cause| incomplete("exact curved cut root isolation", format!("{cause:?}")))?;
    let roots = partition.roots();
    if roots.len() != expected {
        return Err(incomplete(
            "expected distinct transverse source roots",
            format!("{roots:?}"),
        ));
    }
    roots
        .iter()
        .map(|root| {
            let (lo, hi) = root.bounds();
            let pair = lo
                .binary64_bounds()
                .zip(hi.binary64_bounds())
                .map(|(a, b)| [a[0], b[1]]);
            pair.filter(|[a, b]| root.multiplicity() == 1 && 0. < *a && *b < 1.)
                .ok_or_else(|| {
                    incomplete("interior simple source root bounds", format!("{root:?}"))
                })
        })
        .collect()
}

fn vertex(profile: &SketchProfile, id: ProfileVertexId) -> Checked<[f64; 2]> {
    profile
        .vertices
        .iter()
        .find(|vertex| vertex.id == id)
        .map(|vertex| vertex.point)
        .ok_or_else(|| incomplete("authored aperture vertex", format!("{id:?}")))
}
