//! The containers of the tools a transfer of IGES entities uses.
use super::{IgesToBRepBoundaryOps, IgesToBRepIgesBoundary};
use martensite_tkernel::impl_transient;
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::standard::{Handle, SharedState, StandardTransient, Transient};

/// What a tool container makes: the kind of boundary of a transfer.
pub trait IgesToBRepToolKind: Transient {
    /// A new boundary of this kind.
    fn iges_boundary(&self) -> Handle<dyn IgesToBRepBoundaryOps>;
}
impl_transient_interface!(
    dyn IgesToBRepToolKind,
    "IgesToBRepToolKind",
    StandardTransient
);

/// The plain tools.
///
/// ```
/// use martensite_tkdeiges::iges_to_brep::{IgesToBRepToolContainer, IgesToBRepToolKind};
/// assert!(IgesToBRepToolContainer::new().iges_boundary().is_some());
/// ```
#[derive(Default)]
pub struct IgesToBRepToolContainer {
    transient: StandardTransient,
}
impl_transient!(
    IgesToBRepToolContainer,
    "IgesToBRepToolContainer",
    dyn IgesToBRepToolKind,
    transient,
    interfaces: [dyn IgesToBRepToolKind]
);

impl IgesToBRepToolContainer {
    /// The container; `IgesToBRepToolContainer::new()`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl IgesToBRepToolKind for IgesToBRepToolContainer {
    fn iges_boundary(&self) -> Handle<dyn IgesToBRepBoundaryOps> {
        Handle::new(IgesToBRepIgesBoundary::new()).cast()
    }
}

/// The holder of a tool container.
///
/// ```
/// use martensite_tkdeiges::iges_to_brep::IgesToBRepAlgoContainer;
/// assert!(IgesToBRepAlgoContainer::new().tool_container().is_ok());
/// ```
pub struct IgesToBRepAlgoContainer {
    transient: StandardTransient,
    tools: SharedState<Handle<dyn IgesToBRepToolKind>>,
}
impl_transient!(
    IgesToBRepAlgoContainer,
    "IgesToBRepAlgoContainer",
    StandardTransient,
    transient
);

impl Default for IgesToBRepAlgoContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesToBRepAlgoContainer {
    /// A holder of the plain tools; `IgesToBRepAlgoContainer::new()`.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            tools: SharedState::new(Handle::new(IgesToBRepToolContainer::new()).cast()),
        }
    }
    /// Sets the tool container; `algo.set_tool_container(&tools)`.
    pub fn set_tool_container(&self, tools: &Handle<dyn IgesToBRepToolKind>) {
        *self.tools.write() = tools.clone();
    }
    /// The tool container, for the boundary of a transfer;
    /// `algo.tool_container()?.iges_boundary()`.
    ///
    /// # Errors
    /// A null object failure for a container set null.
    pub fn tool_container(
        &self,
    ) -> Result<Handle<dyn IgesToBRepToolKind>, martensite_tkernel::standard::Failure> {
        Ok(self.tools.read().clone())
    }
}
