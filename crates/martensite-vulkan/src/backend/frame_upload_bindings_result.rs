//! Binding publication preserves descriptor ownership and preparation order.
use super::*;
impl PreparedBindings {
    pub(super) fn into_draw(self) -> DrawBindings {
        DrawBindings {
            lighting: self.lighting,
            clipping: self.clipping,
            sprite: self.sprite,
            surface_image: self.surface_image,
            pbr_environment: self.pbr_environment,
            shadow_maps: self.shadow_maps,
            material_maps: self.material_maps,
            sphere_map: self.sphere_map,
        }
    }
}
