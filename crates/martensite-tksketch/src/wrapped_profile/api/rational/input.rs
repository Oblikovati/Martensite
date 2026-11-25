//! Resolve each source use and bind the exact authored region before fitting.
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileLoop, ProfilePlane, ProfileVertex, ProfileVertexId,
    SketchProfile, exact_bernstein_partition,
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedRationalConicSegment, CylindricalProfileOptions, MetricBezier, MetricCurve,
    OrientedMetricCurve, RationalReferenceBounds,
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
    pub map: Option<ConicMap>,
}

#[derive(Clone, Copy)]
pub(super) enum ConicMap {
    Quarter(WrappedConicParameterMap),
    Analytic(WrappedAnalyticArcMap),
}

enum NativeControls<'a> {
    Spline(MetricBezier<'a, 2>),
    Quarter([[f64; 2]; 3]),
    Transported(Box<CertifiedRationalConicSegment>),
}

pub(super) struct Line {
    pub id: ProfileCurveId,
    pub ends: [ProfileVertexId; 2],
    pub points: [[f64; 2]; 2],
}

mod conic;
mod selection;
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
    let vertices = vertices(profile)?;
    let profile = selected_profile(&uses, &vertices)?;
    let mut lines = Vec::new();
    let mut curves = Vec::new();
    let mut analytic_options = CylindricalProfileOptions::default();
    for (used, c) in &uses {
        match c {
            ProfileCurve::Line { id, start, end } => {
                lines.push(line(*id, [*start, *end], *used, &vertices)?)
            }
            ProfileCurve::Spline { .. } => curves.push(rational(c, *used, &vertices)?),
            ProfileCurve::Arc { .. } | ProfileCurve::EllipticArc { .. } => {
                curves.push(conic::analytic_arc(
                    c,
                    *used,
                    &profile,
                    options,
                    &mut analytic_options,
                    cancelled,
                )?);
            }
            ProfileCurve::Circle { .. } | ProfileCurve::Ellipse { .. } => {
                curves.push(conic::closed_conic(*used, &profile, options)?);
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

fn rational<'a>(
    curve: &'a ProfileCurve,
    used: LoopUse,
    vertices: &VertexMap,
) -> Checked<Curve<'a>> {
    let ProfileCurve::Spline {
        id,
        start,
        end,
        degree,
        poles,
        weights,
        knots,
    } = curve
    else {
        return Err(invalid(
            format!("curve={:?}", curve.id()),
            "a clamped positive-rational Bézier",
        ));
    };
    let spans = partition(*id, *degree as usize, poles, weights.as_deref(), knots)?;
    let n = poles.len();
    let endpoints = [point(vertices, *start)?, point(vertices, *end)?];
    if endpoints.map(|p| p.map(f64::to_bits))
        != [poles[0], poles[n - 1]].map(|p| p.map(f64::to_bits))
    {
        return Err(invalid(
            format!(
                "curve={id:?} authored vertices={endpoints:?}, poles={:?}",
                [poles[0], poles[n - 1]]
            ),
            "bitwise matching authored endpoints and first and last poles",
        ));
    }
    Ok(Curve {
        spans,
        occurrence: used,
        ends: Some([*start, *end]),
    })
}

fn partition<'a>(
    id: ProfileCurveId,
    degree: usize,
    poles: &'a [[f64; 2]],
    weights: Option<&'a [f64]>,
    knots: &'a [f64],
) -> Checked<Vec<NativeSpan<'a>>> {
    let spans = exact_bernstein_partition(degree, poles, weights, knots)
        .filter(|spans| spans.len() <= 64 && knots[0] == 0. && knots[knots.len() - 1] == 1.)
        .ok_or_else(|| {
            invalid(
                format!(
                    "curve={id:?}, degree={degree}, poles={}, weights={}, knots={knots:?}",
                    poles.len(),
                    weights.map_or(0, <[f64]>::len)
                ),
                "one through 64 finite native Bernstein intervals with positive weights when supplied on [0,1]",
            )
        })?;
    spans.map(|piece| bind_span(id, piece)).collect()
}

fn bind_span(
    id: ProfileCurveId,
    piece: martensite_tkbo::profile_extrusion::NativeBernsteinSpan<'_>,
) -> Checked<NativeSpan<'_>> {
    let domain = dyadic_cell(piece.interval)
        .filter(|_| piece.matches_homogeneous_roundtrip())
        .ok_or_else(|| {
            invalid(
                format!("curve={id:?}, native interval={:?}", piece.interval),
                "an exact dyadic interval with unaltered homogeneous planar controls",
            )
        })?;
    Ok(NativeSpan {
        controls: NativeControls::Spline(MetricBezier {
            poles: piece.poles,
            weights: piece.weights,
        }),
        domain,
        map: None,
    })
}

fn dyadic_cell([first, last]: [f64; 2]) -> Option<WrappedSourceSpan> {
    if !(0. <= first && first < last && last <= 1.) {
        return None;
    }
    for level in 0..=63 {
        let scale = (1_u64 << level) as f64;
        let [a, b] = [first * scale, last * scale];
        if a.fract() == 0. && b.fract() == 0. && a as u64 + 1 == b as u64 {
            return Some(WrappedSourceSpan {
                level,
                index: a as u64,
                reversed: false,
            });
        }
    }
    None
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
            .filter_map(NativeSpan::rounded)
            .map(|rounded| rounded.area_transfer)
            .fold(0., |sum, bound| (sum + bound).next_up())
    }

    pub(super) fn poles(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        self.curves.iter().flat_map(Curve::poles)
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
    fn rounded(&self) -> Option<&CertifiedRationalConicSegment> {
        if let NativeControls::Transported(value) = &self.controls {
            Some(value)
        } else {
            None
        }
    }

    pub(super) fn enclosure(&self) -> Option<&[[[f64; 2]; 2]; 3]> {
        self.rounded().map(|rounded| &rounded.control_enclosures)
    }

    pub(super) fn transport(&self) -> Option<RationalReferenceBounds> {
        self.rounded().map(|rounded| rounded.transport)
    }

    fn hull_points(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        self.metric().poles.iter().copied().chain(
            self.enclosure()
                .into_iter()
                .flatten()
                .flat_map(|&bbox| enclosure_corners(bbox)),
        )
    }

    pub(super) fn metric(&self) -> MetricBezier<'_, 2> {
        match &self.controls {
            NativeControls::Spline(curve) => *curve,
            NativeControls::Transported(lowered) => MetricBezier {
                poles: &lowered.poles,
                weights: Some(&lowered.weights),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_cells_require_one_exact_dyadic_interval() {
        let span = dyadic_cell([0.75, 1.]).unwrap();
        assert_eq!((span.level, span.index), (2, 3));
        assert!(dyadic_cell([0., 0.75]).is_none());
        assert!(dyadic_cell([0., f64::NAN]).is_none());
        assert!(dyadic_cell([1., 0.]).is_none());
    }
}
