//! Colour scales of a session: created on the top OSD layer at the lower-left corner, sized
//! to the active window when no size is set, and redisplayed after every change.
use super::*;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObject as _;

impl ViewerSession {
    /// The colour scale displayed as `name`, or a new one on the top OSD layer with a 2D
    /// persistence at the lower-left corner, as the viewer's colour-scale command creates it.
    /// Example: `let scale = session.color_scale("cs")?;`.
    pub fn color_scale(&self, name: &str) -> Result<Handle<AisColorScale>, Failure> {
        let bound = self
            .object(name)
            .map(|object| object.cast::<AisColorScale>());
        if let Some(scale) = bound.filter(|scale| !scale.is_null()) {
            return Ok(scale);
        }
        let scale = AisColorScale::new()?;
        if let Some(value) = scale.get() {
            value.set_z_layer(GRAPHIC3D_ZLAYER_ID_TOP_OSD);
        }
        let pers = Graphic3dTransformPers::with_corner(
            Graphic3dTransModeFlags::TMF_2D,
            AspectTypeOfTriedronPosition::LEFT_LOWER,
            Vec2::new(0, 0),
        )?;
        self.context
            .set_transform_persistence(&scale.cast(), &Handle::new(pers).cast())?;
        Ok(scale)
    }

    /// Give a zero breadth or height of `scale` the active window's, then (re)display it as
    /// `name`; `session.show_color_scale("cs", &scale)?`.
    pub fn show_color_scale(
        &mut self,
        name: &str,
        scale: &Handle<AisColorScale>,
    ) -> Result<(), Failure> {
        let (width, height) = self.source_window_size();
        if let Some(value) = scale.get() {
            if value.get_breadth() == 0 {
                value.set_breadth(width);
            }
            if value.get_height() == 0 {
                value.set_height(height);
            }
            value.set_to_update(-1);
        }
        self.display_object(name, scale.cast())
    }
}
