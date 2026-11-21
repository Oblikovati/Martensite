//! Private grid and trihedron classes override Compute and inherit the native structure descriptor.
use super::V3dComputedStructure;
use crate::inherited_native_type::inherited_native_type;
use martensite_tkservice::graphic3d::Graphic3dStructureSource;

inherited_native_type!(
    V3dComputedStructure,
    dyn Graphic3dStructureSource,
    "Martensite computed structure Rust casters",
    inherited,
    [dyn Graphic3dStructureSource]
);
