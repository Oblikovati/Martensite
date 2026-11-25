//! Construct the polynomial reference after proving that the source homotopy stays embedded.
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileLoop, ProfileVertex, ProfileVertexId, SketchProfile,
};
use std::collections::{BTreeMap, BTreeSet};
mod history;
mod separation;

pub(super) struct Proxy {
    pub profile: SketchProfile,
    pub selection: WrappedRegionSelection,
    pub options: WrappedNormalOptions,
    pub spatial_error: f64,
    spans: BTreeMap<SurfaceCurveId, (CurveOrigin, WrappedSourceSpan)>,
    joins: BTreeMap<SurfaceVertexId, (CurveOrigin, WrappedSourcePoint)>,
}

#[derive(Clone, Copy)]
struct CurveOrigin {
    curve: SurfaceCurveId,
    map: Option<WrappedConicParameterMap>,
}

pub(super) fn build(
    options: WrappedNormalOptions,
    source: &input::Source<'_>,
    reference: &[fit::Cover],
    scale: f64,
) -> Checked<Proxy> {
    let pairs = separation::prove(source, reference)?;
    let allocated = allocate(options.spatial_error, reference, scale)?;
    let mut proxy = begin(options, source, allocated);
    let mut ids = Names::new(source);
    for (curve, cover) in pairs {
        expanded(
            &mut proxy,
            &mut ids,
            curve,
            cover.spans.iter().map(reference_controls),
        )?;
    }
    Ok(proxy)
}

fn reference_controls(span: &RationalReferenceSpan) -> SpanControls<'_> {
    SpanControls {
        poles: &span.poles,
        weights: None,
        domain: WrappedSourceSpan {
            level: span.interval.level(),
            index: span.interval.index(),
            reversed: false,
        },
    }
}

pub(super) fn native_partition(
    source: &input::Source<'_>,
    options: WrappedNormalOptions,
) -> Checked<Proxy> {
    let mut proxy = begin(options, source, [options.spatial_error; 2]);
    let mut ids = Names::new(source);
    for curve in source.curves.iter().filter(|curve| curve.spans.len() > 1) {
        let spans = curve.spans.iter().map(native_controls);
        expanded(&mut proxy, &mut ids, curve, spans)?;
    }
    Ok(proxy)
}

fn native_controls<'a>(span: &'a input::NativeSpan<'_>) -> SpanControls<'a> {
    let metric = span.metric();
    SpanControls {
        poles: metric.poles,
        weights: metric.weights,
        domain: span.domain,
    }
}

fn expanded<'a>(
    proxy: &mut Proxy,
    ids: &mut Names,
    original: &input::Curve<'_>,
    spans: impl ExactSizeIterator<Item = SpanControls<'a>>,
) -> Checked<()> {
    let uses = compose(proxy, ids, original, spans)?;
    for cycle in std::iter::once(&mut proxy.selection.outer).chain(&mut proxy.selection.holes) {
        expand_curve(cycle, original.occurrence.curve, &uses);
    }
    Ok(())
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
        spans: BTreeMap::new(),
        joins: BTreeMap::new(),
    }
}

fn allocate(spatial: f64, reference: &[fit::Cover], scale: f64) -> Checked<[f64; 2]> {
    let measured = fit::all_spans(reference)
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

fn compose<'a>(
    proxy: &mut Proxy,
    ids: &mut Names,
    original: &input::Curve<'_>,
    spans: impl ExactSizeIterator<Item = SpanControls<'a>>,
) -> Checked<Vec<LoopUse>> {
    let origin = CurveOrigin {
        curve: SurfaceCurveId(original.occurrence.curve.0),
        map: original.map,
    };
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
            proxy.joint(ids, span, origin)?
        };
        let id = proxy.curve(ids, span, origin, [start, end])?;
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
        origin: CurveOrigin,
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
        self.spans
            .insert(SurfaceCurveId(id.0), (origin, span.domain));
        Ok(id)
    }

    fn joint(
        &mut self,
        ids: &mut Names,
        span: SpanControls<'_>,
        origin: CurveOrigin,
    ) -> Checked<ProfileVertexId> {
        let id = ProfileVertexId(ids.vertex.name()?);
        self.profile.vertices.push(ProfileVertex {
            id,
            point: span.poles[span.poles.len() - 1],
        });
        self.joins.insert(
            SurfaceVertexId(id.0),
            (
                origin,
                history::point(span.domain.index + 1, span.domain.level),
            ),
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
