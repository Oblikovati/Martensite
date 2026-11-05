//! Literal-name visibility uses the existing viewer without copying geometry or creating windows.
use super::{DocumentScene, unsupported};
use martensite_raster_core::viewer::MAXVIEW;
use martensite_tkernel::standard::Failure;
impl DocumentScene {
    /// Clear visibility, redisplay the same binding and restore protected axes; `scene.only("S")?`.
    pub(crate) fn only(&mut self, name: &str) -> Result<(), Failure> {
        let zoom = self.viewer.zoom(1);
        self.viewer.clear();
        self.display_bound(name)?;
        self.display_axes()?;
        assert_eq!(
            self.viewer.zoom(1).to_bits(),
            zoom.to_bits(),
            "donly {name}: expected retained zoom"
        );
        println!("{{\"donly\":\"{name}\",\"retained_zoom\":{zoom}}}");
        Ok(())
    }

    /// Hide named bindings, repaint planar then spatial and flush; `scene.erase(&["Pr_8"])?`.
    pub(crate) fn erase(&mut self, names: &[&str]) -> Result<(), Failure> {
        for name in names {
            if let Some(drawable) = self.bindings.get(*name) {
                self.viewer.remove_drawable(drawable);
            }
        }
        self.viewer.repaint_2d()?;
        self.viewer.repaint_3d()?;
        self.viewer.flush();
        println!("{{\"erase_names\":{names:?}}}");
        Ok(())
    }

    /// Keep every binding while clearing visibility and restoring axes; `scene.erase_all()?`.
    pub(crate) fn erase_all(&mut self) -> Result<(), Failure> {
        self.viewer.clear();
        self.display_axes()?;
        self.viewer.flush();
        println!("{{\"erase_all\":true}}");
        Ok(())
    }

    /// Redisplay the original handles without fitting; `scene.display(&["Pr_8", "Pr_9"])?`.
    pub(crate) fn display(&mut self, names: &[&str]) -> Result<(), Failure> {
        let zoom = self.viewer.zoom(1);
        for name in names {
            self.display_bound(name)?;
        }
        assert_eq!(
            self.viewer.zoom(1).to_bits(),
            zoom.to_bits(),
            "display {names:?}: expected retained zoom"
        );
        println!("{{\"display_names\":{names:?},\"retained_zoom\":{zoom}}}");
        Ok(())
    }

    /// Fit only a permitted existing view, never allocate one; `scene.fit_existing()?`.
    pub(crate) fn fit_existing(&mut self) -> Result<(), Failure> {
        self.validate_views()?;
        let views: Vec<i32> = self.viewer.has_view(1).then_some(1).into_iter().collect();
        if !views.is_empty() {
            self.fit_and_title()?;
        }
        println!("{{\"fit_existing_views\":{views:?}}}");
        Ok(())
    }

    /// Refuse configurations outside this private fixture contract; `scene.validate_views()?`.
    pub(super) fn validate_views(&self) -> Result<(), Failure> {
        let views: Vec<i32> = (0..MAXVIEW as i32)
            .filter(|id| self.viewer.has_view(*id))
            .collect();
        let allowed = views.is_empty() || (views == [1] && self.viewer.view_type(1) == "AXON");
        if !allowed {
            return Err(unsupported(
                &format!("views={views:?}, type1={:?}", self.viewer.view_type(1)),
                "zero views or sole spatial AXON view1",
            ));
        }
        Ok(())
    }

    /// Preserve fit1 camera/repaint/title order with the existing primitive; `scene.fit_and_title()?`.
    pub(super) fn fit_and_title(&mut self) -> Result<(), Failure> {
        self.viewer.fit_view(1, 10)?;
        let zoom = self.viewer.zoom(1);
        self.viewer.set_zoom(1, zoom);
        self.viewer.repaint_view(1)?;
        self.viewer
            .set_title(1, &format!("1 : AXON - Zoom {zoom:.6}"));
        Ok(())
    }
}
