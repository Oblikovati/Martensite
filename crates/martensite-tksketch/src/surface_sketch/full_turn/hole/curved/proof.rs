// SPDX-License-Identifier: GPL-2.0-only
//! Check the decoded fitted trims and their complete periodic occurrence graph.
use super::*;
use crate::surface_sketch::{history as roles, validation};
use martensite_tkbrep::{brep::BRepTool, brep_adaptor::BRepAdaptorSurface, topexp::Explorer};
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};

mod trim;
mod walls;

pub(super) struct CurvedCheck {
    pub body: validation::CheckedBody,
    pub trace_bound: f64,
    pub native_parameter_error: f64,
}

pub(super) fn validate(
    plan: &AperturePlan,
    decoded: &topods::Shape,
    bindings: &binding::Bindings,
    tool: &DetachedWrappedBodies,
    sources: &[binding::FittedCut],
    angular_error: f64,
) -> Checked<CurvedCheck> {
    let walls = tool
        .entities
        .keys()
        .filter(|key| key.role == HistoryRole::SideFace)
        .count();
    check_rooted(decoded, bindings, walls, plan)?;
    check_stock(plan, bindings)?;
    walls::verify(decoded, bindings, tool, sources)?;
    let mut trace_bound: f64 = 0.;
    let mut native_parameter_error: f64 = 0.;
    for depth in [PeriodicDepth::First, PeriodicDepth::Last] {
        let side = Cap {
            plan,
            depth,
            bindings,
            tool,
            sources,
        };
        let [trace, parameter] = side.check(angular_error)?;
        trace_bound = trace_bound.max(trace);
        native_parameter_error = native_parameter_error.max(parameter);
    }
    let native_report = validation::ordered_report(decoded, &plan.stock.sweep)?;
    verify_census(plan, walls, &native_report)?;
    let body = validation::finish(native_report, bindings, &plan.stock.sweep)?;
    Ok(CurvedCheck {
        body,
        trace_bound,
        native_parameter_error,
    })
}

fn verify_census(
    plan: &AperturePlan,
    walls: usize,
    report: &martensite_tkbo::body_check::ValidationReport,
) -> Checked<()> {
    let clear = plan.clear_contours().count();
    let loops = walls + 4 + 2 * usize::from(plan.stock.cylinder) + 2 * clear;
    let genus = i64::try_from(1 + plan.contours.len())
        .map_err(|e| incomplete("representable curved aperture genus", e.to_string()))?;
    if matches!(report.census.as_slice(), [census] if census.shells == 1 && census.loops == loops && census.genus == Some(genus))
    {
        return Ok(());
    }
    Err(incomplete(
        "one shell with the stock handle and one per fitted aperture",
        format!("{:?}", report.census),
    ))
}

fn check_rooted(
    root: &topods::Shape,
    bindings: &binding::Bindings,
    walls: usize,
    plan: &AperturePlan,
) -> Checked<()> {
    if bindings.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return Err(incomplete(
            "unique ordered curved cut keys",
            "duplicates or unsorted keys",
        ));
    }
    let crossings = plan.crossing_count();
    let seam_fragments = 2 * (plan.rails.len() - 1);
    for (kind, count) in [
        (ShapeEnum::Solid, 1),
        (ShapeEnum::Face, walls + 4),
        (
            ShapeEnum::Edge,
            3 * walls + 6 + 2 * usize::from(!plan.stock.cylinder) + 2 * crossings + seam_fragments,
        ),
        (ShapeEnum::Vertex, 2 * walls + 4 + 2 * crossings),
    ] {
        verification::unique_kind(root, bindings, kind, count)?;
    }
    Ok(())
}

fn check_stock(plan: &AperturePlan, bindings: &binding::Bindings) -> Checked<()> {
    let direct = bindings
        .iter()
        .filter_map(|(key, shape)| match key {
            PeriodicImageKey::Retained(key) => Some((*key, shape.clone())),
            _ => None,
        })
        .collect();
    let stock = &plan.stock;
    for index in [0, 2] {
        verification::verify_face(stock, plan.band.rims, &direct, index, None)?;
    }
    for index in 0..4 {
        let key =
            super::super::history::rim_keys(plan.band.rims, stock.sweep.first_depth, index)[0];
        verification::verify_rim(stock, &direct, key, index)?;
    }
    Ok(())
}

fn stock_face(
    plan: &AperturePlan,
    bindings: &binding::Bindings,
    depth: PeriodicDepth,
) -> Checked<topods::Face> {
    let name = PeriodicImageKey::Retained(HistoryKey {
        source: HistorySource::Region(plan.band.region),
        role: roles::face_role(depth.index()),
    });
    topods::face(binding::resolve(bindings, name)?).map_err(|e| native("decoded curved cap", e))
}

struct Cap<'a> {
    plan: &'a AperturePlan,
    depth: PeriodicDepth,
    bindings: &'a binding::Bindings,
    tool: &'a DetachedWrappedBodies,
    sources: &'a [binding::FittedCut],
}

impl Cap<'_> {
    fn check(&self, angular_error: f64) -> Checked<[f64; 2]> {
        let face = stock_face(self.plan, self.bindings, self.depth)?;
        let surface = BRepAdaptorSurface::with_face(&face, true)
            .map_err(|e| native("curved aperture cap carrier", e))?;
        let stock = &self.plan.stock;
        let index = if self.depth.index() == stock.sweep.first_depth {
            3
        } else {
            1
        };
        if stock.cylinder {
            verification::verify_cylinder(stock, &surface, index)?;
        } else {
            verification::verify_cone(stock, &surface, index)?;
        }
        let frame = trim::CapFrame::capture(&surface, stock.cylinder)?;
        let [first, last] = [
            surface
                .first_u_parameter()
                .map_err(|e| native("curved cap U start", e))?,
            surface
                .last_u_parameter()
                .map_err(|e| native("curved cap U end", e))?,
        ];
        let partition = self.verify_partition(&face)?;
        let edges = partition.edges().cloned().collect::<Vec<_>>();
        let rims = self.verify_rims(&face, &surface, &edges)?;
        self.verify_rails(&face, &surface, &edges)?;
        trim::verify(trim::CapCheck {
            face: &face,
            edges: &edges,
            depth: self.depth,
            bindings: self.bindings,
            tool: self.tool,
            sources: self.sources,
            stock,
            angular_error,
            sheet: [first, last],
            rims,
            frame,
        })
    }

    fn verify_partition(&self, face: &topods::Face) -> Checked<verification::DirectedWires> {
        let clear = self.plan.clear_contours().count();
        let rims_and_seams = 2 + 2 * self.plan.rails.len();
        let cutter_edges = self
            .tool
            .entities
            .keys()
            .filter(|key| key.role == roles::edge_role(self.depth.index()))
            .count();
        let expected = [
            1 + clear,
            rims_and_seams + cutter_edges + self.plan.crossing_count(),
        ];
        let wires = verification::DirectedWires::read(face, expected)?;
        for hole in self.plan.clear_contours() {
            let names = self.clear_contour_edges(hole)?;
            wires.identify_wire(&names)?;
        }
        Ok(wires)
    }

    fn clear_contour_edges(&self, hole: &admission::Aperture) -> Checked<Vec<&topods::Shape>> {
        let role = roles::edge_role(self.depth.index());
        self.tool
            .entities
            .keys()
            .filter(|key| {
                key.role == role
                    && curve_span_owner(key.source)
                        .is_some_and(|curve| hole.curves.contains(&curve))
            })
            .map(|&key| binding::resolve(self.bindings, PeriodicImageKey::WrappedRetained(key)))
            .collect()
    }

    fn verify_rims(
        &self,
        face: &topods::Face,
        surface: &BRepAdaptorSurface,
        coedges: &[topods::Edge],
    ) -> Checked<[f64; 2]> {
        let values = self.plan.band.rims.map(|rim| {
            let key = PeriodicImageKey::Retained(HistoryKey {
                source: HistorySource::Curve(rim),
                role: roles::edge_role(self.depth.index()),
            });
            let [edge] = occurrences(coedges, binding::resolve(self.bindings, key)?)?;
            verification::verify_rim_trim(&self.plan.stock, face, surface, &edge, true)?;
            let trim = BRepTool::curve_on_face(&edge, face)
                .map_err(|e| native("curved stock rim pcurve", e))?;
            let line = trim
                .pcurve
                .downcast_ref::<Geom2dLine>()
                .ok_or_else(|| incomplete("literal constant-meridian rim", format!("{key:?}")))?;
            Ok(line.location().y())
        });
        let [first, last] = values;
        let [a, b] = [first?, last?];
        Ok([a.min(b), a.max(b)])
    }

    fn verify_rails(
        &self,
        face: &topods::Face,
        surface: &BRepAdaptorSurface,
        coedges: &[topods::Edge],
    ) -> Checked<()> {
        for &ends in &self.plan.rails {
            let key = PeriodicImageKey::SeamSegment {
                region: self.plan.band.region,
                depth: self.depth,
                ends,
            };
            let [a, b] = occurrences(coedges, binding::resolve(self.bindings, key)?)?;
            let [one, two] = ends.map(|end| {
                vertex_position(binding::resolve(
                    self.bindings,
                    binding::seam_vertex_key(self.depth, end),
                )?)
            });
            verification::verify_seam_fragment(
                &self.plan.stock,
                face,
                surface,
                [&a, &b],
                [one?, two?],
            )?;
        }
        Ok(())
    }
}

fn curve_span_owner(source: WrappedEntitySource) -> Option<SurfaceCurveId> {
    match source {
        WrappedEntitySource::Curve { curve, .. }
        | WrappedEntitySource::MappedCurve { curve, .. }
        | WrappedEntitySource::AnalyticArcCurve { curve, .. }
        | WrappedEntitySource::NativeSplineCurve { curve, .. } => Some(curve),
        _ => None,
    }
}

fn vertex_position(shape: &topods::Shape) -> Checked<[f64; 3]> {
    let vertex = topods::vertex(shape).map_err(|e| native("curved aperture endpoint", e))?;
    let point =
        BRepTool::pnt(&vertex).map_err(|e| native("curved aperture endpoint position", e))?;
    Ok([point.x(), point.y(), point.z()])
}

fn occurrences<const N: usize>(
    coedges: &[topods::Edge],
    shape: &topods::Shape,
) -> Checked<[topods::Edge; N]> {
    let edges = coedges
        .iter()
        .filter(|edge| edge.is_same(shape))
        .cloned()
        .collect::<Vec<_>>();
    edges.try_into().map_err(|found: Vec<_>| {
        incomplete(
            "exact directed occurrence count",
            format!("{} vs {N}", found.len()),
        )
    })
}

fn coedges(face: &topods::Face) -> Checked<Vec<topods::Edge>> {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)
        .map_err(|e| native("curved face coedges", e))?;
    let mut edges = Vec::new();
    while explorer.more() {
        edges.push(topods::edge(explorer.current()).map_err(|e| native("curved face coedge", e))?);
        explorer
            .next()
            .map_err(|e| native("next curved face coedge", e))?;
    }
    Ok(edges)
}

#[cfg(test)]
mod clear_wire_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::surface_sketch::full_turn::tests::{curved_aperture::fixture, options};
    use martensite_tkg3d::geom::GeomCylindricalSurface;

    #[test]
    fn decoded_radius_within_weld_can_still_exceed_the_trace_limit() {
        let (band, request) = fixture();
        let plan = AperturePlan::admit(
            &band,
            &request,
            options(),
            Scope::OneCrossed,
            SourceModel::Polynomial,
        )
        .unwrap();
        let stock = export_stock(&plan.stock, band.rims).unwrap();
        let cutter = fitted_cutter(&plan, &request).unwrap();
        let report = report(&plan, cutter.reports()).unwrap();
        let tool = cutter.export_native().unwrap();
        let fit = FitEvidence::cylinder(&plan, &report, request.support.radius).unwrap();
        let bound = binding::bind(&plan, &stock, &tool, fit.angular_error).unwrap();
        let keys = bound
            .bindings
            .iter()
            .map(|(key, _)| *key)
            .collect::<Vec<_>>();
        let bytes = encoding::encode(&bound.body, &bound.bindings).unwrap();
        let (_, bindings) = encoding::decode(&bytes, &keys).unwrap();
        let depth = PeriodicDepth::First;
        let face = stock_face(&plan, &bindings, depth).unwrap();
        let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
        let cylinder = surface.downcast_ref::<GeomCylindricalSurface>().unwrap();
        cylinder.set_radius(cylinder.radius() + 2.5e-7).unwrap();
        let actual = BRepAdaptorSurface::with_face(&face, true).unwrap();
        verification::verify_cylinder(&plan.stock, &actual, 3).unwrap();
        let cap = Cap {
            plan: &plan,
            depth,
            bindings: &bindings,
            tool: &tool,
            sources: &bound.sources,
        };
        let cause = cap.check(fit.angular_error).unwrap_err();
        assert!(
            matches!(cause, SurfaceFailureCause::Incomplete {
            context: "decoded curved aperture cap image", ref actual,
        } if actual.contains("Uncertified") && actual.contains("4.00000025")),
            "{cause:?}"
        );
    }
}
