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
    sources: &[binding::FittedCut; 2],
    angular_error: f64,
) -> Checked<CurvedCheck> {
    let walls = tool
        .entities
        .keys()
        .filter(|key| key.role == HistoryRole::SideFace)
        .count();
    check_rooted(decoded, bindings, walls, plan.stock.cylinder)?;
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
    if !matches!(native_report.census.as_slice(), [census] if census.shells == 1 && census.loops == walls + 4 + 2 * usize::from(plan.stock.cylinder) && census.genus == Some(2))
    {
        return Err(incomplete(
            "single genus-two curved aperture body",
            format!("{:?}", native_report.census),
        ));
    }
    let body = validation::finish(native_report, bindings, &plan.stock.sweep)?;
    Ok(CurvedCheck {
        body,
        trace_bound,
        native_parameter_error,
    })
}

fn check_rooted(
    root: &topods::Shape,
    bindings: &binding::Bindings,
    walls: usize,
    cylinder: bool,
) -> Checked<()> {
    if bindings.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return Err(incomplete(
            "unique ordered curved cut keys",
            "duplicates or unsorted keys",
        ));
    }
    for (kind, count) in [
        (ShapeEnum::Solid, 1),
        (ShapeEnum::Face, walls + 4),
        (ShapeEnum::Edge, 3 * walls + 12 + 2 * usize::from(!cylinder)),
        (ShapeEnum::Vertex, 2 * walls + 8),
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
    sources: &'a [binding::FittedCut; 2],
}

impl Cap<'_> {
    fn check(&self, angular_error: f64) -> Checked<[f64; 2]> {
        let face = stock_face(self.plan, self.bindings, self.depth)?;
        walls::one_wire(&face)?;
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
        let edges = coedges(&face)?;
        let expected = self
            .tool
            .entities
            .keys()
            .filter(|key| key.role == HistoryRole::SideFace)
            .count()
            + 8;
        if edges.len() != expected {
            return Err(incomplete(
                "complete curved periodic coedges",
                format!("{} edges, expected {expected}", edges.len()),
            ));
        }
        let rims = self.verify_rims(&face, &surface, &edges)?;
        self.verify_rails(&face, &surface, &edges)?;
        trim::verify(trim::CapCheck {
            face: &face,
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
        let ids = self.sources.map(|source| source.crossing.report.id);
        let rail_names = binding::rail_names(&self.plan.band, self.depth, ids);
        for (index, key) in rail_names.into_iter().enumerate() {
            let [a, b] = occurrences(coedges, binding::resolve(self.bindings, key)?)?;
            let root = PeriodicImageKey::CrossingVertex {
                depth: self.depth,
                crossing: ids[index],
            };
            let rim = self.plan.band.rims[index];
            let corner = PeriodicImageKey::Retained(HistoryKey {
                source: HistorySource::Curve(rim),
                role: roles::vertex_role(self.depth.index()),
            });
            let [one, two] =
                [root, corner].map(|key| vertex_position(binding::resolve(self.bindings, key)?));
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
mod tests {
    use super::*;
    use crate::surface_sketch::full_turn::tests::{curved_aperture::fixture, options};
    use martensite_tkg3d::geom::GeomCylindricalSurface;

    #[test]
    fn decoded_radius_within_weld_can_still_exceed_the_trace_limit() {
        let (band, request) = fixture();
        let plan = AperturePlan::admit(&band, &request, options()).unwrap();
        let stock = export_stock(&plan.stock, band.rims).unwrap();
        let cutter = fitted_cutter(&plan, &request).unwrap();
        let report = &cutter.reports()[&plan.region];
        let tool = cutter.export_native().unwrap();
        let fit = FitEvidence::cylinder(&plan, report, request.support.radius).unwrap();
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
