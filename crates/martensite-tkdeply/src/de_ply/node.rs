use super::{RwPlyInternalSection, serialization};
use martensite_tkde::de::{DeConfigurationContext, DeConfigurationNodeBase};
use martensite_tkernel::{
    ncollection::List,
    standard::{SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkrwmesh::rw_mesh::RwMeshCoordinateSystem;

/// The settings scope of this format and vendor.
const SCOPE: &str = "provider.PLY.Martensite";

/// The PLY configuration node.
///
/// Publish with `new_configuration_node(Self::default())` or bind its owning handle before
/// building a provider: a standalone settings value has no owning self.
#[derive(Default)]
pub struct DePlyConfigurationNode {
    pub(super) transient: StandardTransient,
    pub(super) base: DeConfigurationNodeBase,
    /// Mutable PLY transfer parameters.
    pub internal_parameters: SharedState<RwPlyInternalSection>,
}
impl DePlyConfigurationNode {
    /// Default settings; `DePlyConfigurationNode::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Copies settings into independent state; `DePlyConfigurationNode::copied_from(&node)`.
    pub fn copied_from(source: &Self) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: source.base.copied_state(),
            internal_parameters: SharedState::new(source.internal_parameters.snapshot()),
        }
    }
    /// Copies this node's settings; `let copy = node.copy();`.
    pub fn copy(&self) -> Self {
        Self::copied_from(self)
    }
    /// The enabled state and global units; `node.configuration_node_base().is_enabled()`.
    pub fn configuration_node_base(&self) -> &DeConfigurationNodeBase {
        &self.base
    }
    /// Updates defined resource values; `node.load(&context)`.
    pub fn load(&self, context: &DeConfigurationContext) -> bool {
        let mut parameters = self.internal_parameters.write();
        load_common(&mut parameters, context);
        load_flags(&mut parameters, context);
        load_identifiers(&mut parameters, context);
        load_text(&mut parameters, context);
        true
    }
    /// Serializes the complete commented resource; `let text = node.save();`.
    pub fn save(&self) -> AsciiString {
        serialization::save(&self.internal_parameters.snapshot())
    }
    /// Reports PLY import support, which does not exist; `assert!(!node.is_import_supported());`.
    pub fn is_import_supported(&self) -> bool {
        false
    }
    /// Reports PLY export support; `assert!(node.is_export_supported());`.
    pub fn is_export_supported(&self) -> bool {
        true
    }
    /// Identifies the configured format; `assert_eq!(node.get_format(), "PLY".into());`.
    pub fn get_format(&self) -> AsciiString {
        "PLY".into()
    }
    /// Identifies the vendor; `let vendor = node.get_vendor();`.
    pub fn get_vendor(&self) -> AsciiString {
        "Martensite".into()
    }
    /// Lists supported file suffixes; `assert_eq!(node.get_extensions().len(), 1);`.
    pub fn get_extensions(&self) -> List<AsciiString> {
        let mut extensions = List::default();
        extensions.append("ply".into());
        extensions
    }
    /// Recognizes the PLY magic and following ASCII whitespace; `node.check_content(Some(b"ply\n"))`.
    pub fn check_content(&self, buffer: Option<&[u8]>) -> bool {
        let Some(bytes) = buffer.filter(|bytes| bytes.len() >= 4) else {
            return false;
        };
        &bytes[..3] == b"ply" && matches!(bytes[3], b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
    }
}
fn load_common(parameters: &mut RwPlyInternalSection, context: &DeConfigurationContext) {
    parameters.file_length_unit =
        context.real_val("file.length.unit", parameters.file_length_unit, SCOPE);
    parameters.system_cs = RwMeshCoordinateSystem::from_raw(
        context.integer_val("system.cs", parameters.system_cs.as_raw(), SCOPE) % 2,
    );
    parameters.file_cs = RwMeshCoordinateSystem::from_raw(
        context.integer_val("file.cs", parameters.file_cs.as_raw(), SCOPE) % 2,
    );
}
fn load_flags(parameters: &mut RwPlyInternalSection, context: &DeConfigurationContext) {
    parameters.write_normals =
        context.boolean_val("write.normals", parameters.write_normals, SCOPE);
    parameters.write_colors = context.boolean_val("write.colors", parameters.write_colors, SCOPE);
    parameters.write_tex_coords =
        context.boolean_val("write.tex.coords", parameters.write_tex_coords, SCOPE);
}
fn load_identifiers(parameters: &mut RwPlyInternalSection, context: &DeConfigurationContext) {
    parameters.write_part_id =
        context.boolean_val("write.part.id", parameters.write_part_id, SCOPE);
    parameters.write_face_id =
        context.boolean_val("write.face.id", parameters.write_face_id, SCOPE);
}
fn load_text(parameters: &mut RwPlyInternalSection, context: &DeConfigurationContext) {
    parameters.write_comment =
        context.string_val("write.comment", &parameters.write_comment, SCOPE);
    parameters.write_author = context.string_val("write.author", &parameters.write_author, SCOPE);
}
