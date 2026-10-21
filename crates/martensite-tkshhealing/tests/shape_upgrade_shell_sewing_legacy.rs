//! Original ShapeUpgrade_ShellSewing_Test.cxx declarations at the retained behavior3d097a0.
#![allow(non_snake_case)]
mod healing_support;
use healing_support::{compound, shell};
#[path = "shell_sewing_legacy/geometry.rs"]
mod sewing_geometry;
use martensite_tkbrep::{topexp::Explorer, topods::Shape, toptools::MapOfShape};
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkgeombase::gprop::GProps;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeShellSewing;
use martensite_tktopalgo::{
    brep_check::BRepCheckAnalyzer, brep_class3d::BRepClass3dSolidClassifier, brep_gprop::BRepGProp,
};
use sewing_geometry::{box_shape, open_shell, rectangle};
include!("shell_sewing_legacy/support.rs");
include!("shell_sewing_legacy/basic.rs");
include!("shell_sewing_legacy/solids.rs");
include!("shell_sewing_legacy/state.rs");
