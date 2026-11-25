//! Construct the polynomial reference after proving that the source homotopy stays embedded.
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileLoop, ProfileVertex, ProfileVertexId, SketchProfile,
};
use martensite_tkmath::predicates::{
    OrientationSign, orient2d, roots::metric_bounds::RationalReferenceSpan,
};
use std::collections::{BTreeMap, BTreeSet};
mod history;

pub(super) struct Proxy {
    pub profile: SketchProfile,
    pub selection: WrappedRegionSelection,
    pub options: WrappedNormalOptions,
    pub spatial_error: f64,
    source: SurfaceCurveId,
    parameter_map: Option<WrappedConicParameterMap>,
    spans: BTreeMap<SurfaceCurveId, WrappedSourceSpan>,
    joins: BTreeMap<SurfaceVertexId, WrappedSourcePoint>,
}

pub(super) fn build(
    options: WrappedNormalOptions,
    original: &input::Source<'_>,
    reference: &[RationalReferenceSpan],
    scale: f64,
) -> Checked<Proxy> {
    prove_separation(original, reference)?;
    let allocated = allocate(options.spatial_error, reference, scale)?;
    let spans = reference.iter().map(|span| SpanControls {
        poles: &span.poles,
        weights: None,
        domain: WrappedSourceSpan {
            level: span.interval.level(),
            index: span.interval.index(),
            reversed: false,
        },
    });
    expanded(begin(options, original, allocated), original, spans)
}

pub(super) fn native_partition(
    original: &input::Source<'_>,
    options: WrappedNormalOptions,
) -> Checked<Proxy> {
    let spans = original.spans.iter().map(|span| SpanControls {
        poles: span.metric().poles,
        weights: span.metric().weights,
        domain: span.domain,
    });
    expanded(
        begin(options, original, [options.spatial_error; 2]),
        original,
        spans,
    )
}

fn expanded<'a>(
    mut proxy: Proxy,
    original: &input::Source<'_>,
    spans: impl ExactSizeIterator<Item = SpanControls<'a>>,
) -> Checked<Proxy> {
    let uses = compose(&mut proxy, original, spans)?;
    for cycle in std::iter::once(&mut proxy.selection.outer).chain(&mut proxy.selection.holes) {
        expand_curve(cycle, original.occurrence.curve, &uses);
    }
    Ok(proxy)
}

#[derive(Clone, Copy)]
struct SpanControls<'a> {
    poles: &'a [[f64; 2]],
    weights: Option<&'a [f64]>,
    domain: WrappedSourceSpan,
}

fn begin(
    mut options: WrappedNormalOptions,
    original: &input::Source<'_>,
    [allocated, spatial_error]: [f64; 2],
) -> Proxy {
    options.spatial_error = allocated;
    Proxy {
        profile: original.profile.clone(),
        selection: original.selection.clone(),
        options,
        spatial_error,
        source: SurfaceCurveId(original.occurrence.curve.0),
        parameter_map: original.map,
        spans: BTreeMap::new(),
        joins: BTreeMap::new(),
    }
}

fn allocate(spatial: f64, reference: &[RationalReferenceSpan], scale: f64) -> Checked<[f64; 2]> {
    let measured = reference
        .iter()
        .map(|span| span.bounds.position)
        .fold(0., f64::max);
    let transfer = (measured * scale).next_up();
    // Keep rounding of the partition separate from the two actual world-space proofs.
    let local = ((spatial - transfer).next_down()).next_down();
    let combined = (local + transfer).next_up();
    if !(local > 0. && combined <= spatial) {
        return Err(invalid(
            format!("requested spatial limit={spatial}, source transfer upper={transfer}"),
            "positive remaining spatial allowance",
        ));
    }
    Ok([local, combined])
}

fn prove_separation(original: &input::Source<'_>, spans: &[RationalReferenceSpan]) -> Checked<()> {
    let outer = &original.selection.outer;
    let curve_uses = std::iter::once(outer)
        .chain(&original.selection.holes)
        .find(|loop_uses| contains(loop_uses, original.occurrence.curve))
        .ok_or_else(|| {
            invalid(
                format!("curve={:?}", original.occurrence.curve),
                "an authored boundary occurrence",
            )
        })?;
    let interior = if contains(outer, original.occurrence.curve) {
        OrientationSign::Positive
    } else {
        OrientationSign::Negative
    };
    for line in &original.lines {
        if contains(curve_uses, line.id) {
            check_controls(
                line,
                original,
                original.spans.iter().map(|span| span.metric().poles),
                interior,
            )?;
            check_controls(
                line,
                original,
                spans.iter().map(|span| span.poles.as_slice()),
                interior,
            )?;
        } else {
            separate_boundary(line, spans)?;
        }
    }
    Ok(())
}

fn contains(cycle: &ProfileLoop, curve: ProfileCurveId) -> bool {
    cycle.uses.iter().any(|used| used.curve == curve)
}

fn separate_boundary(line: &input::Line, reference: &[RationalReferenceSpan]) -> Checked<()> {
    for span in reference {
        if !(0..2).any(|axis| disjoint_axis(line, span, axis)) {
            return Err(invalid(
                format!(
                    "curve interval={:?}, separate line={:?}",
                    span.interval, line.id
                ),
                "an error-bounded reference tube clear of every other boundary loop",
            ));
        }
    }
    Ok(())
}

fn disjoint_axis(line: &input::Line, span: &RationalReferenceSpan, axis: usize) -> bool {
    let [a, b] = line.points.map(|point| point[axis]);
    let lo = span
        .poles
        .iter()
        .map(|point| point[axis])
        .fold(f64::INFINITY, f64::min);
    let hi = span
        .poles
        .iter()
        .map(|point| point[axis])
        .fold(f64::NEG_INFINITY, f64::max);
    (hi + span.bounds.position).next_up() < a.min(b)
        || b.max(a) < (lo - span.bounds.position).next_down()
}

fn check_controls<'a>(
    line: &input::Line,
    original: &input::Source<'_>,
    poles: impl ExactSizeIterator<Item = &'a [[f64; 2]]>,
    interior: OrientationSign,
) -> Checked<()> {
    let count = poles.len();
    for (index, controls) in poles.enumerate() {
        let ends = [
            (index == 0).then_some(original.ends[0]),
            (index + 1 == count).then_some(original.ends[1]),
        ];
        bound_controls(line, controls, ends, interior)?;
    }
    Ok(())
}

fn bound_controls(
    line: &input::Line,
    controls: &[[f64; 2]],
    ends: [Option<ProfileVertexId>; 2],
    interior: OrientationSign,
) -> Checked<()> {
    let [a, b] = line.points;
    for (index, &point) in controls.iter().enumerate() {
        let endpoint = match index {
            0 => ends[0],
            i if i == controls.len() - 1 => ends[1],
            _ => None,
        };
        let permitted = match orient2d(a, b, point) {
            Ok(sign) if sign == interior => true,
            Ok(OrientationSign::Zero) => endpoint.is_some_and(|id| line.ends.contains(&id)),
            _ => false,
        };
        if !permitted {
            return Err(invalid(
                format!("curve control={point:?}, supporting line={:?}", line.id),
                "strictly interior source and reference controls or a shared authored endpoint",
            ));
        }
    }
    Ok(())
}

fn compose<'a>(
    proxy: &mut Proxy,
    original: &input::Source<'_>,
    spans: impl ExactSizeIterator<Item = SpanControls<'a>>,
) -> Checked<Vec<LoopUse>> {
    let mut ids = Names::new(original);
    proxy
        .profile
        .curves
        .retain(|c| c.id() != original.occurrence.curve);
    let mut start = original.ends[0];
    let count = spans.len();
    let mut uses = Vec::with_capacity(count);
    for (index, span) in spans.enumerate() {
        let end = if index + 1 == count {
            original.ends[1]
        } else {
            proxy.joint(&mut ids, span)?
        };
        let id = proxy.curve(&mut ids, span, [start, end])?;
        uses.push(LoopUse {
            curve: id,
            reversed: original.occurrence.reversed,
        });
        start = end;
    }
    if original.occurrence.reversed {
        uses.reverse();
    }
    Ok(uses)
}

fn expand_curve(cycle: &mut ProfileLoop, curve: ProfileCurveId, uses: &[LoopUse]) {
    if let Some(index) = cycle.uses.iter().position(|used| used.curve == curve) {
        cycle.uses.splice(index..=index, uses.iter().copied());
    }
}

impl Proxy {
    pub(super) fn cylinder(&self, request: &WrappedNormalRequest) -> WrappedNormalRequest {
        WrappedNormalRequest {
            profile: self.profile.clone(),
            selections: vec![self.selection.clone()],
            options: self.options,
            support: request.support,
            depths: request.depths,
        }
    }

    pub(super) fn cone(&self, request: &WrappedConeNormalRequest) -> WrappedConeNormalRequest {
        WrappedConeNormalRequest {
            profile: self.profile.clone(),
            selections: vec![self.selection.clone()],
            options: self.options,
            support: request.support,
            depths: request.depths,
            proof: request.proof,
        }
    }

    fn curve(
        &mut self,
        ids: &mut Names,
        span: SpanControls<'_>,
        [start, end]: [ProfileVertexId; 2],
    ) -> Checked<ProfileCurveId> {
        let id = ProfileCurveId(ids.curve.name()?);
        let count = span.poles.len();
        self.profile.curves.push(ProfileCurve::Spline {
            id,
            start,
            end,
            degree: (count - 1) as u32,
            poles: span.poles.to_vec(),
            weights: span.weights.map(<[f64]>::to_vec),
            knots: std::iter::repeat_n(0., count)
                .chain(std::iter::repeat_n(1., count))
                .collect(),
        });
        self.spans.insert(SurfaceCurveId(id.0), span.domain);
        Ok(id)
    }

    fn joint(&mut self, ids: &mut Names, span: SpanControls<'_>) -> Checked<ProfileVertexId> {
        let id = ProfileVertexId(ids.vertex.name()?);
        self.profile.vertices.push(ProfileVertex {
            id,
            point: span.poles[span.poles.len() - 1],
        });
        self.joins.insert(
            SurfaceVertexId(id.0),
            history::point(span.domain.index + 1, span.domain.level),
        );
        Ok(id)
    }
}

struct Names {
    curve: NamePool,
    vertex: NamePool,
}
struct NamePool {
    used: BTreeSet<u64>,
    cursor: u64,
}

impl Names {
    fn new(source: &input::Source<'_>) -> Self {
        Self {
            curve: NamePool {
                used: source.profile.curves.iter().map(|c| c.id().0).collect(),
                cursor: 0,
            },
            vertex: NamePool {
                used: source.profile.vertices.iter().map(|v| v.id.0).collect(),
                cursor: 0,
            },
        }
    }
}
impl NamePool {
    fn name(&mut self) -> Checked<u64> {
        loop {
            self.cursor = self.cursor.checked_add(1).ok_or_else(|| {
                invalid(
                    "reference ownership exhausted u64",
                    "one available temporary occurrence identifier",
                )
            })?;
            if self.used.insert(self.cursor) {
                return Ok(self.cursor);
            }
        }
    }
}
