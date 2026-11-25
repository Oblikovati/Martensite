//! Keep a paired authored-to-reference homotopy clear except at proven incidences.
use super::*;
use martensite_tkmath::predicates::{OrientationSign, orient2d};

type Paired<'a, 'source> = (&'a input::Curve<'source>, &'a fit::Cover);

pub(super) fn prove<'a, 'source>(
    source: &'a input::Source<'source>,
    reference: &'a [fit::Cover],
) -> Checked<Vec<Paired<'a, 'source>>> {
    let pairs = pair_sources(source, reference)?;
    for (index, &pair) in pairs.iter().enumerate() {
        line_boundaries(source, pair)?;
        for &other in &pairs[..index] {
            curves(other, pair)?;
        }
    }
    Ok(pairs)
}

fn pair_sources<'a, 'source>(
    source: &'a input::Source<'source>,
    reference: &'a [fit::Cover],
) -> Checked<Vec<Paired<'a, 'source>>> {
    source
        .curves
        .iter()
        .map(|curve| {
            let id = curve.occurrence.curve;
            reference
                .iter()
                .find(|cover| cover.curve == id)
                .map(|cover| (curve, cover))
                .ok_or_else(|| {
                    invalid(
                        format!("authored curve={id:?}"),
                        "its certified cubic cover",
                    )
                })
        })
        .collect()
}

fn line_boundaries(source: &input::Source<'_>, pair: Paired<'_, '_>) -> Checked<()> {
    let (curve, cover) = pair;
    let (cycle, interior) = curve_cycle(source, curve.occurrence.curve)?;
    for line in &source.lines {
        if contains(cycle, line.id) {
            halfplane(pair, line.points, line.ends.as_slice(), interior)?;
        } else {
            exclude_tubes(
                cover,
                std::iter::once(Tube::controls(&line.points, 0.)),
                line.id,
            )?;
        }
    }
    Ok(())
}

fn curve_cycle<'a>(
    source: &'a input::Source<'_>,
    id: ProfileCurveId,
) -> Checked<(&'a ProfileLoop, OrientationSign)> {
    if contains(&source.selection.outer, id) {
        return Ok((&source.selection.outer, OrientationSign::Positive));
    }
    source
        .selection
        .holes
        .iter()
        .find(|cycle| contains(cycle, id))
        .map(|cycle| (cycle, OrientationSign::Negative))
        .ok_or_else(|| invalid(format!("curve={id:?}"), "an authored boundary occurrence"))
}

fn contains(cycle: &ProfileLoop, curve: ProfileCurveId) -> bool {
    cycle.uses.iter().any(|used| used.curve == curve)
}

fn curves(left: Paired<'_, '_>, right: Paired<'_, '_>) -> Checked<()> {
    let (a, left_ref) = left;
    let (b, right_ref) = right;
    let shared: BTreeSet<_> = a
        .ends
        .into_iter()
        .flatten()
        .filter(|vertex| b.ends.is_some_and(|ends| ends.contains(vertex)))
        .collect();
    let shared: Vec<_> = shared.into_iter().collect();
    if shared.is_empty() {
        return exclude_tubes(
            left_ref,
            right_ref
                .spans
                .iter()
                .map(|fitted| Tube::span(&fitted.span)),
            right_ref.curve,
        );
    }
    let cut = separating_line(a, b, &shared)?;
    let interior = a
        .poles()
        .filter_map(|p| orient2d(cut[0], cut[1], p).ok())
        .find(|sign| *sign != OrientationSign::Zero)
        .ok_or_else(|| pair_failure(a, b))?;
    halfplane(left, cut, &shared, interior)?;
    halfplane(right, cut, &shared, opposite(interior))
}

fn opposite(sign: OrientationSign) -> OrientationSign {
    match sign {
        OrientationSign::Positive => OrientationSign::Negative,
        OrientationSign::Negative => OrientationSign::Positive,
        OrientationSign::Zero => OrientationSign::Zero,
    }
}

fn separating_line(
    a: &input::Curve<'_>,
    b: &input::Curve<'_>,
    shared: &[ProfileVertexId],
) -> Checked<[[f64; 2]; 2]> {
    match shared {
        [first, last] => Ok([endpoint(a, *first)?, endpoint(a, *last)?]),
        [anchor] => {
            let point = endpoint(a, *anchor)?;
            let [u, v] = [other_endpoint(a, *anchor)?, other_endpoint(b, *anchor)?];
            let cut = [point, [point[0] - (v[1] - u[1]), point[1] + (v[0] - u[0])]];
            if cut.iter().flatten().all(|v| v.is_finite()) && cut[0] != cut[1] {
                return Ok(cut);
            }
            Err(pair_failure(a, b))
        }
        _ => Err(pair_failure(a, b)),
    }
}

fn pair_failure(a: &input::Curve<'_>, b: &input::Curve<'_>) -> RationalWrappedFailure {
    invalid(
        format!(
            "authored curves={:?}, {:?}",
            a.occurrence.curve, b.occurrence.curve
        ),
        "an exact separating line through shared authored endpoints",
    )
}

fn endpoint(curve: &input::Curve<'_>, vertex: ProfileVertexId) -> Checked<[f64; 2]> {
    curve
        .endpoint_ids()
        .iter()
        .position(|&end| end == Some(vertex))
        .map(|index| curve.endpoints()[index])
        .ok_or_else(|| {
            invalid(
                format!("curve={:?}, vertex={vertex:?}", curve.occurrence.curve),
                "an authored endpoint",
            )
        })
}

fn other_endpoint(curve: &input::Curve<'_>, vertex: ProfileVertexId) -> Checked<[f64; 2]> {
    let other = curve
        .ends
        .into_iter()
        .flatten()
        .find(|&end| end != vertex)
        .ok_or_else(|| {
            invalid(
                format!("curve={:?}, vertex={vertex:?}", curve.occurrence.curve),
                "a distinct other endpoint",
            )
        })?;
    endpoint(curve, other)
}

fn halfplane(
    pair: Paired<'_, '_>,
    cut: [[f64; 2]; 2],
    allowed: &[ProfileVertexId],
    sign: OrientationSign,
) -> Checked<()> {
    let (curve, cover) = pair;
    let check = Halfplane {
        cut,
        allowed,
        sign,
        curve: cover.curve,
    };
    check.spans(
        curve.endpoint_ids(),
        curve
            .spans
            .iter()
            .map(|span| (span.metric().poles, span.enclosure())),
    )?;
    check.spans(
        curve.endpoint_ids(),
        cover
            .spans
            .iter()
            .map(|fitted| (fitted.span.poles.as_slice(), None)),
    )
}

struct Halfplane<'a> {
    cut: [[f64; 2]; 2],
    allowed: &'a [ProfileVertexId],
    sign: OrientationSign,
    curve: ProfileCurveId,
}

type ControlInputs<'a> = (&'a [[f64; 2]], Option<&'a [[[f64; 2]; 2]; 3]>);

impl Halfplane<'_> {
    fn spans<'a>(
        &self,
        [start, end]: [Option<ProfileVertexId>; 2],
        poles: impl ExactSizeIterator<Item = ControlInputs<'a>>,
    ) -> Checked<()> {
        let count = poles.len();
        for (index, (controls, exact)) in poles.enumerate() {
            let shared = [
                (index == 0).then_some(start).flatten(),
                (index + 1 == count).then_some(end).flatten(),
            ];
            self.controls(controls, exact, shared)?;
        }
        Ok(())
    }

    fn controls(
        &self,
        controls: &[[f64; 2]],
        exact: Option<&[[[f64; 2]; 2]; 3]>,
        shared: [Option<ProfileVertexId>; 2],
    ) -> Checked<()> {
        for (index, &point) in controls.iter().enumerate() {
            let endpoint = match index {
                0 => shared[0],
                i if i + 1 == controls.len() => shared[1],
                _ => None,
            };
            let permitted = exact.map_or_else(
                || self.permits(point, endpoint),
                |bounds| {
                    input::enclosure_corners(bounds[index])
                        .into_iter()
                        .all(|corner| self.permits(corner, endpoint))
                },
            );
            if !permitted {
                return Err(invalid(
                    format!(
                        "curve={:?}, control={point:?}, separator={:?}",
                        self.curve, self.cut
                    ),
                    "strictly separated controls or a common authored endpoint",
                ));
            }
        }
        Ok(())
    }

    fn permits(&self, point: [f64; 2], endpoint: Option<ProfileVertexId>) -> bool {
        match orient2d(self.cut[0], self.cut[1], point) {
            Ok(sign) if sign == self.sign => true,
            Ok(OrientationSign::Zero) => endpoint.is_some_and(|id| self.allowed.contains(&id)),
            _ => false,
        }
    }
}

fn exclude_tubes(
    cover: &fit::Cover,
    other: impl Iterator<Item = Tube> + Clone,
    id: ProfileCurveId,
) -> Checked<()> {
    for fitted in &cover.spans {
        let span = &fitted.span;
        let tube = Tube::span(span);
        if !other.clone().all(|target| tube.separate(target)) {
            return Err(invalid(
                format!(
                    "curve={:?}, interval={:?}, separate curve={id:?}",
                    cover.curve, span.interval
                ),
                "strictly separated source-to-reference error tubes",
            ));
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct Tube {
    lower: [f64; 2],
    upper: [f64; 2],
}

impl Tube {
    fn span(span: &RationalReferenceSpan) -> Self {
        Self::controls(&span.poles, span.bounds.position)
    }

    fn controls(points: &[[f64; 2]], error: f64) -> Self {
        Self {
            lower: std::array::from_fn(|axis| {
                let low = points
                    .iter()
                    .map(|point| point[axis])
                    .fold(f64::INFINITY, f64::min);
                (low - error).next_down()
            }),
            upper: std::array::from_fn(|axis| {
                let high = points
                    .iter()
                    .map(|point| point[axis])
                    .fold(f64::NEG_INFINITY, f64::max);
                (high + error).next_up()
            }),
        }
    }

    fn separate(self, other: Self) -> bool {
        (0..2).any(|axis| {
            self.upper[axis] < other.lower[axis] || other.upper[axis] < self.lower[axis]
        })
    }
}
