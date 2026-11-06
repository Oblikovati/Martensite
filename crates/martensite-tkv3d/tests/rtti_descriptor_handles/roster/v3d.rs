//! Explicit declaration owners; these entries create no class instances.
use super::super::DescriptorQuery;

pub(super) const QUERIES: &[DescriptorQuery] = &[
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dAmbientLight>(
        "V3dAmbientLight",
        "Graphic3d_CLight",
    ),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dCircularGrid>(
        "V3dCircularGrid",
        "Aspect_CircularGrid",
    ),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dDirectionalLight>(
        "V3dDirectionalLight",
        "V3dPositionLight",
    ),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dPlane>("V3dPlane", "Standard_Transient"),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dPositionLight>(
        "V3dPositionLight",
        "Graphic3d_CLight",
    ),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dPositionalLight>(
        "V3dPositionalLight",
        "V3dPositionLight",
    ),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dRectangularGrid>(
        "V3dRectangularGrid",
        "Aspect_RectangularGrid",
    ),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dSpotLight>("V3dSpotLight", "V3dPositionLight"),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dTrihedron>(
        "V3dTrihedron",
        "Standard_Transient",
    ),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dView>("V3dView", "Standard_Transient"),
    DescriptorQuery::of::<martensite_tkv3d::v3d::V3dViewer>("V3dViewer", "Standard_Transient"),
];
