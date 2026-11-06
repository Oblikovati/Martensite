//! A drawing-only topology handle with explicit reports and local pick ownership.
use crate::{
    HlrSettings, ShapeColors, TopologyRendering, picks::LocalShapePicks, state::DrawableState,
};
use martensite_raster_core::{RasterDisplay, RasterReports, drawable::RasterDrawable};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use std::rc::Rc;
/// Explicit topology rendering settings; `let style = TopologyStyle::default();`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TopologyStyle {
    /// Edge, vertex and iso colors.
    pub colors: ShapeColors,
    /// Finite extent used to display unbounded geometry.
    pub infinity_size: f64,
    /// Iso count in each parameter direction.
    pub isos: i32,
    /// Curve sampling count.
    pub discretization: i32,
    /// Whether arrows and orientation colors are displayed.
    pub orientation: bool,
    /// Whether triangulation edges are displayed.
    pub triangulations: bool,
    /// Whether edge polygons are displayed.
    pub polygons: bool,
    /// Hidden-line display settings, including inactive options.
    pub hidden_lines: HlrSettings,
}
impl Default for TopologyStyle {
    fn default() -> Self {
        Self {
            colors: ShapeColors::standard(),
            infinity_size: 100.,
            isos: 2,
            discretization: 30,
            orientation: false,
            triangulations: false,
            polygons: false,
            hidden_lines: HlrSettings {
                with_hlr: false,
                with_rg1: true,
                with_rgn: false,
                with_hid: false,
                angle: 35. * std::f64::consts::PI / 180.,
            },
        }
    }
}
/// Thread-confined rendering state with caller-owned reports and per-drawable picks.
/// This owns no interpreter, process console, session, or thread-local algorithm state.
pub struct TopologyDrawable {
    rendering: TopologyRendering,
    state: DrawableState,
    reports: Rc<dyn RasterReports>,
    picks: Rc<LocalShapePicks>,
}
impl TopologyDrawable {
    /// Builds a drawing-only shape; `TopologyDrawable::new(shape, style, reports)`.
    pub fn new(shape: Shape, style: TopologyStyle, reports: Rc<dyn RasterReports>) -> Self {
        let picks = Rc::new(LocalShapePicks::default());
        let rendering = configured_topology(shape, style, &picks);
        Self {
            rendering,
            state: DrawableState::default(),
            reports,
            picks,
        }
    }
    /// Borrows explicit rendering controls; `drawable.rendering().change_discret(20);`.
    pub fn rendering(&self) -> &TopologyRendering {
        &self.rendering
    }
    /// Reads this drawable's last selected entity; `let selected = drawable.last_pick();`.
    pub fn last_pick(&self) -> crate::picks::ShapePick {
        self.picks.last_pick()
    }
}
impl RasterDrawable for TopologyDrawable {
    fn draw_on(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        self.rendering
            .draw_with_reports(display, self.reports.as_ref())
    }
    fn is_3d(&self) -> bool {
        true
    }
    fn bounds(&self) -> [f64; 4] {
        self.state.bounds.get()
    }
    fn set_bounds(&self, bounds: [f64; 4]) {
        self.state.bounds.set(bounds);
    }
    fn visible(&self) -> bool {
        self.state.visible.get()
    }
    fn set_visible(&self, visible: bool) {
        self.state.visible.set(visible);
    }
}

fn configured_topology(
    shape: Shape,
    style: TopologyStyle,
    picks: &Rc<LocalShapePicks>,
) -> TopologyRendering {
    let rendering = TopologyRendering::new(
        shape,
        style.colors,
        style.infinity_size,
        style.isos,
        style.discretization,
        picks.clone(),
    );
    rendering.display_orientation(style.orientation);
    rendering.set_display_triangulation(style.triangulations);
    rendering.set_display_polygons(style.polygons);
    rendering.display_hlr(style.hidden_lines);
    rendering
}
