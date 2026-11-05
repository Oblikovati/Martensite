//! `DeStlProvider`: the sixteen transfers of the provider interface.
use super::{provider_documents, provider_shapes};
use martensite_tkbrep::topods::Shape;
use martensite_tkde::de::{
    DeConfigurationNode, DeProvider, DeProviderBase, DeProviderReadStreamList,
    DeProviderWriteStreamList, DeWorkSession,
};
use martensite_tkernel::{
    impl_transient,
    message::ProgressRange,
    standard::{Failure, Handle, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tklcaf::tdoc_std::TDocStdDocument;

/// `DeStlProvider`: transfers STL files and streams with the settings of its node.
/// A work session is accepted and ignored by every overload.
///
/// ```
/// use martensite_tkde::de::{DeConfigurationNode, DeProvider, new_configuration_node};
/// use martensite_tkdestl::de_stl::{DeStlConfigurationNode, DeStlProvider};
/// let node = new_configuration_node(DeStlConfigurationNode::new());
/// let provider = DeStlProvider::with_node(&node.cast::<dyn DeConfigurationNode>());
/// assert_eq!(provider.get_format().as_bytes(), b"STL");
/// ```
#[derive(Default)]
pub struct DeStlProvider {
    transient: StandardTransient,
    base: DeProviderBase,
}
impl_transient!(DeStlProvider, "DeStlProvider", dyn DeProvider, transient, interfaces: [dyn DeProvider]);

impl DeStlProvider {
    /// A provider on a node, keeping that node rather than a copy;
    /// `DeStlProvider::with_node(&node)`.
    pub fn with_node(node: &Handle<dyn DeConfigurationNode>) -> Self {
        Self {
            base: DeProviderBase::with_node(node),
            ..Self::default()
        }
    }
}
impl DeProvider for DeStlProvider {
    fn provider_base(&self) -> &DeProviderBase {
        &self.base
    }
    fn get_format(&self) -> AsciiString {
        AsciiString::new("STL")
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
    fn read_document_streams_with_session(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_document_streams(streams, document, progress)
    }
    fn write_document_streams_with_session(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_document_streams(streams, document, progress)
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
    fn read_document_streams(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        provider_documents::read_streams(self, streams, document, progress)
    }
    fn write_document_streams(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        document: &Handle<TDocStdDocument>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        provider_documents::write_streams(self, streams, document, progress)
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
    fn read_shape_streams_with_session(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        shape: &mut Shape,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.read_shape_streams(streams, shape, progress)
    }
    fn write_shape_streams_with_session(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        shape: &Shape,
        _work_session: &mut DeWorkSession,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.write_shape_streams(streams, shape, progress)
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
    fn read_shape_streams(
        &self,
        streams: &mut DeProviderReadStreamList<'_>,
        shape: &mut Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        provider_shapes::read_streams(self, streams, shape, progress)
    }
    fn write_shape_streams(
        &self,
        streams: &mut DeProviderWriteStreamList<'_>,
        shape: &Shape,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        provider_shapes::write_streams(self, streams, shape, progress)
    }
}
