//! `DeObjProvider`: the file transfers of the provider interface.
use super::{provider_documents, provider_shapes};
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{DeConfigurationNode, DeProvider, DeProviderBase, DeWorkSession};
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    standard::{Failure, Handle, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tklcaf::tdoc_std::TDocStdDocument;

/// Transfers OBJ files with the settings of its node; a work session is accepted and ignored.
///
/// ```
/// use martensite_tkde::de::{DeConfigurationNode, DeProvider, new_configuration_node};
/// use martensite_tkdeobj::de_obj::{DeObjConfigurationNode, DeObjProvider};
/// let node = new_configuration_node(DeObjConfigurationNode::new());
/// let provider = DeObjProvider::with_node(&node.cast::<dyn DeConfigurationNode>());
/// assert_eq!(provider.get_vendor().as_bytes(), b"Martensite");
/// ```
#[derive(Default)]
pub struct DeObjProvider {
    transient: StandardTransient,
    base: DeProviderBase,
}
impl_transient!(DeObjProvider, "DeObjProvider", dyn DeProvider, transient, interfaces: [dyn DeProvider]);

impl DeObjProvider {
    /// A provider with global configuration; `DeObjProvider::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// A provider on a node, keeping that node rather than a copy; `DeObjProvider::with_node(&node)`.
    pub fn with_node(node: &Handle<dyn DeConfigurationNode>) -> Self {
        Self {
            base: DeProviderBase::with_node(node),
            ..Self::default()
        }
    }
}
impl DeProvider for DeObjProvider {
    fn provider_base(&self) -> &DeProviderBase {
        &self.base
    }
    fn get_format(&self) -> AsciiString {
        AsciiString::new("OBJ")
    }
    fn get_vendor(&self) -> AsciiString {
        AsciiString::new("Martensite")
    }
    fn read_document_with_session(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_document(path, document, progress)
    }
    fn write_document_with_session(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_document(path, document, progress)
    }
    fn read_document(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        provider_documents::read_file(self, path, document, progress)
    }
    fn write_document(
        &self,
        path: &AsciiString,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        provider_documents::write_file(self, path, document, progress)
    }
    fn read_shape_with_session(
        &self,
        path: &AsciiString,
        shape: &mut Shape,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_shape(path, shape, progress)
    }
    fn write_shape_with_session(
        &self,
        path: &AsciiString,
        shape: &Shape,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_shape(path, shape, progress)
    }
    fn read_shape(
        &self,
        path: &AsciiString,
        shape: &mut Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        provider_shapes::read_file(self, path, shape, progress)
    }
    fn write_shape(
        &self,
        path: &AsciiString,
        shape: &Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        provider_shapes::write_file(self, path, shape, progress)
    }
}
