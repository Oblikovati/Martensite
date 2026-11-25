//! Close the original cap wire inventory and replay exact coefficient evidence on final faces.
use super::*;
use martensite_tkbrep::{brep_tools::outer_wire, topods, toptools::IndexedMapOfShape};
use martensite_tkmath::predicates::roots::metric_bounds::CertifiedCylindricalTrim;

mod cycle;
use cycle::trace_wire;
mod trace;
pub(super) use trace::Trace;

type Path = Vec<(WrappedHistoryKey, u8)>;
type NearLoops = BTreeMap<Path, bool>;
type FarLoops = BTreeMap<SurfaceRegionId, BTreeSet<Path>>;
type Receipts = BTreeMap<WrappedHistoryKey, CertifiedCylindricalTrim>;

pub(in super::super) struct Trims {
    stock: NearLoops,
    caps: FarLoops,
    proofs: [Receipts; 2],
    angular: [f64; 2],
}

impl Trims {
    pub(in super::super) fn capture(
        attempt: Attempt,
        input: &Admitted,
        tool: &Tool,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<Self> {
        let index = SourceIndex::new(attempt, tool)?;
        let mut owner = Self {
            stock: BTreeMap::new(),
            caps: BTreeMap::new(),
            proofs: std::array::from_fn(|_| BTreeMap::new()),
            angular: [f64::INFINITY, f64::NEG_INFINITY],
        };
        for &region in &tool.regions {
            attempt.poll(cancelled, VALIDATION)?;
            owner.capture_member(attempt, input, tool, &index, region, cancelled)?;
        }
        require(
            attempt,
            VALIDATION,
            owner
                .proofs
                .iter()
                .all(|entries| entries.len() == tool.spans.len()),
            "complete source trim coverage on both caps",
        )?;
        Ok(owner)
    }

    pub(in super::super) fn angular_hull(&self) -> [f64; 2] {
        self.angular
    }

    fn capture_member(
        &mut self,
        attempt: Attempt,
        input: &Admitted,
        tool: &Tool,
        index: &SourceIndex,
        region: SurfaceRegionId,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let member = assembly::named(attempt, &tool.native, region, HistoryRole::Solid)?;
        let faces = assembly::faces(attempt, member)?;
        let mut cap_paths = BTreeSet::new();
        let capture = SourceCap { input, tool, index };
        for (depth, role) in [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace]
            .into_iter()
            .enumerate()
        {
            let key = assembly::named(attempt, &tool.native, region, role)?;
            let source = assembly::directed_cap(attempt, &faces, key)?;
            let face = attempt.native(
                VALIDATION,
                "cumulative oriented material cap",
                topods::face(&assembly::occurrence(attempt.kind, &source)),
            )?;
            let outer = attempt.native(VALIDATION, "source cap outer wire", outer_wire(&face))?;
            for wire in assembly::children(attempt, &face)? {
                let path =
                    trace_wire(attempt, &wire, |edge| index.key(edge))?.ok_or_else(|| {
                        missing(attempt, VALIDATION, "one named source curve per cap edge")
                    })?;
                for shape in assembly::children(attempt, &wire)? {
                    attempt.poll(cancelled, VALIDATION)?;
                    self.source_trace(attempt, &capture, &face, &shape, depth)?;
                }
                if depth == 0 {
                    require(
                        attempt,
                        VALIDATION,
                        self.stock.insert(path, wire.is_same(&outer)).is_none(),
                        "unique source stock-bound loop",
                    )?;
                } else {
                    require(
                        attempt,
                        VALIDATION,
                        cap_paths.insert(path),
                        "unique source exposed-cap loop",
                    )?;
                }
            }
        }
        require(
            attempt,
            VALIDATION,
            self.caps.insert(region, cap_paths).is_none(),
            "unique source exposed-cap owner",
        )
    }

    fn source_trace(
        &mut self,
        attempt: Attempt,
        capture: &SourceCap<'_>,
        face: &Face,
        shape: &Shape,
        depth: usize,
    ) -> Checked<()> {
        let key = capture
            .index
            .key(shape)?
            .ok_or_else(|| missing(attempt, VALIDATION, "source-bound cap edge"))?;
        require(
            attempt,
            VALIDATION,
            key.role == depth_edge(depth),
            "caller-relative source cap edge",
        )?;
        let span = capture
            .tool
            .spans
            .get(&WrappedHistoryKey {
                source: key.source,
                role: HistoryRole::SideFace,
            })
            .ok_or_else(|| {
                attempt.history(CylinderFeatureSource::Tool(key), "matching source side")
            })?;
        let received = usize::from(capture.input.support.normal == NormalSense::Inward) ^ depth;
        let proof = &span.storage.trims.cap_residuals[received];
        let trace = Trace::read(attempt, capture.input, face, shape, proof, depth)?;
        if depth == 0 {
            for &[u, _] in &trace.uv {
                self.angular[0] = self.angular[0].min(u);
                self.angular[1] = self.angular[1].max(u);
            }
        }
        require(
            attempt,
            VALIDATION,
            self.proofs[depth].insert(key, proof.clone()).is_none(),
            "one complete retained proof for each source cap edge",
        )
    }

    pub(super) fn rebind(
        &self,
        attempt: Attempt,
        input: &Admitted,
        candidate: &Candidate,
        index: &body::Index,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<[BTreeMap<WrappedHistoryKey, Trace>; 2]> {
        let mut rebound = FinalTrims {
            traces: std::array::from_fn(|_| BTreeMap::new()),
            stock: BTreeMap::new(),
            caps: BTreeMap::new(),
            native_frames: 0,
        };
        for (&id, parents) in &candidate.history.images {
            attempt.poll(cancelled, VALIDATION)?;
            let Some(owner) = owner(attempt, id, parents)? else {
                continue;
            };
            let shape = entity(attempt, &candidate.native, id)?;
            let face = attempt.native(
                VALIDATION,
                "decoded owned trim-carrier face",
                topods::face(shape),
            )?;
            rebound.face(
                ValidatedFace {
                    attempt,
                    input,
                    candidate,
                    index,
                    source: self,
                },
                owner,
                &face,
                cancelled,
            )?;
        }
        require(
            attempt,
            VALIDATION,
            rebound.native_frames == 1 && rebound.stock == self.stock && rebound.caps == self.caps,
            "all original directed trim cycles on their exact decoded stock/far owner",
        )?;
        require(
            attempt,
            VALIDATION,
            rebound
                .traces
                .iter()
                .zip(&self.proofs)
                .all(|(a, b)| a.len() == b.len()),
            "complete decoded cap trace census",
        )?;
        Ok(rebound.traces)
    }
}

#[derive(Clone, Copy)]
enum Owner {
    Stock,
    Far(SurfaceRegionId),
}
impl Owner {
    fn depth(self) -> usize {
        usize::from(matches!(self, Self::Far(_)))
    }
}

fn owner(
    attempt: Attempt,
    id: CylinderBodyRef,
    parents: &[CylinderFeatureParent],
) -> Checked<Option<Owner>> {
    let mut owners = parents.iter().filter_map(|parent| match parent.source {
        CylinderFeatureSource::SupportFace(key) if key == attempt.attachment => Some(Owner::Stock),
        CylinderFeatureSource::Tool(WrappedHistoryKey {
            source: WrappedEntitySource::Region(region),
            role: HistoryRole::LastDepthFace,
        }) => Some(Owner::Far(region)),
        _ => None,
    });
    let first = owners.next();
    require(
        attempt,
        VALIDATION,
        first.is_none() || (id.kind == ShapeEnum::Face && owners.next().is_none()),
        "unique physical cap face owner",
    )?;
    Ok(first)
}

struct FinalTrims {
    traces: [BTreeMap<WrappedHistoryKey, Trace>; 2],
    stock: NearLoops,
    caps: FarLoops,
    native_frames: usize,
}
struct ValidatedFace<'a> {
    attempt: Attempt,
    input: &'a Admitted,
    candidate: &'a Candidate,
    index: &'a body::Index,
    source: &'a Trims,
}

impl FinalTrims {
    fn face(
        &mut self,
        ctx: ValidatedFace<'_>,
        owner: Owner,
        face: &Face,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let attempt = ctx.attempt;
        let wires = assembly::children(attempt, face)?;
        let mut tool_paths = BTreeSet::new();
        let mut native = 0;
        for wire in &wires {
            let path = trace_wire(attempt, wire, |edge| ctx.tool_key(edge, owner.depth()))?;
            match path {
                Some(path) => {
                    require(
                        attempt,
                        VALIDATION,
                        tool_paths.insert(path),
                        "distinct retained cap wire",
                    )?;
                    self.bind_wire(&ctx, owner.depth(), face, wire, cancelled)?;
                }
                None => native += 1,
            }
        }
        self.publish_paths(attempt, owner, tool_paths, native)
    }

    fn bind_wire(
        &mut self,
        ctx: &ValidatedFace<'_>,
        depth: usize,
        face: &Face,
        wire: &Shape,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        for shape in assembly::children(ctx.attempt, wire)? {
            ctx.attempt.poll(cancelled, VALIDATION)?;
            let key = ctx.tool_key(&shape, depth)?.ok_or_else(|| {
                missing(
                    ctx.attempt,
                    VALIDATION,
                    "final cap edge belongs to its complete directed wire",
                )
            })?;
            let proof = ctx.source.proofs[depth].get(&key).ok_or_else(|| {
                ctx.attempt.history(
                    CylinderFeatureSource::Tool(key),
                    "matching pre-assembly native trace proof",
                )
            })?;
            let trace = Trace::read(ctx.attempt, ctx.input, face, &shape, proof, depth)?;
            require(
                ctx.attempt,
                VALIDATION,
                self.traces[depth].insert(key, trace).is_none(),
                "one occurrence of each authored cap edge in the final chart",
            )?;
        }
        Ok(())
    }

    fn publish_paths(
        &mut self,
        attempt: Attempt,
        owner: Owner,
        paths: BTreeSet<Path>,
        native: usize,
    ) -> Checked<()> {
        match owner {
            Owner::Far(region) => require(
                attempt,
                VALIDATION,
                native == 0 && !paths.is_empty() && self.caps.insert(region, paths).is_none(),
                "all directed exposed cap loops belong to the original region",
            ),
            Owner::Stock => {
                require(
                    attempt,
                    VALIDATION,
                    native == 1 || (native == 0 && paths.len() == 1),
                    "one remainder frame or one isolated counter island",
                )?;
                self.native_frames += native;
                for path in paths {
                    require(
                        attempt,
                        VALIDATION,
                        self.stock.insert(path, native == 1).is_none(),
                        "one original stock trim loop on its expected face",
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl ValidatedFace<'_> {
    fn tool_key(&self, shape: &Shape, depth: usize) -> Checked<Option<WrappedHistoryKey>> {
        let id = self
            .index
            .reference(shape, self.attempt)?
            .ok_or_else(|| missing(self.attempt, VALIDATION, "indexed decoded cap edge"))?;
        let parents =
            self.candidate.history.images.get(&id).ok_or_else(|| {
                missing(self.attempt, VALIDATION, "exact decoded cap edge ancestry")
            })?;
        let mut keys = parents.iter().filter_map(|parent| match parent.source {
            CylinderFeatureSource::Tool(key) if key.role == depth_edge(depth) => Some(key),
            _ => None,
        });
        let key = keys.next();
        require(
            self.attempt,
            VALIDATION,
            keys.next().is_none(),
            "unique retained tool edge ancestry",
        )?;
        Ok(key)
    }
}

fn depth_edge(depth: usize) -> HistoryRole {
    [HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge][depth]
}

struct SourceCap<'a> {
    input: &'a Admitted,
    tool: &'a Tool,
    index: &'a SourceIndex,
}

struct SourceIndex {
    attempt: Attempt,
    index: IndexedMapOfShape,
    keys: Vec<WrappedHistoryKey>,
}
impl SourceIndex {
    fn new(attempt: Attempt, tool: &Tool) -> Checked<Self> {
        let mut index = IndexedMapOfShape::default();
        let mut keys = Vec::new();
        for (&key, shape) in &tool.native.entities {
            if matches!(
                key.role,
                HistoryRole::FirstDepthEdge | HistoryRole::LastDepthEdge
            ) {
                let insertion = index.add(shape.clone());
                require(
                    attempt,
                    VALIDATION,
                    insertion == keys.len() + 1,
                    "one shape per source cap edge",
                )?;
                keys.push(key);
            }
        }
        Ok(Self {
            attempt,
            index,
            keys,
        })
    }
    fn key(&self, shape: &Shape) -> Checked<Option<WrappedHistoryKey>> {
        let index = self.index.find_index(shape);
        if index == 0 {
            return Ok(None);
        }
        self.keys
            .get(index - 1)
            .copied()
            .map(Some)
            .ok_or_else(|| missing(self.attempt, VALIDATION, "indexed source edge owner"))
    }
}
