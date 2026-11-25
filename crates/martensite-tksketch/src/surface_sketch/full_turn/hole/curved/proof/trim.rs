// SPDX-License-Identifier: GPL-2.0-only
//! Certify the actual projected trims on their decoded active parameter ranges.
use super::*;
use martensite_tkbrep::brep::EdgePCurve;
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCurve};
use martensite_tkg3d::geom::GeomBezierCurve;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CylindricalTrimQuery, MetricBezier, certify_cylindrical_trim_ranges,
};

pub(super) struct CapCheck<'a> {
    pub face: &'a topods::Face,
    pub depth: PeriodicDepth,
    pub bindings: &'a binding::Bindings,
    pub tool: &'a DetachedWrappedBodies,
    pub sources: &'a [binding::FittedCut; 2],
    pub region: SurfaceRegionId,
    pub stock: &'a band_admission::BandSection,
    pub report: &'a WrappedMemberReport,
    pub sheet: [f64; 2],
    pub rims: [f64; 2],
    pub support_radius: f64,
    pub frame: CapFrame,
}

pub(super) fn verify(check: CapCheck<'_>) -> Checked<[f64; 2]> {
    let tool_face = source_face(check.tool, check.region, check.depth)?;
    let edges = coedges(check.face)?;
    let mut max_spatial: f64 = 0.;
    let mut max_parameter: f64 = 0.;
    for (key, source) in cap_edges(check.tool, check.depth) {
        let edge = as_edge(source)?;
        let original = space_curve(&edge)?;
        let trim = curve_on(&edge, &tool_face)?;
        let [space, parameter] = check.verify_images(*key, &edges, &original, &trim)?;
        max_spatial = max_spatial.max(space);
        max_parameter = max_parameter.max(parameter);
    }
    Ok([max_spatial, max_parameter])
}

struct TrimImage {
    bound: f64,
    space: [f64; 2],
    angles: [f64; 2],
}

impl CapCheck<'_> {
    fn verify_images(
        &self,
        key: WrappedHistoryKey,
        coedges: &[topods::Edge],
        source: &SpaceCurve,
        original_trim: &EdgePCurve,
    ) -> Checked<[f64; 2]> {
        if let Some(cut) = self.sources.iter().find(|cut| cut.edge(self.depth) == key) {
            self.verify_decoded_ancestry(*cut)?;
            let images = cut
                .pieces(self.depth)
                .map(|name| self.named_image(name, coedges, source));
            let [left, right] = images;
            return self.verify_split(*cut, [left?, right?], original_trim);
        }
        let actual = self.named_image(PeriodicImageKey::WrappedRetained(key), coedges, source)?;
        if actual.space != [0., 1.] {
            return Err(incomplete(
                "unsplit fitted parameter range",
                format!("{key:?}: {:?}", actual.space),
            ));
        }
        Ok([actual.bound, 0.])
    }

    fn verify_decoded_ancestry(&self, cut: binding::FittedCut) -> Checked<()> {
        let [first, last] = cut
            .vertices(self.depth)
            .map(|key| binding::resolve(self.bindings, PeriodicImageKey::WrappedRetained(key)));
        let root = binding::resolve(
            self.bindings,
            PeriodicImageKey::CrossingVertex {
                depth: self.depth,
                crossing: cut.crossing.report.id,
            },
        )?;
        let ends = [[first?, root], [root, last?]];
        for (key, [a, b]) in cut.pieces(self.depth).into_iter().zip(ends) {
            let edge = as_edge(binding::resolve(self.bindings, key)?)?;
            let (start, end) = martensite_tkbrep::topexp::vertices(&edge, true);
            if !a.is_same(&start) || !b.is_same(&end) {
                return Err(incomplete(
                    "decoded fitted fragment endpoints in source order",
                    format!("{key:?}"),
                ));
            }
        }
        Ok(())
    }

    fn named_image(
        &self,
        key: PeriodicImageKey,
        coedges: &[topods::Edge],
        source: &SpaceCurve,
    ) -> Checked<TrimImage> {
        let [edge] = occurrences(coedges, binding::resolve(self.bindings, key)?)?;
        self.verify_image(&edge, source).map_err(|cause| {
            incomplete(
                "decoded curved aperture cap image",
                format!("{key:?}: {cause:?}"),
            )
        })
    }

    fn verify_image(&self, edge: &topods::Edge, source: &SpaceCurve) -> Checked<TrimImage> {
        let curve = space_curve(edge)?;
        let trim = curve_on(edge, self.face)?;
        if curve.poles != source.poles
            || curve.weights != source.weights
            || curve.range != [trim.first, trim.last]
        {
            return Err(incomplete(
                "same captured Bezier coefficients and decoded trim interval",
                format!("{:?} vs {:?}", curve.range, source.range),
            ));
        }
        let (mut controls, chart) = polynomial(&trim)?;
        let separation = effective_limits(self.stock.sweep.options).linear_separation;
        verify_rim_clearance(&controls, self.rims, separation)?;
        let pad = self.frame.canonical_height(&mut controls);
        let limit = (self.spatial_limit(edge)? - pad).next_down();
        let proof = certify_cylindrical_trim_ranges(
            CylindricalTrimQuery {
                curve: MetricBezier {
                    poles: &curve.poles,
                    weights: Some(&curve.weights),
                },
                pcurve: &controls,
                radius: self.frame.radius,
                spatial_limit: limit,
                options: Default::default(),
            },
            [curve.range, chart],
            || false,
        )
        .map_err(|cause| {
            incomplete("certified active decoded curved trim", format!("{cause:?}"))
        })?;
        let angles = chart_angles(&trim, [trim.first, trim.last])?;
        self.check_sheet(&controls, angles, chart)?;
        Ok(TrimImage {
            bound: (proof.spatial_residual_bound() + pad).next_up(),
            space: curve.range,
            angles,
        })
    }

    fn check_sheet(&self, poles: &[[f64; 2]], endpoints: [f64; 2], chart: [f64; 2]) -> Checked<()> {
        let angle = verification::angular(self.stock);
        let is_on_sheet = |u: f64| self.sheet[0] - angle <= u && u <= self.sheet[1] + angle;
        let monotone = poles.windows(2).all(|p| p[0][0] <= p[1][0])
            || poles.windows(2).all(|p| p[0][0] >= p[1][0]);
        if (chart == [0., 1.] && poles.iter().all(|p| is_on_sheet(p[0])))
            || (monotone && endpoints.iter().copied().all(is_on_sheet))
        {
            return Ok(());
        }
        Err(incomplete(
            "complete decoded trim on its certified periodic sheet",
            format!("{endpoints:?} with {poles:?} in {:?}", self.sheet),
        ))
    }

    fn spatial_limit(&self, edge: &topods::Edge) -> Checked<f64> {
        let face =
            BRepTool::face_tolerance(self.face).map_err(|e| native("curved cap tolerance", e))?;
        let edge =
            BRepTool::edge_tolerance(edge).map_err(|e| native("curved edge tolerance", e))?;
        Ok(self.stock.sweep.options.resolution.weld.min(face).min(edge))
    }

    fn verify_split(
        &self,
        cut: binding::FittedCut,
        [one, two]: [TrimImage; 2],
        original: &EdgePCurve,
    ) -> Checked<[f64; 2]> {
        let u = one.space[1];
        if one.space[0] != 0. || two.space != [u, 1.] || !(0. < u && u < 1.) {
            return Err(incomplete(
                "exact partition of original fitter range",
                format!("{:?} {:?}", one.space, two.space),
            ));
        }
        let error = self.check_parameter(cut, u, [one.angles[1], two.angles[0]], original)?;
        Ok([one.bound.max(two.bound), error])
    }

    fn check_parameter(
        &self,
        cut: binding::FittedCut,
        parameter: f64,
        [a, b]: [f64; 2],
        original: &EdgePCurve,
    ) -> Checked<f64> {
        let [left, right] = match cut.crossing.report.id.transition {
            SheetTransition::Increasing => [self.sheet[1], self.sheet[0]],
            SheetTransition::Decreasing => self.sheet,
        };
        let actual = (a - left).abs().max((b - right).abs());
        cylinder_admission::require(
            actual <= verification::angular(self.stock),
            "native split sheet contact",
            actual,
            "both active trims contact the commanded sheet lifts",
        )?;
        let [original_at_cut, _] = chart_angles(original, [parameter; 2])?;
        let error = self.parameter_bound(cut, original_at_cut.abs());
        let [span0, span1] = binding::span_bounds(cut.span);
        let mapped = span0 + parameter * (span1 - span0);
        let [lo, hi] = cut.crossing.report.normalized_parameter;
        cylinder_admission::require(
            mapped >= lo - error && mapped <= hi + error,
            "decoded fitted cut parameter",
            mapped,
            "inside achieved exact-source parameter enclosure",
        )?;
        Ok(error)
    }

    fn parameter_bound(&self, cut: binding::FittedCut, endpoint: f64) -> f64 {
        let profile = (self.report.spatial.profile_error.upper / self.support_radius).next_up();
        let chart = (self.report.spatial.cap_angular_error.upper + profile).next_up();
        let numerator = ((chart + endpoint).next_up() + 64. * f64::EPSILON * TAU).next_up();
        (numerator / cut.crossing.angular_speed).next_up()
    }
}

fn verify_rim_clearance(points: &[[f64; 2]], [lo, hi]: [f64; 2], floor: f64) -> Checked<()> {
    let margin = points
        .iter()
        .map(|point| {
            let v = point[1];
            (v - lo).min(hi - v).next_down()
        })
        .fold(f64::INFINITY, f64::min);
    cylinder_admission::require(
        margin > floor,
        "decoded periodic aperture rim clearance",
        margin,
        "whole literal pcurve hull strictly between the native rim meridians",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn periodic_two_pole_storage_is_not_a_clamped_polynomial_projection() {
        use martensite_tkernel::ncollection::Array1;
        use martensite_tkmath::gp::Point2;

        let mut poles = Array1::with_bounds(1, 2).unwrap();
        poles.set_value(1, Point2::new(0.1, 1.)).unwrap();
        poles.set_value(2, Point2::new(0.2, 2.)).unwrap();
        let mut knots = Array1::with_bounds(1, 2).unwrap();
        knots.set_value(1, 0.).unwrap();
        knots.set_value(2, 1.).unwrap();
        let mut multiplicities = Array1::with_bounds(1, 2).unwrap();
        multiplicities.set_value(1, 2).unwrap();
        multiplicities.set_value(2, 2).unwrap();
        let periodic = Geom2dBSplineCurve::new(&poles, &knots, &multiplicities, 3, true).unwrap();
        assert_eq!(periodic.nb_poles(), 2);
        let error = clamped_projection(&periodic, [0., 1.]).unwrap_err();
        assert!(matches!(
            error,
            SurfaceFailureCause::Incomplete {
                context: "one literal clamped polynomial on the exact active domain",
                ..
            }
        ));
    }

    #[test]
    fn decoded_meridian_hull_leaves_the_original_linear_clearance_to_both_rims() {
        let valid = [[0.1, 1.], [0.2, 5.]];
        verify_rim_clearance(&valid, [0., 6.], 1e-6).unwrap();
        let edited = [[0.1, 1.], [0.2, 6. - 5e-7]];
        let error = verify_rim_clearance(&edited, [0., 6.], 1e-6).unwrap_err();
        assert!(matches!(
            error,
            SurfaceFailureCause::InvalidInput {
                field: "decoded periodic aperture rim clearance",
                ..
            }
        ));
    }
}

impl CapFrame {
    fn canonical_height(self, points: &mut [[f64; 2]]) -> f64 {
        let mut pad: f64 = 0.;
        for point in points {
            let z = self.z + self.sign * point[1];
            pad = pad.max(z.next_up() - z).max(z - z.next_down());
            point[1] = z;
        }
        pad
    }
}

struct SpaceCurve {
    poles: Vec<[f64; 3]>,
    weights: Vec<f64>,
    range: [f64; 2],
}

fn space_curve(edge: &topods::Edge) -> Checked<SpaceCurve> {
    let (handle, first, last) =
        BRepTool::curve(edge).map_err(|e| native("actual curved aperture space edge", e))?;
    let curve = handle
        .downcast_ref::<GeomBezierCurve>()
        .filter(|curve| !curve.has_eval_representation())
        .ok_or_else(|| {
            incomplete(
                "retained literal rational Bezier space edge",
                format!("{first}..{last}"),
            )
        })?;
    let poles = curve
        .poles()
        .iter()
        .map(|point| [point.x(), point.y(), point.z()])
        .collect();
    let weights = (1..=curve.nb_poles())
        .map(|i| {
            curve
                .weight(i)
                .map_err(|e| native("retained Bezier weight", e))
        })
        .collect::<Checked<Vec<_>>>()?;
    Ok(SpaceCurve {
        poles,
        weights,
        range: [first, last],
    })
}

fn polynomial(trim: &EdgePCurve) -> Checked<(Vec<[f64; 2]>, [f64; 2])> {
    let range = [trim.first, trim.last];
    if !trim.is_stored {
        return Err(incomplete(
            "stored decoded cap pcurve",
            format!("{range:?}"),
        ));
    }
    if let Some(curve) = trim.pcurve.downcast_ref::<Geom2dBezierCurve>()
        && !curve.is_rational()
        && !curve.has_eval_representation()
    {
        let points = curve.poles().iter().map(|p| [p.x(), p.y()]).collect();
        return Ok((points, range));
    }
    if let Some(curve) = trim.pcurve.downcast_ref::<Geom2dBSplineCurve>() {
        return clamped_projection(curve, range);
    }
    Err(incomplete(
        "literal polynomial cap pcurve",
        format!("{range:?}"),
    ))
}

fn clamped_projection(
    curve: &Geom2dBSplineCurve,
    range: [f64; 2],
) -> Checked<(Vec<[f64; 2]>, [f64; 2])> {
    let knots = curve.knots().iter().copied().collect::<Vec<_>>();
    let [first, last] = [1, 2].map(|i| {
        curve
            .multiplicity(i)
            .map_err(|e| native("cap pcurve multiplicity", e))
    });
    let poles = curve.nb_poles();
    if curve.is_rational()
        || curve.is_periodic()
        || curve.has_eval_representation()
        || poles != curve.degree() + 1
        || knots != range
        || [first?, last?] != [poles; 2]
    {
        return Err(incomplete(
            "one literal clamped polynomial on the exact active domain",
            format!("{knots:?} vs {range:?}"),
        ));
    }
    let points = curve.poles().iter().map(|p| [p.x(), p.y()]).collect();
    Ok((points, [0., 1.]))
}

fn chart_angles(trim: &EdgePCurve, [a, b]: [f64; 2]) -> Checked<[f64; 2]> {
    let curve = trim
        .pcurve
        .get()
        .ok_or_else(|| incomplete("set curved cap pcurve", "null curve"))?;
    let [a, b] = [a, b].map(|u| {
        curve
            .value(u)
            .map_err(|e| native("curved cap trim endpoint", e))
    });
    Ok([a?.x(), b?.x()])
}

fn cap_edges(
    tool: &DetachedWrappedBodies,
    depth: PeriodicDepth,
) -> impl Iterator<Item = (&WrappedHistoryKey, &topods::Shape)> {
    tool.entities
        .iter()
        .filter(move |(key, _)| key.role == roles::edge_role(depth.index()))
}

fn source_face(
    tool: &DetachedWrappedBodies,
    region: SurfaceRegionId,
    depth: PeriodicDepth,
) -> Checked<topods::Face> {
    let key = WrappedHistoryKey {
        source: WrappedEntitySource::Region(region),
        role: roles::face_role(depth.index()),
    };
    topods::face(binding::tool_image(&tool.entities, key)?)
        .map_err(|e| native("detached curved tool cap", e))
}

fn as_edge(shape: &topods::Shape) -> Checked<topods::Edge> {
    topods::edge(shape).map_err(|e| native("curved cap edge", e))
}

fn curve_on(edge: &topods::Edge, face: &topods::Face) -> Checked<EdgePCurve> {
    BRepTool::curve_on_face(edge, face).map_err(|e| native("curved cap trim", e))
}
