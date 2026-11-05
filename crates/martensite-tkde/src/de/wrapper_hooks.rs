//! Owned callbacks for registry lookup and copying; defaults remain explicit so an
//! override can delegate to the base implementation without recursively calling itself.
use super::{DeProvider, DeWrapper};
use martensite_tkernel::{
    osd::IStream,
    standard::{Failure, Handle},
    tcollection::AsciiString,
};

/// Customize the five virtual lookup/copy signatures of a registry.
///
/// ```
/// use martensite_tkde::de::{DeWrapper, DeWrapperHooks};
/// struct CustomRegistry;
/// impl DeWrapperHooks for CustomRegistry {}
/// let registry = DeWrapper::with_hooks(std::sync::Arc::new(CustomRegistry));
/// assert_eq!(registry.nodes().extent(), 0);
/// ```
pub trait DeWrapperHooks: Send + Sync {
    /// Selects by direction. Example: `hooks.find_provider(&wrapper, &path, true, &mut provider)`.
    fn find_provider(
        &self,
        wrapper: &DeWrapper,
        path: &AsciiString,
        to_import: bool,
        provider: &mut Handle<dyn DeProvider>,
    ) -> Result<bool, Failure> {
        wrapper.default_find_provider(path, to_import, provider)
    }
    /// Selects an input file. Example: `hooks.find_read_provider(&wrapper, &path, false, &mut provider)`.
    fn find_read_provider(
        &self,
        wrapper: &DeWrapper,
        path: &AsciiString,
        check_content: bool,
        provider: &mut Handle<dyn DeProvider>,
    ) -> Result<bool, Failure> {
        wrapper.default_find_read_provider(path, check_content, provider)
    }
    /// Selects an input stream. Example: `hooks.find_read_provider_from_stream(&wrapper, &path, &mut stream, &mut provider)`.
    fn find_read_provider_from_stream(
        &self,
        wrapper: &DeWrapper,
        path: &AsciiString,
        stream: &mut dyn IStream,
        provider: &mut Handle<dyn DeProvider>,
    ) -> Result<bool, Failure> {
        wrapper.default_find_read_provider_from_stream(path, stream, provider)
    }
    /// Selects an output file. Example: `hooks.find_write_provider(&wrapper, &path, &mut provider)`.
    fn find_write_provider(
        &self,
        wrapper: &DeWrapper,
        path: &AsciiString,
        provider: &mut Handle<dyn DeProvider>,
    ) -> bool {
        wrapper.default_find_write_provider(path, provider)
    }
    /// Copies or replaces the registry. Example: `let copy = hooks.copy(&wrapper);`.
    fn copy(&self, wrapper: &DeWrapper) -> Handle<DeWrapper> {
        wrapper.default_copy()
    }
}

/// Base registry behavior without application overrides.
/// Example: `DeWrapper::with_hooks(std::sync::Arc::new(DeDefaultWrapperHooks))`.
#[derive(Default)]
pub struct DeDefaultWrapperHooks;
impl DeWrapperHooks for DeDefaultWrapperHooks {}
