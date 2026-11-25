//! Resolve each source use and bind the exact authored region before fitting.
use super::*;
use martensite_tkbo::profile_extrusion::exact_conic_quarter;
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileLoop, ProfilePlane, ProfileVertex, ProfileVertexId,
    SketchProfile, exact_bernstein_partition,
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    MetricBezier, MetricCurve, OrientedMetricCurve,
};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Source<'a> {
    pub selection: &'a WrappedRegionSelection,
    pub profile: SketchProfile,
    pub spans: Vec<NativeSpan<'a>>,
    pub occurrence: LoopUse,
    pub ends: [ProfileVertexId; 2],
    pub map: Option<WrappedConicParameterMap>,
    pub lines: Vec<Line>,
}

pub(super) struct NativeSpan<'a> {
    controls: NativeControls<'a>,
    pub domain: WrappedSourceSpan,
}

enum NativeControls<'a> {
    Spline(MetricBezier<'a, 2>),
    Quarter([[f64; 2]; 3]),
}

pub(super) struct Line {
    pub id: ProfileCurveId,
    pub ends: [ProfileVertexId; 2],
    pub points: [[f64; 2]; 2],
}

type Use<'a> = (LoopUse, &'a ProfileCurve);
type VertexMap = BTreeMap<ProfileVertexId, [f64; 2]>;

pub(super) fn read<'a>(
    profile: &'a SketchProfile,
    selections: &'a [WrappedRegionSelection],
    options: WrappedNormalOptions,
) -> Checked<Source<'a>> {
    let selection = selected(profile, selections)?;
    let uses = resolve_uses(selection, profile)?;
    let vertices = vertices(profile)?;
    let profile = selected_profile(&uses, &vertices)?;
    let mut lines = Vec::new();
    let mut curve = None;
    for (used, c) in &uses {
        match c {
            ProfileCurve::Line { id, start, end } => {
                lines.push(line(*id, [*start, *end], *used, &vertices)?)
            }
            ProfileCurve::Spline { .. } if curve.is_none() => {
                curve = Some(rational(c, *used, &vertices)?)
            }
            ProfileCurve::Arc { .. } | ProfileCurve::EllipticArc { .. } if curve.is_none() => {
                curve = Some(quarter(c, *used, &profile, options)?);
            }
            other => {
                return Err(invalid(
                    format!("selection curve={:?}", other.id()),
                    "one positive-rational spline or exact analytic arc, with optional straight boundary uses",
                ));
            }
        }
    }
    let Some((spans, occurrence, ends, map)) = curve else {
        return Err(invalid(
            format!(
                "selected region={:?} with no rational boundary",
                selection.region
            ),
            "one positive-rational spline or exact analytic arc",
        ));
    };
    Ok(Source {
        selection,
        profile,
        spans,
        occurrence,
        ends,
        map,
        lines,
    })
}

fn selected<'a>(
    profile: &SketchProfile,
    selections: &'a [WrappedRegionSelection],
) -> Checked<&'a WrappedRegionSelection> {
    let [selection] = selections else {
        return Err(invalid(
            format!("selected regions={}", selections.len()),
            "one requested rational material region",
        ));
    };
    if !selection.holes.is_empty() || !(1..=64).contains(&selection.outer.uses.len()) {
        return Err(invalid(
            format!(
                "{} outer uses and {} holes",
                selection.outer.uses.len(),
                selection.holes.len()
            ),
            "one through 64 directed outer uses and no holes",
        ));
    }
    if profile.plane != ProfilePlane::world_xy() {
        return Err(invalid(
            format!("authored plane={:?}", profile.plane),
            "world-XY developed coordinates",
        ));
    }
    Ok(selection)
}

fn resolve_uses<'a>(
    selection: &WrappedRegionSelection,
    profile: &'a SketchProfile,
) -> Checked<Vec<Use<'a>>> {
    let mut curves = BTreeMap::new();
    for curve in &profile.curves {
        if curves.insert(curve.id(), curve).is_some() {
            return Err(invalid(
                format!("duplicate profile curve={:?}", curve.id()),
                "unique authored curve identifiers",
            ));
        }
    }
    let mut seen = BTreeSet::new();
    selection
        .outer
        .uses
        .iter()
        .map(|&used| {
            let curve = curves
                .get(&used.curve)
                .copied()
                .filter(|_| seen.insert(used.curve))
                .ok_or_else(|| {
                    invalid(
                        format!("missing or repeated selected curve={:?}", used.curve),
                        "one existing curve per selected use",
                    )
                })?;
            Ok((used, curve))
        })
        .collect()
}

fn vertices(profile: &SketchProfile) -> Checked<VertexMap> {
    let mut indexed = BTreeMap::new();
    for vertex in &profile.vertices {
        if indexed.insert(vertex.id, vertex.point).is_some() {
            return Err(invalid(
                format!("duplicate profile vertex={:?}", vertex.id),
                "unique authored vertex identifiers",
            ));
        }
    }
    Ok(indexed)
}

fn point(vertices: &VertexMap, vertex: ProfileVertexId) -> Checked<[f64; 2]> {
    vertices
        .get(&vertex)
        .copied()
        .filter(|point| point.iter().all(|coordinate| coordinate.is_finite()))
        .ok_or_else(|| {
            invalid(
                format!(
                    "profile vertex={vertex:?}, position={:?}",
                    vertices.get(&vertex)
                ),
                "one finite authored endpoint",
            )
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

type LocatedSource<'a> = (
    Vec<NativeSpan<'a>>,
    LoopUse,
    [ProfileVertexId; 2],
    Option<WrappedConicParameterMap>,
);

fn rational<'a>(
    curve: &'a ProfileCurve,
    used: LoopUse,
    vertices: &VertexMap,
) -> Checked<LocatedSource<'a>> {
    let ProfileCurve::Spline {
        id,
        start,
        end,
        degree,
        poles,
        weights: Some(weights),
        knots,
    } = curve
    else {
        return Err(invalid(
            format!("curve={:?}", curve.id()),
            "a clamped positive-rational Bézier",
        ));
    };
    let spans = partition(*id, *degree as usize, poles, weights, knots)?;
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
    Ok((spans, used, [*start, *end], None))
}

fn quarter<'a>(
    curve: &ProfileCurve,
    used: LoopUse,
    profile: &SketchProfile,
    options: WrappedNormalOptions,
) -> Checked<LocatedSource<'a>> {
    let Some(ends) = curve.ends() else {
        return Err(invalid(
            format!("arc={:?}", used.curve),
            "two original analytic endpoints",
        ));
    };
    let quarter = exact_conic_quarter(
        profile,
        used.curve,
        pipeline::resolution(options.spatial_error),
    )
    .map_err(|cause| RationalWrappedFailure::Analytic(Box::new(cause)))?
    .ok_or_else(|| {
        invalid(
            format!("arc={:?}", used.curve),
            "an exact circular or axis-aligned elliptic quarter",
        )
    })?;
    let map = WrappedConicParameterMap {
        first_quarter: quarter.first_quarter,
        increasing: quarter.increasing,
    };
    let span = NativeSpan {
        controls: NativeControls::Quarter(quarter.poles),
        domain: WrappedSourceSpan {
            level: 0,
            index: 0,
            reversed: false,
        },
    };
    Ok((vec![span], used, ends, Some(map)))
}

fn partition<'a>(
    id: ProfileCurveId,
    degree: usize,
    poles: &'a [[f64; 2]],
    weights: &'a [f64],
    knots: &'a [f64],
) -> Checked<Vec<NativeSpan<'a>>> {
    let spans = exact_bernstein_partition(degree, poles, Some(weights), knots)
        .filter(|spans| spans.len() <= 64 && knots[0] == 0. && knots[knots.len() - 1] == 1.)
        .ok_or_else(|| {
            invalid(
                format!(
                    "curve={id:?}, degree={degree}, poles={}, weights={}, knots={knots:?}",
                    poles.len(),
                    weights.len()
                ),
                "one through 64 finite positive-weight native Bernstein intervals on [0,1]",
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

fn selected_profile(uses: &[Use<'_>], vertices: &VertexMap) -> Checked<SketchProfile> {
    let ids: BTreeSet<_> = uses
        .iter()
        .flat_map(|(_, c)| c.ends().into_iter().flatten())
        .collect();
    let vertices = ids
        .into_iter()
        .map(|id| {
            Ok(ProfileVertex {
                id,
                point: point(vertices, id)?,
            })
        })
        .collect::<Checked<_>>()?;
    let curves = uses.iter().map(|(_, curve)| (**curve).clone()).collect();
    Ok(SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices,
        curves,
    })
}

impl Source<'_> {
    pub(super) fn poles(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        self.spans
            .iter()
            .flat_map(|span| span.metric().poles.iter().copied())
    }

    pub(super) fn area_wire(&self) -> Checked<Vec<OrientedMetricCurve<'_>>> {
        let mut wire = Vec::new();
        for used in &self.selection.outer.uses {
            if used.curve == self.occurrence.curve {
                wire.extend(self.metric_spans(used.reversed));
            } else {
                wire.push(self.line_metric(used.curve)?);
            }
        }
        Ok(wire)
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

pub(super) fn matches_boundary(
    selection: &WrappedRegionSelection,
    outer: &ProfileLoop,
    holes: &[ProfileLoop],
) -> Checked<()> {
    let requested = &selection.outer.uses;
    let actual = &outer.uses;
    let same = actual.len() == requested.len()
        && actual
            .iter()
            .position(|used| *used == requested[0])
            .is_some_and(|start| {
                actual
                    .iter()
                    .cycle()
                    .skip(start)
                    .take(requested.len())
                    .eq(requested)
            });
    if !holes.is_empty() || !same {
        return Err(invalid(
            format!("certified outer={actual:?}, holes={}", holes.len()),
            "the original selected directed outer material boundary with no holes",
        ));
    }
    Ok(())
}

impl NativeSpan<'_> {
    pub(super) fn metric(&self) -> MetricBezier<'_, 2> {
        match &self.controls {
            NativeControls::Spline(curve) => *curve,
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
