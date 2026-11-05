//! Shape-only document presentation reuses owned raster algorithms and explicit resources.
mod capture;
mod reports;
mod visibility;
use martensite_geometry_raster::{TopologyDrawable, TopologyStyle, axes::AxisGlyph};
use martensite_raster_core::{
    PaletteColor, PaletteKind, RasterReports, RasterViewer, RasterWindowSystem,
    drawable::RasterDrawable,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
pub(crate) use reports::ProcessReports;
use std::{collections::BTreeMap, path::PathBuf, rc::Rc};
type DrawingHandle = Rc<dyn RasterDrawable>;

/// Confined to one UI child because named Rc drawables and native windows share mutable view state.
/// Only zero views or one spatial AXON view1 are supported; this is not a general drawing session.
pub(crate) struct DocumentScene {
    viewer: RasterViewer<dyn RasterDrawable>,
    bindings: BTreeMap<String, DrawingHandle>,
    isos: i32,
    reports: Rc<dyn RasterReports>,
    capture_directory: Option<PathBuf>,
}
impl DocumentScene {
    /// Inject the actual window/report resources without creating a view; `DocumentScene::new(system, reports, None)?`.
    pub(crate) fn new(
        system: Rc<dyn RasterWindowSystem>,
        reports: Rc<dyn RasterReports>,
        capture_directory: Option<PathBuf>,
    ) -> Result<Self, Failure> {
        let mut scene = Self {
            viewer: RasterViewer::with_adapter(Some(system)),
            bindings: BTreeMap::new(),
            isos: 2,
            reports,
            capture_directory,
        };
        scene.display_axes()?;
        Ok(scene)
    }

    /// Draw on a memory raster, so no display is needed (release 0.8 decision 2). Zero-width lines
    /// follow the rule the reference capture was made under; `DocumentScene::connect(None)?`.
    pub(crate) fn connect(capture_directory: Option<PathBuf>) -> Result<Self, Failure> {
        use martensite_raster_core::{MemoryWindowSystem, ZeroWidthLineRule};
        let _: &(dyn Send + Sync) = &ProcessReports;
        let system = MemoryWindowSystem::with_line_rule(ZeroWidthLineRule::Accelerated);
        Self::new(Rc::new(system), Rc::new(ProcessReports), capture_directory)
    }

    /// Publish the actual shape under its literal name; `scene.publish("S", &shape)?`.
    pub(crate) fn publish(&mut self, name: &str, shape: &Shape) -> Result<(), Failure> {
        let drawable: DrawingHandle = self.shape_drawable(shape);
        let previous = self.bindings.remove(name);
        if let Some(old) = &previous {
            self.viewer.remove_drawable(old);
        }
        self.viewer.display(&drawable)?;
        println!(
            "{{\"binding\":\"{name}\",\"replaced\":{},\"visible\":{},\"space3d\":{}}}",
            previous.is_some(),
            drawable.visible(),
            drawable.is_3d()
        );
        self.bindings.insert(name.to_owned(), drawable);
        Ok(())
    }

    fn shape_drawable(&self, shape: &Shape) -> Rc<TopologyDrawable> {
        let style = TopologyStyle {
            isos: self.isos,
            ..TopologyStyle::default()
        };
        Rc::new(TopologyDrawable::new(
            shape.clone(),
            style,
            self.reports.clone(),
        ))
    }

    /// Change only the default used for later publications; `scene.set_isos(12)`.
    pub(crate) fn set_isos(&mut self, count: i32) {
        self.isos = count;
        println!("{{\"source_isos\":{count}}}");
    }

    fn display_axes(&mut self) -> Result<(), Failure> {
        let color = PaletteColor::new(PaletteKind::Bleu);
        let spatial: DrawingHandle = Rc::new(AxisGlyph::spatial(color, 20));
        let planar: DrawingHandle = Rc::new(AxisGlyph::planar(color, 20));
        self.viewer.display(&spatial)?;
        self.viewer.display(&planar)?;
        Ok(())
    }

    fn display_bound(&mut self, name: &str) -> Result<(), Failure> {
        let drawable = self
            .bindings
            .get(name)
            .ok_or_else(|| unsupported(&format!("name={name:?}"), "existing named shape"))?;
        self.viewer.display(drawable)?;
        Ok(())
    }

    /// Close the sole view only after the caller emitted its end; `scene.close()?`.
    pub(crate) fn close(&mut self) -> Result<(), Failure> {
        self.validate_views()?;
        self.viewer.delete_view(1);
        assert!(
            !self.viewer.has_view(1),
            "document presentation: expected closed view1"
        );
        Ok(())
    }
}
fn unsupported(actual: &str, expected: &str) -> Failure {
    Failure::with_type(
        "DocumentPresentationUnsupported",
        &format!("document presentation {actual}; expected {expected}"),
        "DocumentScene",
    )
}

#[cfg(test)]
#[allow(dead_code)] // Only the dedicated contract host observes these private fixture boundaries.
impl DocumentScene {
    /// Verify malformed-frame and I/O refusal diagnostics; `DocumentScene::control_capture_refusals()`.
    pub(crate) fn control_capture_refusals() {
        // Exercise actual validators with explicit bad bytes and an injected I/O cause.
        capture::assert_capture_refusals();
    }
    /// Inspect the actual owned viewer in contract tests; `scene.control_viewer().has_view(1)`.
    pub(crate) fn control_viewer(&mut self) -> &mut RasterViewer<dyn RasterDrawable> {
        // The borrowed viewer cannot outlive its thread-confined fixture.
        &mut self.viewer
    }
    /// Observe one literal-name identity without inferring it from geometry; `scene.control_binding("S")`.
    pub(crate) fn control_binding(&self, name: &str) -> Option<DrawingHandle> {
        // Clone only the existing handle, keeping the exact object identity observable.
        self.bindings.get(name).cloned()
    }
    /// Exercise the actual shape factory and its independent style; `scene.control_shape(&shape)`.
    pub(crate) fn control_shape(&self, shape: &Shape) -> Rc<TopologyDrawable> {
        // Keep style checks on real render objects, rather than a second settings record.
        self.shape_drawable(shape)
    }
}
