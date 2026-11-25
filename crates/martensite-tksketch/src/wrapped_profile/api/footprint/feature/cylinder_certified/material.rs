//! Admit the positioned authored family and materialize operation-local tool geometry.
use super::*;
use crate::surface_sketch::attachment::DetachedCylinderSupport;
use crate::surface_sketch::wrapped_profile::{
    BindingKey, CurveSpan, Source,
    api::{export_bindings, history_projection},
    history as source_history, native, planar_clearance, planar_cones, storage,
};
use martensite_tkbo::profile_extrusion::Resolution;
use martensite_tkbrep::topods::{Builder, Compound};
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    toploc::Location,
};

const ADMISSION: CylinderFeatureStage = CylinderFeatureStage::Admission;
const TOOL: CylinderFeatureStage = CylinderFeatureStage::Tool;

pub(super) struct Admitted {
    pub stock: DetachedCylinderSupport,
    pub solid: Shape,
    pub resolution: Resolution,
    pub axial: [f64; 2],
    pub support: crate::surface_sketch::CylinderSupport,
    inputs: input::Inputs,
}

impl Admitted {
    pub(super) fn fit_ceiling(&self) -> f64 {
        self.inputs
            .requests
            .iter()
            .map(|request| subdivision::effective_fit_limit(request, self.resolution.sew))
            .fold(0_f64, f64::max)
    }
}

pub(super) fn admit(
    attachment: &CylinderFaceAttachment,
    attempt: Attempt,
    request: &WrappedNormalRequest,
    planar: PlanarProofVersion,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Admitted> {
    attempt.poll(cancelled, ADMISSION)?;
    require(
        attempt,
        ADMISSION,
        matches!(planar, PlanarProofVersion::GeometricCompositeV6 { .. })
            && matches!(request.options.volume, WrappedVolumePolicy::Report),
        "explicit geometric planar V6 and Report volume policy",
    )?;
    let (stock, mut band) = attachment
        .resolve_band(attempt.revision, request)
        .map_err(|cause| attempt.footprint(cause))?;
    require(
        attempt,
        ADMISSION,
        band.angular.is_none() && band.holes.is_empty() && band.outer.is_none(),
        "full cylindrical stock with native rims and no inherited holes",
    )?;
    let solid = super::super::admit(
        attempt,
        &stock.body,
        chart_orientation(stock.face.orientation(), band.reflect_axial),
        request.depths,
        request.support.normal,
    )?;
    let support = request.support;
    require(
        attempt,
        ADMISSION,
        support.axis == [0., 0., 1.]
            && support.radial == [1., 0., 0.]
            && support.chart_origin == 0.
            && support.origin.into_iter().all(f64::is_finite),
        "literal cylindrical stock chart and exact translation",
    )?;
    let mut exclusions = super::super::super::holes::Exclusions::new(&band.holes)
        .map_err(|cause| attempt.footprint(cause.into()))?;
    let inputs =
        super::super::super::extrusion::read_inside(request, &mut band, &mut exclusions, cancelled)
            .map_err(|cause| attempt.footprint(cause))?;
    Ok(Admitted {
        solid,
        stock,
        resolution: attachment.record().resolution,
        axial: band.axial,
        support,
        inputs,
    })
}

pub(super) struct Tool {
    pub native: DetachedWrappedBodies,
    pub authored: Vec<WrappedHistoryRecord>,
    pub regions: Vec<SurfaceRegionId>,
    pub spans: BTreeMap<WrappedHistoryKey, Span>,
    pub joins: BTreeMap<[WrappedHistoryKey; 2], [f64; 2]>,
    pub clearance: planar_clearance::OriginalClearance,
}

impl Tool {
    pub(super) fn authored_fit_upper(&self) -> f64 {
        self.spans
            .values()
            .map(|span| span.storage.profile.developed_position_bound())
            .fold(0_f64, f64::max)
    }

    pub(super) fn normal_storage_upper(&self) -> f64 {
        self.spans
            .values()
            .map(|span| span.storage.depths.normal_sweep_error_bound())
            .fold(0_f64, f64::max)
    }
}

pub(super) struct Span {
    pub storage: storage::StoredProof,
    pub directions: Vec<[f64; 2]>,
}

pub(super) fn prepare(
    attempt: Attempt,
    request: &WrappedNormalRequest,
    input: &Admitted,
    planar: PlanarProofVersion,
    cancelled: &mut CancelPoll<'_>,
    complete: &mut impl FnMut(Phase),
) -> Checked<Tool> {
    let originals = &input.inputs.requests;
    let selected = certify_cancellable(
        attempt,
        TOOL,
        "source-bound selected material",
        cancelled,
        |poll| {
            selection::certify_with_proof(
                originals,
                Default::default(),
                request.options.planar_work,
                selection::Preparation {
                    proof: planar,
                    maximum_fit_error: Some(input.resolution.sew),
                },
                poll,
            )
        },
    )?;
    complete(Phase::AuthoredPlan);
    let prepared = &selected.prepared.prepared;
    let work = build_members(attempt, originals, prepared, cancelled)?;
    complete(Phase::ToolStorage);
    let keys: Vec<_> = work.bindings.iter().map(|(key, _)| *key).collect();
    let lineages = certify(
        attempt,
        TOOL,
        "source tool authored history",
        source_history::certify(originals, &selected, &keys),
    )?;
    let history = certify(
        attempt,
        TOOL,
        "normal-sense authored history",
        history_projection::prepare_physical(
            &keys,
            &lineages,
            &input.inputs.directions,
            request.support.normal,
        ),
    )?;
    let references = work
        .stored
        .iter()
        .map(|(&region, proof)| (region, proof.as_slice()))
        .collect();
    let clearance = selected.prepared.boundary_clearance;
    certify(
        attempt,
        TOOL,
        "source-bound family clearance",
        planar_clearance::bind_many(&clearance, originals, prepared, &references),
    )?;
    let tool = finish(
        attempt,
        work,
        history,
        keys,
        input.support.origin,
        clearance,
    )?;
    complete(Phase::ToolFamily);
    Ok(tool)
}

struct Working {
    root: Shape,
    bindings: Vec<(BindingKey, Shape)>,
    stored: BTreeMap<SurfaceRegionId, Vec<storage::StoredProof>>,
    cones: BTreeMap<CurveSpan, Vec<[f64; 2]>>,
    joins: BTreeMap<[CurveSpan; 2], [f64; 2]>,
}

fn build_members(
    attempt: Attempt,
    originals: &[Request],
    prepared: &[Request],
    cancelled: &mut CancelPoll<'_>,
) -> Checked<Working> {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    let mut working = Working {
        root: compound.into_shape(),
        bindings: Vec::new(),
        stored: BTreeMap::new(),
        cones: BTreeMap::new(),
        joins: BTreeMap::new(),
    };
    let mut ordered: Vec<_> = originals.iter().zip(prepared).collect();
    ordered.sort_by_key(|(source, _)| source.region);
    for (original, request) in ordered {
        working.add(attempt, original, request, cancelled)?;
    }
    working.bindings.sort_by_key(|(key, _)| *key);
    Ok(working)
}

impl Working {
    fn add(
        &mut self,
        attempt: Attempt,
        original: &Request,
        request: &Request,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        attempt.poll(cancelled, TOOL)?;
        require(
            attempt,
            TOOL,
            original.region == request.region,
            "matching canonical region order",
        )?;
        let member = attempt.native(TOOL, "source cylinder member", native::construct(request))?;
        attempt.native(
            TOOL,
            "place source member in operation-local compound",
            Builder.add(&mut self.root, &member.solid),
        )?;
        let receipts =
            certify_cancellable(attempt, TOOL, "source member storage", cancelled, |poll| {
                storage::certify_with_cancel(original, request, &member.bindings, poll)
            })?;
        let cones = certify_cancellable(
            attempt,
            TOOL,
            "source span and join projections",
            cancelled,
            |poll| planar_cones::certify_with_cancel(original, request, &receipts, poll),
        )?;
        certify(
            attempt,
            TOOL,
            "bind authored and cap projections",
            planar_cones::bind(original, request, &receipts, &cones),
        )?;
        self.add_cones(attempt, cones)?;
        require(
            attempt,
            TOOL,
            self.stored.insert(request.region, receipts).is_none(),
            "one member for each selected region",
        )?;
        self.bindings.extend(member.bindings);
        Ok(())
    }

    fn add_cones(&mut self, attempt: Attempt, cones: planar_cones::ConeEvidence) -> Checked<()> {
        for join in &cones.joins {
            let pair: [CurveSpan; 2] = certify(
                attempt,
                TOOL,
                "two directed spans per source join",
                join.spans.as_slice().try_into(),
            )?;
            require(
                attempt,
                TOOL,
                self.joins
                    .insert(pair, join.certificate.direction())
                    .is_none(),
                "one projection per actual source join",
            )?;
        }
        for cone in cones.pieces.into_iter().chain(cones.joins) {
            let direction = cone.certificate.direction();
            for span in cone.spans {
                self.cones.entry(span).or_default().push(direction);
            }
        }
        Ok(())
    }
}

fn finish(
    attempt: Attempt,
    work: Working,
    lineage: history_projection::HistoryPublication,
    keys: Vec<BindingKey>,
    origin: [f64; 3],
    clearance: planar_clearance::OriginalClearance,
) -> Checked<Tool> {
    let names: BTreeMap<_, _> = keys
        .iter()
        .copied()
        .zip(lineage.bindings.iter().copied())
        .collect();
    let regions = work.stored.keys().copied().collect();
    let side_key = |span| {
        names
            .get(&BindingKey {
                source: Source::Span(span),
                role: HistoryRole::SideFace,
            })
            .copied()
            .ok_or_else(|| missing(attempt, TOOL, "one authored key for each retained side"))
    };
    let mut spans = BTreeMap::new();
    for storage in work.stored.into_values().flatten() {
        let directions = work
            .cones
            .get(&storage.span)
            .ok_or_else(|| missing(attempt, TOOL, "retained span projections"))?
            .clone();
        let key = side_key(storage.span)?;
        require(
            attempt,
            TOOL,
            spans
                .insert(
                    key,
                    Span {
                        storage,
                        directions,
                    },
                )
                .is_none(),
            "unique retained side ownership",
        )?;
    }
    let mut joins = BTreeMap::new();
    for ([a, b], direction) in work.joins {
        joins.insert([side_key(a)?, side_key(b)?], direction);
    }
    let native = place(
        attempt,
        work.root,
        work.bindings,
        &keys,
        &lineage.bindings,
        origin,
    )?;
    Ok(Tool {
        native,
        authored: lineage.records,
        regions,
        spans,
        joins,
        clearance,
    })
}

fn place(
    attempt: Attempt,
    root: Shape,
    bindings: Vec<(BindingKey, Shape)>,
    keys: &[BindingKey],
    names: &[WrappedHistoryKey],
    origin: [f64; 3],
) -> Checked<DetachedWrappedBodies> {
    let [x, y, z] = origin;
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(x, y, z));
    let location = Location::new(transform);
    let bodies = attempt.native(
        TOOL,
        "place exact source compound",
        root.moved(&location, true),
    )?;
    let bindings = bindings
        .into_iter()
        .map(|(key, shape)| {
            let shape = attempt.native(
                TOOL,
                "place exact source binding",
                shape.moved(&location, true),
            )?;
            Ok((key, shape))
        })
        .collect::<Checked<Vec<_>>>()?;
    let mut entities = BTreeMap::new();
    certify(
        attempt,
        TOOL,
        "publish only operation-local source identities",
        export_bindings::append(keys, names, bindings, &mut entities),
    )?;
    Ok(DetachedWrappedBodies { bodies, entities })
}
