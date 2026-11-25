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
}

impl FittedCut {
    fn locate(crossing: admission::Crossing, tool: &DetachedWrappedBodies) -> Checked<Self> {
        let id = crossing.report.id;
        let mut candidates = tool
            .entities
            .keys()
            .filter_map(|key| candidate_span(*key, id.curve));
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
        if containing.next().is_some() || span.reversed {
            return Err(incomplete(
                "unique forward certified fitter span",
                format!("{id:?}: {span:?}"),
            ));
        }
        Ok(Self { crossing, span })
    }

    pub(super) fn edge(self, depth: PeriodicDepth) -> WrappedHistoryKey {
        WrappedHistoryKey {
            source: WrappedEntitySource::Curve {
                curve: self.crossing.report.id.curve,
                span: self.span,
            },
            role: roles::edge_role(depth.index()),
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
                WrappedEntitySource::SplitVertex {
                    curve: self.crossing.report.id.curve,
                    point: dyadic(n, level),
                }
            };
            WrappedHistoryKey {
                source,
                role: roles::vertex_role(depth.index()),
            }
        })
    }

    fn ends(self) -> [PeriodicSourceEnd; 2] {
        [self.span.index, self.span.index + 1].map(|n| endpoint(n, self.span.level))
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

fn candidate_span(key: WrappedHistoryKey, id: SurfaceCurveId) -> Option<WrappedSourceSpan> {
    match key {
        WrappedHistoryKey {
            source: WrappedEntitySource::Curve { curve, span },
            role: HistoryRole::FirstDepthEdge,
        } if curve == id => Some(span),
        _ => None,
    }
}

fn endpoint(n: u64, level: u8) -> PeriodicSourceEnd {
    if n == 0 {
        return PeriodicSourceEnd::Start;
    }
    if n == 1u64 << level {
        return PeriodicSourceEnd::End;
    }
    PeriodicSourceEnd::Dyadic(dyadic(n, level))
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
    pub sources: [FittedCut; 2],
}

struct NativeOperands<'a> {
    cut: images::Candidate,
    band: &'a FullTurnBand,
    stock: &'a DetachedNormalBody,
    tool: &'a DetachedWrappedBodies,
    sources: [FittedCut; 2],
}

pub(super) fn bind(
    plan: &AperturePlan,
    stock: &DetachedNormalBody,
    tool: &DetachedWrappedBodies,
    angular_error: f64,
) -> Checked<BoundCut> {
    let tool_solid = WrappedHistoryKey {
        source: WrappedEntitySource::Region(plan.region),
        role: HistoryRole::Solid,
    };
    let cutter = tool_image(&tool.entities, tool_solid)?;
    let [lower, upper] = plan.crossings.map(|item| FittedCut::locate(item, tool));
    let sources = [lower?, upper?];
    for source in sources {
        prove_fragment_clearance(plan, source, angular_error)?;
    }
    let cut = images::subtract(&stock.solid, cutter)?;
    NativeOperands {
        cut,
        band: &plan.band,
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
    let source_angle = (min_parameter * cut.crossing.angular_speed).next_down();
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
        let [a, b] = self.sources.map(|cut| self.splits(cut, depth, output));
        let roots = [a?, b?];
        let seam_key = HistoryKey {
            source: HistorySource::Region(self.band.region),
            role: roles::edge_role(depth.index()),
        };
        let seam =
            crate::surface_sketch::full_turn::history::lookup(&self.stock.entities, seam_key)?;
        let witnesses: [topods::Shape; 2] = self.cut.crossing_vertices(seam, seam_key)?;
        if roots
            .iter()
            .any(|root| !witnesses.iter().any(|w| w.is_same(root)))
        {
            return Err(incomplete(
                "same exact roots of cutter and stock seam",
                format!("{seam_key:?}"),
            ));
        }
        self.bind_rails(depth, seam, seam_key, roots, output)
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
        [a, b]: [topods::Shape; 2],
        output: &mut Bindings,
    ) -> Checked<()> {
        let candidates = self.cut.split_edges(seam, key)?;
        let [lower, upper] = self.band.rims.map(|rim| {
            resolve(
                output,
                PeriodicImageKey::Retained(HistoryKey {
                    source: HistorySource::Curve(rim),
                    role: roles::vertex_role(depth.index()),
                }),
            )
        });
        let [lower, upper] = [lower?, upper?];
        let rails = [
            crossed::oriented_edge(&candidates, lower, &a)?,
            crossed::oriented_edge(&candidates, &b, upper)?,
        ];
        let names = rail_names(
            self.band,
            depth,
            self.sources.map(|source| source.crossing.report.id),
        );
        output.extend(names.into_iter().zip(rails));
        Ok(())
    }
}

pub(super) fn rail_names(
    band: &FullTurnBand,
    depth: PeriodicDepth,
    [a, b]: [PeriodicCrossingId; 2],
) -> [PeriodicImageKey; 2] {
    [
        [
            PeriodicSeamEnd::Rim(band.rims[0]),
            PeriodicSeamEnd::Crossing(a),
        ],
        [
            PeriodicSeamEnd::Crossing(b),
            PeriodicSeamEnd::Rim(band.rims[1]),
        ],
    ]
    .map(|ends| PeriodicImageKey::SeamSegment {
        region: band.region,
        depth,
        ends,
    })
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
