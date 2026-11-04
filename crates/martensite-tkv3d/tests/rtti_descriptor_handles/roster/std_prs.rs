//! Explicit declaration owners; these entries create no class instances.
use super::super::DescriptorQuery;

pub(super) const QUERIES: &[DescriptorQuery] = &[
    DescriptorQuery::of::<martensite_tkv3d::std_prs::StdPrsBRepFont>(
        "StdPrsBRepFont",
        "Standard_Transient",
    ),
    DescriptorQuery::of::<martensite_tkv3d::std_prs::StdPrsBRepFontCache>(
        "StdPrsBRepFontCache",
        "Standard_Transient",
    ),
    DescriptorQuery::of::<martensite_tkv3d::std_prs::StdPrsHlrPolyShape>(
        "StdPrsHLRPolyShape",
        "StdPrsHLRShapeI",
    ),
    DescriptorQuery::of::<martensite_tkv3d::std_prs::StdPrsHlrShape>(
        "StdPrsHLRShape",
        "StdPrsHLRShapeI",
    ),
];
