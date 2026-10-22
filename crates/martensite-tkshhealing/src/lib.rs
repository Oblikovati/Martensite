// Shape healing algorithms for boundary representations.

//! the retained behavior TKShHealing: the Shape Healing algorithms of ModelingAlgorithms (analysis of
//! shapes, fixing of their problems, conversion and division of their geometry, and the
//! operator sequences that data exchange runs on imported shapes).
//!
//! TKShHealing depends on TKBRep, TKernel, TKMath, TKG2d, TKTopAlgo, TKG3d, TKGeomBase
//! and TKGeomAlgo. Each package is one module named after it in snake case (`ShapeExtend` is
//! `shape_extend`, `SHMessage` is `sh_message`, `ShapeProcessAPI` is
//! `shape_process_api`). Each package has one dedicated module with an explicit public boundary.
pub mod sh_message;
pub mod shape_algo;
pub mod shape_analysis;
pub mod shape_build;
pub mod shape_construct;
pub mod shape_custom;
pub mod shape_extend;
pub mod shape_fix;
pub mod shape_process;
pub mod shape_process_api;
pub mod shape_upgrade;
