//! Explicit declaration owners; these entries create no class instances.
use super::super::DescriptorQuery;

pub(super) const QUERIES: &[DescriptorQuery] = &[
    DescriptorQuery::of::<martensite_tkv3d::std_select::StdSelectBRepOwner>(
        "StdSelectBRepOwner",
        "SelectMgrEntityOwner",
    ),
    DescriptorQuery::of::<martensite_tkv3d::std_select::StdSelectEdgeFilter>(
        "StdSelectEdgeFilter",
        "SelectMgrFilter",
    ),
    DescriptorQuery::of::<martensite_tkv3d::std_select::StdSelectFaceFilter>(
        "StdSelectFaceFilter",
        "SelectMgrFilter",
    ),
    DescriptorQuery::of::<martensite_tkv3d::std_select::StdSelectShape>(
        "StdSelectShape",
        "PrsMgrPresentableObject",
    ),
    DescriptorQuery::of::<martensite_tkv3d::std_select::StdSelectShapeTypeFilter>(
        "StdSelectShapeTypeFilter",
        "SelectMgrFilter",
    ),
];
