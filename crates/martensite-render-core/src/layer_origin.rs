//! The local origin of a Z layer (`Graphic3dZLayerSettings::SetOrigin`).
//!
//! GlLayerList::renderLayer moves the layer origin from every structure's model matrix
//! into the world-view matrix (GlStructure::updateLayerTransformation subtracts it from
//! the translation; the world view is `Translate(OrientationMatrix, origin)` in double), so
//! objects far from the world origin keep float precision near the layer origin. Structures
//! with transform persistence keep their world frame: Graphic3dTransformPers::Apply rebuilds
//! their world view from the world camera.
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Point3, Vector3, Xyz};
use martensite_tkmath::toploc::Datum3;
use martensite_tkservice::graphic3d::{
    Graphic3dCStructureFields, Graphic3dCamera, Graphic3dZLayerSettings,
};

/// A non-zero layer origin; `LayerOrigin::of(&settings).is_none()` for the default layer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LayerOrigin(Xyz);

impl LayerOrigin {
    /// The origin when the layer has one (a non-null origin transformation).
    pub fn of(settings: &Graphic3dZLayerSettings) -> Option<Self> {
        (!settings.origin_transformation().is_null()).then(|| Self(settings.origin()))
    }

    /// The origin's world coordinates; `origin.coords()`.
    pub fn coords(&self) -> [f64; 3] {
        [self.0.x(), self.0.y(), self.0.z()]
    }

    /// The camera and structure fields one structure renders with in this layer, or `None`
    /// when the structure keeps the world frame (transform persistence).
    /// Example: `origin.structure_frame(&camera, &fields)`.
    pub fn structure_frame(
        &self,
        camera: &Graphic3dCamera,
        fields: &Graphic3dCStructureFields,
    ) -> Option<(Graphic3dCamera, Graphic3dCStructureFields)> {
        if !fields.trsf_pers.is_null() {
            return None;
        }
        let mut local = fields.clone();
        local.trsf = self.local_transformation(&fields.trsf);
        Some((self.local_camera(camera), local))
    }

    /// The camera whose world view is the layer frame: the eye moves by `-origin` and the
    /// direction, distance and projection stay, so `OrientationMatrix` becomes
    /// `OrientationMatrix * Translate(origin)` (exact without axial scale).
    fn local_camera(&self, camera: &Graphic3dCamera) -> Graphic3dCamera {
        let local = camera.copy_members();
        let eye = camera.eye().xyz() - self.0;
        local.move_eye_to(&Point3::new(eye.x(), eye.y(), eye.z()));
        local
    }

    /// The structure transformation with the origin subtracted from its translation.
    fn local_transformation(&self, trsf: &Handle<Datum3>) -> Handle<Datum3> {
        let mut transform = trsf
            .get()
            .map(|datum| *datum.transformation())
            .unwrap_or_default();
        let translation = transform.translation_part() - self.0;
        transform.set_translation_part(Vector3::from(translation));
        Handle::new(Datum3::new(transform))
    }
}

#[cfg(test)]
#[path = "layer_origin_tests.rs"]
mod tests;
