//! Close the actual adjacency census and transfer authored separation to every other pair.
use super::*;

#[derive(Clone, Copy)]
pub(super) struct Cover {
    pub(super) allowance: [f64; 2],
    pub(super) contact: f64,
    radius: f64,
    nominal: [f64; 2],
}

impl Cover {
    pub(super) fn read(
        attempt: Attempt,
        support: CylinderSupport,
        tool: &Tool,
        candidate: &Candidate,
    ) -> Checked<Self> {
        let first = tool
            .spans
            .values()
            .next()
            .ok_or_else(|| missing(attempt, VALIDATION, "nonempty side cover"))?;
        let nominal = first.storage.certificate.stored_radii();
        let mut allowance = [0_f64; 2];
        for span in tool.spans.values() {
            let storage = &span.storage.certificate;
            require(
                attempt,
                VALIDATION,
                storage.stored_radii() == nominal,
                "one common radial interval for the selected family",
            )?;
            let error = storage.radial_distance_bounds();
            allowance = std::array::from_fn(|i| allowance[i].max(error[i]));
        }
        Ok(Self {
            allowance,
            contact: contact_budget(attempt, &candidate.native)?,
            radius: support.radius,
            nominal,
        })
    }
}

pub(super) fn validate(
    attempt: Attempt,
    tool: &Tool,
    cover: Cover,
    actual: &BTreeSet<[WrappedHistoryKey; 2]>,
) -> Checked<()> {
    let represented = authored_pairs(attempt, tool)?;
    let joins: BTreeSet<_> = tool
        .joins
        .keys()
        .map(|&mut_pair| sorted(mut_pair))
        .collect();
    require(
        attempt,
        VALIDATION,
        actual == &represented && actual == &joins && actual.len() == tool.spans.len(),
        "the exact authored adjacency set equals decoded shared connectors and join projections",
    )?;
    let pair_count = tool
        .spans
        .len()
        .checked_mul(tool.spans.len().saturating_sub(1))
        .map(|pairs| pairs / 2)
        .and_then(|pairs| pairs.checked_sub(actual.len()));
    require(
        attempt,
        VALIDATION,
        pair_count == Some(tool.clearance.certificate.pair_count()),
        "all non-adjacent native source pairs covered",
    )?;
    let required = projection_limit(attempt, tool, cover)?;
    let periodic = certify(
        attempt,
        VALIDATION,
        "directed authored periodic separation",
        tool.regions.iter().copied().map(HistorySource::Region),
        tool.clearance
            .periodic
            .separation_lower_bound()
            .compare_f64(required),
    )?;
    require(
        attempt,
        VALIDATION,
        tool.clearance.certificate.clearance_lower_bound() > required && periodic.is_gt(),
        "disjoint exact spatial contact tubes for all non-adjacent and other-lift authored traces",
    )
}

fn authored_pairs(attempt: Attempt, tool: &Tool) -> Checked<BTreeSet<[WrappedHistoryKey; 2]>> {
    let names: BTreeMap<_, _> = tool
        .spans
        .iter()
        .map(|(&key, span)| (span.storage.span, key))
        .collect();
    let exact = |i| {
        tool.clearance
            .spans
            .get(i)
            .and_then(|span| names.get(span))
            .copied()
            .ok_or_else(|| {
                missing(
                    attempt,
                    VALIDATION,
                    "authored clearance uses actual source span names",
                )
            })
    };
    tool.clearance
        .certificate
        .excluded_pairs()
        .iter()
        .map(|&[a, b]| Ok(sorted([exact(a)?, exact(b)?])))
        .collect()
}

fn sorted([a, b]: [WrappedHistoryKey; 2]) -> [WrappedHistoryKey; 2] {
    [a.min(b), a.max(b)]
}

fn projection_limit(attempt: Attempt, tool: &Tool, cover: Cover) -> Checked<f64> {
    let radius_floor = (cover.nominal[0] - cover.allowance[0]).next_down();
    let mut position_error = 0_f64;
    for span in tool.spans.values() {
        let stored = &span.storage;
        let radial = stored.certificate.normal_sweep_error_bound();
        let sweep = chart_distance(attempt, radius_floor, cover.radius, radial)?;
        let side_error = (stored.profile.developed_position_bound() + sweep).next_up();
        let cap_error =
            (cover.radius.max(1.) * stored.cap_profile.developed_position_bound()).next_up();
        position_error = position_error.max(side_error).max(cap_error);
    }
    let active = chart_distance(attempt, radius_floor, cover.radius, cover.contact)?;
    Ok((2. * position_error + active).next_up())
}

fn chart_distance(attempt: Attempt, radius_floor: f64, scale: f64, distance: f64) -> Checked<f64> {
    // The whole connecting segment stays within the punctured chart. The periodic
    // certificate disambiguates the complete set of angles represented there.
    let gap = (radius_floor - distance).next_down();
    require(
        attempt,
        VALIDATION,
        gap.is_finite() && gap > 0.,
        "strictly positive radius along the entire contact segment",
    )?;
    let angular = (scale / gap).next_up();
    Ok((distance * (1. + angular).next_up()).next_up())
}

fn contact_budget(attempt: Attempt, native: &DetachedCylinderFeature) -> Checked<f64> {
    let confusion = martensite_tkernel::precision::confusion();
    let mut maximum = confusion;
    for (key, shape) in &native.entities {
        let tol = match key.kind {
            ShapeEnum::Face => face_tolerance(
                attempt,
                &attempt.native(VALIDATION, "decoded candidate face", topods::face(shape))?,
            )?,
            ShapeEnum::Edge => attempt.native(
                VALIDATION,
                "decoded candidate edge tolerance",
                BRepTool::edge_tolerance(&attempt.native(
                    VALIDATION,
                    "decoded candidate edge",
                    topods::edge(shape),
                )?),
            )?,
            ShapeEnum::Vertex => attempt.native(
                VALIDATION,
                "decoded candidate vertex tolerance",
                BRepTool::vertex_tolerance(&attempt.native(
                    VALIDATION,
                    "decoded candidate vertex",
                    topods::vertex(shape),
                )?),
            )?,
            ShapeEnum::Solid => continue,
            _ => {
                return Err(missing(
                    attempt,
                    VALIDATION,
                    "supported decoded contact members",
                ));
            }
        };
        require(
            attempt,
            VALIDATION,
            valid_tol(tol),
            "positive finite tolerance of each contact member",
        )?;
        maximum = maximum.max(tol);
    }
    let budget = (2. * maximum + confusion).next_up();
    require(
        attempt,
        VALIDATION,
        budget.is_finite(),
        "finite containing bound on native pair contact",
    )?;
    Ok(budget)
}
