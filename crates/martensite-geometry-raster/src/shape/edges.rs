//

//! The edges, edge polygons and free vertices of a drawable shape.
use super::plot::{Plot, draw_and_check};
use super::{TopologyRendering, color_orientation};
use crate::edge_face::EdgeDrawRecord;
use martensite_raster_core::sampling::{
    bound, draw_direction_arrow as draw_arrow, interval_bounds,
};
use martensite_raster_core::{FormattedText, MarkerShape, RasterDisplay, RasterReports};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Edge, vertex as to_vertex};
use martensite_tkernel::standard::Failure;
use martensite_tkernel::{ncollection::Array1, precision};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::{CurveType, Shape as Continuity};
use martensite_tkmath::gp::Vector2;

/// The marker size of the free vertices (the `DrawMarker` default).
const VERTEX_MARKER_SIZE: i32 = 5;

impl TopologyRendering {
    /// The edge part of `DrawOn`: each edge's curve (with an orientation arrow when
    /// orientations are shown) and its polygon when it has no curve or polygons are
    /// shown. An edge with a reversed range is reported on `out` and skipped.
    pub(super) fn draw_edges(
        &self,
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
        out: &dyn RasterReports,
    ) -> Result<(), Failure> {
        let edges = self.edges.borrow().clone();
        for record in &edges {
            if *halt {
                break;
            }
            self.draw_edge_record(record, display, halt, out)?;
        }
        Ok(())
    }

    fn edge_record_color(&self, record: &EdgeDrawRecord) -> martensite_raster_core::PaletteColor {
        if self.display_orientation.get() {
            return color_orientation(record.edge().orientation());
        }
        record.color()
    }

    fn draw_edge_record(
        &self,
        record: &EdgeDrawRecord,
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
        out: &dyn RasterReports,
    ) -> Result<(), Failure> {
        display.set_color(self.edge_record_color(record));
        let geometric = BRepTool::is_geometric_edge(record.edge())?;
        if geometric && !self.draw_edge_curve(record, display, halt, out)? {
            return Ok(());
        }
        if !geometric || self.polygons.get() {
            draw_edge_polygon(record.edge(), display, halt, self.pick_sink.as_ref())?;
        }
        Ok(())
    }

    /// The curve of one edge; false when the edge is skipped (bad range, or
    /// degenerated), which also skips its polygon.
    fn draw_edge_curve(
        &self,
        record: &EdgeDrawRecord,
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
        out: &dyn RasterReports,
    ) -> Result<bool, Failure> {
        let edge = record.edge();
        if !curve_is_drawable(edge, out)? {
            return Ok(false);
        }
        let curve = BRepAdaptorCurve::with_edge(edge)?;
        let range = self.finite_edge_range(curve.first_parameter()?, curve.last_parameter()?);
        self.draw_trimmed_edge(record, &curve, range, display, halt)?;
        Ok(true)
    }

    fn draw_trimmed_edge(
        &self,
        record: &EdgeDrawRecord,
        curve: &BRepAdaptorCurve,
        (first, last): (f64, f64),
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
    ) -> Result<(), Failure> {
        let trimmed = curve.trim(first, last, precision::confusion())?;
        let trimmed = trimmed
            .get()
            .ok_or_else(|| Failure::with_message("Standard_NullObject", "trimmed edge curve"))?;
        self.plot_and_end_edge(record, (curve, trimmed, last), display, halt)?;
        if self.display_orientation.get() {
            draw_edge_orientation(curve, last, display)?;
        }
        Ok(())
    }

    fn plot_and_end_edge(
        &self,
        record: &EdgeDrawRecord,
        (curve, trimmed, last): (&BRepAdaptorCurve, &dyn Adaptor3dCurve, f64),
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
    ) -> Result<(), Failure> {
        self.plot_edge_intervals(record, (curve, trimmed), display, halt)?;
        let end = curve.d0(trimmed.last_parameter()?)?;
        draw_and_check(
            display,
            end,
            (record.edge().as_ref(), last, 0.0),
            halt,
            self.pick_sink.as_ref(),
        );
        Ok(())
    }

    /// An infinite end is replaced by `Size` from the other end (or `[-Size, Size]`).
    fn finite_edge_range(&self, first: f64, last: f64) -> (f64, f64) {
        let size = self.size;
        if precision::is_negative_infinite(first) {
            if precision::is_positive_infinite(last) {
                return (-size, size);
            }
            return (last - size, last);
        }
        if precision::is_positive_infinite(last) {
            return (first, first + size);
        }
        (first, last)
    }

    /// The points of each CN interval of the trimmed curve: none on lines, `discret`
    /// steps on circles and ellipses (evaluated on the untrimmed curve), adaptive double
    /// steps on the other curves.
    fn plot_edge_intervals(
        &self,
        record: &EdgeDrawRecord,
        (curve, trimmed): (&BRepAdaptorCurve, &dyn Adaptor3dCurve),
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
    ) -> Result<(), Failure> {
        let curve_type = trimmed.get_type()?;
        let bounds = interval_bounds(trimmed.nb_intervals(Continuity::CN)?, |b| {
            trimmed.intervals(b, Continuity::CN)
        })?;
        display.move_to_3d(trimmed.d0(trimmed.first_parameter()?)?);
        let discret = self.discret.get();
        self.walk_edge_intervals(
            record,
            (curve, trimmed),
            (curve_type, &bounds, discret),
            (display, halt),
        )
    }

    fn walk_edge_intervals(
        &self,
        record: &EdgeDrawRecord,
        curves: (&BRepAdaptorCurve, &dyn Adaptor3dCurve),
        (kind, bounds, discret): (CurveType, &Array1<'_, f64>, i32),
        drawing: (&mut RasterDisplay<'_>, &mut bool),
    ) -> Result<(), Failure> {
        for interval in 1..bounds.len() as i32 {
            let t = bound(bounds, interval)?;
            let step = (bound(bounds, interval + 1)? - t) / f64::from(discret);
            self.plot_edge_interval(
                record,
                curves,
                (kind, discret, t, step),
                (&mut *drawing.0, &mut *drawing.1),
            )?;
        }
        Ok(())
    }

    fn plot_edge_interval(
        &self,
        record: &EdgeDrawRecord,
        (curve, trimmed): (&BRepAdaptorCurve, &dyn Adaptor3dCurve),
        (kind, discret, t, step): (CurveType, i32, f64, f64),
        drawing: (&mut RasterDisplay<'_>, &mut bool),
    ) -> Result<(), Failure> {
        if kind == CurveType::Line {
            return Ok(());
        }
        if matches!(kind, CurveType::Circle | CurveType::Ellipse) {
            return self.plot_circular_interval(record, curve, (discret, t, step), drawing);
        }
        self.plot_adaptive_interval(record, trimmed, (discret, t, step), drawing)
    }

    fn plot_circular_interval(
        &self,
        record: &EdgeDrawRecord,
        curve: &BRepAdaptorCurve,
        (discret, mut t, step): (i32, f64, f64),
        (display, halt): (&mut RasterDisplay<'_>, &mut bool),
    ) -> Result<(), Failure> {
        for _ in 1..discret {
            t += step;
            draw_and_check(
                display,
                curve.d0(t)?,
                (record.edge().as_ref(), t, 0.0),
                halt,
                self.pick_sink.as_ref(),
            );
        }
        Ok(())
    }

    fn plot_adaptive_interval(
        &self,
        record: &EdgeDrawRecord,
        trimmed: &dyn Adaptor3dCurve,
        (discret, mut t, step): (i32, f64, f64),
        (display, halt): (&mut RasterDisplay<'_>, &mut bool),
    ) -> Result<(), Failure> {
        for _ in 1..=discret / 2 {
            Plot::new(
                display,
                record.edge().as_ref(),
                halt,
                self.pick_sink.as_ref(),
            )
            .edge(trimmed, t, step * 2.)?;
            t += step * 2.;
        }
        Ok(())
    }

    /// The vertex part of `DrawOn`: a diamond at each vertex that is not in an edge,
    /// in the connected color (or by orientation).
    pub(super) fn draw_vertices(
        &self,
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
    ) -> Result<(), Failure> {
        display.set_color(self.colors.connected);
        let mut explorer = Explorer::new(&self.shape, ShapeEnum::Vertex, ShapeEnum::Edge)?;
        while explorer.more() && !*halt {
            let current = explorer.current().clone();
            self.draw_vertex(&current, display, halt)?;
            explorer.next()?;
        }
        Ok(())
    }

    fn draw_vertex(
        &self,
        current: &martensite_tkbrep::topods::Shape,
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
    ) -> Result<(), Failure> {
        if self.display_orientation.get() {
            display.set_color(color_orientation(current.orientation()));
        }
        let point = BRepTool::pnt(&to_vertex(current)?)?;
        display.draw_marker_3d(point, MarkerShape::Losange, VERTEX_MARKER_SIZE);
        if display.has_picked() {
            self.pick_sink.record_pick(current, 0.0, 0.0);
            *halt = true;
        }
        Ok(())
    }
}

fn curve_is_drawable(edge: &Edge, out: &dyn RasterReports) -> Result<bool, Failure> {
    let (u1, u2) = BRepTool::range(edge)?;
    if u2 < u1 {
        report_invalid_edge(edge, out)?;
        return Ok(false);
    }
    Ok(!BRepTool::degenerated(edge)?)
}

fn report_invalid_edge(edge: &Edge, out: &dyn RasterReports) -> Result<(), Failure> {
    out.write_out("DBRep_DrawableShape : Bad parameters on edge.\n");
    let mut text = FormattedText::new();
    text.write_through(|io| brep_tools::dump(edge.as_ref(), io))?;
    out.write_out(text.text());
    Ok(())
}

fn draw_edge_orientation(
    curve: &BRepAdaptorCurve,
    last: f64,
    display: &mut RasterDisplay<'_>,
) -> Result<(), Failure> {
    let (point, vector) = curve.d1(last)?;
    let p1 = display.project(point);
    let p2 = display.project(point.translated(vector));
    draw_arrow(display, p1, Vector2::between(p1, p2))
}

/// The 3D polygon of an edge, or else its polygon on a triangulation (located).
fn draw_edge_polygon(
    edge: &Edge,
    display: &mut RasterDisplay<'_>,
    halt: &mut bool,
    pick_sink: &dyn crate::picks::ShapePickSink,
) -> Result<(), Failure> {
    let (polygon, location) = BRepTool::polygon3d(edge)?;
    let transform = location.transformation();
    let Some(polygon) = polygon.get() else {
        return draw_mesh_polygon(edge, display, halt, pick_sink);
    };
    let points: Vec<martensite_tkmath::gp::Point3> = polygon.nodes().iter().copied().collect();
    let located: Vec<_> = points.iter().map(|p| p.transformed(transform)).collect();
    draw_polyline(edge, &located, display, halt, pick_sink)
}

fn draw_mesh_polygon(
    edge: &Edge,
    display: &mut RasterDisplay<'_>,
    halt: &mut bool,
    pick_sink: &dyn crate::picks::ShapePickSink,
) -> Result<(), Failure> {
    let (on_mesh, mesh, location) = BRepTool::polygon_on_triangulation_by_index(edge, 1)?;
    let (Some(on_mesh), Some(mesh)) = (on_mesh.get(), mesh.get()) else {
        return Ok(());
    };
    let transform = location.transformation();
    let mut nodes = Vec::new();
    for index in on_mesh.nodes().iter() {
        nodes.push(mesh.node(*index)?.transformed(transform));
    }
    draw_polyline(edge, &nodes, display, halt, pick_sink)
}

/// `dis.Draw(P(i - 1), P(i))` for each pair, recording a pick of the edge.
fn draw_polyline(
    edge: &Edge,
    points: &[martensite_tkmath::gp::Point3],
    display: &mut RasterDisplay<'_>,
    halt: &mut bool,
    pick_sink: &dyn crate::picks::ShapePickSink,
) -> Result<(), Failure> {
    for pair in points.windows(2) {
        display.draw_3d(pair[0], pair[1]);
        if display.has_picked() {
            pick_sink.record_pick(edge.as_ref(), 0.0, 0.0);
            *halt = true;
        }
    }
    Ok(())
}
