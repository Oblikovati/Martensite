//! `DeStlConfigurationNode` as a node of the data exchange framework.
use super::{DeStlConfigurationNode, DeStlProvider};
use martensite_tkde::de::{
    DeConfigurationContext, DeConfigurationNode, DeConfigurationNodeBase, DeProvider, live,
    new_configuration_node,
};
use martensite_tkernel::{
    impl_transient,
    ncollection::{Buffer, List},
    standard::{Failure, Handle},
    tcollection::AsciiString,
};

impl_transient!(DeStlConfigurationNode, "DeStlConfigurationNode", dyn DeConfigurationNode, transient, interfaces: [dyn DeConfigurationNode]);

impl DeConfigurationNode for DeStlConfigurationNode {
    fn configuration_node_base(&self) -> &DeConfigurationNodeBase {
        &self.base
    }
    fn load(&self, resource: &Handle<DeConfigurationContext>) -> Result<bool, Failure> {
        Ok(Self::load(
            self,
            live(resource, "DeStlConfigurationNode::Load")?,
        ))
    }
    fn save(&self) -> AsciiString {
        Self::save(self)
    }
    fn copy(&self) -> Handle<dyn DeConfigurationNode> {
        new_configuration_node(Self::copied_from(self)).cast()
    }
    fn build_provider(&self) -> Handle<dyn DeProvider> {
        // A later Load and every provider must observe this same node, as `this` does.
        let node = self.base.this().expect(
            "DeStlConfigurationNode::BuildProvider: unbound node; expected new_configuration_node or DeWrapper::bind before the factory call",
        );
        Handle::new(DeStlProvider::with_node(&node)).cast()
    }
    fn get_format(&self) -> AsciiString {
        Self::get_format(self)
    }
    fn get_vendor(&self) -> AsciiString {
        Self::get_vendor(self)
    }
    fn get_extensions(&self) -> List<AsciiString> {
        Self::get_extensions(self)
    }
    fn is_import_supported(&self) -> bool {
        Self::is_import_supported(self)
    }
    fn is_export_supported(&self) -> bool {
        Self::is_export_supported(self)
    }
    fn is_stream_supported(&self) -> bool {
        Self::is_stream_supported(self)
    }
    fn check_content(&self, buffer: &Handle<Buffer>) -> bool {
        Self::check_content(self, buffer.get().map(Buffer::as_slice))
    }
}
