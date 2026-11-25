//! Resolve each source use and bind the exact authored region before fitting.
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileLoop, ProfilePlane, ProfileVertex, ProfileVertexId, SketchProfile,
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedNativeSplineSpan, CertifiedRationalConicSegment, CylindricalProfileOptions,
    MetricBezier, MetricCurve, OrientedMetricCurve, RationalReferenceBounds,
};

pub(super) struct Source<'a> {
    pub selection: &'a WrappedRegionSelection,
    pub profile: SketchProfile,
    pub curves: Vec<Curve<'a>>,
    pub lines: Vec<Line>,
}

pub(super) struct Curve<'a> {
    pub spans: Vec<NativeSpan<'a>>,
    pub occurrence: LoopUse,
    pub ends: Option<[ProfileVertexId; 2]>,
}

pub(super) struct NativeSpan<'a> {
    controls: NativeControls<'a>,
    pub domain: WrappedSourceSpan,
    pub map: Option<ParameterMap>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ParameterMap {
    Quarter(WrappedConicParameterMap),
    Analytic(WrappedAnalyticArcMap),
    Admitted(WrappedAdmittedArcMap),
    Spline(WrappedSplineParameterMap),
}

enum NativeControls<'a> {
    Spline(MetricBezier<'a, 2>),
    Quarter([[f64; 2]; 3]),
    TransportedConic(Box<CertifiedRationalConicSegment>),
    SewnConic(Box<conic::SewnConic>),
    TransportedSpline(CertifiedNativeSplineSpan),
}

pub(super) struct Line {
    pub id: ProfileCurveId,
    pub ends: [ProfileVertexId; 2],
    pub points: [[f64; 2]; 2],
}

mod conic;
mod selection;
mod spline;
pub(super) use selection::select_cell;
use selection::{VertexMap, point, resolve_uses, selected, selected_profile, vertices};

pub(super) fn read<'a>(
    profile: &'a SketchProfile,
    selections: &'a [WrappedRegionSelection],
    options: WrappedNormalOptions,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Source<'a>> {
    let selection = selected(profile, selections)?;
    let uses = resolve_uses(selection, profile)?;
    for (_, curve) in &uses {
        spline::preflight(curve, cancelled)?;
    }
    let vertices = vertices(profile)?;
    let profile = selected_profile(&uses, &vertices)?;
    let mut lines = Vec::new();
    let mut curves = Vec::new();
    let mut transport_policy = CylindricalProfileOptions::default();
    for (used, c) in &uses {
        match c {
            ProfileCurve::Line { id, start, end } => {
                lines.push(line(*id, [*start, *end], *used, &vertices)?)
            }
            ProfileCurve::Spline { .. } => {
                curves.push(spline::rational(
                    c,
                    *used,
                    &vertices,
                    &mut transport_policy,
                    cancelled,
                )?);
            }
            ProfileCurve::Arc { .. } | ProfileCurve::EllipticArc { .. } => {
                curves.push(conic::analytic_arc(
                    c,
                    *used,
                    &profile,
                    options,
                    &mut transport_policy,
                    cancelled,
                )?);
            }
            ProfileCurve::Circle { .. } | ProfileCurve::Ellipse { .. } => {
                curves.push(conic::closed_conic(
                    *used,
                    &profile,
                    options,
                    &mut transport_policy,
                    cancelled,
                )?);
            }
        }
    }
    if curves.is_empty() {
        return Err(invalid(
            format!(
                "selected region={:?} with no rational boundary",
                selection.region
            ),
            "at least one curved boundary use",
        ));
    }
    Ok(Source {
        selection,
        profile,
        curves,
        lines,
    })
}

fn line(
    id: ProfileCurveId,
    mut ends: [ProfileVertexId; 2],
    used: LoopUse,
    vertices: &VertexMap,
) -> Checked<Line> {
    if used.reversed {
        ends.reverse();
    }
    Ok(Line {
        id,
        ends,
        points: [point(vertices, ends[0])?, point(vertices, ends[1])?],
    })
}

impl Source<'_> {
    pub(super) fn rounding_operations(&self) -> u32 {
        self.curves
            .iter()
            .flat_map(|curve| &curve.spans)
            .filter_map(NativeSpan::transport)
            .map(|proof| proof.operations)
            .sum()
    }

    pub(super) fn rounded_area_error(&self) -> f64 {
        self.curves
            .iter()
            .flat_map(|curve| &curve.spans)
            .filter_map(NativeSpan::area_error)
            .fold(0., |sum, bound| (sum + bound).next_up())
    }

    pub(super) fn poles(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        self.curves
            .iter()
            .flat_map(|curve| curve.spans.iter().flat_map(NativeSpan::support_hull))
    }

    pub(super) fn area_wire(
        &self,
        boundary: &ProfileLoop,
    ) -> Checked<Vec<OrientedMetricCurve<'_>>> {
        let mut wire = Vec::new();
        for used in &boundary.uses {
            if let Some(curve) = self
                .curves
                .iter()
                .find(|curve| curve.occurrence.curve == used.curve)
            {
                wire.extend(curve.metric_spans(used.reversed));
            } else {
                wire.push(self.line_metric(used.curve)?);
            }
        }
        Ok(wire)
    }

    fn line_metric(&self, curve: ProfileCurveId) -> Checked<OrientedMetricCurve<'_>> {
        let line = self
            .lines
            .iter()
            .find(|line| line.id == curve)
            .ok_or_else(|| {
                invalid(
                    format!("source curve={curve:?}"),
                    "one certified directed line",
                )
            })?;
        Ok(OrientedMetricCurve {
            curve: MetricCurve::Bezier(MetricBezier {
                poles: &line.points,
                weights: None,
            }),
            reversed: false,
        })
    }
}

impl Curve<'_> {
    pub(super) fn endpoint_ids(&self) -> [Option<ProfileVertexId>; 2] {
        self.ends.map_or([None; 2], |ends| ends.map(Some))
    }

    pub(super) fn poles(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        self.spans.iter().flat_map(NativeSpan::hull_points)
    }

    pub(super) fn endpoints(&self) -> [[f64; 2]; 2] {
        let first = self.spans[0].metric().poles[0];
        let last = self.spans[self.spans.len() - 1].metric().poles;
        [first, last[last.len() - 1]]
    }

    pub(super) fn is_mapped_spline(&self) -> bool {
        matches!(self.spans[0].map, Some(ParameterMap::Spline(_)))
    }

    pub(super) fn needs_planar_partition(&self) -> bool {
        self.is_mapped_spline()
            || self.spans.len() > 1 && matches!(self.spans[0].controls, NativeControls::Spline(_))
    }

    pub(super) fn has_sewn_conic_partition(&self) -> bool {
        self.spans.len() > 1 && matches!(self.spans[0].controls, NativeControls::SewnConic(_))
    }

    fn metric_spans(&self, reversed: bool) -> impl Iterator<Item = OrientedMetricCurve<'_>> {
        let n = self.spans.len();
        (0..n).map(move |index| {
            let index = if reversed { n - 1 - index } else { index };
            OrientedMetricCurve {
                curve: MetricCurve::Bezier(self.spans[index].metric()),
                reversed,
            }
        })
    }
}

impl NativeSpan<'_> {
    pub(super) fn enclosure(&self) -> Option<&[[[f64; 2]; 2]]> {
        match &self.controls {
            NativeControls::TransportedConic(value) => Some(&value.control_enclosures),
            NativeControls::TransportedSpline(value) => Some(&value.control_enclosures),
            _ => None,
        }
    }

    pub(super) fn transport(&self) -> Option<RationalReferenceBounds> {
        match &self.controls {
            NativeControls::TransportedConic(value) => Some(value.transport),
            NativeControls::TransportedSpline(value) => Some(value.transport),
            NativeControls::SewnConic(value) => Some(value.proof.transport),
            _ => None,
        }
    }

    fn area_error(&self) -> Option<f64> {
        match &self.controls {
            NativeControls::TransportedConic(value) => Some(value.area_transfer),
            NativeControls::TransportedSpline(value) => Some(value.area_transfer),
            NativeControls::SewnConic(value) => Some(value.proof.area_transfer),
            _ => None,
        }
    }

    fn hull_points(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        self.metric().poles.iter().copied().chain(
            self.enclosure()
                .into_iter()
                .flatten()
                .flat_map(|&bbox| enclosure_corners(bbox)),
        )
    }

    fn support_hull(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        let sewn = match &self.controls {
            NativeControls::SewnConic(value) => Some(value.as_ref()),
            _ => None,
        };
        self.hull_points().chain(sewn.into_iter().flat_map(|value| {
            let error = value.proof.transport.position;
            value.poles.iter().flat_map(move |p| {
                enclosure_corners(p.map(|x| [(x - error).next_down(), (x + error).next_up()]))
            })
        }))
    }

    pub(super) fn sewn_center(&self) -> Option<[f64; 2]> {
        if let NativeControls::SewnConic(value) = &self.controls {
            Some(value.center)
        } else {
            None
        }
    }

    pub(super) fn metric(&self) -> MetricBezier<'_, 2> {
        match &self.controls {
            NativeControls::Spline(curve) => *curve,
            NativeControls::TransportedConic(lowered) => MetricBezier {
                poles: &lowered.poles,
                weights: Some(&lowered.weights),
            },
            NativeControls::SewnConic(lowered) => MetricBezier {
                poles: &lowered.poles,
                weights: Some(&lowered.weights),
            },
            NativeControls::TransportedSpline(lowered) => MetricBezier {
                poles: &lowered.poles,
                weights: lowered.weights.as_deref(),
            },
            NativeControls::Quarter(poles) => MetricBezier {
                poles,
                weights: Some(&[1., 1., 2.]),
            },
        }
    }

    pub(super) fn direction(&self) -> [f64; 2] {
        let poles = self.metric().poles;
        let first = poles[0];
        let last = poles[poles.len() - 1];
        std::array::from_fn(|axis| last[axis] - first[axis])
    }
}

pub(super) fn enclosure_corners([[x0, x1], [y0, y1]]: [[f64; 2]; 2]) -> [[f64; 2]; 4] {
    [[x0, y0], [x0, y1], [x1, y0], [x1, y1]]
}

fn unit_interval() -> WrappedSourceSpan {
    WrappedSourceSpan {
        level: 0,
        index: 0,
        reversed: false,
    }
}
