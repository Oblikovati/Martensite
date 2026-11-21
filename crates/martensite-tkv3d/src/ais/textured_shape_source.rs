//! Virtual native texture-source dispatch and genuine const pixmap handle-reference storage.
use super::AisInteractiveObjectMembers;
use super::{shape::AisShapeSource, textured_shape::AisTexturedShapeBase};
use martensite_tkernel::{
    standard::{Failure, HandleReference},
    tcollection::AsciiString,
};
use martensite_tkservice::image::ImagePixMapObject;

/// Native subclassable texture-source operations; `object.set_texture_file_name(&file)?`.
pub trait AisTexturedShapeSource: AisShapeSource {
    /// Native protected fields; `object.textured_shape_base().fields.read().map_texture`.
    fn textured_shape_base(&self) -> &AisTexturedShapeBase;
    /// Replace the filename and release previous pixmap first; `object.set_texture_file_name(&"7".into())?`.
    fn set_texture_file_name(&self, file: &AsciiString) -> Result<(), Failure> {
        // Keep the virtual hook separate from its native failure-ordered base body.
        super::textured_shape_sources::ais_textured_shape_set_texture_file_name(self, file)
    }
    /// Retain the exact pixmap without changing its row-order flag; `object.set_texture_pix_map(&pixmap)`.
    fn set_texture_pix_map(&self, pixmap: &dyn HandleReference<dyn ImagePixMapObject>) {
        // Forward the exact handle rather than copying image contents.
        super::textured_shape_sources::ais_textured_shape_set_texture_pix_map(self, pixmap);
    }
}
// One concrete descriptor; the source view must not add an ancestor.
impl martensite_tkernel::standard::RuntimeTyped for dyn AisTexturedShapeSource {
    const TYPE_NAME: &'static str = "AisTexturedShape";
    fn static_type() -> &'static martensite_tkernel::standard::RuntimeType {
        // Genuine source casts share the concrete class descriptor and its AisShape parent.
        <super::AisTexturedShape as martensite_tkernel::standard::RuntimeTyped>::static_type()
    }
}
martensite_tkernel::impl_transient_interface_target!(dyn AisTexturedShapeSource);

/// Native nonvirtual getters/operations shared by concrete classes and application subclasses.
/// Example: `object.texture_pix_map().read().object_address()` inspects the live handle field.
pub trait AisTexturedShapeMembers: AisTexturedShapeSource {
    /// Refresh the native currently selected presentation; `object.update_attributes()?` requires an existing context presentation.
    fn update_attributes(&self) -> Result<(), Failure> {
        let presentation = self.presentation()?;
        super::textured_shape_aspects::ais_textured_shape_update_attributes(self, &presentation)
    }
    /// Read mapping enable state; `object.texture_map_state()`.
    fn texture_map_state(&self) -> bool {
        let fields = self.textured_shape_base().fields.read();
        fields.map_texture
    }
    /// Enable mapping without recomputing presentations; `object.set_texture_map_on()`.
    fn set_texture_map_on(&self) {
        let mut fields = self.textured_shape_base().fields.write();
        fields.map_texture = true;
    }
    /// Disable mapping without dropping the old retained texture; `object.set_texture_map_off()`.
    fn set_texture_map_off(&self) {
        let mut fields = self.textured_shape_base().fields.write();
        fields.map_texture = false;
    }
    /// Copy the current native filename prefix for use outside the field's read scope.
    /// Example: `let filename = object.texture_file();`.
    fn texture_file(&self) -> std::ffi::CString {
        let fields = self.textured_shape_base().fields.read();
        native_textured_cstring(&fields.texture_file).to_owned()
    }
    /// Borrow genuine native handle storage across assignments; `let source=object.texture_pix_map(); object.set_texture_pix_map(&next);`.
    fn texture_pix_map(&self) -> super::textured_shape_pixmap_ref::AisTexturedShapePixMapRef<'_> {
        super::textured_shape_pixmap_ref::AisTexturedShapePixMapRef::new(
            &self.textured_shape_base().texture_pix_map,
        )
    }
    /// Enable modulation without recomputing; `object.enable_texture_modulate()`.
    fn enable_texture_modulate(&self) {
        let mut fields = self.textured_shape_base().fields.write();
        fields.modulate = true;
    }
    /// Disable modulation without recomputing; `object.disable_texture_modulate()`.
    fn disable_texture_modulate(&self) {
        let mut fields = self.textured_shape_base().fields.write();
        fields.modulate = false;
    }
    /// Read native modulation state; `object.texture_modulate()`.
    fn texture_modulate(&self) -> bool {
        let fields = self.textured_shape_base().fields.read();
        fields.modulate
    }
    /// Read native triangle-edge request; `object.show_triangles()`.
    fn show_triangles(&self) -> bool {
        let fields = self.textured_shape_base().fields.read();
        fields.show_triangles
    }
    /// Store the triangle-edge request without recomputing; `object.set_show_triangles(true)`.
    fn set_show_triangles(&self, show: bool) {
        let mut fields = self.textured_shape_base().fields.write();
        fields.show_triangles = show;
    }
}
impl<T: AisTexturedShapeSource + ?Sized> AisTexturedShapeMembers for T {}

// Native TextureFile and the file constructor both consume myTextureFile.ToCString().
pub(super) fn native_textured_cstring(
    file: &martensite_tkernel::tcollection::AsciiString,
) -> &std::ffi::CStr {
    std::ffi::CStr::from_bytes_until_nul(file.as_bytes_with_nul()).unwrap_or(c"")
}
