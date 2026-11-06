//! The STL configuration node and provider of the data exchange framework.
mod parameters;
mod provider;
mod provider_documents;
mod provider_shapes;
mod runtime;
use martensite_tkde::de::{DeConfigurationContext, DeConfigurationNodeBase, render_configuration};
use martensite_tkernel::{
    ncollection::List,
    standard::{SharedState, StandardTransient},
    tcollection::AsciiString,
};
pub use parameters::DeStlInternalSection;
pub use provider::DeStlProvider;

/// `DeStlConfigurationNode`.
///
/// Publish with `new_configuration_node(Self::default())` or bind its owning handle before
/// building a provider: a standalone settings value has no owning self.
#[derive(Default)]
pub struct DeStlConfigurationNode {
    transient: StandardTransient,
    base: DeConfigurationNodeBase,
    /// Public internal settings, mutable before transfer.
    pub internal_parameters: SharedState<DeStlInternalSection>,
}

impl DeStlConfigurationNode {
    /// Construct enabled settings with unit scales1; `let node = Self::new();`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Copy values without sharing mutable configuration; `let copy = Self::copied_from(&node);`.
    pub fn copied_from(node: &Self) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: node.base.copied_state(),
            internal_parameters: SharedState::new(node.internal_parameters.snapshot()),
        }
    }

    /// Copy this configuration's values; `let copy = node.copy();`.
    pub fn copy(&self) -> Self {
        Self::copied_from(self)
    }

    /// Access the existing enabled/unit state; `node.configuration_node_base().is_enabled()`.
    pub fn configuration_node_base(&self) -> &DeConfigurationNodeBase {
        &self.base
    }

    /// Load scoped values, retaining fields when resources are absent; `node.load(&context)`.
    pub fn load(&self, resource: &DeConfigurationContext) -> bool {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STL.Martensite";
        p.read_merge_angle = resource.real_val("read.merge.angle", p.read_merge_angle, scope);
        p.read_brep = resource.boolean_val("read.brep", p.read_brep, scope);
        p.write_ascii = resource.boolean_val("write.ascii", p.write_ascii, scope);
        true
    }

    /// Serialize comments and values; `let text = node.save();`.
    pub fn save(&self) -> AsciiString {
        render_configuration(
            include_str!("configuration.template"),
            &self.internal_parameters.snapshot().resource_bindings(),
        )
        .expect("fixed configuration template has exactly one binding per value")
    }

    /// `get_format`; `node.get_format()` returns this format capability.
    pub fn get_format(&self) -> AsciiString {
        AsciiString::new("STL")
    }

    /// `get_vendor`; `node.get_vendor()` returns this format capability.
    pub fn get_vendor(&self) -> AsciiString {
        AsciiString::new("Martensite")
    }

    /// `is_import_supported`; `node.is_import_supported()` returns this format capability.
    pub fn is_import_supported(&self) -> bool {
        true
    }

    /// `is_export_supported`; `node.is_export_supported()` returns this format capability.
    pub fn is_export_supported(&self) -> bool {
        true
    }

    /// `is_stream_supported`; `node.is_stream_supported()` returns this format capability.
    pub fn is_stream_supported(&self) -> bool {
        true
    }

    /// Supported extension order; `let extensions = node.get_extensions();`.
    pub fn get_extensions(&self) -> List<AsciiString> {
        let mut extensions = List::default();
        extensions.append(AsciiString::new("stl"));
        extensions
    }

    /// Check defined buffer content, including the minimum size; `node.check_content(None)`.
    pub fn check_content(&self, content: Option<&[u8]>) -> bool {
        let Some(bytes) = content.filter(|bytes| bytes.len() >= 7) else {
            return false;
        };
        // Both strncmp calls must equal zero; the differently cased prefixes cannot.
        let matches_both = bytes.starts_with(b"solid") && bytes.starts_with(b"SOLID");
        matches_both && bytes[5].is_ascii_whitespace()
    }
}
