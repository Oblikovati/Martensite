use super::*;
use martensite_tkbo::profile_extrusion::{
    ProfileCurve, ProfilePlane, ProfileVertex, ProfileVertexId,
};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, StoredPlanarSplineInput};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum VertexKey {
    Native(CylinderBodyRef),
    Knot(CylinderBodyRef, usize),
}

struct Capture<'a> {
    attempt: Attempt,
    roots: &'a Roots,
    body: &'a DetachedCylinderFeature,
    index: body::Index,
    uv: UvInput,
    covered: BTreeSet<CylinderBodyRef>,
    vertices: BTreeMap<VertexKey, (ProfileVertexId, [u64; 2])>,
}

pub(super) fn read(
    attempt: Attempt,
    roots: &Roots,
    body: &DetachedCylinderFeature,
    topology: &topology::Topology,
    cancelled: &mut CancelPoll<'_>,
) -> Checked<UvInput> {
    poll(cancelled)?;
    let mut capture = Capture {
        attempt,
        roots,
        body,
        index: body::Index::new(attempt, &body.body, STAGE)?,
        uv: UvInput {
            profile: SketchProfile {
                plane: ProfilePlane::world_xy(),
                vertices: vec![],
                curves: vec![],
            },
            loops: vec![],
        },
        covered: BTreeSet::new(),
        vertices: BTreeMap::new(),
    };
    for &id in &roots.faces {
        poll(cancelled)?;
        let wires = topology.wires(id).ok_or(Refusal::Role(id))?;
        capture.face(id, wires, cancelled)?;
    }
    if capture.covered.len() != roots.welds.len()
        || capture.uv.profile.vertices.len() != capture.uv.profile.curves.len()
    {
        return Err(Refusal::Incidence(roots.band));
    }
    Ok(capture.uv)
}

impl Capture<'_> {
    fn entity(&self, id: CylinderBodyRef) -> Checked<&Shape> {
        let shape = self.body.entities.get(&id).ok_or(Refusal::Missing(id))?;
        if self.index.reference(shape, self.attempt)? != Some(id) {
            return Err(Refusal::Missing(id));
        }
        Ok(shape)
    }

    fn face(
        &mut self,
        id: CylinderBodyRef,
        wires: &[Vec<topology::DirectedUse>],
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let face = self.attempt.native(
            STAGE,
            "decoded support trim face",
            topods::face(self.entity(id)?),
        )?;
        if face.orientation() != Orientation::Forward
            || !self.attempt.native(
                STAGE,
                "decoded support trim carrier",
                seams::same_carrier(&face, self.roots.chart.datum),
            )?
        {
            return Err(Refusal::Role(id));
        }
        let remainder = id == self.roots.band;
        if wires.len() < if remainder { 2 } else { 1 } || (!remainder && wires.len() != 1) {
            return Err(Refusal::Role(id));
        }
        let mut original = 0;
        for wire in wires {
            poll(cancelled)?;
            if wire.is_empty() {
                return Err(Refusal::Incidence(id));
            }
            if wire
                .iter()
                .all(|usage| !self.roots.welds.contains_key(&usage.edge))
            {
                original += 1;
                continue;
            }
            self.wire(id, &face, wire, remainder, cancelled)?;
        }
        if original != usize::from(remainder) {
            return Err(Refusal::Role(id));
        }
        Ok(())
    }

    fn wire(
        &mut self,
        id: CylinderBodyRef,
        face: &Face,
        wire: &[topology::DirectedUse],
        remainder: bool,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let first = wire.first().ok_or(Refusal::Incidence(id))?;
        let region = self
            .roots
            .welds
            .get(&first.edge)
            .ok_or(Refusal::Incidence(first.edge))?
            .region;
        let mut curves = Vec::new();
        for usage in wire {
            poll(cancelled)?;
            let key = usage.edge;
            let weld = self.roots.welds.get(&key).ok_or(Refusal::Incidence(key))?;
            if weld.face != id || weld.sense != usage.sense {
                return Err(Refusal::Role(id));
            }
            if weld.region != region || !self.covered.insert(key) {
                return Err(Refusal::Incidence(key));
            }
            self.append(face, usage, *weld, &mut curves, cancelled)?;
        }
        let owner = if remainder {
            Image::SupportRemainder
        } else {
            Image::SupportCounterIsland(region)
        };
        self.uv.loops.push(LoopSource {
            owner,
            region,
            curves,
        });
        Ok(())
    }

    fn append(
        &mut self,
        face: &Face,
        usage: &topology::DirectedUse,
        weld: roots::Weld,
        ids: &mut Vec<ProfileCurveId>,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        let edge = self.attempt.native(
            STAGE,
            "decoded support trim edge",
            topods::edge(&self.entity(usage.edge)?.oriented(usage.sense)),
        )?;
        for id in usage.ends {
            self.entity(id)?;
            if id.kind != ShapeEnum::Vertex {
                return Err(Refusal::Incidence(usage.edge));
            }
        }
        let source = seams::EdgeSource::capture(self.attempt, &edge)?;
        check_source(usage.edge, weld, &source, self.roots)?;
        let remaining = proof::PIECE_LIMIT
            .checked_sub(self.uv.profile.curves.len())
            .ok_or(Refusal::Limit(usage.edge))?;
        let spline = spline(self.attempt, usage.edge, &edge, face, remaining)?;
        let (knots, poles) = checked_spline(usage.edge, &spline, self.roots.chart.sheet)?;
        let reversed = match usage.sense {
            Orientation::Forward => false,
            Orientation::Reversed => true,
            _ => return Err(Refusal::Incidence(usage.edge)),
        };
        self.profile(usage, knots.len() - 1, poles, reversed, ids, cancelled)?;
        Ok(())
    }

    fn profile(
        &mut self,
        usage: &topology::DirectedUse,
        count: usize,
        poles: &[[f64; 2]],
        reversed: bool,
        ids: &mut Vec<ProfileCurveId>,
        cancelled: &mut CancelPoll<'_>,
    ) -> Checked<()> {
        for i in 0..count {
            poll(cancelled)?;
            let segment = if reversed { count - 1 - i } else { i };
            let mut controls: [[f64; 2]; 4] = poles
                .get(3 * segment..3 * segment + 4)
                .and_then(|values| values.try_into().ok())
                .ok_or(Refusal::Curve(usage.edge))?;
            if reversed {
                controls.reverse();
            }
            let [first, _, _, last] = controls;
            let start = self.add_vertex(usage, slot(usage, count, i), first)?;
            let end = self.add_vertex(usage, slot(usage, count, i + 1), last)?;
            let id = ProfileCurveId(self.uv.profile.curves.len() as u64 + 1);
            self.uv.profile.curves.push(ProfileCurve::Spline {
                id,
                start,
                end,
                degree: 3,
                poles: controls.to_vec(),
                knots: vec![0., 0., 0., 0., 1., 1., 1., 1.],
                weights: None,
            });
            ids.push(id);
        }
        Ok(())
    }

    fn add_vertex(
        &mut self,
        usage: &topology::DirectedUse,
        key: VertexKey,
        point: [f64; 2],
    ) -> Checked<ProfileVertexId> {
        let (id, bits) = self.vertices.entry(key).or_insert_with(|| {
            let id = ProfileVertexId(self.uv.profile.vertices.len() as u64 + 1);
            self.uv.profile.vertices.push(ProfileVertex { id, point });
            (id, point.map(f64::to_bits))
        });
        if *bits != point.map(f64::to_bits) {
            return Err(Refusal::Incidence(usage.edge));
        }
        Ok(*id)
    }
}

fn slot(usage: &topology::DirectedUse, count: usize, i: usize) -> VertexKey {
    match i {
        0 => VertexKey::Native(usage.ends[0]),
        _ if i == count => VertexKey::Native(usage.ends[1]),
        _ => VertexKey::Knot(usage.edge, i),
    }
}

fn check_source(
    key: CylinderBodyRef,
    weld: roots::Weld,
    source: &seams::EdgeSource,
    roots: &Roots,
) -> Checked<()> {
    if !weld.gap.is_finite()
        || weld.gap < 0.
        || weld.gap >= roots.chart.sew
        || weld.gap > source.raw_tolerance
        || roots.cap.is_some_and(|cap| source.raw_tolerance > cap)
    {
        return Err(Refusal::Limit(key));
    }
    if source
        .poles
        .iter()
        .any(|p| !p.iter().all(|x| x.is_finite()) || p[3] <= 0.)
    {
        return Err(Refusal::Curve(key));
    }
    Ok(())
}

fn spline(
    attempt: Attempt,
    key: CylinderBodyRef,
    edge: &topods::Edge,
    face: &Face,
    remaining: usize,
) -> Checked<StoredPlanarSplineInput> {
    let trace = attempt.native(
        STAGE,
        "actual stock edge pcurve",
        BRepTool::curve_on_face(edge, face),
    )?;
    if !trace.is_stored || [trace.first, trace.last].map(f64::to_bits) != [0., 1.].map(f64::to_bits)
    {
        return Err(Refusal::Curve(key));
    }
    let handle = trace.pcurve.downcast::<Geom2dBSplineCurve>();
    let curve = handle.get().ok_or(Refusal::Curve(key))?;
    let lease = curve.try_input_lease().map_err(|_| Refusal::Curve(key))?;
    let [p, w, k, m, flat] = lease.stored_sizes();
    let Some(count) = k
        .checked_sub(1)
        .filter(|count| (1..=remaining).contains(count))
    else {
        return Err(Refusal::Limit(key));
    };
    if p != 3 * count + 1 || w != p || m != k || flat != p + 4 {
        return Err(Refusal::Curve(key));
    }
    Ok(lease.stored_input())
}

type Controls<'a> = (&'a [f64], &'a [[f64; 2]]);

fn checked_spline(
    key: CylinderBodyRef,
    curve: &StoredPlanarSplineInput,
    sheet: [[f64; 2]; 2],
) -> Checked<Controls<'_>> {
    if curve.degree() != 3
        || curve.rational()
        || curve.periodic()
        || curve.evaluation_override()
        || curve.lower_bounds() != [1; 5]
        || curve.domain().map(f64::to_bits) != [0., 1.].map(f64::to_bits)
        || curve
            .weights()
            .iter()
            .any(|v| v.to_bits() != 1.0_f64.to_bits())
    {
        return Err(Refusal::Curve(key));
    }
    let knots = curve.knots();
    if knots.first().map(|x| x.to_bits()) != Some(0.0_f64.to_bits())
        || knots.last().map(|x| x.to_bits()) != Some(1.0_f64.to_bits())
        || !knots.windows(2).all(|w| w[0] < w[1])
    {
        return Err(Refusal::Curve(key));
    }
    let expanded = knots.iter().enumerate().flat_map(|(i, k)| {
        std::iter::repeat_n(
            k.to_bits(),
            if i == 0 || i + 1 == knots.len() { 4 } else { 3 },
        )
    });
    let mult = (0..knots.len()).map(|i| if i == 0 || i + 1 == knots.len() { 4 } else { 3 });
    if !curve.flat_knots().iter().map(|x| x.to_bits()).eq(expanded)
        || !curve.multiplicities().iter().copied().eq(mult)
        || !curve
            .poles()
            .iter()
            .all(|p| p.iter().zip(sheet).all(|(x, r)| r[0] < *x && *x < r[1]))
    {
        return Err(Refusal::Curve(key));
    }
    Ok((knots, curve.poles()))
}
