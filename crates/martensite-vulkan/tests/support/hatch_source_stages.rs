//! Full source stages stay separate from the owned mask-fixture shortcuts.
use super::*;
use martensite_tkernel::standard::Failure;
use martensite_tkservice::image::ImagePixMap;

impl HatchBoxScene {
    /// Display with actual context/object defaults; `scene.source_display_default()?`.
    pub fn source_display_default(&self) -> Result<(), Failure> {
        self.source_command_update(
            || self._context.display(&self.object.cast(), false),
            || self._context.update_current_viewer(),
        )
    }

    /// Apply the global mode without creating an own object mode; `scene.source_global_mode(1)?`.
    pub fn source_global_mode(&self, mode: i32) -> Result<(), Failure> {
        let requested: i32 = mode;
        bare_global_mode::source_bare_global_mode(&self._context, requested)
    }
    /// Initialize a view after the genuine box exists; `HatchBoxScene::from_shape(&driver, shape)?`.
    pub fn from_shape(
        driver: &Handle<Graphic3dRenderDriver>,
        shape: Shape,
    ) -> Result<Self, Failure> {
        let initialized = HatchSourceView::new(driver)?;
        Ok(initialized.with_shape(shape))
    }

    /// Display mode one then perform the actual updater; `scene.source_display_shaded()?`.
    pub fn source_display_shaded(&self) -> Result<(), Failure> {
        self.source_command_update(
            || {
                self._context.display_with_modes(
                    &self.object.cast(),
                    1,
                    -1,
                    false,
                    PrsMgrDisplayStatus::None,
                )
            },
            || self._context.update_current_viewer(),
        )
    }

    /// Fit without an implicit update, then execute the updater redraw; `scene.source_fit()?`.
    pub fn source_fit(&self) -> Result<(), Failure> {
        // Fit uses a view updater rather than the context updater used by display.
        self.source_command_update(|| self.view.fit_all(0.01, false), || self.view.redraw())
    }

    /// Set the own shading aspect and recompute the presentation; `scene.source_hatch4()?`.
    pub fn source_hatch4(&self) -> Result<(), Failure> {
        self.source_command_update(
            || self.source_hatch_aspect(Some(AspectHatchStyle::GridWide)),
            || self._context.update_current_viewer(),
        )
    }

    /// Use Vertical only for an absent pattern; `scene.source_hatch_default()?`.
    pub fn source_hatch_default(&self) -> Result<(), Failure> {
        self.source_command_update(
            || self.source_hatch_aspect(None),
            || self._context.update_current_viewer(),
        )
    }

    fn source_hatch_aspect(&self, pattern: Option<AspectHatchStyle>) -> Result<(), Failure> {
        let drawer = self.object.attributes();
        let created = drawer.setup_own_shading_aspect(&self._context.default_drawer().clone())?;
        let aspect = drawer.shading_aspect().aspect();
        source_aspect::apply_source_hatch_pattern(&aspect, pattern);
        self.source_hatch_presentation(pattern.is_some() || created)
    }

    fn source_hatch_presentation(&self, recompute: bool) -> Result<(), Failure> {
        if recompute {
            return self._context.redisplay(&self.object.cast(), false, false);
        }
        self.object.synchronize_aspects();
        Ok(())
    }

    fn source_command_update(
        &self,
        operation: impl FnOnce() -> Result<(), Failure>,
        update: impl FnOnce(),
    ) -> Result<(), Failure> {
        view_output::with_source_view_update(&*self.view, operation, update)
    }

    /// Dump the actual RGB/mono default layer range; `scene.source_dump(&image)?`.
    pub fn source_dump(&self, image: &ImagePixMap) -> Result<bool, Failure> {
        let dumped: bool = SourceViewOutput::source_dump(self, image)?;
        Ok(dumped)
    }

    /// Retire the actual view only after the caller END; `scene.close()`.
    pub fn close(&self) {
        // The caller owns command completion; construction must not hide early retirement.
        SourceViewOutput::close(self);
    }
}

#[cfg(test)]
#[path = "hatch_source_mode_controls.rs"]
mod source_mode_controls;
