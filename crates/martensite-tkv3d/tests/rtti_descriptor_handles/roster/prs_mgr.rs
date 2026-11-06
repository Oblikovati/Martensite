//! Explicit declaration owners; these entries create no class instances.
use super::super::DescriptorQuery;

pub(super) const QUERIES: &[DescriptorQuery] = &[
    DescriptorQuery::of::<martensite_tkv3d::prs_mgr::PrsMgrPresentation>(
        "PrsMgrPresentation",
        "Graphic3d_Structure",
    ),
    DescriptorQuery::of::<martensite_tkv3d::prs_mgr::PrsMgrPresentationManager>(
        "PrsMgrPresentationManager",
        "Standard_Transient",
    ),
];
