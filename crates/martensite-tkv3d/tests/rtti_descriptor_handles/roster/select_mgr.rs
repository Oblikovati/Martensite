//! Explicit declaration owners; these entries create no class instances.
use super::super::DescriptorQuery;

pub(super) const QUERIES: &[DescriptorQuery] = &[
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrAndFilter>(
        "SelectMgrAndFilter",
        "SelectMgrCompositionFilter",
    ),
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrAndOrFilter>(
        "SelectMgrAndOrFilter",
        "SelectMgrCompositionFilter",
    ),
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrBvhThreadPool>(
        "SelectMgrBVHThreadPool",
        "Standard_Transient",
    ),
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrEntityOwner>(
        "SelectMgrEntityOwner",
        "Standard_Transient",
    ),
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrFrustumBuilder>(
        "SelectMgrFrustumBuilder",
        "Standard_Transient",
    ),
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrOrFilter>(
        "SelectMgrOrFilter",
        "SelectMgrCompositionFilter",
    ),
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrSelection>(
        "SelectMgrSelection",
        "Standard_Transient",
    ),
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrSelectionManager>(
        "SelectMgrSelectionManager",
        "Standard_Transient",
    ),
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrSensitiveEntity>(
        "SelectMgrSensitiveEntity",
        "Standard_Transient",
    ),
    DescriptorQuery::of::<martensite_tkv3d::select_mgr::SelectMgrViewerSelector>(
        "SelectMgrViewerSelector",
        "Standard_Transient",
    ),
];
