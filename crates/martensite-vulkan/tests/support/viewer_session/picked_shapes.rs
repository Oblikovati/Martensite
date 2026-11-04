//! Every selected shape (the sub-shape of a selected owner, else the
//! selected object's whole shape) displayed again as a new shape object under `<name><index>`.
use super::*;
use martensite_tkv3d::ais::AisShapeSource;

impl ViewerSession {
    /// Display a new shape object for each selected owner, named `prefix` for the first and
    /// `prefix` plus its rank for the next, then update once. The command's default prefix is
    /// `PickedShape_<call>_`. Example: `session.pick_selected("PickedShape_0_")?`.
    pub fn pick_selected(&mut self, prefix: &str) -> Result<(), Failure> {
        let shapes = self.selected_shapes()?;
        for (index, shape) in shapes.iter().enumerate() {
            let name = if index == 0 {
                prefix.to_owned()
            } else {
                format!("{prefix}{index}")
            };
            let object: Handle<dyn AisInteractiveObject> = AisShape::new(shape).cast();
            self.objects.push(&name, object.clone());
            self.context.display(&object, false)?;
        }
        self.context.update_current_viewer();
        Ok(())
    }

    /// The shapes of the selected owners in selection order.
    fn selected_shapes(&self) -> Result<Vec<Shape>, Failure> {
        let mut shapes = Vec::new();
        self.context.init_selected();
        while self.context.more_selected() {
            shapes.push(if self.context.has_selected_shape() {
                self.context.selected_shape()?
            } else {
                let object = self.context.selected_interactive()?;
                let shape = object.cast::<dyn AisShapeSource>();
                shape.get().map(|s| s.shape()).unwrap_or_default()
            });
            self.context.next_selected();
        }
        Ok(shapes)
    }
}
