// SPDX-License-Identifier: GPL-2.0-only
//! Compose exact fitter-span and stock occurrence ancestry through the native cut.
use super::*;
use crate::surface_sketch::history as roles;
use std::collections::BTreeMap;

pub(super) type Bindings = Vec<(PeriodicImageKey, topods::Shape)>;

#[derive(Clone, Copy)]
pub(super) struct FittedCut {
    pub crossing: admission::Crossing,
    pub span: WrappedSourceSpan,
    pub angular_speed: f64,
}

impl FittedCut {
    fn locate(crossing: admission::Crossing, tool: &DetachedWrappedBodies) -> Checked<Self> {
        let id = crossing.report.id;
        let mut candidates = tool
            .entities
            .keys()
            .filter_map(|key| candidate_span(*key, crossing.report));
        let enclosed = |span: &WrappedSourceSpan| {
            let [a, b] = span_bounds(*span);
            let [lo, hi] = crossing.report.normalized_parameter;
            a < lo && hi < b
        };
        let mut containing = candidates.by_ref().filter(enclosed);
        let span = containing.next().ok_or_else(|| {
            incomplete(
                "root interior to one certified fitter span",
                format!("{id:?}"),
            )
        })?;
        if containing.next().is_some() {
            return Err(incomplete(
                "unique certified fitter span",
                format!("{id:?}: {span:?}"),
            ));
        }
        let angular_speed = crossing.angular_speed(span_bounds(span))?;
        Ok(Self {
            crossing,
            span,
            angular_speed,
        })
    }

    pub(super) fn source_parameter(self, parameter: f64) -> f64 {
        let [a, b] = span_bounds(self.span);
        let pos = parameter * (b - a);
        if self.span.reversed { b - pos } else { a + pos }
    }

    pub(super) fn edge(self, depth: PeriodicDepth) -> WrappedHistoryKey {
        WrappedHistoryKey {
            source: self.curve_source(),
            role: roles::edge_role(depth.index()),
        }
    }

    fn curve_source(self) -> WrappedEntitySource {
        let curve = self.crossing.report.id.curve;
        let span = self.span;
        match self.crossing.report.conic_map {
            None => WrappedEntitySource::Curve { curve, span },
            Some(PeriodicConicMap::Quarter(map)) => {
                WrappedEntitySource::MappedCurve { curve, span, map }
            }
            Some(PeriodicConicMap::Arc(map)) => {
                WrappedEntitySource::AnalyticArcCurve { curve, span, map }
            }
        }
    }

    fn split_point(self, point: WrappedSourcePoint) -> WrappedEntitySource {
        let curve = self.crossing.report.id.curve;
        match self.crossing.report.conic_map {
            None => WrappedEntitySource::SplitVertex { curve, point },
            Some(PeriodicConicMap::Quarter(map)) => {
                WrappedEntitySource::MappedSplitVertex { curve, point, map }
            }
            Some(PeriodicConicMap::Arc(map)) => {
                WrappedEntitySource::AnalyticArcSplitVertex { curve, point, map }
            }
        }
    }

    pub(super) fn vertices(self, depth: PeriodicDepth) -> [WrappedHistoryKey; 2] {
        let level = self.span.level;
        [self.span.index, self.span.index + 1].map(|n| {
            let source = if n == 0 {
                WrappedEntitySource::Vertex(self.crossing.ends[0])
            } else if n == 1u64 << level {
                WrappedEntitySource::Vertex(self.crossing.ends[1])
            } else {
                self.split_point(dyadic(n, level))
            };
            WrappedHistoryKey {
                source,
                role: roles::vertex_role(depth.index()),
            }
        })
    }

    fn ends(self) -> [PeriodicSourceEnd; 2] {
        [self.span.index, self.span.index + 1]
            .map(|n| endpoint(n, self.span.level, self.crossing.report.conic_map))
    }

    pub(super) fn pieces(self, depth: PeriodicDepth) -> [PeriodicImageKey; 2] {
        let id = self.crossing.report.id;
        let root = PeriodicSourceEnd::Cut(id);
        let [a, b] = self.ends();
        [[a, root], [root, b]].map(|interval| PeriodicImageKey::AperturePiece {
            curve: id.curve,
            depth,
            interval,
        })
    }
}

pub(super) fn span_bounds(span: WrappedSourceSpan) -> [f64; 2] {
    let scale = martensite_cmath::powi(2., -i32::from(span.level));
    [span.index as f64 * scale, (span.index + 1) as f64 * scale]
}

fn candidate_span(key: WrappedHistoryKey, root: PeriodicCrossing) -> Option<WrappedSourceSpan> {
    if key.role != HistoryRole::FirstDepthEdge {
        return None;
    }
    let (curve, span, map) = match key.source {
        WrappedEntitySource::Curve { curve, span } => (curve, span, None),
        WrappedEntitySource::MappedCurve { curve, span, map } => {
            (curve, span, Some(PeriodicConicMap::Quarter(map)))
        }
        WrappedEntitySource::AnalyticArcCurve { curve, span, map } => {
            (curve, span, Some(PeriodicConicMap::Arc(map)))
        }
        _ => return None,
    };
    (curve == root.id.curve && map == root.conic_map).then_some(span)
}

fn endpoint(n: u64, level: u8, map: Option<PeriodicConicMap>) -> PeriodicSourceEnd {
    if n == 0 {
        return PeriodicSourceEnd::Start;
    }
    if n == 1u64 << level {
        return PeriodicSourceEnd::End;
    }
    let point = dyadic(n, level);
    match map {
        Some(map) => PeriodicSourceEnd::ConicDyadic { point, map },
        None => PeriodicSourceEnd::Dyadic(point),
    }
}

fn dyadic(n: u64, level: u8) -> WrappedSourcePoint {
    let reduced = n.trailing_zeros() as u8;
    WrappedSourcePoint {
        level: level - reduced,
        numerator: n >> reduced,
    }
}

pub(super) struct BoundCut {
    pub body: topods::Shape,
    pub bindings: Bindings,
    pub sources: Vec<FittedCut>,
}

struct NativeOperands<'a> {
    cut: images::Candidate,
    plan: &'a AperturePlan,
    stock: &'a DetachedNormalBody,
    tool: &'a DetachedWrappedBodies,
    sources: Vec<FittedCut>,
}

pub(super) fn bind(
    plan: &AperturePlan,
    stock: &DetachedNormalBody,
    tool: &DetachedWrappedBodies,
    angular_error: f64,
) -> Checked<BoundCut> {
    let cutters = plan
        .contours
        .iter()
        .map(|contour| {
            let key = WrappedHistoryKey {
                source: WrappedEntitySource::Region(contour.region),
                role: HistoryRole::Solid,
            };
            Ok(tool_image(&tool.entities, key)?.clone())
        })
        .collect::<Checked<Vec<_>>>()?;
    let sources = plan
        .crossings()
        .map(|crossing| FittedCut::locate(crossing, tool))
        .collect::<Checked<Vec<_>>>()?;
    for &source in &sources {
        prove_fragment_clearance(plan, source, angular_error)?;
    }
    let cut = images::subtract_all(&stock.solid, cutters)?;
    NativeOperands {
        cut,
        plan,
        stock,
        tool,
        sources,
    }
    .bind()
}

fn prove_fragment_clearance(
    plan: &AperturePlan,
    cut: FittedCut,
    angular_error: f64,
) -> Checked<()> {
    let [lo, hi] = cut.crossing.report.normalized_parameter;
    let [first, last] = span_bounds(cut.span);
    let min_parameter = (lo - first).min(last - hi).next_down();
    let source_angle = (min_parameter * cut.angular_speed).next_down();
    let floor = effective_limits(plan.stock.sweep.options).linear_separation;
    require_periodic_chord(
        (source_angle - angular_error).next_down(),
        plan.minimum_radius(),
        floor,
    )
}

impl NativeOperands<'_> {
    fn bind(self) -> Checked<BoundCut> {
        let mut bindings = self.direct()?;
        for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
            self.bind_depth(depth, &mut bindings)?;
        }
        bindings.sort_by_key(|(key, _)| *key);
        Ok(BoundCut {
            body: self.cut.body,
            bindings,
            sources: self.sources,
        })
    }

    fn direct(&self) -> Checked<Bindings> {
        let mut bindings = Vec::new();
        for (key, shape) in &self.stock.entities {
            if !matches!(key.source, HistorySource::Region(_)) || !is_cap_edge(key.role) {
                let (_, image) = self.cut.image(*key, shape)?;
                bindings.push((PeriodicImageKey::Retained(*key), image));
            }
        }
        for (&key, shape) in &self.tool.entities {
            if matches!(key.source, WrappedEntitySource::Region(_)) {
                self.cut.deleted_cap(key, shape)?;
            } else if !self.is_crossed(key) {
                let image = self.cut.image_by_role(key, key.role, shape)?;
                bindings.push((PeriodicImageKey::WrappedRetained(key), image));
            }
        }
        Ok(bindings)
    }

    fn is_crossed(&self, key: WrappedHistoryKey) -> bool {
        self.sources.iter().any(|source| {
            [PeriodicDepth::First, PeriodicDepth::Last]
                .into_iter()
                .any(|depth| source.edge(depth) == key)
        })
    }

    fn bind_depth(&self, depth: PeriodicDepth, output: &mut Bindings) -> Checked<()> {
        let mut roots = BTreeMap::new();
        for &source in &self.sources {
            let root = self.splits(source, depth, output)?;
            roots.insert(source.crossing.report.id, root);
        }
        let key = HistoryKey {
            source: HistorySource::Region(self.plan.band.region),
            role: roles::edge_role(depth.index()),
        };
        let seam = crate::surface_sketch::full_turn::history::lookup(&self.stock.entities, key)?;
        let witnesses = self.cut.crossing_vertices_count(seam, key, roots.len())?;
        verify_roots(&roots, &witnesses)?;
        self.bind_rails(depth, seam, key, output)
    }

    fn splits(
        &self,
        source: FittedCut,
        depth: PeriodicDepth,
        output: &mut Bindings,
    ) -> Checked<topods::Shape> {
        let key = source.edge(depth);
        let curve = tool_image(&self.tool.entities, key)?;
        let [root] = self.cut.crossing_vertices(curve, key)?;
        let candidates = self.cut.split_edges(curve, key)?;
        let [first, last] = source
            .vertices(depth)
            .map(|key| resolve(output, PeriodicImageKey::WrappedRetained(key)));
        let images = [
            crossed::oriented_edge(&candidates, first?, &root)?,
            crossed::oriented_edge(&candidates, &root, last?)?,
        ];
        output.extend(source.pieces(depth).into_iter().zip(images));
        output.push((
            PeriodicImageKey::CrossingVertex {
                depth,
                crossing: source.crossing.report.id,
            },
            root.clone(),
        ));
        Ok(root)
    }

    fn bind_rails(
        &self,
        depth: PeriodicDepth,
        seam: &topods::Shape,
        key: HistoryKey,
        output: &mut Bindings,
    ) -> Checked<()> {
        let count = self.plan.rails.len();
        let candidates = if count == 1 {
            vec![self.cut.image(key, seam)?.1]
        } else {
            self.cut.split_edges_count(seam, key, count)?
        };
        let region = self.plan.band.region;
        for &ends in &self.plan.rails {
            let [a, b] = ends.map(|end| resolve(output, seam_vertex_key(depth, end)));
            let image = crossed::oriented_edge(&candidates, a?, b?)?;
            output.push((
                PeriodicImageKey::SeamSegment {
                    region,
                    depth,
                    ends,
                },
                image,
            ));
        }
        Ok(())
    }
}

fn verify_roots(
    roots: &BTreeMap<PeriodicCrossingId, topods::Shape>,
    witnesses: &[topods::Shape],
) -> Checked<()> {
    if roots
        .values()
        .any(|root| !witnesses.iter().any(|w| w.is_same(root)))
    {
        return Err(incomplete(
            "same exact roots of cutters and stock seam",
            format!("{} verified roots", roots.len()),
        ));
    }
    Ok(())
}

pub(super) fn seam_vertex_key(depth: PeriodicDepth, end: PeriodicSeamEnd) -> PeriodicImageKey {
    match end {
        PeriodicSeamEnd::Rim(rim) => PeriodicImageKey::Retained(HistoryKey {
            source: HistorySource::Curve(rim),
            role: roles::vertex_role(depth.index()),
        }),
        PeriodicSeamEnd::Crossing(crossing) => PeriodicImageKey::CrossingVertex { depth, crossing },
    }
}

fn is_cap_edge(role: HistoryRole) -> bool {
    matches!(
        role,
        HistoryRole::FirstDepthEdge | HistoryRole::LastDepthEdge
    )
}

pub(super) fn tool_image(
    images: &BTreeMap<WrappedHistoryKey, topods::Shape>,
    key: WrappedHistoryKey,
) -> Checked<&topods::Shape> {
    images
        .get(&key)
        .ok_or_else(|| incomplete("detached curved tool image", format!("{key:?}")))
}

pub(super) fn resolve(bindings: &Bindings, key: PeriodicImageKey) -> Checked<&topods::Shape> {
    bindings
        .iter()
        .find_map(|(candidate, shape)| (*candidate == key).then_some(shape))
        .ok_or_else(|| incomplete("named curved cut image", format!("{key:?}")))
}
