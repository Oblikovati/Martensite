//! Per-object material, transparency and display mode, as the aspect and display-mode commands
//! apply them: the context setter without an update, then the aspects synchronized (or the mode
//! switched) and one viewer update.
use super::aspects::synchronize;
use super::*;

impl ViewerSession {
    /// Give the object `name` the predefined material `material`.
    /// Example: `session.set_material("result", Graphic3dNameOfMaterial::Silver)?`.
    pub fn set_material(
        &self,
        name: &str,
        material: Graphic3dNameOfMaterial,
    ) -> Result<(), Failure> {
        let object = self.named(name)?;
        let aspect = Graphic3dMaterialAspect::new(material);
        self.with_update(|context| {
            context.set_material(&object, &aspect, false)?;
            synchronize(&object);
            Ok(())
        })
    }

    /// Set the transparency (0 opaque, 1 invisible) of the object `name`.
    /// Example: `session.set_transparency("result", 0.6)?`.
    pub fn set_transparency(&self, name: &str, transparency: f64) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.with_update(|context| {
            context.set_transparency(&object, transparency, false)?;
            synchronize(&object);
            Ok(())
        })
    }

    /// Switch the object `name` to display mode `mode` and update the viewer, as the
    /// display-mode command does without an auto-updater.
    /// Example: `session.set_object_display_mode("result", 1)?`.
    pub fn set_object_display_mode(&self, name: &str, mode: i32) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.context.set_display_mode(&object, mode, true)
    }

    /// Give the object `name` the colour `color`, as the colour aspect command does.
    /// Example: `session.set_color("result", Color::named(NamedColor::GREEN))?`.
    pub fn set_color(
        &self,
        name: &str,
        color: martensite_tkernel::quantity::Color,
    ) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.with_update(|context| {
            context.set_color(&object, color, false)?;
            synchronize(&object);
            Ok(())
        })
    }
}
