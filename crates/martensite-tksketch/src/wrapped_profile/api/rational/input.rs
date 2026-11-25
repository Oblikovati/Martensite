//! Resolve each source use and bind the exact authored region before fitting.
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileLoop, ProfilePlane, ProfileVertex, ProfileVertexId, SketchProfile,
};
use martensite_tkmath::predicates::roots::metric_bounds::{
    MetricBezier, MetricCurve, OrientedMetricCurve,
};
use std::collections::{BTreeMap, BTreeSet};

pub(super) struct Source<'a> {
    pub selection: &'a WrappedRegionSelection,
    pub profile: SketchProfile,
    pub curve: MetricBezier<'a, 2>,
    pub occurrence: LoopUse,
    pub ends: [ProfileVertexId; 2],
    pub lines: Vec<Line>,
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
) -> Checked<Source<'a>> {
    let selection = selected(profile, selections)?;
    let uses = resolve_uses(selection, profile)?;
    let vertices = vertices(profile)?;
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
            other => {
                return Err(invalid(
                    format!("selection curve={:?}", other.id()),
                    "one positive-rational Bézier and otherwise straight directed boundary uses",
                ));
            }
        }
    }
    let Some((curve, occurrence, ends)) = curve else {
        return Err(invalid(
            format!(
                "selected region={:?} with no rational boundary",
                selection.region
            ),
            "one positive-rational Bézier boundary",
        ));
    };
    let profile = selected_profile(&uses, &vertices)?;
    Ok(Source {
        selection,
        profile,
        curve,
        occurrence,
        ends,
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
    if !selection.holes.is_empty() || !(3..=64).contains(&selection.outer.uses.len()) {
        return Err(invalid(
            format!(
                "{} outer uses and {} holes",
                selection.outer.uses.len(),
                selection.holes.len()
            ),
            "three through 64 directed outer uses and no holes",
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

fn rational<'a>(
    curve: &'a ProfileCurve,
    used: LoopUse,
    vertices: &VertexMap,
) -> Checked<(MetricBezier<'a, 2>, LoopUse, [ProfileVertexId; 2])> {
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
    let n = poles.len();
    if !(2..=17).contains(&n)
        || *degree != (n - 1) as u32
        || !knots
            .iter()
            .copied()
            .eq(std::iter::repeat_n(0., n).chain(std::iter::repeat_n(1., n)))
    {
        return Err(invalid(
            format!("curve={id:?}, degree={degree}, poles={n}, knots={knots:?}"),
            "one clamped rational Bézier span on [0,1] of degree 1 through 16",
        ));
    }
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
    Ok((
        MetricBezier {
            poles,
            weights: Some(weights),
        },
        used,
        [*start, *end],
    ))
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
    pub(super) fn area_wire(&self) -> Checked<Vec<OrientedMetricCurve<'_>>> {
        self.selection
            .outer
            .uses
            .iter()
            .map(|used| {
                if used.curve == self.occurrence.curve {
                    return Ok(OrientedMetricCurve {
                        curve: MetricCurve::Bezier(self.curve),
                        reversed: used.reversed,
                    });
                }
                let line = self
                    .lines
                    .iter()
                    .find(|line| line.id == used.curve)
                    .ok_or_else(|| {
                        invalid(
                            format!("source curve={:?}", used.curve),
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
            })
            .collect()
    }

    pub(super) fn direction(&self) -> [f64; 2] {
        let first = self.curve.poles[0];
        let last = self.curve.poles[self.curve.poles.len() - 1];
        std::array::from_fn(|axis| last[axis] - first[axis])
    }

    pub(super) fn matches_boundary(
        &self,
        outer: &ProfileLoop,
        holes: &[ProfileLoop],
    ) -> Checked<()> {
        let requested = &self.selection.outer.uses;
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
}
