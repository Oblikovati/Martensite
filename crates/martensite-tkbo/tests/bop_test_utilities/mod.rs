//! Boolean fixture operations reuse lower-layer shape construction and measurements.
#![allow(dead_code, non_snake_case)]
use martensite_tkbo::bop_algo::{BOPAlgoBOP, BOPAlgoPaveFiller, Operation};
use martensite_tkbo::brep_algo_api::{BRepAlgoAPICommon, BRepAlgoAPICut, BRepAlgoAPIFuse};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::List;
#[allow(unused_imports)] // Each retained test module uses its original fixture subset.
pub use martensite_topology_test_support::*;
mod fixtures;
#[allow(unused_imports)] // Operation wrappers are shared by tests with different needs.
pub use fixtures::*;
