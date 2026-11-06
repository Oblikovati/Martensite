use super::{RwObjInternalSection, serialization};
use martensite_tkde::de::{DeConfigurationContext, DeConfigurationNodeBase};
use martensite_tkernel::{
    ncollection::List,
    standard::{SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkrwmesh::rw_mesh::RwMeshCoordinateSystem;

/// The settings scope of this format and vendor.
const SCOPE: &str = "provider.OBJ.Martensite";

/// The OBJ configuration node.
///
/// Publish with `new_configuration_node(Self::default())` or bind its owning handle before
/// building a provider: a standalone settings value has no owning self.
#[derive(Default)]
pub struct DeObjConfigurationNode {
    pub(super) transient: StandardTransient,
    pub(super) base: DeConfigurationNodeBase,
    /// Mutable OBJ transfer parameters.
    pub internal_parameters: SharedState<RwObjInternalSection>,
}
impl DeObjConfigurationNode {
    /// Default settings; `DeObjConfigurationNode::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Copies settings into independent state; `DeObjConfigurationNode::copied_from(&node)`.
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
        load_read_shapes(&mut parameters, context);
        load_read_document(&mut parameters, context);
        parameters.write_comment =
            context.string_val("write.comment", &parameters.write_comment, SCOPE);
        parameters.write_author =
            context.string_val("write.author", &parameters.write_author, SCOPE);
        true
    }
    /// Serializes the complete commented resource; `let text = node.save();`.
    pub fn save(&self) -> AsciiString {
        serialization::save(&self.internal_parameters.snapshot())
    }
    /// Reports OBJ import support; `assert!(node.is_import_supported());`.
    pub fn is_import_supported(&self) -> bool {
        true
    }
    /// Reports OBJ export support; `assert!(node.is_export_supported());`.
    pub fn is_export_supported(&self) -> bool {
        true
    }
    /// Identifies the configured format; `assert_eq!(node.get_format(), "OBJ".into());`.
    pub fn get_format(&self) -> AsciiString {
        "OBJ".into()
    }
    /// Identifies the vendor; `let vendor = node.get_vendor();`.
    pub fn get_vendor(&self) -> AsciiString {
        "Martensite".into()
    }
    /// Lists supported file suffixes; `assert_eq!(node.get_extensions().len(), 1);`.
    pub fn get_extensions(&self) -> List<AsciiString> {
        let mut extensions = List::default();
        extensions.append("obj".into());
        extensions
    }
}
fn load_common(parameters: &mut RwObjInternalSection, context: &DeConfigurationContext) {
    parameters.file_length_unit =
        context.real_val("file.length.unit", parameters.file_length_unit, SCOPE);
    parameters.system_cs = RwMeshCoordinateSystem::from_raw(
        context.integer_val("system.cs", parameters.system_cs.as_raw(), SCOPE) % 2,
    );
    parameters.file_cs = RwMeshCoordinateSystem::from_raw(
        context.integer_val("file.cs", parameters.file_cs.as_raw(), SCOPE) % 2,
    );
}
fn load_read_shapes(parameters: &mut RwObjInternalSection, context: &DeConfigurationContext) {
    parameters.read_single_precision = context.boolean_val(
        "read.single.precision",
        parameters.read_single_precision,
        SCOPE,
    );
    parameters.read_create_shapes =
        context.boolean_val("read.create.shapes", parameters.read_create_shapes, SCOPE);
    parameters.read_root_prefix =
        context.string_val("read.root.prefix", &parameters.read_root_prefix, SCOPE);
}
fn load_read_document(parameters: &mut RwObjInternalSection, context: &DeConfigurationContext) {
    parameters.read_fill_doc =
        context.boolean_val("read.fill.doc", parameters.read_fill_doc, SCOPE);
    parameters.read_fill_incomplete = context.boolean_val(
        "read.fill.incomplete",
        parameters.read_fill_incomplete,
        SCOPE,
    );
    parameters.read_memory_limit_mib = context.integer_val(
        "read.memory.limit.mib",
        parameters.read_memory_limit_mib,
        SCOPE,
    );
}
