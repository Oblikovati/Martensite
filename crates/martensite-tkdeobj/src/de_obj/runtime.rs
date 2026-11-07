//! `DeObjConfigurationNode` as a node of the data exchange framework.
use super::{DeObjConfigurationNode, DeObjProvider};
use martensite_tkde::de::{
    DeConfigurationContext, DeConfigurationNode, DeConfigurationNodeBase, DeProvider, live,
    new_configuration_node,
};
use martensite_tkernel::{
    impl_transient,
    ncollection::List,
    standard::{Failure, Handle},
    tcollection::AsciiString,
};

impl_transient!(DeObjConfigurationNode, "DeObjConfigurationNode", dyn DeConfigurationNode, transient, interfaces: [dyn DeConfigurationNode]);

impl DeConfigurationNode for DeObjConfigurationNode {
    fn configuration_node_base(&self) -> &DeConfigurationNodeBase {
        &self.base
    }
    fn load(&self, resource: &Handle<DeConfigurationContext>) -> Result<bool, Failure> {
        Ok(Self::load(
            self,
            live(resource, "DeObjConfigurationNode::Load")?,
        ))
    }
    fn save(&self) -> AsciiString {
        Self::save(self)
    }
    fn copy(&self) -> Handle<dyn DeConfigurationNode> {
        new_configuration_node(Self::copied_from(self)).cast()
    }
    fn build_provider(&self) -> Handle<dyn DeProvider> {
        // A later Load and every provider must observe this same node.
        let node = self.base.this().expect(
            "DeObjConfigurationNode::BuildProvider: unbound node; expected new_configuration_node or DeWrapper::bind before the factory call",
        );
        Handle::new(DeObjProvider::with_node(&node)).cast()
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
}
