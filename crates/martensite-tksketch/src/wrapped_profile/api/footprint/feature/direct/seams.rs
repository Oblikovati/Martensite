//! A fitted seam can be installed only after full-interval correspondence to its native edge.
use super::*;
use martensite_tkbrep::topexp::{first_vertex, last_vertex};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomSurface};
use martensite_tkmath::{
    gp::Point2,
    predicates::roots::metric_bounds::{
        CertifiedConicalTrim, ConicalTrimCarrier, ConicalTrimQuery, MetricBezier,
    },
    toploc::Location,
};
mod enclosure;
mod fast;
mod fit;
mod frozen;
mod trim_proof;
use enclosure::{Homogeneous, world_homogeneous};
use fit::Fit;
pub(super) use frozen::{FrozenIncidence, FrozenSeam, InstalledSeam, ReboundSeam, same_carrier};
use trim_proof::TrimProof;

pub(super) struct Carrier {
    surface: Handle<dyn GeomSurface>,
    location: Location,
    datum: ConicalTrimCarrier,
    sheet: [[f64; 2]; 2],
    sew: f64,
}

impl Carrier {
    pub(super) fn capture(
        attempt: Attempt,
        face: &Face,
        resolution: &Resolution,
    ) -> FeatureChecked<Self> {
        let (surface, location) = attempt.native(
            ASSEMBLY,
            "stock cone carrier",
            BRepTool::surface_with_location(face),
        )?;
        let cone = surface.downcast::<GeomConicalSurface>();
        if cone.is_null()
            || !location.is_identity()
            || resolution.sew <= 0.
            || !resolution.sew.is_finite()
        {
            return Err(refused(attempt, "positive sew and world-axis cone"));
        }
        let cone = attempt.native(ASSEMBLY, "stock cone datum", cone.cone())?;
        let pos = cone.position();
        if *pos.x_direction().coordinates() != [1., 0., 0.]
            || *pos.y_direction().coordinates() != [0., 1., 0.]
            || *pos.direction().coordinates() != [0., 0., 1.]
            || pos.location().x() != 0.
            || pos.location().y() != 0.
        {
            return Err(refused(
                attempt,
                "stock cone frame aligned with world X/Y/Z",
            ));
        }
        let datum = ConicalTrimCarrier {
            radius: cone.ref_radius(),
            axial_origin: pos.location().z(),
            semi_angle: cone.semi_angle(),
        };
        let sheet = rectangle(attempt, face)?;
        Ok(Self {
            surface,
            location,
            datum,
            sheet,
            sew: resolution.sew,
        })
    }

    pub(super) fn stock_chart(&self) -> super::stock::StockChart {
        super::stock::StockChart {
            datum: self.datum,
            sheet: self.sheet,
            sew: self.sew,
        }
    }

    pub(super) fn certify<'a>(
        &'a self,
        attempt: Attempt,
        edge: topods::Edge,
        cancelled: &mut CancelPoll<'_>,
    ) -> FeatureChecked<StrictSeam<'a>> {
        let source = EdgeSource::capture(attempt, &edge)?;
        let exact = source
            .poles
            .iter()
            .map(|p| world_homogeneous(*p, &source.location))
            .collect::<Option<Homogeneous>>()
            .ok_or_else(|| refused(attempt, "finite positive homogeneous seam controls"))?;
        let control = Fit {
            carrier: self,
            allowance: self.sew.min(source.tolerance),
            endpoints: source.endpoints,
        };
        let mut pieces = Vec::new();
        control.fit(attempt, exact, [0., 1.], 0, &mut pieces, cancelled)?;
        Ok(StrictSeam {
            edge,
            source,
            pieces,
            carrier: self,
        })
    }
}

fn rectangle(attempt: Attempt, face: &Face) -> FeatureChecked<[[f64; 2]; 2]> {
    let edges = occurrences(attempt, face, ShapeEnum::Edge)?;
    if edges.len() != 4 {
        return Err(refused(attempt, "exact four-use native stock band"));
    }
    let mut sides = [Vec::new(), Vec::new()];
    let mut periodic_ranges = Vec::new();
    for edge in edges {
        let edge = attempt.native(ASSEMBLY, "stock band edge", topods::edge(&edge))?;
        let p = attempt.native(
            ASSEMBLY,
            "stock band pcurve",
            BRepTool::curve_on_face(&edge, face),
        )?;
        let line = p.pcurve.downcast::<Geom2dLine>();
        if !p.is_stored || line.is_null() || p.first != 0. {
            return Err(refused(attempt, "stored axis-aligned stock boundary"));
        }
        let direction = line.direction();
        let origin = line.location();
        if direction.x() == 1.
            && direction.y() == 0.
            && origin.x() == 0.
            && p.last == std::f64::consts::TAU
        {
            sides[1].push(origin.y());
        } else if direction.x() == 0. && direction.y() == 1. && origin.y() == 0. {
            sides[0].push(origin.x());
            periodic_ranges.push([p.first, p.last]);
        } else {
            return Err(refused(
                attempt,
                "rectangular stock boundaries in the native sheet",
            ));
        }
    }
    for pair in &mut sides {
        pair.sort_by(f64::total_cmp);
    }
    let [u, v] = sides;
    if u != [0., std::f64::consts::TAU]
        || v.len() != 2
        || v[0] >= v[1]
        || !v.iter().all(|x| x.is_finite())
        || periodic_ranges
            .iter()
            .any(|range| range.as_slice() != v.as_slice())
    {
        return Err(refused(
            attempt,
            "one periodic sheet and ordered finite axial trims",
        ));
    }
    Ok([[u[0], u[1]], [v[0], v[1]]])
}

#[derive(Debug)]
pub(super) struct EdgeSource {
    pub(super) poles: Vec<[f64; 4]>,
    pub(super) location: [f64; 13],
    pub(super) endpoints: [[f64; 3]; 2],
    pub(super) tolerance: f64,
    pub(super) raw_tolerance: f64,
}

impl EdgeSource {
    pub(super) fn capture(attempt: Attempt, edge: &topods::Edge) -> FeatureChecked<Self> {
        let native = attempt.native(
            ASSEMBLY,
            "actual native seam curve",
            BRepTool::curve_with_location(edge),
        )?;
        let curve = native.curve.downcast::<GeomBezierCurve>();
        let transform = native.location.transformation();
        if curve.is_null()
            || curve.has_eval_representation()
            || [native.first, native.last] != [0., 1.]
            || transform.scale_factor() != 1.
            || !(2..=10).contains(&curve.nb_poles())
        {
            return Err(refused(
                attempt,
                "native rational Bézier and full unit parameter range",
            ));
        }
        let poles = (1..=curve.nb_poles())
            .map(|i| {
                let p = attempt.native(ASSEMBLY, "seam pole", curve.pole(i))?;
                let w = attempt.native(ASSEMBLY, "seam weight", curve.weight(i))?;
                Ok([p.x(), p.y(), p.z(), w])
            })
            .collect::<FeatureChecked<Vec<_>>>()?;
        let mut location = [1.; 13];
        location[1..4].copy_from_slice(transform.translation_part().coordinates());
        for (i, row) in transform.matrix().coefficients().iter().enumerate() {
            location[4 + 3 * i..7 + 3 * i].copy_from_slice(row);
        }
        let first = attempt.native(
            ASSEMBLY,
            "seam first vertex",
            BRepTool::pnt(&first_vertex(edge, false)),
        )?;
        let last = attempt.native(
            ASSEMBLY,
            "seam last vertex",
            BRepTool::pnt(&last_vertex(edge, false)),
        )?;
        let raw_tolerance = attempt
            .native(
                ASSEMBLY,
                "raw seam tolerance",
                martensite_tkbrep::brep::access::edge_tshape(edge, "direct seam source"),
            )?
            .tolerance();
        let tolerance = raw_tolerance.max(martensite_tkernel::precision::confusion());
        let endpoints = [*first.coordinates(), *last.coordinates()];
        let finite = location
            .iter()
            .chain(endpoints.iter().flatten())
            .all(|x| x.is_finite());
        if !finite || !raw_tolerance.is_finite() || raw_tolerance < 0. {
            return Err(refused(
                attempt,
                "finite seam endpoints and transform and positive native tolerance",
            ));
        }
        Ok(Self {
            poles,
            location,
            endpoints,
            tolerance,
            raw_tolerance,
        })
    }

    fn stored_bits(&self) -> impl Iterator<Item = u64> + '_ {
        self.poles
            .iter()
            .flatten()
            .chain(&self.location)
            .chain(self.endpoints.iter().flatten())
            .copied()
            .chain([self.raw_tolerance, self.tolerance])
            .map(f64::to_bits)
    }
}

struct Span {
    fit: SpanFit,
    proof: TrimProof,
}

struct SpanFit {
    range: [f64; 2],
    poles: Vec<[f64; 3]>,
    weights: Vec<f64>,
    pcurve: [[f64; 2]; 4],
    limit: f64,
    roundoff: f64,
}

impl SpanFit {
    fn query(&self, carrier: ConicalTrimCarrier) -> ConicalTrimQuery<'_> {
        ConicalTrimQuery {
            curve: MetricBezier {
                poles: &self.poles,
                weights: Some(&self.weights),
            },
            pcurve: &self.pcurve,
            carrier,
            spatial_limit: self.limit,
            options: Default::default(),
        }
    }
}
pub(super) struct StrictSeam<'a> {
    edge: topods::Edge,
    source: EdgeSource,
    pieces: Vec<Span>,
    carrier: &'a Carrier,
}

impl<'a> StrictSeam<'a> {
    pub(super) fn install(
        self,
        attempt: Attempt,
        target: &Face,
    ) -> FeatureChecked<InstalledSeam<'a>> {
        let (shape, loc) = attempt.native(
            ASSEMBLY,
            "target cone carrier",
            BRepTool::surface_with_location(target),
        )?;
        let actual = EdgeSource::capture(attempt, &self.edge)?;
        if shape != self.carrier.surface
            || !loc.is_equal(&self.carrier.location)
            || !attempt.native(
                ASSEMBLY,
                "target cone frame",
                frozen::same_carrier(target, self.carrier.datum),
            )?
            || !actual.stored_bits().eq(self.source.stored_bits())
        {
            return Err(refused(
                attempt,
                "retained edge and stock carrier match the strict seam source",
            ));
        }
        let spline = self.layout(attempt)?.make(attempt)?;
        let builder = BRepBuilder::default();
        attempt.native(
            ASSEMBLY,
            "store certified support pcurve",
            builder.update_edge_pcurve_on_face(
                &self.edge,
                &spline,
                target,
                self.source.raw_tolerance,
            ),
        )?;
        attempt.native(
            ASSEMBLY,
            "complete certified support pcurve range",
            builder.range_on_face(&self.edge, target, 0., 1.),
        )?;
        Ok(InstalledSeam {
            target: target.clone(),
            original: self,
        })
    }

    fn layout(&self, attempt: Attempt) -> FeatureChecked<SplineLayout> {
        let first = self
            .pieces
            .first()
            .ok_or_else(|| refused(attempt, "nonempty certified seam"))?;
        let mut knots = vec![0.];
        let mut points = vec![first.fit.pcurve[0]];
        for span in &self.pieces {
            let fit = &span.fit;
            if !span.proof.matches_source(fit.query(self.carrier.datum))
                || knots.last() != Some(&fit.range[0])
                || points.last().map(|p| p.map(f64::to_bits))
                    != Some(fit.pcurve[0].map(f64::to_bits))
            {
                return Err(refused(attempt, "contiguous exact certified seam pieces"));
            }
            knots.push(fit.range[1]);
            points.extend(&fit.pcurve[1..]);
        }
        let end = knots.len() - 1;
        if knots[end] != 1. {
            return Err(refused(attempt, "complete unit seam interval"));
        }
        let objects = points
            .into_iter()
            .map(|[u, v]| Point2::new(u, v))
            .collect::<Vec<_>>();
        let mut multiplicities = vec![3; knots.len()];
        multiplicities[0] = 4;
        multiplicities[end] = 4;
        Ok(SplineLayout {
            knots,
            poles: objects,
            multiplicities,
        })
    }
}

struct SplineLayout {
    knots: Vec<f64>,
    poles: Vec<Point2>,
    multiplicities: Vec<i32>,
}
impl SplineLayout {
    fn make(&self, attempt: Attempt) -> FeatureChecked<Handle<dyn Geom2dCurve>> {
        let made = attempt.native(
            ASSEMBLY,
            "certified piecewise cubic B-spline",
            Geom2dBSplineCurve::new(
                &Array1::from_shared(&self.poles, 1),
                &Array1::from_shared(&self.knots, 1),
                &Array1::from_shared(&self.multiplicities, 1),
                3,
                false,
            ),
        )?;
        if !attempt.native(
            ASSEMBLY,
            "rebind certified B-spline coefficients",
            self.matches(&made),
        )? {
            return Err(refused(
                attempt,
                "native spline stores the exact certified knots and poles",
            ));
        }
        Ok(Handle::new(made).into_upcast())
    }

    fn matches(&self, curve: &Geom2dBSplineCurve) -> Result<bool, Failure> {
        if curve.degree() != 3
            || curve.is_rational()
            || curve.is_periodic()
            || curve.has_eval_representation()
            || curve.nb_knots() as usize != self.knots.len()
            || curve.nb_poles() as usize != self.poles.len()
        {
            return Ok(false);
        }
        let knot_matches = |i: usize| -> Result<bool, Failure> {
            Ok(
                curve.knot(i as i32 + 1)?.to_bits() == self.knots[i].to_bits()
                    && curve.multiplicity(i as i32 + 1)? == self.multiplicities[i],
            )
        };
        let pole_matches = |(i, expected): (usize, &Point2)| -> Result<bool, Failure> {
            let actual = curve.pole(i as i32 + 1)?;
            Ok([actual.x().to_bits(), actual.y().to_bits()]
                == [expected.x().to_bits(), expected.y().to_bits()])
        };
        for i in 0..self.knots.len() {
            if !knot_matches(i)? {
                return Ok(false);
            }
        }
        for item in self.poles.iter().enumerate() {
            if !pole_matches(item)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
