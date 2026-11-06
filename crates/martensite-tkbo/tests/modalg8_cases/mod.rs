//! Original Boolean regression bodies shared by geometry and complete native-view hosts.
#![allow(dead_code)]
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools::{self, BRepToolsShapeSet},
    topexp,
    topods::{Shape, ShapeIterator},
    toptools::{IndexedMapOfShape, ShapeSet},
};
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::List,
    precision,
    storage::{IoStream, format_general},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};
mod assertions;
#[path = "../common_operation/mod.rs"]
mod common_operation;
use assertions::*;
mod cone_prism;
pub use cone_prism::checked_cone_prism;
mod common_solids;
pub use common_solids::{checked_32942, checked_32943, checked_32944, checked_32945};
