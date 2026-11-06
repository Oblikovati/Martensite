//! Z-layers of the session's viewer: settings, adding and removing layers, object layers and
//! display priorities.
use super::*;
use martensite_tkmath::gp::Xyz;

impl ViewerSession {
    /// Move the coordinate origin of Z-layer `layer` to `origin`, which keeps rendering of far
    /// objects precise; `session.set_z_layer_origin(0, [1.0e6, 0., 0.])?`.
    pub fn set_z_layer_origin(&self, layer: i32, [x, y, z]: [f64; 3]) -> Result<(), Failure> {
        self.with_update(|_| {
            let mut settings = self.viewer.z_layer_settings(layer)?;
            settings.set_origin(Xyz::new(x, y, z));
            self.viewer.set_z_layer_settings(layer, &settings)
        })
    }
}

impl ViewerSession {
    /// A new default-settings layer before the top layer, then redraw; returns
    /// its id. Example: `let id = session.add_z_layer()?;`.
    pub fn add_z_layer(&self) -> Result<i32, Failure> {
        use martensite_tkservice::graphic3d::Graphic3dZLayerSettings;
        let id = self
            .viewer
            .add_z_layer(&Graphic3dZLayerSettings::default())?;
        let id = id.ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(
                "can not add a new z layer; expected a free layer id",
            )
        })?;
        self.view().redraw();
        Ok(id)
    }

    /// Move every named object of layer `id` to the default layer, remove the
    /// layer, then redraw; `session.remove_z_layer(1)?`.
    pub fn remove_z_layer(&self, id: i32) -> Result<(), Failure> {
        self.remove_z_layer_without_update(id)?;
        self.view().redraw();
        Ok(())
    }

    /// Move actual named objects and retain RemoveZLayer's bool without a redraw.
    /// Example: `let removed = session.remove_z_layer_without_update(1)?;` before a command updater.
    pub fn remove_z_layer_without_update(&self, id: i32) -> Result<bool, Failure> {
        use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers as _;
        for (_, object) in self.objects.iter() {
            if let Some(value) = object.get().filter(|o| o.z_layer() == id) {
                value.set_z_layer(martensite_tkservice::graphic3d::GRAPHIC3D_ZLAYER_ID_DEFAULT);
            }
        }
        self.viewer.remove_z_layer(id)
    }

    /// The viewer's layer ids in order; `session.z_layer_ids()`.
    pub fn z_layer_ids(&self) -> Vec<i32> {
        self.viewer.get_all_z_layers()
    }

    /// The object's layer, through the context, without a redraw.
    /// Example: `session.set_object_z_layer("b", 1)?`.
    pub fn set_object_z_layer(&self, name: &str, id: i32) -> Result<(), Failure> {
        self.context.set_z_layer(&self.named(name)?, id);
        Ok(())
    }

    /// The text "Z layer id: <id>"; `session.object_z_layer_text("b")?`.
    pub fn object_z_layer_text(&self, name: &str) -> Result<String, Failure> {
        Ok(format!(
            "Z layer id: {}",
            self.context.get_z_layer(&self.named(name)?)
        ))
    }

    /// The object's display priority, then update the viewer.
    /// Example: `session.set_display_priority("b", Graphic3dDisplayPriority::Above)?`.
    pub fn set_display_priority(
        &self,
        name: &str,
        priority: martensite_tkservice::graphic3d::Graphic3dDisplayPriority,
    ) -> Result<(), Failure> {
        let object = self.named(name)?;
        self.with_update(|context| context.set_display_priority(&object, priority))
    }
}
