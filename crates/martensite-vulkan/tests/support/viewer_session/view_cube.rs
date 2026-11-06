//! The view cube widget of a session: created with the interactive viewer's defaults.
use super::*;

impl ViewerSession {
    /// The view cube displayed as `name`, or a new one with a 50% grey box, the session's
    /// camera animation and an event-loop (not fixed) animation; `session.view_cube("vc")?`.
    pub fn view_cube(&self, name: &str) -> Result<Handle<AisViewCube>, Failure> {
        let bound = self.object(name).map(|object| object.cast::<AisViewCube>());
        if let Some(cube) = bound.filter(|cube| !cube.is_null()) {
            return Ok(cube);
        }
        let cube = AisViewCube::new()?;
        cube.set_box_color(Color::named(NamedColor::GRAY50))?;
        cube.set_view_animation(&self.controller.view_animation().cast());
        cube.set_fixed_animation_loop(false);
        Ok(cube)
    }
}
