//! The tool container of the checking boundary, and the holder of it.
use super::IgesControlIgesBoundary;
use crate::iges_to_brep::{IgesToBRepAlgoContainer, IgesToBRepBoundaryOps, IgesToBRepToolKind};
use martensite_tkernel::impl_transient;
use martensite_tkernel::standard::{Handle, StandardTransient};

/// Makes the boundary that checks its representations.
///
/// ```
/// use martensite_tkdeiges::iges_control::IgesControlToolContainer;
/// use martensite_tkdeiges::iges_to_brep::IgesToBRepToolKind;
/// assert!(IgesControlToolContainer::new().iges_boundary().is_some());
/// ```
#[derive(Default)]
pub struct IgesControlToolContainer {
    transient: StandardTransient,
}
impl_transient!(
    IgesControlToolContainer,
    "IgesControlToolContainer",
    dyn IgesToBRepToolKind,
    transient,
    interfaces: [dyn IgesToBRepToolKind]
);

impl IgesControlToolContainer {
    /// The container; `IgesControlToolContainer::new()`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl IgesToBRepToolKind for IgesControlToolContainer {
    fn iges_boundary(&self) -> Handle<dyn IgesToBRepBoundaryOps> {
        Handle::new(IgesControlIgesBoundary::new()).cast()
    }
}

/// A holder of the checking tools.
///
/// ```
/// use martensite_tkdeiges::iges_control::IgesControlAlgoContainer;
/// use martensite_tkernel::standard::RuntimeTyped;
/// assert!(IgesControlAlgoContainer::new_container().tool_container().is_ok());
/// assert_eq!(IgesControlAlgoContainer::TYPE_NAME, "IgesControlAlgoContainer");
/// ```
#[derive(Default)]
pub struct IgesControlAlgoContainer {
    transient: StandardTransient,
}
impl_transient!(
    IgesControlAlgoContainer,
    "IgesControlAlgoContainer",
    IgesToBRepAlgoContainer,
    transient
);

impl IgesControlAlgoContainer {
    /// The holder; `IgesControlAlgoContainer::new_container()`.
    pub fn new_container() -> IgesToBRepAlgoContainer {
        let container = IgesToBRepAlgoContainer::new();
        container.set_tool_container(&Handle::new(IgesControlToolContainer::new()).cast());
        container
    }
}
