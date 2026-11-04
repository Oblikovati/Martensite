//

//! The hidden-line display of a drawable shape, cached per view.
use super::{HlrSettings, TopologyRendering};
use crate::hide_data::HiddenLineRecords;
use martensite_raster_core::RasterDisplay;
use martensite_raster_core::{PaletteColor, display::ViewQuery};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use martensite_tkhlr::hlr_brep::poly_hlr_angle_and_deflection;
use martensite_tkmath::gp::Transform3;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkmesh::imesh_tools::MeshParameters;

struct HiddenView {
    id: i32,
    matrix: Transform3,
    focal: f64,
    settings: HlrSettings,
}
type HiddenStyle = ((bool, bool, bool), (PaletteColor, PaletteColor));

impl TopologyRendering {
    /// `DisplayHiddenLines`: meshes the shape (relative deflection from the HLR angle),
    /// then draws the hidden-line result of this view, computed again when the view
    /// rotated (or, in perspective, moved or changed focal) or the angle changed.
    pub fn display_hidden_lines(&self, display: &mut RasterDisplay<'_>) -> Result<(), Failure> {
        let id = display.view_id();
        let Some(view) = display.view_query() else {
            return Ok(());
        };
        let focal = hidden_focal(&view);
        let settings = self.hlr.get();
        mesh_for_hidden(&self.shape, settings.angle)?;
        self.draw_hidden_view(
            display,
            HiddenView {
                id,
                matrix: view.matrix,
                focal,
                settings,
            },
        )
    }

    fn draw_hidden_view(
        &self,
        display: &mut RasterDisplay<'_>,
        view: HiddenView,
    ) -> Result<(), Failure> {
        let style = self.hidden_style(view.settings);
        let mut cache = self.hidden.borrow_mut();
        if self.draw_current_hidden(&mut cache, display, &view, style) {
            return Ok(());
        }
        let mut data = self.prepare_hidden_data(&view)?;
        draw_cached(&mut data, display, style, self.pick_sink.as_ref());
        cache.push(data);
        Ok(())
    }

    fn hidden_style(&self, settings: HlrSettings) -> HiddenStyle {
        let flags = (settings.with_rg1, settings.with_rgn, settings.with_hid);
        let colors = (self.colors.connected, self.colors.isos);
        (flags, colors)
    }

    fn draw_current_hidden(
        &self,
        cache: &mut Vec<HiddenLineRecords>,
        display: &mut RasterDisplay<'_>,
        view: &HiddenView,
        style: HiddenStyle,
    ) -> bool {
        let Some(index) = cache.iter().position(|data| data.view_id() == view.id) else {
            return false;
        };
        let recompute = !cache[index].is_same(&view.matrix, view.focal)
            || view.settings.angle != cache[index].angle();
        if !recompute {
            draw_cached(&mut cache[index], display, style, self.pick_sink.as_ref());
            return true;
        }
        cache.remove(index);
        false
    }

    fn prepare_hidden_data(&self, view: &HiddenView) -> Result<HiddenLineRecords, Failure> {
        let mut data = HiddenLineRecords::default();
        data.set(
            view.id,
            (view.matrix, view.focal),
            &self.shape,
            view.settings.angle,
        )?;
        Ok(data)
    }
}

fn hidden_focal(view: &ViewQuery) -> f64 {
    if view.view_type == "PERS" {
        return view.focal;
    }
    -1.0
}

fn mesh_for_hidden(shape: &Shape, angle: f64) -> Result<(), Failure> {
    let (angle, deflection) = poly_hlr_angle_and_deflection(angle);
    let parameters = MeshParameters {
        relative: true,
        deflection,
        angle,
        ..MeshParameters::default()
    };
    BRepMeshIncrementalMesh::with_parameters(shape, &parameters, &ProgressRange::default())?;
    Ok(())
}

/// Draws one cached result and records its picked shape (parameters 0).
fn draw_cached(
    data: &mut HiddenLineRecords,
    display: &mut RasterDisplay<'_>,
    (flags, colors): HiddenStyle,
    pick_sink: &dyn crate::picks::ShapePickSink,
) {
    data.draw_on(display, flags, colors);
    if display.has_picked() {
        pick_sink.record_pick(data.last_pick(), 0.0, 0.0);
    }
}
