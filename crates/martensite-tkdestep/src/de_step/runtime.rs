//! `DeStepConfigurationNode` as a node of the data exchange framework.
use super::{DeStepConfigurationNode, DeStepProvider};
use martensite_tkde::de::{
    DeConfigurationContext, DeConfigurationNode, DeConfigurationNodeBase, DeProvider,
    DeShapeFixConfigurationNode, DeShapeFixConfigurationNodeBase, live, new_configuration_node,
};
use martensite_tkernel::{
    impl_transient,
    ncollection::{Buffer, List},
    standard::{Failure, Handle},
    tcollection::AsciiString,
};

impl_transient!(DeStepConfigurationNode, "DeStepConfigurationNode", dyn DeShapeFixConfigurationNode, transient, interfaces: [dyn DeConfigurationNode, dyn DeShapeFixConfigurationNode]);

impl DeConfigurationNode for DeStepConfigurationNode {
    fn configuration_node_base(&self) -> &DeConfigurationNodeBase {
        self.base.configuration_node_base()
    }
    fn load(&self, resource: &Handle<DeConfigurationContext>) -> Result<bool, Failure> {
        Ok(Self::load(
            self,
            live(resource, "DeStepConfigurationNode::Load")?,
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
        let node = self.base.configuration_node_base().this().expect(
            "DeStepConfigurationNode::BuildProvider: unbound node; expected new_configuration_node or DeWrapper::bind before the factory call",
        );
        Handle::new(DeStepProvider::with_node(&node)).cast()
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

impl DeShapeFixConfigurationNode for DeStepConfigurationNode {
    fn shape_fix_base(&self) -> &DeShapeFixConfigurationNodeBase {
        &self.base
    }
}
