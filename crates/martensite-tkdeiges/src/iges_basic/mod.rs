//! Basic IGES entities: groups, names, references, subfigures, and hierarchies.
#[cfg(test)]
mod behaviour_tests;
mod entities_01;
mod entities_02;
pub mod entity_shape;
mod group_counted;
pub(crate) mod nested_arrays;
#[cfg(test)]
pub(crate) mod samples_01;
pub(crate) mod shared_runs;
#[cfg(test)]
mod tests_01;
mod tools_01;
mod tools_02;
mod tools_03;
mod tools_04;
mod tools_05;
mod tools_06;
pub use entities_01::{
    IgesBasicAssocGroupType, IgesBasicExternalRefFile, IgesBasicExternalRefFileIndex,
    IgesBasicExternalRefFileName, IgesBasicExternalRefLibName, IgesBasicExternalRefName,
    IgesBasicExternalReferenceFile, IgesBasicGroup, IgesBasicGroupWithoutBackP, IgesBasicHierarchy,
    IgesBasicName,
};
pub use entities_02::{
    IgesBasicOrderedGroup, IgesBasicOrderedGroupWithoutBackP, IgesBasicSingleParent,
    IgesBasicSingularSubfigure, IgesBasicSubfigureDef,
};
pub use nested_arrays::{
    IgesBasicHArray1OfHArray1OfIgesEntity, IgesBasicHArray1OfHArray1OfInteger,
    IgesBasicHArray1OfHArray1OfReal, IgesBasicHArray1OfHArray1OfXy, IgesBasicHArray1OfHArray1OfXyz,
};
pub use tools_01::{
    IgesBasicToolAssocGroupType, IgesBasicToolExternalRefFile, IgesBasicToolExternalRefFileIndex,
};
pub use tools_02::{
    IgesBasicToolExternalRefFileName, IgesBasicToolExternalRefLibName,
    IgesBasicToolExternalRefName, IgesBasicToolExternalReferenceFile,
};
pub use tools_03::{IgesBasicToolGroup, IgesBasicToolGroupWithoutBackP};
pub use tools_04::{IgesBasicToolHierarchy, IgesBasicToolName, IgesBasicToolOrderedGroup};
pub use tools_05::{IgesBasicToolOrderedGroupWithoutBackP, IgesBasicToolSingleParent};
pub use tools_06::{IgesBasicToolSingularSubfigure, IgesBasicToolSubfigureDef};
mod modules;
pub use modules::{
    IgesBasic, IgesBasicGeneralModule, IgesBasicProtocol, IgesBasicReadWriteModule,
    IgesBasicSpecificModule,
};
