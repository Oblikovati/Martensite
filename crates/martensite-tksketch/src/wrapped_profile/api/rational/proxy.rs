//! Construct the polynomial reference after proving that the source homotopy stays embedded.
use super::*;
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileVertex, ProfileVertexId, SketchProfile,
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
    let mut proxy = begin(options, original, allocated);
    let uses = compose(&mut proxy, original, reference)?;
    proxy.selection.outer.uses = replace_uses(original, uses);
    Ok(proxy)
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
    for line in &original.lines {
        positive_controls(line, original.curve.poles, original.ends.map(Some))?;
        for (index, span) in spans.iter().enumerate() {
            let ends = [
                (index == 0).then_some(original.ends[0]),
                (index + 1 == spans.len()).then_some(original.ends[1]),
            ];
            positive_controls(line, &span.poles, ends)?;
        }
    }
    Ok(())
}

fn positive_controls(
    line: &input::Line,
    controls: &[[f64; 2]],
    ends: [Option<ProfileVertexId>; 2],
) -> Checked<()> {
    let [a, b] = line.points;
    for (index, &point) in controls.iter().enumerate() {
        let endpoint = match index {
            0 => ends[0],
            i if i == controls.len() - 1 => ends[1],
            _ => None,
        };
        let permitted = match orient2d(a, b, point) {
            Ok(OrientationSign::Positive) => true,
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

fn compose(
    proxy: &mut Proxy,
    original: &input::Source<'_>,
    spans: &[RationalReferenceSpan],
) -> Checked<Vec<LoopUse>> {
    let mut ids = Names::new(original);
    proxy
        .profile
        .curves
        .retain(|c| c.id() != original.occurrence.curve);
    let mut start = original.ends[0];
    let mut uses = Vec::with_capacity(spans.len());
    for (index, span) in spans.iter().enumerate() {
        let end = if index + 1 == spans.len() {
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

fn replace_uses(original: &input::Source<'_>, uses: Vec<LoopUse>) -> Vec<LoopUse> {
    let mut expanded = Vec::new();
    for &used in &original.selection.outer.uses {
        if used.curve == original.occurrence.curve {
            expanded.extend_from_slice(&uses);
        } else {
            expanded.push(used);
        }
    }
    expanded
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
        span: &RationalReferenceSpan,
        [start, end]: [ProfileVertexId; 2],
    ) -> Checked<ProfileCurveId> {
        let id = ProfileCurveId(ids.curve.name()?);
        self.profile.curves.push(ProfileCurve::Spline {
            id,
            start,
            end,
            degree: 3,
            poles: span.poles.to_vec(),
            weights: None,
            knots: vec![0., 0., 0., 0., 1., 1., 1., 1.],
        });
        self.spans.insert(
            SurfaceCurveId(id.0),
            WrappedSourceSpan {
                level: span.interval.level(),
                index: span.interval.index(),
                reversed: false,
            },
        );
        Ok(id)
    }

    fn joint(&mut self, ids: &mut Names, span: &RationalReferenceSpan) -> Checked<ProfileVertexId> {
        let id = ProfileVertexId(ids.vertex.name()?);
        self.profile.vertices.push(ProfileVertex {
            id,
            point: span.poles[3],
        });
        self.joins.insert(
            SurfaceVertexId(id.0),
            history::point(span.interval.index() + 1, span.interval.level()),
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
